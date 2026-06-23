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

use consortium_common::config::SharedMemConfig;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tokio::process::Command;

/// Injects a shared-memory region's layout as `CONSORTIUM_<PREFIX>_*` env vars
/// onto a pending `cargo build` command, so the firmware/application crates can
/// read them at compile time with `option_env!`. A `None` config injects
/// nothing, leaving the consuming crate to emit its "build via consortium-builder"
/// hint.
pub fn inject_shared_mem(cmd: &mut Command, prefix: &str, config: Option<&SharedMemConfig>) {
    if let Some(cfg) = config {
        for (key, value) in cfg.env_pairs(prefix) {
            cmd.env(key, value);
        }
    }
}

/// Strips ST SDK path entries from PATH using OECORE_NATIVE_SYSROOT as the
/// anchor. All ST-injected entries are subdirectories of the native sysroot,
/// so this is precise - upstream rustup/cargo/rustc remain reachable.
pub fn clean_path(native_sysroot: &str) -> String {
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
pub fn resolve_cross_tool(tool: &str) -> Result<String, String> {
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
pub async fn resolve_rustup(tool: &str) -> Result<String, String> {
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
pub async fn binary_name(path: &Path) -> Result<String, String> {
    let cargo_toml_path = path.join("Cargo.toml");
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
pub async fn verify_elf(path: &PathBuf) -> Result<(), String> {
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

/// Resolves the cargo binary path, preferring `CONSORTIUM_CARGO` env var over `rustup which`.
pub async fn resolve_cargo() -> Result<String, String> {
    match std::env::var("CONSORTIUM_CARGO") {
        Ok(v) => Ok(v),
        Err(_) => resolve_rustup("cargo").await,
    }
}

/// Resolves the rustc binary path, preferring `CONSORTIUM_RUSTC` env var over `rustup which`.
pub async fn resolve_rustc() -> Result<String, String> {
    match std::env::var("CONSORTIUM_RUSTC") {
        Ok(v) => Ok(v),
        Err(_) => resolve_rustup("rustc").await,
    }
}

/// Returns the expected ELF output path after a successful build.
pub async fn elf_path(path: &Path, target: &str, release: bool) -> Result<PathBuf, String> {
    let binary_name = binary_name(path).await?;
    let profile = if release { "release" } else { "debug" };

    Ok(path
        .join("target")
        .join(target)
        .join(profile)
        .join(binary_name))
}
