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

//! The fast feedback loop: validate the manifest and type-check every Rust
//! endpoint, the way `cargo check` does for one crate.
//!
//! A check runs the same front half as [`run`](super::run) — load and validate the
//! manifest, lower it, render and write each endpoint's `consortium.gen.rs` (and
//! each firmware's `memory.x`) — then runs `cargo check` instead of `cargo build`
//! in each Cargo project, with the same target, flags, and injected
//! `CONSORTIUM_*` environment. Nothing is linked, post-processed, signed, or
//! staged: no objcopy, no HMI frontend build, no `dist/`, no `dtc`, no BitBake.
//!
//! Two differences from a build are deliberate:
//!
//! - **The AP's embedded debug ELFs.** The AP init module `include_bytes!`es each
//!   `[dbg.<core>]` firmware's `*_original.elf`, and `include_bytes!` is resolved
//!   during `cargo check` too — but checking the firmware produces no ELF. When a
//!   previous build left the real image on disk it is embedded as usual; otherwise
//!   an empty stand-in is written next to where it would be (see
//!   [`placeholder_elf_path`]) so the application still type-checks. The generated
//!   module then points at that stand-in until the next `csti build` rewrites it.
//! - **Host provisioning is optional.** The device tree is rendered (not written or
//!   compiled) only when a `--linux` checkout is supplied, and the TA is checked
//!   only when an OP-TEE dev kit is available: a TA build script needs
//!   `TA_DEV_KIT_DIR`, but a laptop iterating on firmware usually has none. The
//!   plan records the skip so the caller can say so.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg_common::config::VisibleCore;

use super::application::{ApplicationBuildPlan, plan_application_build};
use super::firmware::{FirmwareBuildPlan, firmware_elf_path, plan_firmware_builds};
use super::tee::{TeeToolchain, plan_ta_build};
use super::{
    CargoOptions, PipelineError, StepRunner, dts, elf_map, load_session, resolve_ty_crate,
    write_generated_memory_x, write_generated_module,
};
use crate::compile::application::ApplicationCompiler;
use crate::compile::firmware::{FirmwareCompiler, original_elf_path};
use crate::compile::optee::{OpteeCompiler, OpteeConfiguration};
use crate::progress::BuildReporter;

/// All parameters for a check run.
pub struct CheckConfig {
    /// Path to the `Consortium.toml` manifest.
    pub manifest_path: PathBuf,
    /// Linux kernel checkout to render the device tree against, or `None` to skip
    /// device-tree validation.
    pub linux: Option<PathBuf>,
    /// Check the release profile.
    pub release: bool,
    /// Pass `--offline` to every cargo invocation.
    pub offline: bool,
    /// OP-TEE toolchain, or `None` to skip the TA check.
    pub tee_toolchain: Option<TeeToolchain>,
    /// Treat a missing remoteproc DTS node as an error (see
    /// [`PipelineConfig::strict_remoteproc`](super::PipelineConfig::strict_remoteproc)).
    pub strict_remoteproc: bool,
}

impl CheckConfig {
    /// The cargo flags every Rust check of this run shares.
    pub fn cargo(&self) -> CargoOptions {
        CargoOptions {
            release: self.release,
            offline: self.offline,
        }
    }
}

/// Why the TA is not part of a check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaSkip {
    /// The manifest declares `[tee]`, but no OP-TEE toolchain was supplied.
    NoToolchain,
}

/// The fully resolved plan for a check run.
pub struct CheckPlan {
    /// One plan per remote coprocessor firmware, checked in order.
    pub firmware: Vec<FirmwareBuildPlan>,
    /// The TA to check, when the manifest declares `[tee]` and a toolchain exists.
    pub ta: Option<OpteeConfiguration>,
    /// Set when the manifest declares `[tee]` but the TA will not be checked.
    pub ta_skipped: Option<TaSkip>,
    /// The application check parameters and its generated AP init module.
    pub application: ApplicationBuildPlan,
    /// Debug ELFs with no image on disk yet: empty stand-ins written before the
    /// application is checked, so its `include_bytes!` resolves.
    pub placeholder_elfs: Vec<PathBuf>,
    /// File names of the device trees rendered for validation. Empty when no
    /// `--linux` checkout was supplied or the manifest needs no device tree.
    pub dts_files: Vec<String>,
}

/// The empty stand-in written for a debug firmware ELF that has not been built.
///
/// Lives beside the raw cargo output (`.../<profile>/<name>_check.elf`), so it
/// shares the firmware's `target/` lifetime and never shadows the real
/// `*_original.elf` a later build produces.
pub fn placeholder_elf_path(raw: &Path) -> PathBuf {
    let stem = raw
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("firmware");
    raw.with_file_name(format!("{stem}_check.elf"))
}

/// Resolves everything a check needs without running cargo.
///
/// Fails exactly where [`plan`](super::plan) would for the same manifest, except
/// that the device tree is rendered only when [`CheckConfig::linux`] is set and a
/// missing OP-TEE toolchain skips the TA instead of failing.
pub async fn plan_check(
    session: &Session,
    config: &CheckConfig,
) -> Result<CheckPlan, PipelineError> {
    let manifest_dir = config
        .manifest_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let ty_crate = resolve_ty_crate(session, &manifest_dir).await?;
    let cargo = config.cargo();
    let workspace = super::resolve_workspace(session, &manifest_dir)?;

    let firmware = plan_firmware_builds(session, &workspace, cargo, &ty_crate)?;

    // The AP embeds each debug core's original ELF: the real one when a previous
    // build left it on disk, an empty stand-in otherwise.
    let mut dbg_sources: HashMap<VisibleCore, PathBuf> = HashMap::new();
    for plan in &firmware {
        let raw = firmware_elf_path(&plan.config).await?;
        let original = original_elf_path(&raw);
        let source = if original.exists() {
            original
        } else {
            placeholder_elf_path(&raw)
        };
        dbg_sources.insert(plan.core, source);
    }
    let ir = session.lower()?;
    let dbg_elfs = elf_map::resolve_dbg_elf_paths(&dbg_sources, ir.dbg.rings().keys().copied())?;
    let placeholder_elfs = dbg_elfs.values().filter(|p| !p.exists()).cloned().collect();

    let (ta, ta_skipped) = match (&session.config.tee, &config.tee_toolchain) {
        (Some(_), None) => (None, Some(TaSkip::NoToolchain)),
        _ => (
            plan_ta_build(
                session,
                &workspace,
                config.tee_toolchain.as_ref(),
                cargo,
                false,
            )?,
            None,
        ),
    };

    // Inject the same HMI environment a build would. The on-target serve path is
    // derived from `deploy_root`, so the frontend need not have been built.
    let hmi_dist = session
        .config
        .hmi
        .as_ref()
        .map(|hmi| manifest_dir.join(&hmi.build));
    let application = plan_application_build(
        session,
        &workspace,
        hmi_dist.as_deref(),
        &dbg_elfs,
        cargo,
        &ty_crate,
    )?;

    let mut dts_files = Vec::new();
    if let Some(linux) = &config.linux {
        let profile = &session.config.profile;
        let target_cores: Vec<String> = firmware.iter().map(|p| p.core.to_string()).collect();
        if let Some(rendered) = dts::render_linux_dts(
            &profile.chip,
            profile.devkit.as_deref(),
            session.config.dts.base.as_deref(),
            session.config.dts.mode,
            &ir,
            &target_cores,
            linux,
            config.strict_remoteproc,
        )? {
            for warning in &rendered.warnings {
                eprintln!("warning: {warning}");
            }
            dts_files.push(rendered.file_name);
        }
    }

    Ok(CheckPlan {
        firmware,
        ta,
        ta_skipped,
        application,
        placeholder_elfs,
        dts_files,
    })
}

/// Plans and runs a check, returning the plan that was checked.
pub async fn run(config: CheckConfig) -> Result<CheckPlan, PipelineError> {
    run_inner(config, None).await
}

/// Runs a check while reporting step progress and subprocess output.
pub async fn run_with_reporter(
    config: CheckConfig,
    reporter: &dyn BuildReporter,
) -> Result<CheckPlan, PipelineError> {
    run_inner(config, Some(reporter)).await
}

async fn run_inner(
    config: CheckConfig,
    reporter: Option<&dyn BuildReporter>,
) -> Result<CheckPlan, PipelineError> {
    consortium_log::trace!("check run: manifest {}", config.manifest_path.display());
    let session = load_session(&config.manifest_path).await?;
    let plan = plan_check(&session, &config).await?;

    let total = plan.firmware.len() + usize::from(plan.ta.is_some()) + 1;
    let mut steps = StepRunner::new(reporter, total);

    for fw in &plan.firmware {
        let detail = format!("{} ({})", fw.core, fw.config.target);
        steps
            .run("Check firmware", Some(&detail), async {
                write_generated_module(&fw.config.path, &fw.generated).await?;
                write_generated_memory_x(&fw.config).await?;
                FirmwareCompiler::check_with_reporter(&fw.config, reporter).await?;
                Ok(())
            })
            .await?;
    }

    if let Some(ta) = &plan.ta {
        steps
            .run("Check TEE", Some("trusted application"), async {
                OpteeCompiler::check_with_reporter(ta, reporter).await?;
                Ok(())
            })
            .await?;
    }

    let app_detail = plan.application.config.target.clone();
    steps
        .run("Check application", Some(&app_detail), async {
            for path in &plan.placeholder_elfs {
                write_placeholder_elf(path).await?;
            }
            write_generated_module(&plan.application.config.path, &plan.application.generated)
                .await?;
            ApplicationCompiler::check_with_reporter(&plan.application.config, reporter).await?;
            Ok(())
        })
        .await?;

    Ok(plan)
}

/// Writes an empty stand-in ELF, creating its directory if the firmware has never
/// been built for this profile.
async fn write_placeholder_elf(path: &Path) -> Result<(), PipelineError> {
    let io_error = |source| PipelineError::GeneratedWrite {
        path: path.display().to_string(),
        source,
    };
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(io_error)?;
    }
    tokio::fs::write(path, []).await.map_err(io_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    /// A project tree with per-endpoint `Cargo.toml`s and no sources, enough for
    /// [`plan_check`] to resolve crate names and artifact paths.
    fn fabricate_project(tag: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("consortium-check-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("Consortium.toml"), STM32).unwrap();
        for (dir, pkg) in [
            ("app", "melt-pot-app"),
            ("ta", "melt-pot-ta"),
            ("firmware", "melt-pot-mcu"),
            ("shared", "melt-pot-shared"),
        ] {
            let crate_dir = root.join(dir);
            std::fs::create_dir_all(&crate_dir).unwrap();
            std::fs::write(
                crate_dir.join("Cargo.toml"),
                format!("[package]\nname = \"{pkg}\"\nversion = \"0.0.0\"\n"),
            )
            .unwrap();
        }
        root
    }

    fn config(root: &Path, tee_toolchain: Option<TeeToolchain>) -> CheckConfig {
        CheckConfig {
            manifest_path: root.join("Consortium.toml"),
            linux: None,
            release: false,
            offline: true,
            tee_toolchain,
            strict_remoteproc: true,
        }
    }

    fn raw_mcu_elf(root: &Path) -> PathBuf {
        root.join("./firmware/target/thumbv8m.main-none-eabihf/debug/melt-pot-mcu")
    }

    #[tokio::test]
    async fn unbuilt_debug_firmware_gets_a_placeholder() {
        let root = fabricate_project("placeholder");
        let session = Session::from_toml(STM32).unwrap();

        let plan = plan_check(&session, &config(&root, None))
            .await
            .expect("check plans without --linux or a TEE toolchain");

        assert_eq!(plan.firmware.len(), 1);
        assert!(plan.firmware[0].config.offline);
        // No DTS without a Linux checkout.
        assert!(plan.dts_files.is_empty());
        // `[tee]` is declared but no toolchain was given: skipped, not an error.
        assert!(plan.ta.is_none());
        assert_eq!(plan.ta_skipped, Some(TaSkip::NoToolchain));

        // cm33 declares `[dbg.cm33]` and has never been built, so the AP embeds
        // an empty stand-in beside the raw cargo output, named relative to the
        // firmware crate through `CSTI_MCU_PATH`.
        let placeholder = std::path::absolute(placeholder_elf_path(&raw_mcu_elf(&root))).unwrap();
        assert_eq!(plan.placeholder_elfs, vec![placeholder.clone()]);
        let generated = &plan.application.generated;
        assert!(
            generated.contains("::core::env!(\"CSTI_MCU_PATH\")"),
            "{generated}"
        );
        assert!(
            generated
                .contains("\"/target/thumbv8m.main-none-eabihf/debug/melt-pot-mcu_check.elf\"")
        );
        assert!(!generated.contains(&root.display().to_string()));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn built_debug_firmware_is_embedded_as_is() {
        let root = fabricate_project("built");
        let original = original_elf_path(&raw_mcu_elf(&root));
        std::fs::create_dir_all(original.parent().unwrap()).unwrap();
        std::fs::write(&original, b"\x7fELF").unwrap();
        let session = Session::from_toml(STM32).unwrap();
        let toolchain = TeeToolchain {
            ta_dev_kit_dir: PathBuf::from("/opt/optee/export-ta_arm64"),
            optee_signer: PathBuf::from("/opt/optee/scripts/sign_encrypt.py"),
            linker: None,
        };

        let plan = plan_check(&session, &config(&root, Some(toolchain)))
            .await
            .expect("check plans");

        assert!(plan.placeholder_elfs.is_empty());
        assert!(
            plan.application
                .generated
                .contains("\"/target/thumbv8m.main-none-eabihf/debug/melt-pot-mcu_original.elf\"")
        );
        // With a toolchain the TA is checked, and a check never signs.
        let ta = plan.ta.expect("TA planned");
        assert!(!ta.sign);
        assert!(plan.ta_skipped.is_none());

        let _ = std::fs::remove_dir_all(&root);
    }
}
