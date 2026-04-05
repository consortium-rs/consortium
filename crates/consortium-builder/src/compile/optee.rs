use std::path::PathBuf;

use tokio::process::Command;
use uuid::Uuid;

use crate::compile::utils::{elf_path, resolve_cargo, verify_elf};

// Only aarch64-unknown-linux-gnu is currently supported for OP-TEE builds. Armv7 targets (Cortex-A7) are not supported by the OP-TEE SDK, and Cortex-M targets are out of scope for now.
pub const OPTEE_TARGET: &str = "aarch64-unknown-linux-gnu";

pub struct OpteeBuilder {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    pub release: bool,
    pub linker: Option<String>,
}

pub struct OpteeEnvironment {
    pub ta_dev_kit_dir: PathBuf,
    pub optee_signer: PathBuf,
    pub ta_uuid: Uuid,
}

impl OpteeBuilder {
    pub async fn build(&self, env: &OpteeEnvironment) -> Result<PathBuf, String> {
        // Step 1: Build the TA using cargo with the OP-TEE target and custom linker if provided.
        let cargo = resolve_cargo().await?;
        let mut cmd = Command::new(&cargo);

        if !env.ta_dev_kit_dir.exists() {
            return Err(format!(
                "TA_DEV_KIT_DIR does not exist: {:?}",
                env.ta_dev_kit_dir
            ));
        }

        cmd.arg("build")
            .arg("--target")
            .arg(OPTEE_TARGET)
            .current_dir(&self.path)
            .env("TA_DEV_KIT_DIR", &env.ta_dev_kit_dir);

        if self.release {
            cmd.arg("--release");
        }

        if let Some(linker) = &self.linker {
            cmd.arg("--config")
                .arg(format!("aarch64-unknown-linux-gnu.linker={linker}"));
        }

        let status = cmd
            .status()
            .await
            .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

        if !status.success() {
            return Err(format!("Cargo build failed with status: {}", status));
        }

        let elf = elf_path(&self.path, OPTEE_TARGET, self.release).await?;

        if !elf.exists() {
            return Err(format!("ELF not found at expected path: {:?}", elf));
        }

        verify_elf(&elf).await?;

        // Step 2. Sign the TA using the OP-TEE signer tool and the provided UUID.
        if !env.optee_signer.exists() {
            return Err(format!(
                "OP-TEE signer script not found at expected path: {:?}",
                env.optee_signer
            ));
        }

        let ta_key = env.ta_dev_kit_dir.join("keys").join("default_ta.pem");
        if !ta_key.exists() {
            return Err(format!(
                "TA signing key not found at expected path: {:?}",
                ta_key
            ));
        }

        // Step 2.1. Check if python exists (python, python3, ...)
        let python = resolve_python().await?;

        // Step 2.2. Run `python <script> --uuid <uuid> --key <key> --in <in> --out <out>`
        // <out> is <uuid>.ta
        let ta = elf
            .parent()
            .ok_or_else(|| format!("Could not determine output directory for ELF: {:?}", elf))?
            .join(format!("{}.ta", env.ta_uuid));

        let status = Command::new(&python)
            .arg(&env.optee_signer)
            .arg("--uuid")
            .arg(env.ta_uuid.to_string())
            .arg("--key")
            .arg(&ta_key)
            .arg("--in")
            .arg(&elf)
            .arg("--out")
            .arg(&ta)
            .env("TA_DEV_KIT_DIR", &env.ta_dev_kit_dir)
            .status()
            .await
            .map_err(|e| format!("Failed to execute OP-TEE signer via {python}: {e}"))?;

        if !status.success() {
            return Err(format!("OP-TEE signing failed with status: {}", status));
        }

        if !ta.exists() {
            return Err(format!(
                "Signed TA not found at expected path after signing: {:?}",
                ta
            ));
        }

        Ok(ta)
    }
}

async fn resolve_python() -> Result<String, String> {
    for candidate in ["python", "python3"] {
        let status = Command::new(candidate).arg("--version").status().await;

        if let Ok(status) = status {
            if status.success() {
                return Ok(candidate.to_string());
            }
        }
    }

    Err("Could not find a working Python interpreter on PATH (tried: python, python3)".to_string())
}
