// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! The `build` subcommand: the end-to-end `Consortium.toml` build pipeline.

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use clap::Args;
use consortium_builder::pipeline::{
    self, PipelineConfig, bitbake::BitbakeAction, tee::TeeToolchain,
};
use consortium_builder::{BuildReporter, BuildStep};

#[derive(Args)]
pub struct BuildArgs {
    /// Path to the `Consortium.toml` manifest.
    #[arg(long, default_value = "Consortium.toml")]
    manifest: PathBuf,

    /// Directory to assemble the `dist/` staging tree into.
    #[arg(long, default_value = "dist")]
    dist: PathBuf,

    /// Build in release mode.
    #[arg(long)]
    release: bool,

    /// Build the OP-TEE TA but skip signing it into a `<uuid>.ta`. The unsigned
    /// ELF is staged instead, so no signer script or signing keys are needed.
    #[arg(long)]
    skip_tee_sign: bool,

    /// Stage the assembled `dist/` tree under a bitbake recipe's `files/` dir.
    #[arg(long, value_name = "DIR")]
    bitbake_files_dir: Option<PathBuf>,

    /// Invoke bitbake on this recipe (requires `--bitbake-build-dir`).
    #[arg(long, value_name = "RECIPE")]
    bitbake_recipe: Option<String>,

    /// The Yocto build directory `bitbake` is invoked in.
    #[arg(long, value_name = "DIR")]
    bitbake_build_dir: Option<PathBuf>,
}

/// Resolves the optional bitbake hand-off from the CLI flags.
fn resolve_bitbake(args: &BuildArgs) -> Result<Option<BitbakeAction>, String> {
    match (
        &args.bitbake_files_dir,
        &args.bitbake_recipe,
        &args.bitbake_build_dir,
    ) {
        (None, None, None) => Ok(None),
        (Some(dir), None, None) => Ok(Some(BitbakeAction::CopyIntoRecipe {
            recipe_files_dir: dir.clone(),
        })),
        (None, Some(recipe), Some(build_dir)) => Ok(Some(BitbakeAction::InvokeBitbake {
            build_dir: build_dir.clone(),
            recipe: recipe.clone(),
        })),
        (None, Some(_), None) => Err("--bitbake-recipe requires --bitbake-build-dir".to_string()),
        (None, None, Some(_)) => Err("--bitbake-build-dir requires --bitbake-recipe".to_string()),
        _ => Err(
            "choose either --bitbake-files-dir or (--bitbake-recipe + --bitbake-build-dir), \
             not both"
                .to_string(),
        ),
    }
}

#[derive(Clone, Copy)]
enum StepStatus {
    Running,
    Done,
    Failed,
}

struct CurrentStep {
    index: usize,
    total: usize,
    name: String,
    detail: Option<String>,
    status: StepStatus,
}

struct ReporterState {
    current: Option<CurrentStep>,
    pending_output: String,
}

struct StickyReporter {
    state: Mutex<ReporterState>,
    interactive: bool,
    color: bool,
}

impl StickyReporter {
    fn new() -> Self {
        let interactive = io::stderr().is_terminal();
        let color = interactive
            && std::env::var_os("NO_COLOR").is_none()
            && std::env::var("TERM")
                .map(|term| term != "dumb")
                .unwrap_or(true);
        Self {
            state: Mutex::new(ReporterState {
                current: None,
                pending_output: String::new(),
            }),
            interactive,
            color,
        }
    }

    fn finish_success(&self) {
        self.finish("ok", "Build complete", "32;1");
    }

    fn finish_failure(&self) {
        self.finish("x", "Build failed", "31;1");
    }

    fn finish(&self, mark: &str, message: &str, color: &str) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        self.flush_pending(&mut state, &mut stderr);
        state.current = None;
        let _ = writeln!(stderr, "{} {}", self.paint(color, mark), message);
        let _ = stderr.flush();
    }

    fn update_step(&self, step: BuildStep<'_>, status: StepStatus) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        self.flush_pending(&mut state, &mut stderr);

        state.current = Some(CurrentStep {
            index: step.index,
            total: step.total,
            name: step.name.to_string(),
            detail: step.detail.map(str::to_string),
            status,
        });

        if let Some(current) = state.current.as_ref() {
            let prompt = self.render_prompt(current);
            if self.interactive {
                let _ = write!(stderr, "{prompt}");
            } else {
                let _ = writeln!(stderr, "{prompt}");
            }
        }
        let _ = stderr.flush();
    }

    fn flush_pending(&self, state: &mut ReporterState, stderr: &mut impl Write) {
        if state.pending_output.is_empty() {
            return;
        }
        let pending = std::mem::take(&mut state.pending_output);
        let _ = write!(stderr, "{pending}");
        if !pending.ends_with('\n') && !pending.ends_with('\r') {
            let _ = writeln!(stderr);
        }
    }

    fn drain_complete_output(&self, state: &mut ReporterState) -> String {
        let Some(end) = state
            .pending_output
            .char_indices()
            .filter_map(|(idx, ch)| (ch == '\n' || ch == '\r').then_some(idx + ch.len_utf8()))
            .last()
        else {
            return String::new();
        };

        let complete = state.pending_output[..end].to_string();
        state.pending_output.replace_range(..end, "");
        normalize_line_endings(&complete)
    }

    fn render_prompt(&self, current: &CurrentStep) -> String {
        let (status, color) = match current.status {
            StepStatus::Running => ("running", "36;1"),
            StepStatus::Done => ("done", "32;1"),
            StepStatus::Failed => ("failed", "31;1"),
        };
        let mut prompt = format!(
            "{} {} {} {}",
            self.paint("36;1", "> consortium"),
            self.paint("33", format!("[{}/{}]", current.index, current.total)),
            self.paint(color, status),
            self.paint("1", &current.name),
        );
        if let Some(detail) = &current.detail {
            prompt.push(' ');
            prompt.push_str(&self.paint("2", detail));
        }
        prompt
    }

    fn paint(&self, color: &str, text: impl AsRef<str>) -> String {
        let text = text.as_ref();
        if self.color {
            format!("\x1b[{color}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }
}

impl BuildReporter for StickyReporter {
    fn step_started(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Running);
    }

    fn step_succeeded(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Done);
    }

    fn step_failed(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Failed);
    }

    fn command_output(&self, output: &str) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        state.pending_output.push_str(output);
        let complete = self.drain_complete_output(&mut state);
        if complete.is_empty() {
            return;
        }

        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        let _ = write!(stderr, "{complete}");
        if self.interactive {
            if let Some(current) = state.current.as_ref() {
                let _ = write!(stderr, "\r\x1b[2K{}", self.render_prompt(current));
            }
        }
        let _ = stderr.flush();
    }
}

fn normalize_line_endings(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut previous_was_cr = false;
    for ch in text.chars() {
        match ch {
            '\r' => {
                normalized.push('\n');
                previous_was_cr = true;
            }
            '\n' if previous_was_cr => {
                previous_was_cr = false;
            }
            _ => {
                normalized.push(ch);
                previous_was_cr = false;
            }
        }
    }
    normalized
}

/// Runs the end-to-end build pipeline for the given `Consortium.toml` manifest.
pub async fn run(args: BuildArgs) -> Result<(), String> {
    let session = pipeline::load_session(&args.manifest)
        .await
        .map_err(|e| e.to_string())?;
    let manifest_dir = args
        .manifest
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let hmi =
        pipeline::derive_hmi_configuration(&session, &manifest_dir).map_err(|e| e.to_string())?;
    let tee_toolchain = TeeToolchain::from_env();
    let bitbake = resolve_bitbake(&args)?;

    let config = PipelineConfig {
        manifest_path: args.manifest.clone(),
        dist_root: args.dist.clone(),
        release: args.release,
        hmi,
        tee_toolchain,
        sign_tee: !args.skip_tee_sign,
        bitbake,
    };

    // Print a plan summary so the user sees what will be built before the (long)
    // cross-compilation begins.
    let plan = pipeline::plan(&session, &config)
        .await
        .map_err(|e| e.to_string())?;
    println!(
        "Building {} for chip {}",
        args.manifest.display(),
        session.database.name
    );
    for fw in &plan.firmware {
        println!("  firmware: {} ({})", fw.core, fw.config.target);
    }
    if plan.hmi.is_some() {
        let source = session
            .config
            .hmi
            .as_ref()
            .map(|h| h.source.as_str())
            .unwrap_or("-");
        println!("  hmi:      {source}");
    }
    if plan.ta.is_some() {
        let signed = if args.skip_tee_sign {
            " (unsigned: --skip-tee-sign)"
        } else {
            ""
        };
        println!("  tee:      trusted application{signed}");
    }
    println!("  app:      {}", plan.application.config.target);

    let reporter = StickyReporter::new();
    let dist = match pipeline::run_with_reporter(config, &reporter).await {
        Ok(dist) => {
            reporter.finish_success();
            dist
        }
        Err(error) => {
            reporter.finish_failure();
            return Err(error.to_string());
        }
    };
    println!("Assembled dist tree at {}", dist.display());
    Ok(())
}
