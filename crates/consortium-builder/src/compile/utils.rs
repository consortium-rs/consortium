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

//! Shared helpers for the compilers: environment injection and tool resolution.
//!
//! Two jobs, both of which every compiler needs and none of which belongs to any one of
//! them.
//!
//! **Environment injection** (`inject_shared_mem`, `inject_dbg`, `inject_hmi`) turns the
//! carriers in `compile::env` into the `CONSORTIUM_*` variables set on a
//! `cargo`/`bun` invocation. Numeric values are written as decimal strings so a
//! firmware-side consumer can const-parse them with `const-str` rather than depending on
//! the std-oriented schema crates.
//!
//! **Tool resolution** finds the right binary for a cross build: `resolve_cross_tool`
//! locates a prefixed toolchain binary, `clean_path` strips host directories that would
//! otherwise shadow SDK tools, and the HMI resolvers pick a frontend build command —
//! `CONSORTIUM_HMI_BUILD_CMD` if set, then `vp`, then `npm` for Cog; `bun tools/build.ts`
//! for Pocket. The override exists because the frontend sometimes has to be built
//! elsewhere: inside a Linux container the host's `node_modules/` is already populated,
//! so pointing the command at `/bin/true` lets the pipeline consume the existing dist.
//!
//! Each frontend resolver has a `fetch` twin that runs the package manager's `install`
//! instead, overridden by `CONSORTIUM_HMI_FETCH_CMD`. Installing is the network half of
//! the frontend build, so `csti fetch` owns it and `csti build --linux <DIR> --offline` does not
//! repeat it.

use crate::compile::env::{DbgConfig, HmiConfig, SharedMemConfig};
use crate::compile::web::HmiBuildCommand;
use crate::progress::BuildReporter;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncRead, AsyncReadExt};
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

/// Injects the debug ring's `CONSORTIUM_DBG_BASE`/`_LENGTH` env vars onto a
/// pending `cargo build` command, so `consortium-dbg`'s `config` module can read
/// them at compile time with `option_env!`. A `None` config injects nothing,
/// leaving the logger unconfigured (a no-op).
pub fn inject_dbg(cmd: &mut Command, config: Option<&DbgConfig>) {
    if let Some(cfg) = config {
        for (key, value) in cfg.env_pairs() {
            cmd.env(key, value);
        }
    }
}

/// Injects the HMI build configuration as `CONSORTIUM_HMI_*` env vars onto a
/// pending `cargo build` command, so `consortium-hmi-webkit` can read them at compile
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

/// Runs a child command, streaming its output through `reporter` when present.
///
/// Without a reporter, the command inherits stdout/stderr just like
/// `Command::status`; with a reporter, output is piped so the CLI can redraw its
/// sticky status prompt after each chunk.
pub async fn run_command(
    cmd: &mut Command,
    reporter: Option<&dyn BuildReporter>,
) -> std::io::Result<ExitStatus> {
    let Some(reporter) = reporter else {
        return cmd.status().await;
    };

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let stdout_stream = stream_output(stdout, reporter);
    let stderr_stream = stream_output(stderr, reporter);
    let wait = child.wait();
    let (stdout_result, stderr_result, status) = tokio::join!(stdout_stream, stderr_stream, wait);
    stdout_result?;
    stderr_result?;
    status
}

async fn stream_output<R>(reader: Option<R>, reporter: &dyn BuildReporter) -> std::io::Result<()>
where
    R: AsyncRead + Unpin,
{
    let Some(mut reader) = reader else {
        return Ok(());
    };
    let mut buf = [0_u8; 8192];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        reporter.command_output(&String::from_utf8_lossy(&buf[..n]));
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
        resolve_cross_tool(tool).ok()
    })
}

/// The env-var/PATH resolution logic for [`resolve_hmi_build_command`], factored
/// out so it can be tested deterministically without touching the process's
/// global `PATH`/env (which parallel tests would race on).
fn resolve_hmi_build_command_from(
    env_cmd: Option<String>,
    resolve_tool: impl Fn(&str) -> Option<String>,
) -> Result<HmiBuildCommand, String> {
    if let Some(cmd) = env_cmd {
        let parts: Vec<String> = cmd.split_whitespace().map(String::from).collect();
        if parts.is_empty() {
            return Err("CONSORTIUM_HMI_BUILD_CMD is set but empty".to_string());
        }
        return Ok(HmiBuildCommand::Custom(parts));
    }
    if let Some(program) = resolve_tool("vp") {
        return Ok(HmiBuildCommand::Resolved {
            program,
            args: vec!["build".to_string()],
        });
    }
    if let Some(program) = resolve_tool("npm") {
        return Ok(HmiBuildCommand::Resolved {
            program,
            args: vec!["run".to_string(), "build".to_string()],
        });
    }
    Err(
        "no HMI build tool found: set CONSORTIUM_HMI_BUILD_CMD, or install `vp` \
         or `npm` on PATH"
            .to_string(),
    )
}

/// Resolves the PocketJS frontend build command for `app`, writing artifacts to
/// `outdir` (the manifest's resolved `[hmi].build`).
///
/// A `CONSORTIUM_HMI_BUILD_CMD` override wins verbatim; otherwise this runs
/// `bun tools/build.ts <app> --outdir=<outdir>` from the project directory.
pub fn resolve_pocket_build_command(app: &str, outdir: &Path) -> Result<HmiBuildCommand, String> {
    resolve_pocket_build_command_from(
        std::env::var("CONSORTIUM_HMI_BUILD_CMD").ok(),
        app,
        outdir,
        |tool| resolve_cross_tool(tool).ok(),
    )
}

/// Testable core of [`resolve_pocket_build_command`]; see
/// [`resolve_hmi_build_command_from`] for why resolution is injected.
fn resolve_pocket_build_command_from(
    env_cmd: Option<String>,
    app: &str,
    outdir: &Path,
    resolve_tool: impl Fn(&str) -> Option<String>,
) -> Result<HmiBuildCommand, String> {
    if let Some(cmd) = env_cmd {
        let parts: Vec<String> = cmd.split_whitespace().map(String::from).collect();
        if parts.is_empty() {
            return Err("CONSORTIUM_HMI_BUILD_CMD is set but empty".to_string());
        }
        return Ok(HmiBuildCommand::Custom(parts));
    }
    let program = resolve_tool("bun").ok_or_else(|| {
        "PocketJS build needs `bun`: install it (https://bun.sh) or set \
         CONSORTIUM_HMI_BUILD_CMD"
            .to_string()
    })?;
    Ok(HmiBuildCommand::Resolved {
        program,
        args: vec![
            "tools/build.ts".to_string(),
            app.to_string(),
            format!("--outdir={}", outdir.display()),
        ],
    })
}

/// Resolves the command that installs the Cog frontend's dependencies,
/// preferring the `CONSORTIUM_HMI_FETCH_CMD` override, then `vp`, then `npm`.
///
/// This is the network half of the frontend build, which is why it is a separate
/// command from [`resolve_hmi_build_command`]: `csti fetch` runs the installer so
/// `csti build --linux <DIR> --offline` can run against populated `node_modules/`.
pub fn resolve_hmi_fetch_command() -> Result<HmiBuildCommand, String> {
    resolve_hmi_fetch_command_from(std::env::var(HMI_FETCH_CMD_ENV).ok(), |tool| {
        resolve_cross_tool(tool).ok()
    })
}

/// The override variable that replaces the resolved HMI dependency installer.
///
/// Mirrors `CONSORTIUM_HMI_BUILD_CMD`, and has the same escape-hatch role: in the
/// Linux container flow the host already populated `node_modules/`, so setting
/// this to `/bin/true` skips the install without skipping the stage.
pub const HMI_FETCH_CMD_ENV: &str = "CONSORTIUM_HMI_FETCH_CMD";

/// Testable core of [`resolve_hmi_fetch_command`]; see
/// [`resolve_hmi_build_command_from`] for why resolution is injected.
fn resolve_hmi_fetch_command_from(
    env_cmd: Option<String>,
    resolve_tool: impl Fn(&str) -> Option<String>,
) -> Result<HmiBuildCommand, String> {
    if let Some(cmd) = custom_command(env_cmd)? {
        return Ok(cmd);
    }
    for tool in ["vp", "npm"] {
        if let Some(program) = resolve_tool(tool) {
            return Ok(HmiBuildCommand::Resolved {
                program,
                args: vec!["install".to_string()],
            });
        }
    }
    Err(format!(
        "no HMI package manager found: set {HMI_FETCH_CMD_ENV}, or install `vp` \
         or `npm` on PATH"
    ))
}

/// Resolves the command that installs the PocketJS frontend's dependencies.
///
/// A `CONSORTIUM_HMI_FETCH_CMD` override wins verbatim; otherwise `bun install`.
pub fn resolve_pocket_fetch_command() -> Result<HmiBuildCommand, String> {
    resolve_pocket_fetch_command_from(std::env::var(HMI_FETCH_CMD_ENV).ok(), |tool| {
        resolve_cross_tool(tool).ok()
    })
}

/// Testable core of [`resolve_pocket_fetch_command`].
fn resolve_pocket_fetch_command_from(
    env_cmd: Option<String>,
    resolve_tool: impl Fn(&str) -> Option<String>,
) -> Result<HmiBuildCommand, String> {
    if let Some(cmd) = custom_command(env_cmd)? {
        return Ok(cmd);
    }
    let program = resolve_tool("bun").ok_or_else(|| {
        format!(
            "PocketJS dependency install needs `bun`: install it (https://bun.sh) \
             or set {HMI_FETCH_CMD_ENV}"
        )
    })?;
    Ok(HmiBuildCommand::Resolved {
        program,
        args: vec!["install".to_string()],
    })
}

/// Splits an override variable into a [`HmiBuildCommand::Custom`], rejecting a
/// set-but-blank value rather than launching an empty program.
fn custom_command(env_cmd: Option<String>) -> Result<Option<HmiBuildCommand>, String> {
    let Some(cmd) = env_cmd else {
        return Ok(None);
    };
    let parts: Vec<String> = cmd.split_whitespace().map(String::from).collect();
    if parts.is_empty() {
        return Err("HMI command override is set but empty".to_string());
    }
    Ok(Some(HmiBuildCommand::Custom(parts)))
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
    fn inject_shared_mem_sets_all_ipc_vars() {
        let cfg = SharedMemConfig {
            length: 0x1000,
            uio: 3,
            mtu: 64,
        };
        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "IPC", Some(&cfg));

        let vars = injected_vars(&cmd);
        // No base: nothing reads `CONSORTIUM_IPC_BASE`.
        assert!(!vars.contains_key("CONSORTIUM_IPC_BASE"));
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
        assert_eq!(vars.len(), 3);
    }

    #[test]
    fn inject_shared_mem_none_injects_nothing() {
        // A missing `[ipc]` section must leave the consuming crate to emit its own
        // "build via consortium-builder" hint, not a silent zero.
        let mut cmd = Command::new("true");
        inject_shared_mem(&mut cmd, "IPC", None);
        assert!(injected_vars(&cmd).is_empty());
    }

    #[test]
    fn inject_dbg_emits_base_and_length() {
        // The debug carrier keeps the base + length `consortium-dbg` reads.
        let cfg = DbgConfig::new(0x204C_1000, 0x800);
        let mut cmd = Command::new("true");
        inject_dbg(&mut cmd, Some(&cfg));

        let vars = injected_vars(&cmd);
        assert!(vars.keys().all(|k| k.starts_with("CONSORTIUM_DBG_")));
        assert_eq!(
            vars.get("CONSORTIUM_DBG_BASE").map(String::as_str),
            Some("541855744")
        );
        assert_eq!(
            vars.get("CONSORTIUM_DBG_LENGTH").map(String::as_str),
            Some("2048")
        );
        assert!(!vars.contains_key("CONSORTIUM_IPC_BASE"));
    }

    #[test]
    fn inject_dbg_none_injects_nothing() {
        let mut cmd = Command::new("true");
        inject_dbg(&mut cmd, None);
        assert!(injected_vars(&cmd).is_empty());
    }

    /// End-to-end: a `Consortium.toml` `[ipc]` table parses into a
    /// [`SharedMemConfig`], the builder injects it as env vars, and parsing those
    /// values back as decimals recovers the original numbers.
    /// Hex TOML literals are normalized to decimal env-var strings.
    #[test]
    fn toml_ipc_section_round_trips_through_injected_env() {
        #[derive(serde::Deserialize)]
        struct Doc {
            ipc: SharedMemConfig,
        }

        // No base: the IPC carrier resolves its region from generated code, not
        // the environment.
        let manifest = "\
            [ipc]\n\
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
        assert_eq!(read("LENGTH"), "4096");
        assert_eq!(read("UIO"), "0");
        assert_eq!(read("MTU"), "64");
        assert_eq!(read("LENGTH").parse::<u64>().unwrap(), cfg.length);
        assert_eq!(read("UIO").parse::<u32>().unwrap(), cfg.uio);
        assert_eq!(read("MTU").parse::<u32>().unwrap(), cfg.mtu);

        assert!(!vars.contains_key("CONSORTIUM_IPC_BASE"));
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
        let manifest = "[ipc]\nlength = 0\nuio = 0\nmtu = 0\nbogus = 1\n";
        let err = match toml::from_str::<Doc>(manifest) {
            Ok(_) => panic!("unknown field `bogus` should have been rejected"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("bogus"), "unexpected error: {err}");
    }

    // ── HMI ─────────────────────────────────────────────────────────────────

    #[test]
    fn inject_hmi_sets_dist_and_debug_url() {
        let hmi = HmiConfig::Cog {
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
        let hmi = HmiConfig::Cog {
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
    /// vars `consortium-hmi-webkit` reads at compile time.
    #[test]
    fn toml_hmi_section_round_trips_through_injected_env() {
        #[derive(serde::Deserialize)]
        struct Doc {
            hmi: HmiConfig,
        }
        let manifest = "\
            [hmi]\n\
            engine    = \"cog\"\n\
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
        let cmd = resolve_hmi_build_command_from(Some("pnpm build".to_string()), |_| {
            Some("/tools/ignored".to_string())
        })
        .expect("override resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Custom(vec!["pnpm".to_string(), "build".to_string()])
        );
    }

    #[test]
    fn hmi_command_prefers_vp_then_npm() {
        // With no override, `vp` wins when present and keeps its resolved path.
        let cmd = resolve_hmi_build_command_from(None, |tool| Some(format!("/tools/{tool}")))
            .expect("vp resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/vp".to_string(),
                args: vec!["build".to_string()],
            }
        );

        // Falls back to `npm` when only it is on PATH.
        let cmd = resolve_hmi_build_command_from(None, |tool| {
            (tool == "npm").then(|| "/tools/npm".to_string())
        })
        .expect("npm resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/npm".to_string(),
                args: vec!["run".to_string(), "build".to_string()],
            }
        );
    }

    #[test]
    fn hmi_command_errors_when_nothing_available() {
        let err = resolve_hmi_build_command_from(None, |_| None)
            .expect_err("no tool and no override is an error");
        assert!(err.contains("CONSORTIUM_HMI_BUILD_CMD"));

        // An empty override is rejected rather than launching an empty program.
        let err = resolve_hmi_build_command_from(Some("   ".to_string()), |_| None)
            .expect_err("empty override is an error");
        assert!(err.contains("empty"));
    }

    #[test]
    fn pocket_command_runs_bun_build_with_outdir() {
        let cmd =
            resolve_pocket_build_command_from(None, "hero-main", Path::new("/proj/dist"), |tool| {
                (tool == "bun").then(|| "/tools/bun".to_string())
            })
            .expect("bun resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/bun".to_string(),
                args: vec![
                    "tools/build.ts".to_string(),
                    "hero-main".to_string(),
                    "--outdir=/proj/dist".to_string(),
                ],
            }
        );
    }

    #[test]
    fn pocket_command_override_beats_bun() {
        let cmd = resolve_pocket_build_command_from(
            Some("just pocket hero-main".to_string()),
            "hero-main",
            Path::new("/proj/dist"),
            |_| Some("/tools/ignored".to_string()),
        )
        .expect("override resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Custom(vec![
                "just".to_string(),
                "pocket".to_string(),
                "hero-main".to_string(),
            ])
        );
    }

    #[test]
    fn pocket_command_errors_without_bun() {
        let err =
            resolve_pocket_build_command_from(None, "hero-main", Path::new("/proj/dist"), |_| None)
                .expect_err("missing bun is an error");
        assert!(err.contains("bun"));
    }

    // ── HMI dependency-install resolution ────────────────────────────────────

    #[test]
    fn hmi_fetch_command_prefers_vp_then_npm() {
        // The installer is the network half of the frontend build, so it resolves
        // the same tools as the builder but runs their `install` subcommand.
        let cmd = resolve_hmi_fetch_command_from(None, |tool| Some(format!("/tools/{tool}")))
            .expect("vp resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/vp".to_string(),
                args: vec!["install".to_string()],
            }
        );

        let cmd = resolve_hmi_fetch_command_from(None, |tool| {
            (tool == "npm").then(|| "/tools/npm".to_string())
        })
        .expect("npm resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/npm".to_string(),
                args: vec!["install".to_string()],
            }
        );
    }

    #[test]
    fn hmi_fetch_command_override_beats_path_tools() {
        // The container flow points this at `/bin/true`: the host already
        // populated `node_modules/`, so the install must not run again.
        let cmd = resolve_hmi_fetch_command_from(Some("/bin/true".to_string()), |_| {
            Some("/tools/ignored".to_string())
        })
        .expect("override resolves");
        assert_eq!(cmd, HmiBuildCommand::Custom(vec!["/bin/true".to_string()]));
    }

    #[test]
    fn hmi_fetch_command_errors_when_nothing_available() {
        let err = resolve_hmi_fetch_command_from(None, |_| None)
            .expect_err("no package manager and no override is an error");
        assert!(err.contains(HMI_FETCH_CMD_ENV));

        let err = resolve_hmi_fetch_command_from(Some("  ".to_string()), |_| None)
            .expect_err("empty override is an error");
        assert!(err.contains("empty"));
    }

    #[test]
    fn pocket_fetch_command_runs_bun_install() {
        let cmd = resolve_pocket_fetch_command_from(None, |tool| {
            (tool == "bun").then(|| "/tools/bun".to_string())
        })
        .expect("bun resolves");
        assert_eq!(
            cmd,
            HmiBuildCommand::Resolved {
                program: "/tools/bun".to_string(),
                args: vec!["install".to_string()],
            }
        );

        let err =
            resolve_pocket_fetch_command_from(None, |_| None).expect_err("missing bun is an error");
        assert!(err.contains("bun"));
    }

    #[test]
    fn injected_vars_helper_ignores_unrelated_env() {
        // Sanity check the test helper itself does not over-match.
        let mut cmd = Command::new("true");
        cmd.env(OsStr::new("PATH"), "/usr/bin");
        assert!(injected_vars(&cmd).is_empty());
    }
}
