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

/// Temperature reading sent from the MCU to the application processor over IPC.
///
/// Serialized with postcard for the IPC channel; the `IpcSafe` bound ensures
/// no address-space-local constructs (pointers, references) cross the core boundary.
#[derive(Clone, Copy, Debug, IpcSafe, Deserialize, Serialize)]
pub struct SensorReading {
    /// Temperature in millidegrees Celsius (e.g. 25_000 = 25.000 °C).
    pub temperature_mdc: i32,
    /// Monotonic sequence counter; wraps at u32::MAX.
    pub seq: u32,
}
