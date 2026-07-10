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

//! Stage 5: compile the Linux application.
//!
//! The AP's generated init module embeds each debug ring's firmware ELF via
//! `include_bytes!`, so this stage substitutes Stage 1's resolved ELF paths into
//! the debug IR before generating that module.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg_common::config::{AcoreName, Party, VisibleCore};

use super::{PipelineError, WWW_SUBDIR, ipc_config_for};
use crate::compile::application::ApplicationConfiguration;
use crate::compile::env::HmiConfig;

/// A resolved application build plus its generated AP init module.
pub struct ApplicationBuildPlan {
    /// `cargo build` parameters for the Linux application.
    pub config: ApplicationConfiguration,
    /// The generated AP init module, rendered to Rust source, to be written into
    /// the application crate before building. Its `DbgRingSpec`s already embed the
    /// resolved firmware ELF paths.
    pub generated: String,
}

/// Plans the Linux application build.
///
/// `hmi_dist` is `Some` when Stage 3 built the frontend; it only gates whether an
/// [`HmiConfig`] is injected — the *on-target* serve path is derived from
/// `deploy_root`, not the build-host dist. `resolved_dbg_elfs` maps each debug
/// core to the firmware ELF the AP embeds.
pub fn plan_application_build(
    session: &Session,
    manifest_dir: &Path,
    hmi_dist: Option<&Path>,
    resolved_dbg_elfs: &HashMap<VisibleCore, PathBuf>,
    release: bool,
    ty_crate: &syn::Path,
) -> Result<ApplicationBuildPlan, PipelineError> {
    let root = session
        .config
        .endpoints
        .get(&Party::AcoreNonSecure)
        .ok_or_else(|| PipelineError::MissingEndpoint(Party::AcoreNonSecure.to_string()))?;
    let path = manifest_dir.join(root);

    let target = acore_linux_triple(&session.database.ap)?;

    // The HMI carrier points the deployed app at the on-target install path
    // (`{deploy_root}/www`), which a packaging recipe populates from `dist/www`.
    let hmi = match (&session.config.hmi, hmi_dist) {
        (Some(hmi), Some(_)) => Some(HmiConfig::from_hmi(
            hmi,
            PathBuf::from(WWW_SUBDIR),
            &session.config.profile.deploy_root,
        )),
        _ => None,
    };

    let ipc = ipc_config_for(&session.lower()?, VisibleCore::Ap);

    // A relative `[profile] sysroot` is anchored at the manifest directory; an
    // absolute one is used verbatim.
    let sysroot = session
        .config
        .profile
        .sysroot
        .as_ref()
        .map(|s| manifest_dir.join(s));

    // Substitute the resolved firmware ELF paths into the debug IR, then generate
    // the AP init module that `include_bytes!`es them.
    let mut ir = session.lower()?;
    ir.dbg = std::mem::take(&mut ir.dbg).with_resolved_elf_paths(resolved_dbg_elfs);
    consortium_log::trace!("rendering AP init module for target {}", target);
    let generated =
        consortium_cfg::render(ir.initialize(VisibleCore::Ap, ty_crate)).map_err(|e| {
            consortium_log::trace!("AP init codegen render failed: {}", e);
            PipelineError::Codegen(e.to_string())
        })?;

    Ok(ApplicationBuildPlan {
        config: ApplicationConfiguration {
            path,
            target,
            release,
            hmi,
            ipc,
            // The AP is the debug *host*: it reads rings through the generated UIO
            // code and the embedded ELF, not via `CONSORTIUM_DBG_*` (those
            // configure a ring *writer*), so no debug layout is injected here.
            dbg: None,
            sysroot,
        },
        generated,
    })
}

/// The Linux Rust target triple for the AP core.
///
/// Derived from the AP core's bitness rather than hardcoded, so a 32-bit A-core
/// (e.g. Cortex-A7) targets armv7 instead of aarch64. This is the Linux/`gnu`
/// ABI, distinct from [`AcoreName::as_rust_triple`]'s bare-metal `-none` triples.
fn acore_linux_triple(ap: &str) -> Result<String, PipelineError> {
    let name: AcoreName = ap
        .parse()
        .map_err(|_| PipelineError::UnknownCore(ap.to_string()))?;
    Ok(if name.is_64_bit() {
        "aarch64-unknown-linux-gnu".to_string()
    } else {
        "armv7-unknown-linux-gnueabihf".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    fn cm33() -> VisibleCore {
        VisibleCore::Mcore("cm33".parse().unwrap())
    }

    #[test]
    fn acore_triple_tracks_bitness() {
        assert_eq!(
            acore_linux_triple("ca35").unwrap(),
            "aarch64-unknown-linux-gnu"
        );
        assert_eq!(
            acore_linux_triple("ca7").unwrap(),
            "armv7-unknown-linux-gnueabihf"
        );
        assert!(acore_linux_triple("bogus").is_err());
    }

    #[test]
    fn plans_application_with_hmi_and_embedded_dbg() {
        let session = Session::from_toml(STM32).expect("manifest loads");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
        let resolved = HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]);

        let plan = plan_application_build(
            &session,
            Path::new("/proj"),
            Some(Path::new("/build/website/dist")),
            &resolved,
            true,
            &ty_crate,
        )
        .expect("plan succeeds");

        assert_eq!(plan.config.path, Path::new("/proj/app"));
        assert_eq!(plan.config.target, "aarch64-unknown-linux-gnu");
        assert!(plan.config.release);

        // The HMI serve path resolves under the deploy root, not the build dist.
        let hmi = plan.config.hmi.expect("hmi injected");
        assert_eq!(hmi.dist, "/opt/consortium/www");

        // The generated AP module embeds the resolved firmware ELF.
        assert!(
            plan.generated
                .contains("include_bytes!(\"/build/cm33.elf\")")
        );
        // The AP is the host, so no debug writer layout is injected.
        assert!(plan.config.dbg.is_none());

        // The stm32 example does not declare a sysroot, so none is planned.
        assert!(plan.config.sysroot.is_none());
    }

    #[test]
    fn relative_profile_sysroot_anchors_at_manifest_dir() {
        let toml = STM32.replace(
            "deploy_root = \"/opt/consortium\"",
            "deploy_root = \"/opt/consortium\"\nsysroot     = \"sdk/sysroot\"",
        );
        let session = Session::from_toml(&toml).expect("manifest loads");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();

        let plan = plan_application_build(
            &session,
            Path::new("/proj"),
            None,
            &HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]),
            false,
            &ty_crate,
        )
        .expect("plan succeeds");

        assert_eq!(
            plan.config.sysroot.as_deref(),
            Some(Path::new("/proj/sdk/sysroot"))
        );
    }

    #[test]
    fn absolute_profile_sysroot_used_verbatim() {
        let toml = STM32.replace(
            "deploy_root = \"/opt/consortium\"",
            "deploy_root = \"/opt/consortium\"\nsysroot     = \"/sdk/target/sysroot\"",
        );
        let session = Session::from_toml(&toml).expect("manifest loads");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();

        let plan = plan_application_build(
            &session,
            Path::new("/proj"),
            None,
            &HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]),
            false,
            &ty_crate,
        )
        .expect("plan succeeds");

        assert_eq!(
            plan.config.sysroot.as_deref(),
            Some(Path::new("/sdk/target/sysroot"))
        );
    }

    #[test]
    fn omits_hmi_when_frontend_not_built() {
        let session = Session::from_toml(STM32).expect("manifest loads");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
        let plan = plan_application_build(
            &session,
            Path::new("/proj"),
            None,
            &HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]),
            false,
            &ty_crate,
        )
        .expect("plan succeeds");
        assert!(plan.config.hmi.is_none());
    }
}
