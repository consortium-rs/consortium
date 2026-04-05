use std::path::PathBuf;
use tokio::process::Command;

use crate::compile::utils::{
    clean_path, elf_path, resolve_cargo, resolve_cross_tool, resolve_rustc, verify_elf,
};

/// Builds a Cargo application targeting the STM32MP2 Cortex-A35 (OpenSTLinux).
///
/// # Environment
///
/// Source the ST SDK environment-setup script before running:
///   `source /opt/st/.../environment-setup-cortexa35-ostl-linux`
///
/// This exports all required variables (`SDKTARGETSYSROOT`, `OECORE_NATIVE_SYSROOT`,
/// `OECORE_TUNE_CCARGS`, etc.) automatically.
///
/// Optional overrides:
///   `CONSORTIUM_RUSTC`        — absolute path to upstream rustc (default: resolved via rustup)
///   `CONSORTIUM_CARGO`        — absolute path to upstream cargo (default: resolved via rustup)
///   `OSTL_SDK_SYSROOT`  — override target sysroot
///   `OSTL_SDK_EXTRA_FLAGS` — extra RUSTFLAGS appended verbatim
///
/// TODO: Add support for NXP i.MX 9x series (i.MX 93, i.MX 95, etc.) by detecting environment-setup script for those platforms and adjusting tool names and flags accordingly.
pub struct ApplicationBuilder {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    pub release: bool,
    pub target: String,
}

struct SdkEnv {
    /// cortexa35-ostl-linux target sysroot
    sysroot: String,
    /// Absolute path to aarch64-ostl-linux-gcc (resolved before PATH sanitization)
    cc_abs: String,
    /// Absolute path to aarch64-ostl-linux-g++ (resolved before PATH sanitization)
    cxx_abs: String,
    /// Tune flags from OECORE_TUNE_CCARGS, e.g. "-mcpu=cortex-a35+crc -mbranch-protection=standard"
    arch_flags: String,
    /// ST SDK native sysroot, used to sanitize PATH
    native_sysroot: String,
    /// Optional extra RUSTFLAGS
    extra_flags: String,
}

impl ApplicationBuilder {
    pub async fn build(&self) -> Result<PathBuf, String> {
        let sdk = Self::sdk_env()?;

        let rustc = resolve_rustc().await?;
        let cargo = resolve_cargo().await?;

        let rustflags = Self::build_rustflags(&sdk);
        let clean_path = clean_path(&sdk.native_sysroot);

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--target")
            .arg(&self.target)
            .arg("--config")
            .arg("aarch64-unknown-linux-gnu.linker=".to_string() + &sdk.cc_abs)
            .current_dir(&self.path)
            // Sanitized PATH: ST rustc/tools no longer shadow upstream
            .env("PATH", &clean_path)
            // Explicit upstream rustc, not whatever PATH resolves
            .env("RUSTC", &rustc)
            .env("RUSTFLAGS", &rustflags)
            // For cc-rs based deps (openssl-sys, ring, etc.)
            .env("SYSROOT", &sdk.sysroot)
            .env("CC", format!("{} --sysroot={}", sdk.cc_abs, sdk.sysroot))
            .env("CXX", format!("{} --sysroot={}", sdk.cxx_abs, sdk.sysroot))
            // For deps that use pkg-config to find target libraries
            .env("PKG_CONFIG_SYSROOT_DIR", &sdk.sysroot)
            .env(
                "PKG_CONFIG_PATH",
                format!(
                    "{sysroot}/usr/lib/pkgconfig:{sysroot}/usr/share/pkgconfig",
                    sysroot = sdk.sysroot,
                ),
            );

        if self.release {
            cmd.arg("--release");
        }

        let status = cmd
            .status()
            .await
            .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

        if !status.success() {
            return Err(format!("cargo build failed with status: {}", status));
        }

        let elf = elf_path(&self.path, &self.target, self.release).await?;

        if !elf.exists() {
            return Err(format!("ELF not found at expected path: {:?}", elf));
        }

        verify_elf(&elf).await?;

        Ok(elf)
    }

    /// Resolves and validates all SDK environment variables.
    /// Must be called while the ST SDK PATH is still intact so cross tools
    /// can be resolved to absolute paths before sanitization.
    fn sdk_env() -> Result<SdkEnv, String> {
        let sysroot = std::env::var("OSTL_SDK_SYSROOT")
            .or_else(|_| std::env::var("SDKTARGETSYSROOT"))
            .or_else(|_| std::env::var("OECORE_TARGET_SYSROOT"))
            .map_err(|_| {
                "Could not determine target sysroot. \
                 Source the ST environment-setup script or set OSTL_SDK_SYSROOT."
                    .to_string()
            })?;

        let native_sysroot = std::env::var("OECORE_NATIVE_SYSROOT").map_err(|_| {
            "OECORE_NATIVE_SYSROOT is not set. \
                 Source the ST environment-setup script first."
                .to_string()
        })?;

        // Resolve cross tools to absolute paths NOW, before PATH is sanitized
        let cc_abs = resolve_cross_tool("aarch64-ostl-linux-gcc")?;
        let cxx_abs = resolve_cross_tool("aarch64-ostl-linux-g++")?;

        // Derive tune flags from ST's own variable; fall back to known-good defaults
        let arch_flags = std::env::var("OECORE_TUNE_CCARGS")
            .unwrap_or_else(|_| "-mcpu=cortex-a35+crc -mbranch-protection=standard".to_string());

        let extra_flags = std::env::var("OSTL_SDK_EXTRA_FLAGS").unwrap_or_default();

        if !PathBuf::from(&sysroot).exists() {
            return Err(format!("Target sysroot does not exist: {}", sysroot));
        }

        Ok(SdkEnv {
            sysroot,
            cc_abs,
            cxx_abs,
            arch_flags,
            native_sysroot,
            extra_flags,
        })
    }

    /// Builds the RUSTFLAGS string for cross-compilation.
    fn build_rustflags(sdk: &SdkEnv) -> String {
        // Map each space-separated GCC tune flag to a -C link-arg so the
        // linker driver passes them through to the assembler/linker correctly.
        let arch_link_args: String = sdk
            .arch_flags
            .split_whitespace()
            .map(|f| format!(" -C link-arg={}", f))
            .collect();

        let mut flags = format!(
            "-C linker={cc} -C link-arg=--sysroot={sysroot}{arch_link_args}",
            cc = sdk.cc_abs,
            sysroot = sdk.sysroot,
            arch_link_args = arch_link_args,
        );

        if !sdk.extra_flags.is_empty() {
            flags.push(' ');
            flags.push_str(&sdk.extra_flags);
        }

        flags
    }
}
