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

use super::VisibleCore;

/// A `[peripheral.<name>]` entry: the ownership and pinout of one peripheral.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peripheral {
    /// Core that owns this peripheral (e.g. `ap`, `cm33`).
    pub owner: VisibleCore,
    /// Whether the peripheral lives in the secure domain.
    #[serde(default)]
    pub secure: bool,
    /// Bus frequency in Hz, for peripherals that need one (e.g. an I2C bus
    /// clock). Ignored by peripherals without a configurable rate, such as GPIO.
    #[serde(default)]
    pub frequency: Option<u32>,
    /// Input/functional clock in Hz feeding the peripheral, used to derive
    /// dividers (e.g. an I2C instance's kernel clock). Required for generated
    /// I2C bring-up; ignored by GPIO.
    #[serde(default)]
    pub clock: Option<u32>,
    /// Signal-to-pad assignments, keyed by the vendor's signal name and valued
    /// by the vendor's pad name: `sda = "PA2"` on STM32MP2 (`I2C1_SDA` on PA2),
    /// `sin = "UART1_RXD"` on i.MX 95 (`LPSPI2_SIN` on pad `UART1_RXD`).
    ///
    /// Validated against the chip's signal map (`consortium_data::signal`): the
    /// pad must carry `<NAME>_<SIGNAL>`, must be bonded out in the package that
    /// `[profile] chip` names (when the part number is specific enough to name
    /// one), and may be assigned only once across the manifest.
    #[serde(default)]
    pub pin: HashMap<String, String>,
}
