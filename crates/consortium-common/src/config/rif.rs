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

//! RIF (Resource Interface) configuration schema for STM32MP peripheral maps.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Source metadata for the RIF peripheral map.
///
/// Typically points to the original resource manager repository and the path
/// within that repository where the data was generated from.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct RifSource {
    /// Organization or author of the source repository (e.g., "STMicroelectronics").
    pub org: Option<String>,
    /// Repository name (e.g., "STM32CubeMP2").
    pub repo: Option<String>,
    /// Path within the repository (e.g., "Utilities/ResourcesManager").
    pub resources_path: Option<String>,
}

/// A peripheral definition within a RIF peripheral map.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct RifPeripheral {
    /// Name of the peripheral (e.g., "spi2", "tim1").
    pub name: String,
    /// RIFSC (Resource Interface Security Controller) ID for this peripheral.
    pub rifsc_id: u8,
    /// Optional list of chip variants that support this peripheral.
    /// If absent, the peripheral is available on all variants.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub variants: Option<Vec<String>>,
}

/// RIF peripheral map configuration for a chip family.
///
/// This represents the parsed schema of a RIFSC YAML file, containing:
/// - Chip family identifier
/// - Maximum RIFSC ID value for the family
/// - List of supported chip variants
/// - Mapping of peripherals to RIFSC IDs
/// - Optional source metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct RifConfig {
    /// Chip family identifier (e.g., "stm32mp25xxxx", "stm32mp23xxxx").
    pub chip_family: String,
    /// Maximum RIFSC peripheral ID for this chip family.
    pub rifsc_id_max: u8,
    /// List of all supported chip variants for this family (e.g., "STM32MP251Axx").
    pub all_variants: Vec<String>,
    /// RIFSC peripheral definitions.
    pub peripherals: Vec<RifPeripheral>,
    /// Optional source metadata.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source: Option<RifSource>,
}

impl RifConfig {
    /// Get the list of peripheral names defined in this config.
    pub fn peripheral_names(&self) -> Vec<&str> {
        self.peripherals.iter().map(|p| p.name.as_str()).collect()
    }

    /// Get the list of RIFSC IDs defined in this config.
    pub fn rifsc_ids(&self) -> Vec<u8> {
        self.peripherals.iter().map(|p| p.rifsc_id).collect()
    }

    /// Look up a peripheral by name.
    pub fn peripheral_by_name(&self, name: &str) -> Option<&RifPeripheral> {
        self.peripherals.iter().find(|p| p.name == name)
    }

    /// Look up a peripheral by RIFSC ID.
    pub fn peripheral_by_id(&self, id: u8) -> Option<&RifPeripheral> {
        self.peripherals.iter().find(|p| p.rifsc_id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rif_config_lookup() {
        let config = RifConfig {
            chip_family: "stm32mp25xxxx".to_string(),
            rifsc_id_max: 127,
            all_variants: vec!["STM32MP251Axx".to_string(), "STM32MP253Cxx".to_string()],
            peripherals: vec![
                RifPeripheral {
                    name: "tim1".to_string(),
                    rifsc_id: 0,
                    variants: None,
                },
                RifPeripheral {
                    name: "spi2".to_string(),
                    rifsc_id: 23,
                    variants: Some(vec!["STM32MP251Axx".to_string()]),
                },
            ],
            source: None,
        };

        assert_eq!(
            config.peripheral_by_name("tim1").map(|p| p.rifsc_id),
            Some(0)
        );
        assert_eq!(
            config.peripheral_by_id(23).map(|p| p.name.as_str()),
            Some("spi2")
        );
        assert!(config.peripheral_by_name("nonexistent").is_none());
    }
}
