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

/// `[memory]` section: the shared-memory layout used by the IPC and debug
/// transports.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Memory {
    /// `[memory.ipc]`: the IPC channel region.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipc: Option<SharedMemory>,
    /// `[memory.dbg]`: the defmt debug-log region.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbg: Option<SharedMemory>,
}

/// A single shared-memory region (`[memory.ipc]` or `[memory.dbg]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedMemory {
    /// Physical base address of the region, as seen by the M-core.
    pub base: u64,
    /// Length of the region in bytes.
    pub length: u64,
    /// Maximum transfer unit (payload bytes per slot) for the channel.
    pub mtu: u32,
}
