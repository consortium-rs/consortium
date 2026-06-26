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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Shared-memory region layout for an IPC or debug channel.
///
/// Declared in `Consortium.toml` under `[ipc]` and `[dbg]` and injected as
/// compile-time environment variables by the builder before invoking
/// `cargo build`. The IPC and debug regions use the **same** on-wire format
/// (one shared descriptor at `base`, followed by shared slots); only the magic
/// in the descriptor header differs (`CIPC` vs `CDBG`).
///
/// There is a single descriptor and a single set of slots per region, shared by
/// both cores: the MCU reaches them through the raw physical `base`, while the
/// Linux application reaches the *same* memory through the UIO device named by
/// `uio` (so the application-side virtual address is dynamic and never hard
/// coded).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
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
}
