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

//! Stage 4: compile and sign the OP-TEE Trusted Application.
//!
//! This is mostly wiring: [`OpteeCompiler`](crate::compile::optee::OpteeCompiler)
//! already builds and signs. The manifest supplies the TA UUID and the
//! `optee`/`[endpoints]` project path; the OP-TEE SDK toolchain (dev-kit dir and
//! signer) is a host concern supplied by the caller (typically from the standard
//! `TA_DEV_KIT_DIR` environment variable).

use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg_common::config::Party;

use super::PipelineError;
use crate::compile::optee::OpteeConfiguration;

/// The host-side OP-TEE toolchain needed to build and sign a TA.
#[derive(Debug, Clone)]
pub struct TeeToolchain {
    /// `TA_DEV_KIT_DIR`: the OP-TEE TA dev-kit directory.
    pub ta_dev_kit_dir: PathBuf,
    /// Path to the OP-TEE signer script (`sign_encrypt.py`).
    pub optee_signer: PathBuf,
    /// Optional explicit linker (the OP-TEE SDK cross-compiler).
    pub linker: Option<String>,
}

impl TeeToolchain {
    /// Resolves the toolchain from the environment.
    ///
    /// `TA_DEV_KIT_DIR` is the standard OP-TEE SDK variable. The signer defaults
    /// to `$TA_DEV_KIT_DIR/scripts/sign_encrypt.py` unless `CONSORTIUM_OPTEE_SIGNER`
    /// overrides it; the linker comes from `CONSORTIUM_TA_LINKER` when set.
    /// Returns `None` when `TA_DEV_KIT_DIR` is not set.
    pub fn from_env() -> Option<Self> {
        let ta_dev_kit_dir = PathBuf::from(std::env::var("TA_DEV_KIT_DIR").ok()?);
        let optee_signer = std::env::var("CONSORTIUM_OPTEE_SIGNER")
            .map(PathBuf::from)
            .unwrap_or_else(|_| ta_dev_kit_dir.join("scripts").join("sign_encrypt.py"));
        let linker = std::env::var("CONSORTIUM_TA_LINKER").ok();
        Some(Self {
            ta_dev_kit_dir,
            optee_signer,
            linker,
        })
    }
}

/// Plans the TA build, or `None` when the manifest declares no `[tee]`.
///
/// Resolves the project path from the `optee` (secure A-core) endpoint and the
/// UUID from `[tee]`. Fails with [`PipelineError::MissingTeeToolchain`] when a
/// `[tee]` section is present but no toolchain was supplied, or with
/// [`PipelineError::MissingEndpoint`] when no `optee` endpoint is declared.
pub fn plan_ta_build(
    session: &Session,
    manifest_dir: &Path,
    toolchain: Option<&TeeToolchain>,
    release: bool,
) -> Result<Option<OpteeConfiguration>, PipelineError> {
    let Some(tee) = &session.config.tee else {
        return Ok(None);
    };

    let toolchain = toolchain.ok_or(PipelineError::MissingTeeToolchain)?;

    let root = session
        .config
        .endpoints
        .get(&Party::AcoreSecure)
        .ok_or_else(|| PipelineError::MissingEndpoint(Party::AcoreSecure.to_string()))?;

    Ok(Some(OpteeConfiguration {
        path: manifest_dir.join(root),
        release,
        linker: toolchain.linker.clone(),
        ta_dev_kit_dir: toolchain.ta_dev_kit_dir.clone(),
        optee_signer: toolchain.optee_signer.clone(),
        ta_uuid: tee.uuid,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    fn toolchain() -> TeeToolchain {
        TeeToolchain {
            ta_dev_kit_dir: PathBuf::from("/opt/optee/export-ta_arm64"),
            optee_signer: PathBuf::from("/opt/optee/scripts/sign_encrypt.py"),
            linker: None,
        }
    }

    #[test]
    fn plans_ta_from_manifest_uuid_and_endpoint() {
        let session = Session::from_toml(STM32).expect("manifest loads");
        let cfg = plan_ta_build(&session, Path::new("/proj"), Some(&toolchain()), true)
            .expect("plan succeeds")
            .expect("manifest has [tee]");
        assert_eq!(cfg.path, Path::new("/proj/ta"));
        assert_eq!(
            cfg.ta_uuid.to_string().to_uppercase(),
            "48551DD5-74B2-4D24-BF9D-A09F02F5C56F"
        );
        assert!(cfg.release);
    }

    #[test]
    fn errors_when_toolchain_absent_but_tee_declared() {
        let session = Session::from_toml(STM32).expect("manifest loads");
        let err = match plan_ta_build(&session, Path::new("/proj"), None, false) {
            Ok(_) => panic!("no toolchain is an error when [tee] is present"),
            Err(err) => err,
        };
        assert!(matches!(err, PipelineError::MissingTeeToolchain));
    }

    #[test]
    fn returns_none_without_tee_section() {
        let manifest = "\
            [profile]\n\
            chip = \"stm32mp257fak3\"\n\
            shared = \"./shared\"\n";
        let session = Session::from_toml(manifest).expect("manifest loads");
        let plan = plan_ta_build(&session, Path::new("/proj"), Some(&toolchain()), false)
            .expect("plan succeeds");
        assert!(plan.is_none());
    }
}
