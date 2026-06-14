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

//! CFG session management with chip-aware peripheral configuration and RIFSC/TRDC perid assignment.

use std::collections::HashMap;

use consortium_common::config::{Config, chip_map::Chip, iomux::IomuxConfig, rif::RifConfig};

use crate::error::CfgError;
use crate::peripheral::Peripheral;

/// Session holding chip identity and resolved peripheral configuration.
///
/// `CfgSession` is the main entry point for generating DTS snippets and RIFSC
/// configuration for a specific chip. Construct it with `CfgSession::new()`,
/// providing the user config, the chip-specific RIFSC peripheral map, and
/// optionally the pin mux database for the target package variant.
#[derive(Debug, Clone)]
pub struct CfgSession {
    /// The target chip family for this session.
    pub chip: Chip,
    /// Specific chip package variant (e.g., "STM32MP257FAKx") used for IOMUX pin mux configuration.
    pub variant: Option<String>,
    /// Resolved peripheral configuration, keyed by peripheral name.
    pub peripherals: HashMap<String, Peripheral>,
}

impl CfgSession {
    /// Build a session from a project `Config`, optional RIFSC peripheral map, and optional IOMUX pin database.
    ///
    /// For each peripheral in `config.peripheral`:
    /// - Creates an `CfgSession::Peripheral` from the common peripheral config.
    /// - Assigns `perid` from the matching entry in `rif` if provided (leaves `None` otherwise).
    /// - When `iomux` is provided:
    ///   - Verifies each pin in the peripheral's `dts` field exists in the IOMUX data.
    ///   - Verifies the pin supports the peripheral (returns `PinNotAvailable` if not).
    ///   - Populates `PeripheralPinMux::position` with `"AFN"` (mux list index as AF number) and
    ///     `PeripheralPinMux::alias` with `"PERIPH_FUNC"` (e.g., `"I2C1_SDA"`).
    ///
    /// # Arguments
    /// * `config` - Project configuration with chip and peripheral definitions.
    /// * `rif` - Optional RIFSC peripheral map (STM32MP only; pass `None` for i.MX or other chips without RIFSC).
    /// * `iomux` - Optional IOMUX pin database (required for AF/pin validation; pass `None` when package variant is unknown).
    ///
    /// # Errors
    /// - `CfgError::InvalidPin` - pin in `dts` doesn't exist in IOMUX.
    /// - `CfgError::PinNotAvailable` - pin exists but doesn't support the peripheral.
    pub fn new(
        config: &Config,
        rif: Option<&RifConfig>,
        iomux: Option<&IomuxConfig>,
    ) -> Result<Self, CfgError> {
        let mut peripherals = HashMap::new();

        for (name, common_periph) in &config.peripheral {
            let mut periph = Peripheral::from((common_periph.clone(), name.clone()));

            // Assign perid from RIFSC map if available (STM32MP only)
            if let Some(rif) = rif {
                if let Some(rif_periph) = rif.peripheral_by_name(name) {
                    periph.perid = Some(rif_periph.rifsc_id as u32);
                }
            }

            // Validate pins and populate AF position + alias from IOMUX if provided
            if let Some(iomux) = iomux {
                for pin in &mut periph.pins {
                    let pin_name = format!("P{}{}", pin.group, pin.pin_id);

                    let iomux_pin = iomux
                        .get_pin(&pin_name)
                        .ok_or_else(|| CfgError::InvalidPin(pin_name.clone()))?;

                    let af_idx = iomux_pin
                        .mux
                        .iter()
                        .position(|sig| sig.name.to_lowercase() == name.to_lowercase())
                        .ok_or_else(|| {
                            let allowed = iomux.find_pins_with_peripheral(name);
                            CfgError::PinNotAvailable(
                                pin_name.clone(),
                                name.clone(),
                                String::new(),
                                allowed,
                            )
                        })?;

                    pin.position = format!("AF{}", af_idx);

                    if let Some(func) = &iomux_pin.mux[af_idx].pin {
                        pin.alias = format!("{}_{}", name.to_uppercase(), func.to_uppercase());
                    }
                }
            }

            peripherals.insert(name.clone(), periph);
        }

        Ok(CfgSession {
            chip: config.general.chip.clone(),
            variant: config.general.variant.clone(),
            peripherals,
        })
    }
}
