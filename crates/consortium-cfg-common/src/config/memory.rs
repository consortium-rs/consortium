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

/// A shared-memory region used by a pattern-specific feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryRegion {
    /// Physical base address of the region, as seen by the M-core.
    pub base: u32,
    /// Length of the region in bytes.
    pub length: u32,
}

/// A shared-memory region with MTU metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedMemory {
    /// Physical base address of the region, as seen by the M-core.
    pub base: u32,
    /// Length of the region in bytes.
    pub length: u32,
    /// Maximum transfer unit (payload bytes per slot) for the channel.
    pub mtu: u32,
}

impl SharedMemory {
    pub const fn region(self) -> MemoryRegion {
        MemoryRegion {
            base: self.base,
            length: self.length,
        }
    }
}
