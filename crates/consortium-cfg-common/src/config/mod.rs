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

//! Vendor-agnostic `Consortium.toml` schema.

pub mod acore_name;
pub mod codec;
pub mod core;
pub mod dbg;
pub mod dts;
pub mod endpoints;
pub mod hmi;
pub mod ipc;
pub mod mcore_name;
pub mod memory;
pub mod party;
pub mod peripheral;
pub mod profile;
pub mod rcore_name;
pub mod remoteproc;
pub mod rvcore_name;
pub mod ta;
pub mod tee;
pub mod xcore_name;

pub use acore_name::AcoreName;
pub use codec::Codec;
pub use core::{Core, VisibleCore};
pub use dbg::{Dbg, DbgCore, DbgMode};
pub use dts::{Dts, DtsMode};
pub use endpoints::Endpoints;
pub use hmi::{Hmi, HmiBackend, HmiEngine, HmiLogical};
pub use ipc::{Ipc, IpcChannel, IpcChannelConfig, IpcEndpoint, IpcPattern};
pub use mcore_name::McoreName;
pub use memory::MemoryRegion;
pub use party::Party;
pub use peripheral::Peripheral;
pub use profile::{CSTI_MANIFEST_PATH_ENV, CSTI_MCU_PATH_ENV, Profile, Workspace};
pub use rcore_name::RcoreName;
pub use remoteproc::{
    Remoteproc, RemoteprocCore, RemoteprocRegion, RemoteprocSection, RemoteprocWindowOverride,
};
pub use rvcore_name::{RvCoreName, RvExtensions, RvXlen};
pub use ta::Ta;
pub use tee::{TEE_UUID_ENV, Tee};
pub use xcore_name::XcoreName;

use std::collections::HashMap;

/// A fully parsed `Consortium.toml` manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Build profile: which chip the project targets.
    pub profile: Profile,
    /// Build artifact roots keyed by runtime endpoint name.
    #[serde(default)]
    pub endpoints: Endpoints,
    /// Debug transport configuration.
    #[serde(default)]
    pub dbg: Dbg,
    /// Linux device-tree generation configuration.
    #[serde(default)]
    pub dts: Dts,
    /// Per-core Linux remoteproc memory-layout overrides.
    #[serde(default)]
    pub remoteproc: Remoteproc,
    /// Typed IPC channel declarations.
    #[serde(default)]
    pub ipc: Ipc,
    /// Trusted execution configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tee: Option<Tee>,
    /// Optional human-machine-interface (web UI) configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmi: Option<Hmi>,
    /// Peripheral resource assignments, keyed by peripheral name (e.g. `i2c1`).
    #[serde(default)]
    pub peripheral: HashMap<String, Peripheral>,
}
