use consortium_common::config::mcu_core::CortexMcuCore;
use std::path::PathBuf;
use tokio::process::Command;

use crate::compile::utils::{elf_path, resolve_cargo, verify_elf};

pub struct FirmwareBuilder {
    pub arch: CortexMcuCore,
    pub path: PathBuf,
    pub release: bool,
}

impl FirmwareBuilder {
    pub async fn build(&self) -> Result<PathBuf, String> {
        let cargo = resolve_cargo().await?;

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--target")
            .arg(self.arch.to_rust_target())
            .current_dir(&self.path);

        if self.release {
            cmd.arg("--release");
        }

        let status = cmd
            .status()
            .await
            .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

        if !status.success() {
            return Err(format!("Cargo build failed with status: {}", status));
        }

        let elf = elf_path(&self.path, self.arch.to_rust_target(), self.release).await?;

        if !elf.exists() {
            return Err(format!("ELF not found at expected path: {:?}", elf));
        }

        verify_elf(&elf).await?;

        Ok(elf)
    }
}
