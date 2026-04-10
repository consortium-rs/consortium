#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::config::chip_map::Chip;
use crate::config::mcu_core::CortexMcuCore;

pub mod app_core;
pub mod chip_map;
pub mod mcu_core;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct GeneralConfig {
    pub chip: Chip,
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
}

impl Config {
    pub fn validate_rt_cores(&self) -> Result<(), String> {
        for (name, _) in self.project.firmware.iter() {
            if !self.general.chip.has_rt_core_name(name) {
                return Err(format!("Invalid firmware entry for core {}", name));
            }
        }
        Ok(())
    }
}
