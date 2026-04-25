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

use consortium_common::config::mcu_core::CortexMcuCore;
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

use super::Compiler;
use crate::compile::utils::{elf_path, resolve_cargo, verify_elf};

/// Error type for firmware compilation.
#[derive(Error, Debug)]
pub enum FirmwareCompileError {
    #[error("Failed to resolve cargo: {0}")]
    CargoResolveError(String),

    #[error("Cargo build failed: {0}")]
    BuildFailed(String),

    #[error("ELF not found at expected path: {0:?}")]
    ElfNotFound(PathBuf),

    #[error("{0}")]
    UtilityError(String),
}

/// Build parameters for bare-metal Cortex-M firmware.
#[derive(Clone)]
pub struct FirmwareConfiguration {
    /// Target MCU core (determines the Rust target triple).
    pub arch: CortexMcuCore,
    /// Path to the firmware Cargo project root.
    pub path: PathBuf,
    /// Build in release mode.
    pub release: bool,
}

/// Compiler for bare-metal Cortex-M firmware.
pub struct FirmwareCompiler;

impl Compiler for FirmwareCompiler {
    type Configuration = FirmwareConfiguration;
    type Error = FirmwareCompileError;
    type Output = PathBuf;

    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error> {
        let cargo = resolve_cargo()
            .await
            .map_err(FirmwareCompileError::CargoResolveError)?;

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--target")
            .arg(config.arch.to_rust_target())
            .current_dir(&config.path);

        if config.release {
            cmd.arg("--release");
        }

        let status = cmd
            .status()
            .await
            .map_err(|e| FirmwareCompileError::BuildFailed(e.to_string()))?;

        if !status.success() {
            return Err(FirmwareCompileError::BuildFailed(format!(
                "cargo build failed with status: {status}"
            )));
        }

        let elf = Self::output_path(&config).await?;

        if !elf.exists() {
            return Err(FirmwareCompileError::ElfNotFound(elf));
        }

        verify_elf(&elf)
            .await
            .map_err(FirmwareCompileError::UtilityError)?;

        Ok(elf)
    }

    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error> {
        elf_path(&config.path, config.arch.to_rust_target(), config.release)
            .await
            .map_err(FirmwareCompileError::UtilityError)
    }
}

/// Backward-compatible builder; delegates to [`FirmwareCompiler`].
pub struct FirmwareBuilder {
    pub arch: CortexMcuCore,
    pub path: PathBuf,
    pub release: bool,
}

impl FirmwareBuilder {
    /// Builds the firmware, returning the path to the output ELF.
    pub async fn build(&self) -> Result<PathBuf, String> {
        FirmwareCompiler::build(FirmwareConfiguration {
            arch: self.arch.clone(),
            path: self.path.clone(),
            release: self.release,
        })
        .await
        .map_err(|e| e.to_string())
    }

    /// Returns the expected output ELF path without building.
    pub async fn elf_path(&self) -> Result<PathBuf, String> {
        FirmwareCompiler::output_path(&FirmwareConfiguration {
            arch: self.arch.clone(),
            path: self.path.clone(),
            release: self.release,
        })
        .await
        .map_err(|e| e.to_string())
    }
}
