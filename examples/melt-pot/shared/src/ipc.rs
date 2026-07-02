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

use consortium_ipc::IpcSafe;
use serde::{Deserialize, Serialize};

// These message types are referenced by name from the generated `consortium.gen.rs`
// modules (the `party.<name>.type` entries in `Consortium.toml`). Both directions
// of the `sensor` channel use the default postcard codec, so each type derives
// `serde::Serialize`/`Deserialize`. `IpcSafe` rejects address-space-local
// constructs (pointers, references) crossing the core boundary.

/// Temperature reading sent from the coprocessor (CM33 / CM7) to the application
/// processor over the `sensor` IPC channel.
#[derive(Clone, Copy, Debug, IpcSafe, Deserialize, Serialize)]
pub struct SensorReading {
    /// Temperature in millidegrees Celsius (e.g. 25_000 = 25.000 °C).
    pub temperature_mdc: i32,
    /// Monotonic sequence counter; wraps at u32::MAX.
    pub seq: u32,
}

/// Control command sent from the application processor back to the coprocessor
/// on the `sensor` channel (e.g. a new target set-point).
#[derive(Clone, Copy, Debug, IpcSafe, Deserialize, Serialize)]
pub struct Command {
    /// Requested set-point in millidegrees Celsius.
    pub set_point_mdc: i32,
}
