use std::path::PathBuf;
use tokio::{io::AsyncReadExt, process::Command};

const TARGET_TRIPLE: &str = "aarch64-unknown-linux-gnu";

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
///   `OSTL_RUSTC`        — absolute path to upstream rustc (default: resolved via rustup)
///   `OSTL_CARGO`        — absolute path to upstream cargo (default: resolved via rustup)
///   `OSTL_SDK_SYSROOT`  — override target sysroot
///   `OSTL_SDK_EXTRA_FLAGS` — extra RUSTFLAGS appended verbatim
pub struct ApplicationBuilder {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    pub release: bool,
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

        let rustc = match std::env::var("OSTL_RUSTC") {
            Ok(v) => v,
            Err(_) => Self::resolve_rustup("rustc").await?,
        };
        let cargo = match std::env::var("OSTL_CARGO") {
            Ok(v) => v,
            Err(_) => Self::resolve_rustup("cargo").await?,
        };

        let rustflags = Self::build_rustflags(&sdk);
        let clean_path = Self::clean_path(&sdk.native_sysroot);

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--target")
            .arg(TARGET_TRIPLE)
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

        let elf = self.elf_path().await?;

        if !elf.exists() {
            return Err(format!("ELF not found at expected path: {:?}", elf));
        }

        Self::verify_elf(&elf).await?;

        Ok(elf)
    }

    /// Returns the expected ELF output path after a successful build.
    pub async fn elf_path(&self) -> Result<PathBuf, String> {
        let binary_name = self.binary_name().await?;
        let profile = if self.release { "release" } else { "debug" };

        Ok(self
            .path
            .join("target")
            .join(TARGET_TRIPLE)
            .join(profile)
            .join(binary_name))
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
        let cc_abs = Self::resolve_cross_tool("aarch64-ostl-linux-gcc")?;
        let cxx_abs = Self::resolve_cross_tool("aarch64-ostl-linux-g++")?;

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

    /// Strips ST SDK path entries from PATH using OECORE_NATIVE_SYSROOT as the
    /// anchor. All ST-injected entries are subdirectories of the native sysroot,
    /// so this is precise — upstream rustup/cargo/rustc remain reachable.
    fn clean_path(native_sysroot: &str) -> String {
        let path = std::env::var("PATH").unwrap_or_default();

        std::env::split_paths(&path)
            .filter(|p| {
                if native_sysroot.is_empty() {
                    // Fallback heuristic if OECORE_NATIVE_SYSROOT was somehow empty
                    let s = p.to_string_lossy();
                    !s.contains("aarch64-ostl_sdk-linux") && !s.contains("aarch64-ostl-linux")
                } else {
                    !p.starts_with(native_sysroot)
                }
            })
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(":")
    }

    /// Resolves a cross-tool binary to its absolute path by walking PATH.
    /// Must be called before PATH is sanitized.
    fn resolve_cross_tool(tool: &str) -> Result<String, String> {
        let path = std::env::var("PATH").unwrap_or_default();

        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(tool);
            if candidate.is_file() {
                return Ok(candidate.to_string_lossy().into_owned());
            }
        }

        Err(format!(
            "Cross tool '{}' not found on PATH. \
             Is the ST environment-setup script sourced?",
            tool
        ))
    }

    /// Resolves upstream cargo or rustc via `rustup which --toolchain stable`.
    async fn resolve_rustup(tool: &str) -> Result<String, String> {
        let output = Command::new("rustup")
            .args(["which", "--toolchain", "stable", tool])
            .output()
            .await
            .map_err(|e| format!("Failed to run rustup: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "rustup could not resolve '{}' for the stable toolchain: {}",
                tool,
                String::from_utf8_lossy(&output.stderr).trim(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Reads the binary name from Cargo.toml, preferring [[bin]] over [package].
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

    /// Verifies that the output file has a valid ELF magic header.
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
