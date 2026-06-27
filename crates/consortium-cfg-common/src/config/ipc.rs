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

use std::collections::HashMap;

use super::{MemoryRegion, Party};

/// `[ipc]` section: typed channels between visible cores.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ipc {
    /// `[[ipc.channel]]`: declared IPC channels.
    #[serde(default, rename = "channel")]
    pub channels: Vec<IpcChannel>,
}

/// A `[[ipc.channel]]` entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcChannel {
    /// Doorbell peripheral name (e.g. `hsem`, `ipcc1`). Matched against the
    /// chip database's doorbell list.
    pub doorbell: String,
    /// Channel configuration pattern.
    pub pattern: IpcPattern,
    /// Pattern-specific channel configuration.
    pub config: IpcChannelConfig,
    /// Maximum transfer unit for this IPC channel.
    pub mtu: u32,
    /// Endpoint declarations keyed by visible core name.
    #[serde(default, rename = "party")]
    pub endpoints: HashMap<Party, IpcEndpoint>,
}

impl IpcChannel {
    pub const fn memory(&self) -> Option<MemoryRegion> {
        self.config.memory
    }
}

/// IPC channel configuration pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IpcPattern {
    Memory,
}

/// Pattern-specific `[[ipc.channel]].config`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpcChannelConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryRegion>,
}

/// One core's endpoint on an IPC channel.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpcEndpoint {
    /// Transport channel index used by this core.
    pub chan: u32,
    /// Rust message type carried by this endpoint.
    #[serde(rename = "type")]
    pub r#type: String,
}
