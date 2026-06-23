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

use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

use consortium_common::config::HmiConfig;

use super::Compiler;
use crate::compile::utils::{
    clean_path, elf_path, resolve_cargo, resolve_cross_tool, resolve_rustc, verify_elf,
};

/// Error type for application compilation.
#[derive(Error, Debug)]
pub enum ApplicationCompileError {
    #[error("Failed to resolve sysroot: {0}")]
    SysrootError(String),

    #[error("Failed to resolve rustup tool: {0}")]
    RustupError(String),

    #[error("Cargo build failed: {0}")]
    BuildFailed(String),

    #[error("ELF not found at expected path: {0:?}")]
    ElfNotFound(PathBuf),

    #[error("{0}")]
    UtilityError(String),
}

/// Result of cross-compiler detection, including whether an explicit compiler was found.
struct CrossCompiler {
    /// Absolute path to the C cross-compiler, or `None` when falling back to the generic linker.
    cc: Option<String>,
    /// Absolute path to the C++ cross-compiler, or `None` when falling back to the generic linker.
    cxx: Option<String>,
}

impl CrossCompiler {
    /// Detect the cross-compiler in priority order:
    /// 1. `CC` / `CXX` env vars (explicit, platform-agnostic)
    /// 2. STM32MP2 / OpenSTLinux pattern (`aarch64-ostl-linux-gcc`)
    /// 3. i.MX Yocto pattern (`aarch64-poky-linux-gcc`)
    /// 4. `None` → generic Rust linker
    fn detect() -> Self {
        // 1. Explicit CC/CXX: honour whatever the user or SDK set
        if let Ok(cc) = std::env::var("CC") {
            let cxx = std::env::var("CXX").unwrap_or_else(|_| cc.clone());
            return CrossCompiler {
                cc: Some(cc),
                cxx: Some(cxx),
            };
        }

        // 2. STM32MP2 / OpenSTLinux
        if let (Ok(cc), Ok(cxx)) = (
            resolve_cross_tool("aarch64-ostl-linux-gcc"),
            resolve_cross_tool("aarch64-ostl-linux-g++"),
        ) {
            return CrossCompiler {
                cc: Some(cc),
                cxx: Some(cxx),
            };
        }

        // 3. i.MX Yocto (Poky)
        if let (Ok(cc), Ok(cxx)) = (
            resolve_cross_tool("aarch64-poky-linux-gcc"),
            resolve_cross_tool("aarch64-poky-linux-g++"),
        ) {
            return CrossCompiler {
                cc: Some(cc),
                cxx: Some(cxx),
            };
        }

        // 4. Nothing found
        CrossCompiler {
            cc: None,
            cxx: None,
        }
    }
}

struct SdkEnv {
    sysroot: String,
    cross: CrossCompiler,
    /// Tune flags from `OECORE_TUNE_CCARGS`
    arch_flags: String,
    /// Native sysroot used to strip SDK entries from PATH
    native_sysroot: String,
    /// Optional extra RUSTFLAGS (`OSTL_SDK_EXTRA_FLAGS`)
    extra_flags: String,
}

impl SdkEnv {
    fn resolve() -> Result<Self, ApplicationCompileError> {
        let sysroot = std::env::var("OSTL_SDK_SYSROOT")
            .or_else(|_| std::env::var("SDKTARGETSYSROOT"))
            .or_else(|_| std::env::var("OECORE_TARGET_SYSROOT"))
            .map_err(|_| {
                ApplicationCompileError::SysrootError(
                    "Could not determine target sysroot. \
                 Set OSTL_SDK_SYSROOT or source a Yocto environment-setup script."
                        .to_string(),
                )
            })?;

        if !PathBuf::from(&sysroot).exists() {
            return Err(ApplicationCompileError::SysrootError(format!(
                "Target sysroot does not exist: {sysroot}"
            )));
        }

        // OECORE_NATIVE_SYSROOT is optional; only needed for PATH sanitization
        let native_sysroot = std::env::var("OECORE_NATIVE_SYSROOT").unwrap_or_default();

        let arch_flags = std::env::var("OECORE_TUNE_CCARGS")
            .unwrap_or_else(|_| "-mcpu=cortex-a35+crc -mbranch-protection=standard".to_string());

        let extra_flags = std::env::var("OSTL_SDK_EXTRA_FLAGS").unwrap_or_default();

        Ok(SdkEnv {
            sysroot,
            cross: CrossCompiler::detect(),
            arch_flags,
            native_sysroot,
            extra_flags,
        })
    }

    fn rustflags(&self) -> String {
        let arch_link_args: String = self
            .arch_flags
            .split_whitespace()
            .map(|f| format!(" -C link-arg={f}"))
            .collect();

        let mut flags = match &self.cross.cc {
            Some(cc) => format!(
                "-C linker={cc} -C link-arg=--sysroot={sysroot}{arch_link_args}",
                sysroot = self.sysroot,
            ),
            None => format!(
                "-C link-arg=--sysroot={sysroot}{arch_link_args}",
                sysroot = self.sysroot,
            ),
        };

        if !self.extra_flags.is_empty() {
            flags.push(' ');
            flags.push_str(&self.extra_flags);
        }

        flags
    }
}

/// Build parameters for a Linux application (typically targeting an OpenSTLinux / Yocto A-core).
///
/// # Environment variables (in priority order)
///
/// Cross-compiler detection:
///   `CC` / `CXX`: explicit cross-compiler (any Yocto flavor)
///   Auto-detected: `aarch64-ostl-linux-gcc` (STM32MP2) or `aarch64-poky-linux-gcc` (i.MX Yocto)
///   Fallback: generic Rust linker
///
/// Sysroot:
///   `OSTL_SDK_SYSROOT` > `SDKTARGETSYSROOT` > `OECORE_TARGET_SYSROOT`
///
/// Rustup overrides:
///   `CONSORTIUM_RUSTC`, `CONSORTIUM_CARGO`
///
/// Extra flags:
///   `OSTL_SDK_EXTRA_FLAGS`: appended verbatim to RUSTFLAGS
///
/// HMI (when `hmi` is `Some`):
///   `CONSORTIUM_HMI_DIST`: absolute path to the compiled frontend dist directory
///   `CONSORTIUM_HMI_DEBUG_URL`: dev-server URL (only injected when present)
#[derive(Clone)]
pub struct ApplicationConfiguration {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    /// Rust target triple (e.g. `"aarch64-unknown-linux-gnu"`).
    pub target: String,
    /// Build in release mode.
    pub release: bool,
    /// Optional HMI config. `dist` must be an absolute path (resolved by the
    /// caller from the manifest directory before constructing this struct).
    pub hmi: Option<HmiConfig>,
}

/// Compiler for Linux applications targeting Yocto / OpenSTLinux A-cores.
pub struct ApplicationCompiler;

impl Compiler for ApplicationCompiler {
    type Configuration = ApplicationConfiguration;
    type Error = ApplicationCompileError;
    type Output = PathBuf;

    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error> {
        let sdk = SdkEnv::resolve()?;

        let rustc = resolve_rustc()
            .await
            .map_err(ApplicationCompileError::RustupError)?;
        let cargo = resolve_cargo()
            .await
            .map_err(ApplicationCompileError::RustupError)?;

        let rustflags = sdk.rustflags();
        let sanitized_path = clean_path(&sdk.native_sysroot);

        let mut cmd = Command::new(&cargo);
        cmd.arg("build")
            .arg("--target")
            .arg(&config.target)
            .current_dir(&config.path)
            .env("PATH", &sanitized_path)
            .env("RUSTC", &rustc)
            .env("RUSTFLAGS", &rustflags)
            .env("SYSROOT", &sdk.sysroot)
            .env("PKG_CONFIG_SYSROOT_DIR", &sdk.sysroot)
            .env(
                "PKG_CONFIG_PATH",
                format!(
                    "{s}/usr/lib/pkgconfig:{s}/usr/share/pkgconfig",
                    s = sdk.sysroot,
                ),
            );

        if let Some(ref cc) = sdk.cross.cc {
            cmd.arg("--config")
                .arg(format!("{}.linker={cc}", config.target));
            cmd.env("CC", format!("{cc} --sysroot={}", sdk.sysroot));
        }

        if let Some(ref cxx) = sdk.cross.cxx {
            cmd.env("CXX", format!("{cxx} --sysroot={}", sdk.sysroot));
        }

        if let Some(ref hmi) = config.hmi {
            cmd.env("CONSORTIUM_HMI_DIST", &hmi.dist);
            if let Some(ref url) = hmi.debug_url {
                cmd.env("CONSORTIUM_HMI_DEBUG_URL", url);
            }
        }

        if config.release {
            cmd.arg("--release");
        }

        let status = cmd
            .status()
            .await
            .map_err(|e| ApplicationCompileError::BuildFailed(e.to_string()))?;

        if !status.success() {
            return Err(ApplicationCompileError::BuildFailed(format!(
                "cargo build failed with status: {status}"
            )));
        }

        let elf = Self::output_path(&config).await?;

        if !elf.exists() {
            return Err(ApplicationCompileError::ElfNotFound(elf));
        }

        verify_elf(&elf)
            .await
            .map_err(ApplicationCompileError::UtilityError)?;

        Ok(elf)
    }

    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error> {
        elf_path(&config.path, &config.target, config.release)
            .await
            .map_err(ApplicationCompileError::UtilityError)
    }
}

/// Backward-compatible builder; delegates to [`ApplicationCompiler`].
///
/// Cross-compiler detection and environment handling is identical to
/// [`ApplicationCompiler`]. See [`ApplicationConfiguration`] for env var docs.
pub struct ApplicationBuilder {
    /// Path to the Cargo project root.
    pub path: PathBuf,
    pub release: bool,
    pub target: String,
    pub hmi: Option<HmiConfig>,
}

impl ApplicationBuilder {
    /// Builds the Linux application, returning the path to the output ELF.
    pub async fn build(&self) -> Result<PathBuf, String> {
        ApplicationCompiler::build(ApplicationConfiguration {
            path: self.path.clone(),
            target: self.target.clone(),
            release: self.release,
            hmi: self.hmi.clone(),
        })
        .await
        .map_err(|e| e.to_string())
    }

    /// Returns the expected output ELF path without building.
    pub async fn elf_path(&self) -> Result<PathBuf, String> {
        ApplicationCompiler::output_path(&ApplicationConfiguration {
            path: self.path.clone(),
            target: self.target.clone(),
            release: self.release,
            hmi: self.hmi.clone(),
        })
        .await
        .map_err(|e| e.to_string())
    }
}
