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

pub mod doorbell;
pub mod hmi;
pub mod memory;
pub mod peripheral;
pub mod profile;
pub mod ta;

pub use doorbell::Doorbell;
pub use hmi::{Hmi, HmiBackend, HmiEngine};
pub use memory::{Memory, SharedMemory};
pub use peripheral::Peripheral;
pub use profile::Profile;
pub use ta::Ta;

use std::collections::HashMap;

/// A fully parsed `Consortium.toml` manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Build profile: which chip the project targets.
    pub profile: Profile,
    /// Shared-memory layout for IPC and debug channels.
    pub memory: Memory,
    /// Doorbell peripherals wired up for IPC signalling.
    #[serde(default, rename = "doorbell")]
    pub doorbells: Vec<Doorbell>,
    /// OP-TEE trusted applications bundled with the project.
    #[serde(default, rename = "ta")]
    pub tas: Vec<Ta>,
    /// Optional human-machine-interface (web UI) configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmi: Option<Hmi>,
    /// Peripheral resource assignments, keyed by peripheral name (e.g. `i2c1`).
    #[serde(default)]
    pub peripheral: HashMap<String, Peripheral>,
}
