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

use std::path::PathBuf;

use clap::Args;
use consortium_builder::pipeline::{
    self, PipelineConfig, bitbake::BitbakeAction, tee::TeeToolchain,
};

use crate::reporter::StickyReporter;

#[derive(Args)]
pub struct BuildArgs {
    /// Path to the `Consortium.toml` manifest.
    #[arg(long, default_value = "Consortium.toml")]
    manifest: PathBuf,

    /// Directory to assemble the `dist/` staging tree into.
    #[arg(long, default_value = "dist")]
    dist: PathBuf,

    /// Linux kernel source checkout providing the selected BSP DTS/DTSI files,
    /// their include closure, and dt-bindings headers.
    #[arg(long, value_name = "DIR")]
    linux: PathBuf,

    /// Build in release mode.
    #[arg(long)]
    release: bool,

    /// Build without network access: every cargo invocation runs `--offline`,
    /// resolving only what is already in the local registry cache. Populate that
    /// cache first with `csti fetch`.
    #[arg(long)]
    offline: bool,

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

    /// Resolve and report the bitbake command without running it. Use when the
    /// Consortium build and the Yocto/bitbake build run on separate machines:
    /// `dist/` is assembled here and `bitbake` is run later on the Yocto host.
    /// Requires `--bitbake-recipe` (+ `--bitbake-build-dir`).
    #[arg(long)]
    bitbake_dry_run: bool,
}

/// Resolves the ordered bitbake hand-off actions from the CLI flags.
///
/// Staging (`--bitbake-files-dir`) and invoking (`--bitbake-recipe` +
/// `--bitbake-build-dir`) compose: when both are given, `dist/` is staged into
/// the recipe's `files/` dir first, then bitbake runs (or, with
/// `--bitbake-dry-run`, the command is recorded but not run). This is the
/// split-machine flow — produce the recipe input here, defer the build.
fn resolve_bitbake(args: &BuildArgs) -> Result<Vec<BitbakeAction>, String> {
    let mut actions = Vec::new();

    if let Some(dir) = &args.bitbake_files_dir {
        actions.push(BitbakeAction::CopyIntoRecipe {
            recipe_files_dir: dir.clone(),
        });
    }

    match (&args.bitbake_recipe, &args.bitbake_build_dir) {
        (Some(recipe), Some(build_dir)) => actions.push(BitbakeAction::InvokeBitbake {
            build_dir: build_dir.clone(),
            recipe: recipe.clone(),
            dry_run: args.bitbake_dry_run,
        }),
        (Some(_), None) => return Err("--bitbake-recipe requires --bitbake-build-dir".to_string()),
        (None, Some(_)) => return Err("--bitbake-build-dir requires --bitbake-recipe".to_string()),
        (None, None) if args.bitbake_dry_run => {
            return Err("--bitbake-dry-run requires --bitbake-recipe".to_string());
        }
        (None, None) => {}
    }

    Ok(actions)
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
        linux: args.linux.clone(),
        release: args.release,
        offline: args.offline,
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
    if args.offline {
        println!("  cargo:    --offline (local registry cache only)");
    }

    let reporter = StickyReporter::new();
    let dist = match pipeline::run_with_reporter(config, &reporter).await {
        Ok(dist) => {
            reporter.finish_success("Build complete");
            dist
        }
        Err(error) => {
            reporter.finish_failure("Build failed");
            return Err(error.to_string());
        }
    };
    println!("Assembled dist tree at {}", dist.display());
    Ok(())
}
