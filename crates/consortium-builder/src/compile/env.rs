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

//! Compile-time environment-injection carriers.
//!
//! The builder lowers high-level build parameters into the `CONSORTIUM_*`
//! environment variables that firmware/application/HMI crates read with
//! `option_env!` at compile time. These types describe *what* the builder
//! injects, not the parsed `Consortium.toml` schema (that lives in
//! `consortium-cfg-common`); the mapping from the current schema onto these
//! carriers is still being reconciled, so they stay local to the builder.

use serde::{Deserialize, Serialize};

/// Shared-memory region layout for an IPC or debug channel.
///
/// Injected as compile-time environment variables by the builder before
/// invoking `cargo build`. The IPC and debug regions use the **same** on-wire
/// format (one shared descriptor at `base`, followed by shared slots); only the
/// magic in the descriptor header differs (`CIPC` vs `CDBG`).
///
/// There is a single descriptor and a single set of slots per region, shared by
/// both cores: the MCU reaches them through the raw physical `base`, while the
/// Linux application reaches the *same* memory through the UIO device named by
/// `uio` (so the application-side virtual address is dynamic and never hard
/// coded).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SharedMemConfig {
    /// Physical base address of the region, as seen by the MCU (raw memory).
    pub base: u64,
    /// Length of the region in bytes.
    pub length: u64,
    /// UIO device index the Linux application mmaps to reach the same region.
    pub uio: u32,
    /// Maximum transfer unit (payload bytes per slot) for the channel.
    pub mtu: u32,
}

impl SharedMemConfig {
    /// Environment variable suffixes used when the builder injects this config.
    ///
    /// The builder emits `CONSORTIUM_<PREFIX>_BASE`, `_LENGTH`, `_UIO`, and
    /// `_MTU`, where `<PREFIX>` is `IPC` or `DBG`. Consuming crates read these
    /// with `option_env!` and parse them in a `const` context.
    pub fn env_pairs(&self, prefix: &str) -> [(String, String); 4] {
        [
            (format!("CONSORTIUM_{prefix}_BASE"), self.base.to_string()),
            (
                format!("CONSORTIUM_{prefix}_LENGTH"),
                self.length.to_string(),
            ),
            (format!("CONSORTIUM_{prefix}_UIO"), self.uio.to_string()),
            (format!("CONSORTIUM_{prefix}_MTU"), self.mtu.to_string()),
        ]
    }
}

/// HMI build configuration consumed by `consortium-hmi` at compile time.
///
/// Injected as environment variables by `ApplicationCompiler` before invoking
/// `cargo build`. `consortium-hmi` reads them via `option_env!` at compile
/// time; no file I/O occurs on the target device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct HmiConfig {
    /// Absolute path to the compiled frontend dist directory.
    /// Served via the `csti://` protocol in release builds.
    /// Relative paths in `Consortium.toml` are resolved to absolute by the
    /// builder before injection.
    pub dist: String,
    /// Dev-server URL loaded directly in debug builds.
    /// When absent, debug builds fall back to the `csti://` static handler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_url: Option<String>,
}

impl HmiConfig {
    /// Environment variables the builder injects for this config.
    ///
    /// `CONSORTIUM_HMI_DIST` is always emitted; `CONSORTIUM_HMI_DEBUG_URL` is
    /// emitted only when [`debug_url`](Self::debug_url) is set, so its absence
    /// lets debug builds fall back to the static `csti://` handler. Consuming
    /// crates read these with `option_env!` at compile time.
    pub fn env_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![("CONSORTIUM_HMI_DIST".to_string(), self.dist.clone())];
        if let Some(url) = &self.debug_url {
            pairs.push(("CONSORTIUM_HMI_DEBUG_URL".to_string(), url.clone()));
        }
        pairs
    }
}

/// Cortex-M core a firmware crate targets; selects the Rust target triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CortexMcuCore {
    CortexM0p,
    CortexM33,
    CortexM7,
    CortexM4,
    CortexM4f,
}

impl CortexMcuCore {
    /// Parses a dash-separated core name string (e.g., `"cortex-m33"`) into a variant.
    pub fn as_key(s: &str) -> Option<Self> {
        match s {
            "cortex-m0p" => Some(Self::CortexM0p),
            "cortex-m33" => Some(Self::CortexM33),
            "cortex-m7" => Some(Self::CortexM7),
            "cortex-m4" => Some(Self::CortexM4),
            "cortex-m4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    /// Returns the short abbreviation for this core (e.g., `"cm33"`).
    pub fn abbr(&self) -> &'static str {
        match self {
            Self::CortexM0p => "cm0p",
            Self::CortexM33 => "cm33",
            Self::CortexM7 => "cm7",
            Self::CortexM4 => "cm4",
            Self::CortexM4f => "cm4f",
        }
    }

    /// Parses a short abbreviation (e.g., `"cm33"`) into a variant.
    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "cm0p" => Some(Self::CortexM0p),
            "cm33" => Some(Self::CortexM33),
            "cm7" => Some(Self::CortexM7),
            "cm4" => Some(Self::CortexM4),
            "cm4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    /// Returns the Rust target triple for cross-compiling to this core.
    pub fn to_rust_target(&self) -> &'static str {
        match self {
            Self::CortexM0p => "thumbv6m-none-eabi",
            Self::CortexM33 => "thumbv8m.main-none-eabi",
            Self::CortexM7 | Self::CortexM4 | Self::CortexM4f => "thumbv7em-none-eabi",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: SharedMemConfig = SharedMemConfig {
        base: 0x204C_0000,
        length: 0x1000,
        uio: 0,
        mtu: 64,
    };

    #[test]
    fn env_pairs_emit_the_ipc_layout_the_app_reads() {
        // Keys and string values must match exactly what `option_env!`/`env!`
        // read on the consuming side (`consortium-ipc-transport-memory`'s
        // `config` module). Decimal stringification keeps them `parse_u64`-able.
        let pairs = SAMPLE.env_pairs("IPC");
        assert_eq!(
            pairs,
            [
                ("CONSORTIUM_IPC_BASE".to_string(), "541851648".to_string()),
                ("CONSORTIUM_IPC_LENGTH".to_string(), "4096".to_string()),
                ("CONSORTIUM_IPC_UIO".to_string(), "0".to_string()),
                ("CONSORTIUM_IPC_MTU".to_string(), "64".to_string()),
            ]
        );
    }

    #[test]
    fn env_pairs_honour_the_prefix() {
        // The debug region uses the same layout under a `DBG` prefix.
        let keys: Vec<String> = SAMPLE
            .env_pairs("DBG")
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        assert_eq!(
            keys,
            [
                "CONSORTIUM_DBG_BASE",
                "CONSORTIUM_DBG_LENGTH",
                "CONSORTIUM_DBG_UIO",
                "CONSORTIUM_DBG_MTU",
            ]
        );
    }

    #[test]
    fn hmi_env_pairs_emit_dist_and_optional_debug_url() {
        let with_url = HmiConfig {
            dist: "ui/dist".to_string(),
            debug_url: Some("http://localhost:5173".to_string()),
        };
        assert_eq!(
            with_url.env_pairs(),
            [
                ("CONSORTIUM_HMI_DIST".to_string(), "ui/dist".to_string()),
                (
                    "CONSORTIUM_HMI_DEBUG_URL".to_string(),
                    "http://localhost:5173".to_string()
                ),
            ]
        );

        let no_url = HmiConfig {
            dist: "ui/dist".to_string(),
            debug_url: None,
        };
        assert_eq!(
            no_url.env_pairs(),
            [("CONSORTIUM_HMI_DIST".to_string(), "ui/dist".to_string())]
        );
    }

    #[test]
    fn mcu_core_resolves_rust_target_triples() {
        assert_eq!(
            CortexMcuCore::CortexM33.to_rust_target(),
            "thumbv8m.main-none-eabi"
        );
        assert_eq!(
            CortexMcuCore::CortexM7.to_rust_target(),
            "thumbv7em-none-eabi"
        );
        assert_eq!(
            CortexMcuCore::as_key("cortex-m33"),
            Some(CortexMcuCore::CortexM33)
        );
        assert_eq!(
            CortexMcuCore::from_abbr("cm0p"),
            Some(CortexMcuCore::CortexM0p)
        );
        assert_eq!(CortexMcuCore::CortexM4f.abbr(), "cm4f");
    }
}
