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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::config::chip_map::Chip;
use crate::config::mcu_core::CortexMcuCore;

pub mod app_core;
pub mod chip_map;
#[cfg(feature = "serde")]
pub mod hmi;
#[cfg(feature = "serde")]
pub mod iomux;
#[cfg(all(feature = "serde", feature = "std"))]
pub mod iomux_loader;
pub mod mcu_core;
pub mod peripheral;
pub mod rif;
#[cfg(all(feature = "serde", feature = "std"))]
pub mod rifsc_loader;

// Convenient re-exports for RIF types
pub use rif::{RifConfig, RifPeripheral, RifSource};
// Convenient re-exports for peripheral types
#[cfg(feature = "serde")]
pub use hmi::HmiConfig;
#[cfg(feature = "serde")]
pub use iomux::{IomuxConfig, Pin, SignalAssignment};
#[cfg(all(feature = "serde", feature = "std"))]
pub use iomux_loader::{load_iomux_from_path, load_stm32mp_iomux};
pub use peripheral::Peripheral;
#[cfg(all(feature = "serde", feature = "std"))]
pub use rifsc_loader::{load_all_rifsc_chips, load_rifsc_peripherals};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct GeneralConfig {
    pub chip: Chip,
    /// Specific chip package variant for pin mux configuration (e.g., "STM32MP257FAKx").
    /// This is used to load the correct IOMUX YAML file. If not specified, pin mux
    /// validation cannot be performed, but RIFSC configuration can still be generated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub variant: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct FirmwareEntry {
    pub core: CortexMcuCore,
    pub index: usize,
    pub path: String,
    /// Optional: override flash address for this core's firmware
    pub flash_address: Option<u64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct ProjectConfig {
    pub application: String,
    /// List of firmware to be built and flashed to the device.
    pub firmware: HashMap<String, FirmwareEntry>,
    /// Human-machine interface (HMI) to be used for the project. This is optional and can be used to specify a custom HMI for the project.
    pub interface: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct Config {
    pub general: GeneralConfig,
    pub project: ProjectConfig,
    /// Peripheral resource configuration.
    /// Maps peripheral names to their ownership and security domain settings.
    #[cfg_attr(feature = "serde", serde(default))]
    pub peripheral: HashMap<String, Peripheral>,
    /// HMI build configuration injected as compile-time env vars by the builder.
    #[cfg(feature = "serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmi: Option<HmiConfig>,
}

impl Config {
    /// Validates that every firmware entry names a real-time core that exists on the configured chip.
    pub fn validate_rt_cores(&self) -> Result<(), String> {
        for (name, _) in self.project.firmware.iter() {
            if !self.general.chip.has_rt_core_name(name) {
                return Err(format!("Invalid firmware entry for core {}", name));
            }
        }
        Ok(())
    }
}
