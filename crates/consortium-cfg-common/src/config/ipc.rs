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

use crate::config::codec::Codec;

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
    /// Channel name, used for codegen and logging.
    pub name: String,
    /// Doorbell peripheral name (e.g. `hsem`, `ipcc1`). Matched against the
    /// chip database's doorbell list.
    pub doorbell: String,
    /// Channel configuration pattern.
    pub pattern: IpcPattern,
    /// Pattern-specific channel configuration.
    pub config: IpcChannelConfig,
    /// Maximum transfer unit for this IPC channel.
    pub mtu: u32,
    /// Party acting as the transport primary (the Linux/A-core convention).
    /// Must name one of the [`endpoints`](Self::endpoints).
    pub primary: Party,
    /// Party acting as the transport secondary (the firmware/M-core
    /// convention). Must name one of the [`endpoints`](Self::endpoints).
    pub secondary: Party,
    /// Endpoint declarations keyed by visible core name.
    #[serde(default, rename = "party")]
    pub endpoints: HashMap<Party, IpcEndpoint>,
    /// Codec name for this channel's message type. Matched against the codec
    /// list. Defaults to [`Codec::Postcard`] when omitted.
    #[serde(default)]
    pub codec: Codec,
}

impl IpcChannel {
    pub const fn memory(&self) -> Option<MemoryRegion> {
        self.config.memory
    }

    pub const fn cache(&self) -> bool {
        self.config.cache
    }

    /// Whether this channel wires acknowledgement doorbell lines.
    pub const fn ack(&self) -> bool {
        self.config.ack
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
    /// Whether the controller-side mapping of this window is cacheable.
    #[serde(default)]
    pub cache: bool,
    /// Whether to wire acknowledgement doorbell lines for this channel.
    ///
    /// This makes `send_acked` — a send that resolves only once the peer has taken the
    /// message — *available*. It is a capability, not a mode: ordinary `send` stays
    /// fire-and-forget on the same channel and costs nothing extra, because whether an
    /// individual message wants acknowledging travels in that message's slot header.
    ///
    /// Setting it requires each party to declare an `ack_chan` alongside its `chan`. Those
    /// are two more doorbell channels per endpoint, which is the price of every line
    /// carrying exactly one waiter.
    #[serde(default)]
    pub ack: bool,
}

/// One core's endpoint on an IPC channel.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpcEndpoint {
    /// Transport channel index used by this core.
    pub chan: u32,
    /// Doorbell channel this core rings to acknowledge the peer's messages.
    ///
    /// Required when the channel sets `config.ack`, and rejected otherwise. Must not
    /// collide with any other doorbell channel on the same peripheral.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_chan: Option<u32>,
    /// Rust message type carried by this endpoint.
    #[serde(rename = "type")]
    pub r#type: String,
}
