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

use crate::compile::env::{HmiConfig, SharedMemConfig};
use crate::compile::web::HmiBuildCommand;
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

/// Injects the HMI build configuration as `CONSORTIUM_HMI_*` env vars onto a
/// pending `cargo build` command, so `consortium-hmi` can read them at compile
/// time with `option_env!`.
///
/// `CONSORTIUM_HMI_DIST` (the compiled frontend dist directory) is always set
/// when a config is present; `CONSORTIUM_HMI_DEBUG_URL` is set only when the
/// manifest declares a dev-server URL, so its absence lets debug builds fall
/// back to the static `csti://` handler. A `None` config injects nothing.
pub fn inject_hmi(cmd: &mut Command, config: Option<&HmiConfig>) {
    if let Some(hmi) = config {
        for (key, value) in hmi.env_pairs() {
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

/// Resolves the HMI frontend build command, preferring the
/// `CONSORTIUM_HMI_BUILD_CMD` override, then `vp`, then `npm` on PATH.
///
/// Mirrors [`resolve_cargo`]'s env-var-first pattern: an explicit override wins,
/// otherwise the first available known toolchain is used.
pub fn resolve_hmi_build_command() -> Result<HmiBuildCommand, String> {
    resolve_hmi_build_command_from(std::env::var("CONSORTIUM_HMI_BUILD_CMD").ok(), |tool| {
        resolve_cross_tool(tool).is_ok()
    })
}

/// The env-var/PATH resolution logic for [`resolve_hmi_build_command`], factored
/// out so it can be tested deterministically without touching the process's
/// global `PATH`/env (which parallel tests would race on).
fn resolve_hmi_build_command_from(
    env_cmd: Option<String>,
    has_tool: impl Fn(&str) -> bool,
) -> Result<HmiBuildCommand, String> {
    if let Some(cmd) = env_cmd {
        let parts: Vec<String> = cmd.split_whitespace().map(String::from).collect();
        if parts.is_empty() {
            return Err("CONSORTIUM_HMI_BUILD_CMD is set but empty".to_string());
        }
        return Ok(HmiBuildCommand::Custom(parts));
    }
    if has_tool("vp") {
        return Ok(HmiBuildCommand::VpBuild);
    }
    if has_tool("npm") {
        return Ok(HmiBuildCommand::NpmRunBuild);
    }
    Err("no HMI build tool found: set CONSORTIUM_HMI_BUILD_CMD, or install `vp` \
         or `npm` on PATH"
        .to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::env_value::{parse_u32, parse_u64};
    use std::collections::HashMap;
    use std::ffi::OsStr;

    /// Collects the `CONSORTIUM_*` env vars a builder set on a command, so a test
    /// can assert exactly what a downstream `cargo build` would see.
    fn injected_vars(cmd: &Command) -> HashMap<String, String> {
        cmd.as_std()
            .get_envs()
            .filter_map(|(k, v)| {
                let key = k.to_str()?;
                if !key.starts_with("CONSORTIUM_") {
                    return None;
                }
                // `get_envs` yields `None` for vars marked for removal; the
                // builder only ever sets, so a present value is expected.
                Some((key.to_string(), v?.to_str()?.to_string()))
            })
            .collect()
    }

    #[test]
    fn inject_shared_mem_sets_all_four_vars() {
        let cfg = SharedMemConfig {
            base: 0x204C_0000,
            length: 0x1000,
            uio: 3,
            mtu: 64,
        };
        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "IPC", Some(&cfg));

        let vars = injected_vars(&cmd);
        assert_eq!(
            vars.get("CONSORTIUM_IPC_BASE").map(String::as_str),
            Some("541851648")
        );
        assert_eq!(
            vars.get("CONSORTIUM_IPC_LENGTH").map(String::as_str),
            Some("4096")
        );
        assert_eq!(
            vars.get("CONSORTIUM_IPC_UIO").map(String::as_str),
            Some("3")
        );
        assert_eq!(
            vars.get("CONSORTIUM_IPC_MTU").map(String::as_str),
            Some("64")
        );
        assert_eq!(vars.len(), 4);
    }

    #[test]
    fn inject_shared_mem_none_injects_nothing() {
        // A missing `[ipc]`/`[dbg]` section must leave the consuming crate to
        // emit its own "build via consortium-builder" hint, not a silent zero.
        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "IPC", None);
        assert!(injected_vars(&cmd).is_empty());
    }

    #[test]
    fn inject_shared_mem_honours_prefix() {
        let cfg = SharedMemConfig {
            base: 0x204C_1000,
            length: 0x800,
            uio: 1,
            mtu: 64,
        };
        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "DBG", Some(&cfg));

        let vars = injected_vars(&cmd);
        assert!(vars.keys().all(|k| k.starts_with("CONSORTIUM_DBG_")));
        assert_eq!(
            vars.get("CONSORTIUM_DBG_BASE").map(String::as_str),
            Some("541855744")
        );
        assert!(!vars.contains_key("CONSORTIUM_IPC_BASE"));
    }

    /// End-to-end: a `Consortium.toml` `[ipc]` table parses into a
    /// [`SharedMemConfig`], the builder injects it as env vars, and parsing
    /// those values back with the same `const` helpers the application uses
    /// (`consortium-ipc-transport-memory`'s `config` module) recovers the
    /// original numbers. This is the contract that ties the manifest, the
    /// builder, and the compile-time `config` constants together.
    #[test]
    fn toml_ipc_section_round_trips_through_injected_env() {
        #[derive(serde::Deserialize)]
        struct Doc {
            ipc: SharedMemConfig,
        }

        // Hex base, decimal/hex mix, and `_` separators all per the manifest.
        let manifest = "\
            [ipc]\n\
            base   = 0x204C0000\n\
            length = 0x1000\n\
            uio    = 0\n\
            mtu    = 64\n";

        let doc: Doc = toml::from_str(manifest).expect("[ipc] should deserialize");
        let cfg = doc.ipc;

        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "IPC", Some(&cfg));
        let vars = injected_vars(&cmd);

        // Re-parse the injected strings exactly as the consuming crate does.
        let read = |suffix: &str| -> &str {
            vars.get(&format!("CONSORTIUM_IPC_{suffix}"))
                .map(String::as_str)
                .unwrap_or_else(|| panic!("missing CONSORTIUM_IPC_{suffix}"))
        };
        assert_eq!(parse_u64(read("BASE")), cfg.base);
        assert_eq!(parse_u64(read("LENGTH")), cfg.length);
        assert_eq!(parse_u32(read("UIO")), cfg.uio);
        assert_eq!(parse_u32(read("MTU")), cfg.mtu);

        // And the manifest's hex values landed where we expect.
        assert_eq!(cfg.base, 0x204C_0000);
        assert_eq!(cfg.uio, 0);
    }

    #[test]
    fn toml_ipc_section_rejects_unknown_fields() {
        // `SharedMemConfig` is `deny_unknown_fields`, so a typo'd key is a hard
        // error rather than a silently dropped layout parameter.
        #[derive(serde::Deserialize)]
        struct Doc {
            #[allow(dead_code)]
            ipc: SharedMemConfig,
        }
        let manifest = "[ipc]\nbase = 0\nlength = 0\nuio = 0\nmtu = 0\nbogus = 1\n";
        let err = match toml::from_str::<Doc>(manifest) {
            Ok(_) => panic!("unknown field `bogus` should have been rejected"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("bogus"), "unexpected error: {err}");
    }

    // ── HMI ─────────────────────────────────────────────────────────────────

    #[test]
    fn inject_hmi_sets_dist_and_debug_url() {
        let hmi = HmiConfig {
            dist: "/opt/app/dist".to_string(),
            debug_url: Some("http://localhost:5173".to_string()),
        };
        let mut cmd = Command::new("true");
        inject_hmi(&mut cmd, Some(&hmi));

        let vars = injected_vars(&cmd);
        assert_eq!(
            vars.get("CONSORTIUM_HMI_DIST").map(String::as_str),
            Some("/opt/app/dist")
        );
        assert_eq!(
            vars.get("CONSORTIUM_HMI_DEBUG_URL").map(String::as_str),
            Some("http://localhost:5173")
        );
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn inject_hmi_omits_debug_url_when_absent() {
        // No dev-server URL: only DIST is injected, so debug builds fall back to
        // the static `csti://` handler rather than a stale URL.
        let hmi = HmiConfig {
            dist: "/opt/app/dist".to_string(),
            debug_url: None,
        };
        let mut cmd = Command::new("true");
        inject_hmi(&mut cmd, Some(&hmi));

        let vars = injected_vars(&cmd);
        assert!(vars.contains_key("CONSORTIUM_HMI_DIST"));
        assert!(!vars.contains_key("CONSORTIUM_HMI_DEBUG_URL"));
    }

    #[test]
    fn inject_hmi_none_injects_nothing() {
        let mut cmd = Command::new("true");
        inject_hmi(&mut cmd, None);
        assert!(injected_vars(&cmd).is_empty());
    }

    /// End-to-end: a `Consortium.toml` `[hmi]` table parses into an
    /// [`HmiConfig`] and the builder injects exactly the `CONSORTIUM_HMI_*` env
    /// vars `consortium-hmi` reads at compile time.
    #[test]
    fn toml_hmi_section_round_trips_through_injected_env() {
        #[derive(serde::Deserialize)]
        struct Doc {
            hmi: HmiConfig,
        }
        let manifest = "\
            [hmi]\n\
            dist      = \"dist\"\n\
            debug_url = \"http://localhost:5173\"\n";

        let doc: Doc = toml::from_str(manifest).expect("[hmi] should deserialize");
        let mut cmd = Command::new("true");
        inject_hmi(&mut cmd, Some(&doc.hmi));

        let vars = injected_vars(&cmd);
        assert_eq!(
            vars.get("CONSORTIUM_HMI_DIST").map(String::as_str),
            Some("dist")
        );
        assert_eq!(
            vars.get("CONSORTIUM_HMI_DEBUG_URL").map(String::as_str),
            Some("http://localhost:5173")
        );
    }

    // ── TEE ─────────────────────────────────────────────────────────────────

    /// End-to-end: a `Consortium.toml` `[tee]` table parses into a [`Tee`] whose
    /// `uuid` is a real [`Uuid`]. The OP-TEE compiler injects that UUID under
    /// [`TEE_UUID_ENV`] (`config.ta_uuid.to_string()`), and each TA's `build.rs`
    /// reads the same variable back, so the manifest, the builder, and the TA
    /// build script all agree on one value.
    #[test]
    fn toml_tee_section_round_trips_to_injected_uuid() {
        use consortium_cfg_common::config::{TEE_UUID_ENV, Tee};

        #[derive(serde::Deserialize)]
        struct Doc {
            tee: Tee,
        }
        let manifest = "[tee]\nuuid = \"12345678-1234-1234-1234-123456789012\"\n";
        let doc: Doc = toml::from_str(manifest).expect("[tee] should deserialize");

        // Mirror the env var the OP-TEE compiler sets from the parsed UUID, then
        // check it round-trips back to the manifest's canonical form.
        let mut cmd = Command::new("true");
        cmd.env(TEE_UUID_ENV, doc.tee.uuid.to_string());
        let vars = injected_vars(&cmd);
        assert_eq!(
            vars.get(TEE_UUID_ENV).map(String::as_str),
            Some("12345678-1234-1234-1234-123456789012")
        );
        assert_eq!(TEE_UUID_ENV, "CONSORTIUM_TEE_UUID");
    }

    // ── HMI build command resolution ─────────────────────────────────────────

    #[test]
    fn hmi_command_override_beats_path_tools() {
        // An explicit override is honoured verbatim even when `vp`/`npm` exist.
        let cmd = resolve_hmi_build_command_from(Some("pnpm build".to_string()), |_| true)
            .expect("override resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Custom(vec!["pnpm".to_string(), "build".to_string()])
        );
    }

    #[test]
    fn hmi_command_prefers_vp_then_npm() {
        // With no override, `vp` wins when present.
        let cmd = resolve_hmi_build_command_from(None, |tool| tool == "vp" || tool == "npm")
            .expect("vp resolves");
        assert_eq!(cmd, HmiBuildCommand::VpBuild);

        // Falls back to `npm` when only it is on PATH.
        let cmd = resolve_hmi_build_command_from(None, |tool| tool == "npm").expect("npm resolves");
        assert_eq!(cmd, HmiBuildCommand::NpmRunBuild);
    }

    #[test]
    fn hmi_command_errors_when_nothing_available() {
        let err = resolve_hmi_build_command_from(None, |_| false)
            .expect_err("no tool and no override is an error");
        assert!(err.contains("CONSORTIUM_HMI_BUILD_CMD"));

        // An empty override is rejected rather than launching an empty program.
        let err = resolve_hmi_build_command_from(Some("   ".to_string()), |_| false)
            .expect_err("empty override is an error");
        assert!(err.contains("empty"));
    }

    #[test]
    fn injected_vars_helper_ignores_unrelated_env() {
        // Sanity check the test helper itself does not over-match.
        let mut cmd = Command::new("true");
        cmd.env(OsStr::new("PATH"), "/usr/bin");
        assert!(injected_vars(&cmd).is_empty());
    }
}
