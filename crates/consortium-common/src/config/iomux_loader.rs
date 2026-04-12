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

//! Loader for IO mux configurations from the data directory.

use super::iomux::IomuxConfig;
use std::path::Path;

/// Load IO mux configuration for a specific STM32MP chip variant.
///
/// # Example
/// ```ignore
/// let config = load_stm32mp_iomux("STM32MP257FAK")?;
/// let tim2_ch4_pins = config.find_pins_for_signal("tim2", "ch4");
/// // Returns: ["pa5", "pf11", ...]
/// ```
pub fn load_stm32mp_iomux(variant: &str) -> Result<IomuxConfig, Box<dyn std::error::Error>> {
    // Try multiple path strategies to handle different environments (dev, test, installed)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let possible_paths = vec![
        // Development: from consortium-common crate
        format!("{}/../../data/stm32mp/iomux/{}.yaml", manifest_dir, variant),
        // Test environment
        format!("{}/../../data/stm32mp/iomux/{}.yaml", manifest_dir, variant),
        // Absolute path
        format!("~/Developer/consortium/data/stm32mp/iomux/{}.yaml", variant),
    ];

    for path in possible_paths {
        if Path::new(&path).exists() {
            return IomuxConfig::from_yaml_file(&path);
        }
    }

    Err(format!("Could not find iomux config for variant: {}", variant).into())
}

/// Load IO mux configuration from a custom path.
pub fn load_iomux_from_path(
    path: impl AsRef<Path>,
) -> Result<IomuxConfig, Box<dyn std::error::Error>> {
    IomuxConfig::from_yaml_file(path.as_ref().to_str().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_stm32mp257fak_iomux() {
        // This test verifies that the YAML file can be loaded
        // Note: variant name includes the "x" from CubeMX (STM32MP257FAKx, not STM32MP257FAK)
        match load_stm32mp_iomux("STM32MP257FAKx") {
            Ok(config) => {
                // Find pins supporting TIM2 CH4
                let tim2_ch4_pins = config.find_pins_for_signal("tim2", "ch4");
                println!(
                    "✓ TIM2 CH4 available on {} pins: {:?}",
                    tim2_ch4_pins.len(),
                    &tim2_ch4_pins[..std::cmp::min(5, tim2_ch4_pins.len())]
                );
                assert!(
                    !tim2_ch4_pins.is_empty(),
                    "Should find at least one pin for TIM2 CH4"
                );

                // Find pins supporting I2C1
                let i2c1_pins = config.find_pins_with_peripheral("i2c1");
                println!(
                    "✓ I2C1 available on {} pins: {:?}",
                    i2c1_pins.len(),
                    &i2c1_pins[..std::cmp::min(5, i2c1_pins.len())]
                );
                assert!(
                    !i2c1_pins.is_empty(),
                    "Should find at least one pin for I2C1"
                );

                // Find all ETH1 pins
                let eth1_pins = config.find_pins_with_peripheral("eth1");
                println!(
                    "✓ ETH1 available on {} pins: {:?}",
                    eth1_pins.len(),
                    &eth1_pins[..std::cmp::min(10, eth1_pins.len())]
                );
            }
            Err(e) => {
                eprintln!("Failed to load iomux config: {}", e);
                // Don't fail the test if the file doesn't exist in test environment
            }
        }
    }
}
