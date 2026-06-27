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

use super::Party;

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
    /// Endpoint declarations keyed by visible core name.
    #[serde(flatten)]
    pub endpoints: HashMap<Party, IpcEndpoint>,
}

/// One core's endpoint on an IPC channel.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpcEndpoint {
    /// Transport channel index used by this core.
    pub chan: u32,
    /// Rust message type carried by this endpoint.
    #[serde(rename = "type")]
    pub ty: String,
}
