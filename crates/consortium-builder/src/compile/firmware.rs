use consortium_common::config::mcu_core::CortexMcuCore;
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

use crate::compile::utils::{elf_path, resolve_cargo, verify_elf};
use super::Compiler;

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
    pub async fn build(&self) -> Result<PathBuf, String> {
        FirmwareCompiler::build(FirmwareConfiguration {
            arch: self.arch.clone(),
            path: self.path.clone(),
            release: self.release,
        })
        .await
        .map_err(|e| e.to_string())
    }

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
