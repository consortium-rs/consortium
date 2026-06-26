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

use consortium_cfg_common::config::peripheral::Peripheral as CommonPeripheral;
use consortium_dts::Cell;
use core::fmt::Display;

use crate::error::CfgError;

#[derive(Debug, Clone)]
pub struct Peripheral {
    /// Peripheral name (e.g., "i2c1", "uart2", "spi1")
    pub name: String,
    /// Owner/master assignment (e.g., "m33", "a55", "dsp")
    pub owner: String,
    /// Whether this peripheral requires secure context
    pub secure: bool,
    /// Validated pin multiplexing configurations
    pub pins: Vec<PeripheralPinMux>,
    /// Original pin configuration string from project config (e.g., "PA5,PA6")
    pub dts: Option<String>,
    /// RIFSC peripheral ID (STM32MP only); `None` for i.MX or non-RIFSC peripherals
    pub perid: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PeripheralPinMux {
    /// GPIO group letter (e.g., 'A', 'B', 'C', ..., 'Z')
    pub group: char,
    /// GPIO pin number within the group (0-255)
    pub pin_id: u8,
    /// Alternate function position (e.g., "AF0", "AF1", ..., "ANALOG").
    /// Derived from the IOMUX database mux list index for this peripheral.
    pub position: String,
    /// DTS comment alias (e.g., "I2C1_SDA", "SPI2_MOSI").
    /// Derived from peripheral name and function.
    pub alias: String,
}

impl PeripheralPinMux {
    /// Build a `STM32_PINMUX` cell for use in a DTS pinmux property.
    ///
    /// When `sleep` is `true`, the function position is replaced with `ANALOG`
    /// (the conventional STM32 sleep-mode pin state).
    pub fn to_dts_cell(&self, sleep: bool) -> Cell {
        let function = if sleep {
            "ANALOG"
        } else {
            self.position.as_str()
        };
        Cell::macro_call(
            "STM32_PINMUX",
            [
                format!("'{}'", self.group),
                self.pin_id.to_string(),
                function.to_string(),
            ],
        )
    }

    /// Generate a DTS pinmux macro invocation for this pin.
    ///
    /// Produces a `STM32_PINMUX` macro call with optional sleep-mode override.
    /// When `sleep=true`, the function is replaced with `ANALOG` (common sleep mode).
    ///
    /// Example output:
    /// - Active: `<STM32_PINMUX('A', 5, AF1)>, /* I2C1_SDA */`
    /// - Sleep:  `<STM32_PINMUX('A', 5, ANALOG)>, /* I2C1_SDA */`
    pub fn generate_dts_pinmux(&self, sleep: bool) -> String {
        let function = if sleep {
            "ANALOG"
        } else {
            self.position.as_str()
        };
        format!(
            "<STM32_PINMUX('{}', {}, {})>, /* {} */",
            self.group, self.pin_id, function, self.alias
        )
    }

    /// Deprecated: use `generate_dts_pinmux` instead.
    #[deprecated(since = "0.1.0", note = "use `generate_dts_pinmux` instead")]
    pub fn generate_dts_iommx(&self, sleep: bool) -> String {
        self.generate_dts_pinmux(sleep)
    }
}

impl Display for PeripheralPinMux {
    /// Format as a pin string (e.g., "A5" for group='A', pin_id=5).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.group, self.pin_id)
    }
}

impl TryFrom<(&str, &str, &str)> for PeripheralPinMux {
    type Error = CfgError;

    /// Parse a pin string (e.g., "PA5") into group and pin ID components.
    ///
    /// Format: `P<group><pin_id>` where:
    /// - `P` is literal prefix
    /// - `<group>` is a single ASCII letter (A-Z)
    /// - `<pin_id>` is a decimal number (0-255)
    ///
    /// # Errors
    ///
    /// Returns `CfgError::InvalidPin` if:
    /// - String does not start with 'P'
    /// - Group character is not ASCII alphabetic
    /// - Pin ID cannot be parsed as u8
    fn try_from((s, position, alias): (&str, &str, &str)) -> Result<Self, Self::Error> {
        if !s.starts_with('P') {
            return Err(CfgError::InvalidPin(s.to_string()));
        }

        let group = s[1..2]
            .chars()
            .next()
            .ok_or_else(|| CfgError::InvalidPin(s.to_string()))?;

        if !group.is_ascii_alphabetic() {
            return Err(CfgError::InvalidPin(s.to_string()));
        }

        let pin_id = s[2..]
            .parse::<u8>()
            .map_err(|_| CfgError::InvalidPin(s.to_string()))?;

        Ok(Self {
            group,
            pin_id,
            position: position.to_string(),
            alias: alias.to_string(),
        })
    }
}

impl From<(CommonPeripheral, String)> for Peripheral {
    /// Convert a common peripheral config into an RPI peripheral.
    ///
    /// Parses the DTS pin string (comma-separated pin names) into `PeripheralPinMux` entries.
    /// If parsing fails for a pin, it is silently skipped.
    /// The `position` and `alias` fields are left empty; they are populated later
    /// by `RpiSession::new()` when IOMUX data is available.
    fn from((common, name): (CommonPeripheral, String)) -> Self {
        let pins = common.dts.as_ref().map_or(Vec::new(), |dts| {
            dts.split(',')
                .filter_map(|s| PeripheralPinMux::try_from((s.trim(), "", "")).ok())
                .collect()
        });

        Self {
            name,
            owner: common.owner.clone(),
            secure: common.is_secure(),
            pins,
            dts: common.dts,
            perid: None,
        }
    }
}
