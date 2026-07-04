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

use consortium_cfg_common::config::TEE_UUID_ENV;
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;
use uuid::Uuid;

use super::Compiler;
use crate::compile::utils::{binary_name, elf_path, resolve_cargo, run_command, verify_elf};
use crate::progress::BuildReporter;

// Only aarch64-unknown-linux-gnu is currently supported for OP-TEE builds.
// Armv7 targets (Cortex-A7) are not supported by the OP-TEE SDK, and
// Cortex-M targets are out of scope for now.
pub const OPTEE_TARGET: &str = "aarch64-unknown-linux-gnu";

/// Error type for OP-TEE compilation and signing.
#[derive(Error, Debug)]
pub enum OpteeCompileError {
    #[error("TA_DEV_KIT_DIR does not exist: {0:?}")]
    TaDevKitNotFound(PathBuf),

    #[error("OP-TEE signer not found at: {0:?}")]
    SignerNotFound(PathBuf),

    #[error("TA signing key not found at: {0:?}")]
    SigningKeyNotFound(PathBuf),

    #[error("Python interpreter not found on PATH (tried: python, python3)")]
    PythonNotFound,

    #[error("Cargo build failed: {0}")]
    BuildFailed(String),

    #[error("ELF not found at expected path: {0:?}")]
    ElfNotFound(PathBuf),

    #[error("OP-TEE signing failed: {0}")]
    SigningFailed(String),

    #[error("Signed TA not found after signing: {0:?}")]
    SignedTaNotFound(PathBuf),

    #[error("{0}")]
    UtilityError(String),
}

/// All build + signing parameters for an OP-TEE Trusted Application.
pub struct OpteeConfiguration {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    /// Build in release mode.
    pub release: bool,
    /// Optional explicit linker (e.g. the cross-compiler from the OP-TEE SDK).
    pub linker: Option<String>,
    /// Path to the OP-TEE TA dev-kit directory (`TA_DEV_KIT_DIR`).
    pub ta_dev_kit_dir: PathBuf,
    /// Path to the OP-TEE signer script (e.g. `sign_encrypt.py`).
    pub optee_signer: PathBuf,
    /// UUID assigned to this Trusted Application.
    pub ta_uuid: Uuid,
    /// Sign the built ELF into a loadable `<uuid>.ta`. When `false`, the build
    /// stops after producing (and verifying) the unsigned ELF and returns that
    /// path — useful when the OP-TEE signer/keys are unavailable and you only
    /// need to check that the TA compiles and links.
    pub sign: bool,
}

/// Compiler for OP-TEE Trusted Applications: builds the ELF then signs it.
pub struct OpteeCompiler;

impl OpteeCompiler {
    pub async fn build_with_reporter(
        config: OpteeConfiguration,
        reporter: Option<&dyn BuildReporter>,
    ) -> Result<PathBuf, OpteeCompileError> {
        // ── Step 1: build ───────────────────────────────────────────────────
        if !config.ta_dev_kit_dir.exists() {
            return Err(OpteeCompileError::TaDevKitNotFound(config.ta_dev_kit_dir));
        }

        let cargo = resolve_cargo()
            .await
            .map_err(OpteeCompileError::UtilityError)?;

        // The TA ELF is the `[[bin]]` target (the `#[ta_*]` entry points + the
        // generated `user_ta_header.rs`), gated behind the `ta` feature. Build it
        // explicitly: a plain `cargo build` would compile only the default-feature
        // `rlib` (the `ca` callers) and skip the feature-gated binary entirely.
        let bin = binary_name(&config.path)
            .await
            .map_err(OpteeCompileError::UtilityError)?;

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--bin")
            .arg(&bin)
            .arg("--features")
            .arg("ta")
            .arg("--target")
            .arg(OPTEE_TARGET)
            .current_dir(&config.path)
            .env("TA_DEV_KIT_DIR", &config.ta_dev_kit_dir)
            .env(TEE_UUID_ENV, config.ta_uuid.to_string())
            .env("RUSTFLAGS", "-C panic=abort".to_string());

        if config.release {
            cmd.arg("--release");
        }

        if let Some(ref linker) = config.linker {
            cmd.arg("--config")
                .arg(format!("{OPTEE_TARGET}.linker={linker}"));
        }

        let status = run_command(&mut cmd, reporter)
            .await
            .map_err(|e| OpteeCompileError::BuildFailed(e.to_string()))?;

        if !status.success() {
            return Err(OpteeCompileError::BuildFailed(format!(
                "cargo build failed with status: {status}"
            )));
        }

        let elf = Self::output_path(&config).await?;

        if !elf.exists() {
            return Err(OpteeCompileError::ElfNotFound(elf));
        }

        verify_elf(&elf)
            .await
            .map_err(OpteeCompileError::UtilityError)?;

        // Stop at the unsigned ELF when signing is disabled (no signer/keys
        // required). The dist stage stages whatever path is returned here.
        if !config.sign {
            return Ok(elf);
        }

        // ── Step 2: sign ────────────────────────────────────────────────────
        if !config.optee_signer.exists() {
            return Err(OpteeCompileError::SignerNotFound(config.optee_signer));
        }

        let ta_key = config.ta_dev_kit_dir.join("keys").join("default_ta.pem");
        if !ta_key.exists() {
            return Err(OpteeCompileError::SigningKeyNotFound(ta_key));
        }

        let python = resolve_python().await?;

        let ta = elf
            .parent()
            .ok_or_else(|| {
                OpteeCompileError::UtilityError(format!(
                    "Could not determine output directory for ELF: {elf:?}"
                ))
            })?
            .join(format!("{}.ta", config.ta_uuid));

        let mut sign_cmd = Command::new(&python);
        sign_cmd
            .arg(&config.optee_signer)
            .arg("--uuid")
            .arg(config.ta_uuid.to_string())
            .arg("--key")
            .arg(&ta_key)
            .arg("--in")
            .arg(&elf)
            .arg("--out")
            .arg(&ta)
            .env("TA_DEV_KIT_DIR", &config.ta_dev_kit_dir);

        let status = run_command(&mut sign_cmd, reporter).await.map_err(|e| {
            OpteeCompileError::SigningFailed(format!(
                "Failed to execute OP-TEE signer via {python}: {e}"
            ))
        })?;

        if !status.success() {
            return Err(OpteeCompileError::SigningFailed(format!(
                "OP-TEE signing failed with status: {status}"
            )));
        }

        if !ta.exists() {
            return Err(OpteeCompileError::SignedTaNotFound(ta));
        }

        Ok(ta)
    }
}

impl Compiler for OpteeCompiler {
    type Configuration = OpteeConfiguration;
    type Error = OpteeCompileError;
    type Output = PathBuf;

    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error> {
        Self::build_with_reporter(config, None).await
    }

    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error> {
        elf_path(&config.path, OPTEE_TARGET, config.release)
            .await
            .map_err(OpteeCompileError::UtilityError)
    }
}

/// Backward-compatible builder; delegates to [`OpteeCompiler`].
pub struct OpteeBuilder {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    pub release: bool,
    pub linker: Option<String>,
}

/// Backward-compatible environment for [`OpteeBuilder`].
pub struct OpteeEnvironment {
    pub ta_dev_kit_dir: PathBuf,
    pub optee_signer: PathBuf,
    pub ta_uuid: Uuid,
}

impl OpteeBuilder {
    pub async fn build(&self, env: &OpteeEnvironment) -> Result<PathBuf, String> {
        OpteeCompiler::build(OpteeConfiguration {
            path: self.path.clone(),
            release: self.release,
            linker: self.linker.clone(),
            ta_dev_kit_dir: env.ta_dev_kit_dir.clone(),
            optee_signer: env.optee_signer.clone(),
            ta_uuid: env.ta_uuid,
            sign: true,
        })
        .await
        .map_err(|e| e.to_string())
    }
}

async fn resolve_python() -> Result<String, OpteeCompileError> {
    for candidate in ["python", "python3"] {
        if let Ok(status) = Command::new(candidate).arg("--version").status().await
            && status.success()
        {
            return Ok(candidate.to_string());
        }
    }
    Err(OpteeCompileError::PythonNotFound)
}
