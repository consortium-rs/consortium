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

//! The pre-build download stage: everything [`run`](super::run) would reach the
//! network for, done up front.
//!
//! A Consortium build spans several independent Cargo projects — one firmware
//! crate per remote coprocessor, the TA, the Linux application — plus a frontend
//! project with its own package manager. Each of those pulls from a different
//! place, which is what makes an air-gapped or reproducible build awkward: there
//! is no single `cargo fetch` that covers the system.
//!
//! This stage resolves the same project set the build plan does and fetches each
//! one, so a later `build --offline` finds every input in the local caches:
//!
//! 1. `rustup target add` for each distinct triple (a cross build needs the
//!    target's prebuilt `std`, which `cargo fetch` does not provide);
//! 2. `cargo fetch --target <triple>` per Cargo project; and
//! 3. the frontend's package install (`vp`/`npm`/`bun install`).
//!
//! Deliberately *not* resolved here: the OP-TEE dev kit, a Yocto sysroot, kernel
//! sources for `dtc`, and BitBake's own downloads. Those are host provisioning
//! rather than project dependencies, and each has its own vendor-specific
//! acquisition path.
//!
//! Unlike [`plan`](super::plan) this needs no code generation and no OP-TEE
//! toolchain: fetching reads Cargo manifests only, so it works on a machine that
//! could not yet complete a build.

use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg_common::config::Party;
use tokio::process::Command;

use super::application::acore_linux_triple;
use super::firmware::remote_coprocessors;
use super::{PipelineError, core_rust_triple, visible_core_of};
use crate::compile::optee::OPTEE_TARGET;
use crate::compile::utils::run_command;
use crate::compile::web::HmiBuildCommand;
use crate::progress::BuildReporter;

/// One Cargo project to fetch dependencies for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CargoFetch {
    /// What this project builds, for progress output (a core name, `ta`, `app`).
    pub label: String,
    /// The Cargo project root.
    pub path: PathBuf,
    /// The Rust target triple the fetch is scoped to.
    pub target: String,
}

/// The HMI frontend's dependency install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HmiFetch {
    /// The frontend project root (the manifest's `[hmi].source`, absolute).
    pub project_dir: PathBuf,
    /// The package manager command that installs its dependencies.
    pub command: HmiBuildCommand,
}

/// Every download a build of this manifest needs, resolved without running any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FetchPlan {
    /// One entry per Cargo project, in build order.
    pub cargo: Vec<CargoFetch>,
    /// The frontend install, when the manifest declares `[hmi]` and it was not
    /// skipped by the caller.
    pub hmi: Option<HmiFetch>,
}

impl FetchPlan {
    /// The distinct target triples the build compiles for, in first-use order.
    ///
    /// Deduplicated because several projects can share a triple — the TA and the
    /// Linux application both target `aarch64-unknown-linux-gnu` — and installing
    /// one target's `std` twice is pure latency.
    pub fn rust_targets(&self) -> Vec<String> {
        let mut targets: Vec<String> = Vec::new();
        for entry in &self.cargo {
            if !targets.contains(&entry.target) {
                targets.push(entry.target.clone());
            }
        }
        targets
    }
}

/// All parameters for a fetch run.
pub struct FetchConfig {
    /// Path to the `Consortium.toml` manifest.
    pub manifest_path: PathBuf,
    /// The frontend install, or `None` to skip it. Derive with
    /// [`derive_hmi_fetch`].
    pub hmi: Option<HmiFetch>,
    /// Install each target's Rust standard library with `rustup target add`.
    /// A host that manages its own toolchain (no rustup, a vendored `std`) can
    /// turn this off; a missing `rustup` is a skipped step, not a failure.
    pub rust_targets: bool,
}

/// Resolves every Cargo project a build of `session` would compile.
///
/// Mirrors the build plan's project set — one firmware crate per remote
/// coprocessor with an endpoint, the TA when `[tee]` is declared, and the Linux
/// application — but resolves only paths and triples.
pub fn plan_fetch(
    session: &Session,
    manifest_dir: &Path,
    hmi: Option<HmiFetch>,
) -> Result<FetchPlan, PipelineError> {
    let mut cargo = Vec::new();

    // Firmware, one crate per remote coprocessor that declares an endpoint.
    for cp in remote_coprocessors(&session.database) {
        let core = visible_core_of(cp)?;
        let Some((_, root)) = session
            .config
            .endpoints
            .iter()
            .find(|(party, _)| party.core() == core)
        else {
            continue;
        };
        cargo.push(CargoFetch {
            label: core.to_string(),
            path: manifest_dir.join(root),
            target: core_rust_triple(core),
        });
    }

    // The TA. Unlike `tee::plan_ta_build` this needs no `TeeToolchain`: the dev
    // kit is a build and signing input, not a dependency-resolution one, so a
    // host can populate the cache before it has an OP-TEE SDK.
    if session.config.tee.is_some() {
        let root = session
            .config
            .endpoints
            .get(&Party::AcoreSecure)
            .ok_or_else(|| PipelineError::MissingEndpoint(Party::AcoreSecure.to_string()))?;
        cargo.push(CargoFetch {
            label: "ta".to_string(),
            path: manifest_dir.join(root),
            target: OPTEE_TARGET.to_string(),
        });
    }

    // The Linux application.
    let root = session
        .config
        .endpoints
        .get(&Party::AcoreNonSecure)
        .ok_or_else(|| PipelineError::MissingEndpoint(Party::AcoreNonSecure.to_string()))?;
    cargo.push(CargoFetch {
        label: "app".to_string(),
        path: manifest_dir.join(root),
        target: acore_linux_triple(&session.database.ap)?,
    });

    Ok(FetchPlan { cargo, hmi })
}

/// Derives the HMI dependency install from the manifest's `[hmi]` section.
///
/// Returns `None` when the manifest declares no frontend. The installer is
/// resolved per engine (`vp`/`npm install` for Cog, `bun install` for Pocket) and
/// is overridable with `CONSORTIUM_HMI_FETCH_CMD`.
pub fn derive_hmi_fetch(
    session: &Session,
    manifest_dir: &Path,
) -> Result<Option<HmiFetch>, PipelineError> {
    use consortium_cfg_common::config::HmiEngine;

    let Some(hmi) = &session.config.hmi else {
        return Ok(None);
    };
    hmi.validate().map_err(PipelineError::Codegen)?;

    let command = match hmi.engine {
        HmiEngine::Cog => crate::compile::utils::resolve_hmi_fetch_command(),
        HmiEngine::Pocket => crate::compile::utils::resolve_pocket_fetch_command(),
    }
    .map_err(PipelineError::Codegen)?;

    Ok(Some(HmiFetch {
        project_dir: manifest_dir.join(&hmi.source),
        command,
    }))
}

/// Loads the manifest, plans the fetch, and runs it.
pub async fn run(config: FetchConfig) -> Result<FetchPlan, PipelineError> {
    run_inner(config, None).await
}

/// Runs the fetch while reporting step progress and subprocess output.
pub async fn run_with_reporter(
    config: FetchConfig,
    reporter: &dyn BuildReporter,
) -> Result<FetchPlan, PipelineError> {
    run_inner(config, Some(reporter)).await
}

async fn run_inner(
    config: FetchConfig,
    reporter: Option<&dyn BuildReporter>,
) -> Result<FetchPlan, PipelineError> {
    let session = super::load_session(&config.manifest_path).await?;
    let manifest_dir = config
        .manifest_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let plan = plan_fetch(&session, &manifest_dir, config.hmi)?;
    let targets = plan.rust_targets();
    let install_targets = config.rust_targets && !targets.is_empty();

    let total = usize::from(install_targets) + plan.cargo.len() + usize::from(plan.hmi.is_some());
    let mut steps = super::StepRunner::new(reporter, total);

    if install_targets {
        let detail = targets.join(" ");
        steps
            .run("Install Rust targets", Some(&detail), async {
                install_rust_targets(&targets, reporter).await
            })
            .await?;
    }

    for entry in &plan.cargo {
        let detail = format!("{} ({})", entry.label, entry.target);
        steps
            .run("Fetch dependencies", Some(&detail), async {
                cargo_fetch(entry, reporter).await
            })
            .await?;
    }

    if let Some(hmi) = &plan.hmi {
        let detail = hmi.project_dir.display().to_string();
        steps
            .run("Install HMI dependencies", Some(&detail), async {
                hmi_install(hmi, reporter).await
            })
            .await?;
    }

    Ok(plan)
}

/// Downloads one Cargo project's dependencies into the local registry cache.
///
/// Scoped with `--target` so a bare-metal firmware crate does not drag in a host
/// target's dependency set. No feature selection is needed: `cargo fetch` works
/// from the lock file, which carries optional dependencies whether or not their
/// feature is active.
async fn cargo_fetch(
    entry: &CargoFetch,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), PipelineError> {
    let cargo = crate::compile::utils::resolve_cargo()
        .await
        .map_err(|message| PipelineError::Fetch {
            label: entry.label.clone(),
            target: entry.target.clone(),
            message,
        })?;

    let mut cmd = Command::new(&cargo);
    cmd.arg("fetch")
        .arg("--target")
        .arg(&entry.target)
        .current_dir(&entry.path);

    let status = run_command(&mut cmd, reporter)
        .await
        .map_err(|e| PipelineError::Fetch {
            label: entry.label.clone(),
            target: entry.target.clone(),
            message: e.to_string(),
        })?;

    if !status.success() {
        return Err(PipelineError::Fetch {
            label: entry.label.clone(),
            target: entry.target.clone(),
            message: format!("`cargo fetch` exited with status: {status}"),
        });
    }
    Ok(())
}

/// Installs the prebuilt standard library for each triple with rustup.
///
/// A host without rustup manages its own toolchain (a Yocto SDK, a distro
/// rustc), so a missing binary is reported and skipped rather than failing the
/// run — the subsequent `cargo fetch` steps are still worth doing. A rustup that
/// *is* present and fails is a real error.
async fn install_rust_targets(
    targets: &[String],
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), PipelineError> {
    let mut cmd = Command::new("rustup");
    cmd.args(["target", "add", "--toolchain", "stable"])
        .args(targets);

    let status = match run_command(&mut cmd, reporter).await {
        Ok(status) => status,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "note: rustup not found; skipping `rustup target add {}`. Ensure the \
                 standard library for these targets is installed by other means.",
                targets.join(" ")
            );
            return Ok(());
        }
        Err(e) => {
            return Err(PipelineError::RustTarget {
                targets: targets.join(" "),
                message: e.to_string(),
            });
        }
    };

    if !status.success() {
        return Err(PipelineError::RustTarget {
            targets: targets.join(" "),
            message: format!("`rustup target add` exited with status: {status}"),
        });
    }
    Ok(())
}

/// Runs the frontend's package install in the frontend project directory.
async fn hmi_install(
    hmi: &HmiFetch,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), PipelineError> {
    let dir = hmi.project_dir.display().to_string();
    let (program, args) = hmi.command.program_args();
    if program.is_empty() {
        return Err(PipelineError::HmiFetch {
            dir,
            message: "HMI install command is empty".to_string(),
        });
    }

    match tokio::fs::metadata(&hmi.project_dir).await {
        Ok(meta) if meta.is_dir() => {}
        _ => {
            return Err(PipelineError::HmiFetch {
                dir,
                message: "project directory not found".to_string(),
            });
        }
    }

    let mut cmd = Command::new(program);
    cmd.args(&args).current_dir(&hmi.project_dir);

    let status = run_command(&mut cmd, reporter)
        .await
        .map_err(|e| PipelineError::HmiFetch {
            dir: dir.clone(),
            message: format!("failed to launch `{program}`: {e}"),
        })?;

    if !status.success() {
        return Err(PipelineError::HmiFetch {
            dir,
            message: format!("`{program}` exited with status: {status}"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    fn session() -> Session {
        Session::from_toml(STM32).expect("example manifest loads")
    }

    #[test]
    fn plans_one_fetch_per_cargo_project() {
        let plan = plan_fetch(&session(), Path::new("/proj"), None).expect("plan succeeds");

        // The stm32 example declares cm33 firmware, a TA, and the application.
        // cm0p is a manager core, so it is not a firmware target and not fetched.
        let entries: Vec<(&str, &str)> = plan
            .cargo
            .iter()
            .map(|e| (e.label.as_str(), e.target.as_str()))
            .collect();
        assert_eq!(
            entries,
            vec![
                ("cm33", "thumbv8m.main-none-eabihf"),
                ("ta", "aarch64-unknown-linux-gnu"),
                ("app", "aarch64-unknown-linux-gnu"),
            ]
        );

        // Paths anchor at the manifest directory, matching the build plan.
        assert_eq!(plan.cargo[0].path, Path::new("/proj/./firmware"));
        assert_eq!(plan.cargo[1].path, Path::new("/proj/./ta"));
        assert_eq!(plan.cargo[2].path, Path::new("/proj/./app"));

        assert!(plan.hmi.is_none());
    }

    #[test]
    fn rust_targets_are_deduplicated_in_first_use_order() {
        // The TA and the application share aarch64-unknown-linux-gnu; installing
        // that std twice would be wasted work.
        let plan = plan_fetch(&session(), Path::new("/proj"), None).expect("plan succeeds");
        assert_eq!(
            plan.rust_targets(),
            vec!["thumbv8m.main-none-eabihf", "aarch64-unknown-linux-gnu"]
        );
    }

    #[test]
    fn plan_carries_the_hmi_install_through() {
        let hmi = HmiFetch {
            project_dir: PathBuf::from("/proj/website"),
            command: HmiBuildCommand::Custom(vec!["pnpm".into(), "install".into()]),
        };
        let plan =
            plan_fetch(&session(), Path::new("/proj"), Some(hmi.clone())).expect("plan succeeds");
        assert_eq!(plan.hmi, Some(hmi));
    }

    #[test]
    fn plan_omits_the_ta_without_a_tee_section() {
        // Without `[tee]` there is no TA crate to fetch, and the secure-A-core
        // endpoint is not required.
        let manifest = "\
            [profile]\n\
            chip = \"stm32mp257fak3\"\n\
            shared = \"./shared\"\n\
            deploy_root = \"/opt/consortium\"\n\
            \n\
            [endpoints]\n\
            linux = \"./app\"\n";
        let session = Session::from_toml(manifest).expect("manifest loads");
        let plan = plan_fetch(&session, Path::new("/proj"), None).expect("plan succeeds");
        let labels: Vec<&str> = plan.cargo.iter().map(|e| e.label.as_str()).collect();
        assert_eq!(labels, vec!["app"]);
    }

    #[test]
    fn plan_errors_without_an_application_endpoint() {
        let manifest = "\
            [profile]\n\
            chip = \"stm32mp257fak3\"\n\
            shared = \"./shared\"\n\
            deploy_root = \"/opt/consortium\"\n";
        let session = Session::from_toml(manifest).expect("manifest loads");
        let err = plan_fetch(&session, Path::new("/proj"), None)
            .expect_err("a build always needs the Linux application");
        assert!(matches!(err, PipelineError::MissingEndpoint(_)));
    }

    #[tokio::test]
    async fn hmi_install_runs_the_command_in_the_project_dir() {
        let dir = std::env::var("SCRATCH_DIR")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join(format!("consortium-fetch-hmi-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let hmi = HmiFetch {
            project_dir: dir.clone(),
            command: HmiBuildCommand::Custom(vec![
                "sh".into(),
                "-c".into(),
                "printf x > installed".into(),
            ]),
        };
        hmi_install(&hmi, None).await.expect("install succeeds");
        assert!(dir.join("installed").exists());

        // A failing package manager surfaces as a fetch error, not a silent skip.
        let failing = HmiFetch {
            project_dir: dir.clone(),
            command: HmiBuildCommand::Custom(vec!["false".into()]),
        };
        let err = hmi_install(&failing, None)
            .await
            .expect_err("non-zero exit is an error");
        assert!(matches!(err, PipelineError::HmiFetch { .. }));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn hmi_install_errors_when_project_dir_missing() {
        let hmi = HmiFetch {
            project_dir: PathBuf::from("/nonexistent/consortium-website"),
            command: HmiBuildCommand::Custom(vec!["true".into()]),
        };
        let err = hmi_install(&hmi, None)
            .await
            .expect_err("missing project dir is an error");
        assert!(matches!(err, PipelineError::HmiFetch { .. }));
    }
}
