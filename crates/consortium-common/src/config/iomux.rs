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

//! IO multiplexing (pin) configuration for STM32MP and i.MX series.
//!
//! This module provides types and functions to represent, load, and query
//! device pin configurations derived from STM32CubeMX or manufacturer device databases.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A signal/function assignment for a pin.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignalAssignment {
    /// Peripheral name (e.g., "tim2", "i2c1", "eth1")
    pub name: String,
    /// Pin function within the peripheral (e.g., "ch4", "sda", "rgmii_rxd1")
    /// Some peripherals have no pin field (e.g., global clock signals)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pin: Option<String>,
}

/// A physical pin on the device.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Pin {
    /// Pin name (e.g., "PA5", "PF11", "PG6")
    pub name: String,
    /// Physical position on the package (e.g., "A11")
    pub position: String,
    /// Pin type (e.g., "I/O", "Power", "Reset")
    pub pin_type: String,
    /// List of available signal assignments (mux options)
    pub mux: Vec<SignalAssignment>,
}

/// Intermediate deserializable pin configuration (without the pin name, which comes from the key).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct PinData {
    pub position: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub pin_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mux: Vec<SignalAssignment>,
}

/// Complete IO multiplexing configuration for a device.
#[derive(Debug, Clone)]
pub struct IomuxConfig {
    /// Map of pin name to pin configuration
    pins: HashMap<String, Pin>,
}

impl IomuxConfig {
    /// Create a new empty IO mux configuration.
    pub fn new() -> Self {
        Self {
            pins: HashMap::new(),
        }
    }

    /// Load IO mux configuration from a YAML string.
    ///
    /// The YAML format follows STM32CubeMX export structure:
    /// ```yaml
    /// PA5:
    ///   position: A11
    ///   type: I/O
    ///   mux:
    ///     - name: tim2
    ///       pin: ch4
    ///     - name: i2c1
    ///       pin: sda
    /// ```
    #[cfg(feature = "serde")]
    pub fn from_yaml(yaml_content: &str) -> Result<Self, serde_yaml::Error> {
        let pin_data: HashMap<String, PinData> = serde_yaml::from_str(yaml_content)?;
        let pins = pin_data
            .into_iter()
            .map(|(name, data)| {
                (
                    name.clone(),
                    Pin {
                        name,
                        position: data.position,
                        pin_type: data.pin_type,
                        mux: data.mux,
                    },
                )
            })
            .collect();
        Ok(Self { pins })
    }

    /// Load IO mux configuration from a YAML file.
    #[cfg(all(feature = "serde", feature = "std"))]
    pub fn from_yaml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Find all pins that have a signal assignment matching (peripheral_name, pin_function).
    ///
    /// Returns pin names in lowercase (e.g., "pa5", "pf11", "pg6").
    ///
    /// # Example
    /// ```ignore
    /// let config = IomuxConfig::from_yaml(yaml_str)?;
    /// let pins = config.find_pins_for_signal("tim2", "ch4");
    /// assert_eq!(pins, vec!["pa5", "pb3"]);
    /// ```
    pub fn find_pins_for_signal(&self, peripheral: &str, pin_func: &str) -> Vec<String> {
        let peripheral = peripheral.to_lowercase();
        let pin_func = pin_func.to_lowercase();

        self.pins
            .iter()
            .filter_map(|(pin_name, pin)| {
                if pin.mux.iter().any(|sig| {
                    sig.name.to_lowercase() == peripheral
                        && sig.pin.as_ref().map(|p| p.to_lowercase()) == Some(pin_func.clone())
                }) {
                    Some(pin_name.to_lowercase())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all pin names in the configuration.
    pub fn pin_names(&self) -> Vec<&String> {
        self.pins.keys().collect()
    }

    /// Get a specific pin configuration by name (case-insensitive).
    pub fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(&name.to_uppercase())
    }

    /// Get all pins with a specific peripheral available.
    pub fn find_pins_with_peripheral(&self, peripheral: &str) -> Vec<String> {
        let peripheral = peripheral.to_lowercase();

        self.pins
            .iter()
            .filter_map(|(pin_name, pin)| {
                if pin
                    .mux
                    .iter()
                    .any(|sig| sig.name.to_lowercase() == peripheral)
                {
                    Some(pin_name.to_lowercase())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for IomuxConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn test_iomux_from_yaml() {
        let yaml = r#"
PA5:
  position: A11
  type: I/O
  mux:
    - name: tim2
      pin: ch4
    - name: i2c1
      pin: sda

PF11:
  position: B5
  type: I/O
  mux:
    - name: tim2
      pin: ch4
    - name: spi2
      pin: mosi

PG6:
  position: C3
  type: I/O
  mux:
    - name: uart1
      pin: tx
    - name: clk
"#;

        let config = IomuxConfig::from_yaml(yaml).expect("Failed to parse YAML");
        assert_eq!(config.pin_names().len(), 3);
    }

    #[test]
    fn test_find_pins_for_signal() {
        let yaml = r#"
PA5:
  position: A11
  type: I/O
  mux:
    - name: tim2
      pin: ch4

PF11:
  position: B5
  type: I/O
  mux:
    - name: tim2
      pin: ch4

PG6:
  position: C3
  type: I/O
  mux:
    - name: uart1
      pin: tx
"#;

        let config = IomuxConfig::from_yaml(yaml).expect("Failed to parse YAML");
        let mut pins = config.find_pins_for_signal("tim2", "ch4");
        pins.sort();
        assert_eq!(pins, vec!["pa5", "pf11"]);
    }

    #[test]
    fn test_find_pins_case_insensitive() {
        let yaml = r#"
PA5:
  position: A11
  type: I/O
  mux:
    - name: TIM2
      pin: CH4
"#;

        let config = IomuxConfig::from_yaml(yaml).expect("Failed to parse YAML");
        let pins = config.find_pins_for_signal("tim2", "ch4");
        assert_eq!(pins, vec!["pa5"]);

        let pins = config.find_pins_for_signal("TIM2", "CH4");
        assert_eq!(pins, vec!["pa5"]);
    }

    #[test]
    fn test_find_pins_with_peripheral() {
        let yaml = r#"
PA5:
  position: A11
  type: I/O
  mux:
    - name: tim2
      pin: ch4

PF11:
  position: B5
  type: I/O
  mux:
    - name: tim2
      pin: ch1

PG6:
  position: C3
  type: I/O
  mux:
    - name: uart1
      pin: tx
"#;

        let config = IomuxConfig::from_yaml(yaml).expect("Failed to parse YAML");
        let mut pins = config.find_pins_with_peripheral("tim2");
        pins.sort();
        assert_eq!(pins, vec!["pa5", "pf11"]);
    }

    #[test]
    fn test_get_pin() {
        let yaml = r#"
PA5:
  position: A11
  type: I/O
  mux: []
"#;

        let config = IomuxConfig::from_yaml(yaml).expect("Failed to parse YAML");
        let pin = config.get_pin("PA5");
        assert!(pin.is_some());
        assert_eq!(pin.unwrap().name, "PA5");

        let pin = config.get_pin("pa5");
        assert!(pin.is_some());
    }
}
