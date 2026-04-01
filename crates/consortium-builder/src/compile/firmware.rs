use consortium_common::config::mcu_core::CortexMcuCore;
use std::path::PathBuf;
use tokio::{io::AsyncReadExt, process::Command};

pub struct FirmwareBuilder {
    pub arch: CortexMcuCore,
    pub path: PathBuf,
    pub release: bool,
}

impl FirmwareBuilder {
    /// Returns the expected ELF output path after a successful build.
    pub async fn elf_path(&self) -> Result<PathBuf, String> {
        let binary_name = self.binary_name().await?;
        let profile = if self.release { "release" } else { "debug" };
        let target = self.arch.to_rust_target();

        Ok(self
            .path
            .join("target")
            .join(target)
            .join(profile)
            .join(binary_name))
    }

    /// Reads the binary name from the firmware's Cargo.toml.
    async fn binary_name(&self) -> Result<String, String> {
        let cargo_toml_path = self.path.join("Cargo.toml");
        let content = tokio::fs::read_to_string(&cargo_toml_path)
            .await
            .map_err(|e| format!("Failed to read Cargo.toml at {:?}: {}", cargo_toml_path, e))?;

        let manifest: toml::Value =
            toml::from_str(&content).map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

        // 1. Prefer [[bin]] name if explicitly declared
        if let Some(name) = manifest
            .get("bin")
            .and_then(|b| b.as_array())
            .and_then(|arr| arr.first())
            .and_then(|b| b.get("name"))
            .and_then(|n| n.as_str())
        {
            return Ok(name.to_string());
        }

        // 2. Fall back to [package] name (default binary)
        manifest
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .map(|n| n.to_string())
            .ok_or_else(|| "Could not determine binary name from Cargo.toml".to_string())
    }

    pub async fn build(&self) -> Result<PathBuf, String> {
        let mut cmd = Command::new("cargo");
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

        let elf = self.elf_path().await?;

        if !elf.exists() {
            return Err(format!("ELF not found at expected path: {:?}", elf));
        }

        Self::verify_elf(&elf).await?;

        Ok(elf)
    }

    /// Verifies that the file at the given path is a valid ELF binary.
    async fn verify_elf(path: &PathBuf) -> Result<(), String> {
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|e| format!("Failed to open ELF file {:?}: {}", path, e))?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)
            .await
            .map_err(|e| format!("Failed to read ELF magic bytes: {}", e))?;

        if magic != [0x7f, b'E', b'L', b'F'] {
            return Err(format!(
                "File {:?} is not a valid ELF binary (magic: {:02x?})",
                path, magic
            ));
        }

        Ok(())
    }
}
