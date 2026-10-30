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

//! Stage 1: compile firmware for remote coprocessors.
//!
//! Only [`CoprocessorVariant::Remote`] cores are firmware targets. The AP is
//! never in the chip database's `cp` list, and `Manager` cores (e.g. STM32MP25's
//! `cm0p`, MIMX95's `cm33`) are excluded here even though they may still appear
//! as IPC/dbg peers.

use std::path::PathBuf;

use consortium_cfg::Session;
use consortium_cfg_common::config::{Party, VisibleCore};
use consortium_cfg_common::database::{ChipDatabase, Coprocessor, CoprocessorVariant};

use super::{CargoOptions, PipelineError, core_rust_triple, visible_core_of};
use crate::compile::env::CstiWorkspace;
use crate::compile::firmware::FirmwareConfiguration;

/// A resolved firmware build for one remote coprocessor.
pub struct FirmwareBuildPlan {
    /// The coprocessor this firmware runs on.
    pub core: VisibleCore,
    /// The manifest party projecting to that core (its `[endpoints]` key).
    pub party: Party,
    /// The `cargo build` parameters for this firmware.
    pub config: FirmwareConfiguration,
    /// The generated per-core init module, rendered to Rust source, to be
    /// written into the firmware crate before building.
    pub generated: String,
}

/// Every remote (firmware-target) coprocessor in `database`.
pub fn remote_coprocessors(database: &ChipDatabase) -> impl Iterator<Item = &Coprocessor> {
    database
        .cp
        .iter()
        .filter(|cp| cp.variant == CoprocessorVariant::Remote)
}

/// Every remote coprocessor that declares an `[endpoints]` entry: its core, the
/// manifest party projecting to it, and its crate root as written in the
/// manifest.
///
/// A remote coprocessor with no matching entry is skipped: there is no firmware
/// crate to build for it.
pub fn firmware_endpoints(
    session: &Session,
) -> Result<Vec<(VisibleCore, Party, &str)>, PipelineError> {
    let mut endpoints = Vec::new();
    for cp in remote_coprocessors(&session.database) {
        let core = visible_core_of(cp)?;
        // The party whose core projects to this coprocessor and has an endpoint.
        if let Some((party, root)) = session
            .config
            .endpoints
            .iter()
            .find(|(party, _)| party.core() == core)
        {
            endpoints.push((core, *party, root.as_str()));
        }
    }
    Ok(endpoints)
}

/// Plans one firmware build per remote coprocessor that declares an endpoint.
///
/// A remote coprocessor with no matching `[endpoints]` entry is skipped: there is
/// no firmware crate to build for it. Each plan resolves the Cargo project path,
/// the Rust target triple (from the core name, the single source of truth), the
/// IPC/debug shared-memory layouts, and the generated init module.
pub fn plan_firmware_builds(
    session: &Session,
    workspace: &CstiWorkspace,
    cargo: CargoOptions,
    ty_crate: &syn::Path,
) -> Result<Vec<FirmwareBuildPlan>, PipelineError> {
    let ir = session.lower()?;
    let mut plans = Vec::new();

    for (core, party, root) in firmware_endpoints(session)? {
        let path = workspace.manifest_dir.join(root);
        let target = core_rust_triple(core);

        // The controller core's generated init module (MPU/NVIC weave plus its
        // disconnected IPC endpoints). This is the existing codegen; the pipeline
        // only writes it and builds.
        consortium_log::trace!("rendering firmware init module for core {}", core);
        let generated = consortium_cfg::render(ir.initialize(core, ty_crate)).map_err(|e| {
            consortium_log::trace!("firmware init codegen render failed for {}: {}", core, e);
            PipelineError::Codegen(e.to_string())
        })?;

        plans.push(FirmwareBuildPlan {
            core,
            party,
            config: FirmwareConfiguration {
                target,
                path,
                release: cargo.release,
                offline: cargo.offline,
                remoteproc: ir.remoteproc.get(&core).cloned(),
                workspace: Some(workspace.clone()),
            },
            generated,
        });
    }

    Ok(plans)
}

/// The build-host path of a firmware crate's compiled ELF for the given target.
pub async fn firmware_elf_path(config: &FirmwareConfiguration) -> Result<PathBuf, PipelineError> {
    crate::compile::utils::elf_path(
        &config.path,
        config.workspace.as_ref(),
        &config.target,
        config.release,
    )
    .await
    .map_err(PipelineError::Codegen)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    fn session() -> Session {
        Session::from_toml(STM32).expect("example manifest loads")
    }

    #[test]
    fn remote_coprocessors_excludes_managers() {
        let session = session();
        let remote: Vec<_> = remote_coprocessors(&session.database)
            .map(|cp| cp.name.clone())
            .collect();
        // STM32MP25: cm33 is the remote firmware target; cm0p is a manager.
        assert!(remote.iter().any(|n| n == "cm33"));
        assert!(!remote.iter().any(|n| n == "cm0p"));
    }

    #[test]
    fn plans_one_build_per_remote_endpoint() {
        let session = session();
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
        let cargo = CargoOptions {
            release: true,
            offline: true,
        };
        let workspace = crate::pipeline::resolve_workspace(&session, Path::new("/proj"))
            .expect("nested workspace");
        let plans = plan_firmware_builds(&session, &workspace, cargo, &ty_crate).expect("plans");

        // Only cm33 (remote, with a `cm33 = "./firmware"` endpoint) is planned.
        assert_eq!(plans.len(), 1);
        let plan = &plans[0];
        assert_eq!(plan.core, VisibleCore::Mcore("cm33".parse().unwrap()));
        assert_eq!(plan.config.target, "thumbv8m.main-none-eabihf");
        assert_eq!(plan.config.path, Path::new("/proj/firmware"));
        // The lone firmware crate is exposed as `CSTI_MCU_PATH`.
        assert_eq!(
            workspace.mcu_dir.as_deref(),
            Some(Path::new("/proj/firmware"))
        );
        assert!(plan.config.release);
        // The shared cargo flags reach every firmware plan.
        assert!(plan.config.offline);
        // cm33 declares `[dbg.cm33]`, so its generated init hands the ring to the
        // logger; no `CONSORTIUM_DBG_*` variable is involved.
        assert!(plan.generated.contains("ConsortiumLogger::init("));
        let remoteproc = plan.config.remoteproc.as_ref().expect("remoteproc layout");
        assert_eq!(remoteproc.flash.origin, 0x8010_1000);
        assert_eq!(remoteproc.ram.origin, 0x80a0_0000);
        assert_eq!(remoteproc.vector_section, ".isr_vectors");
        // The generated module is this core's controller init: the synchronous
        // entry point handing back the peripheral and channel bundles, plus the
        // `sensor` endpoint the manifest declares for cm33. `render` already
        // fails if the emitted tokens are not valid Rust.
        assert!(
            plan.generated
                .contains("fn init(config: Config) -> (Peripherals, Channels)"),
            "{}",
            plan.generated
        );
        assert!(
            plan.generated.contains("channels!(SENSOR)"),
            "{}",
            plan.generated
        );
    }
}
