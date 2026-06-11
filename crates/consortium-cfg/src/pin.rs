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

use crate::peripheral::PeripheralPinMux;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    /// CMOS push-pull output
    PushPull,
    /// Open-drain output (requires external pull-up)
    OpenDrain,
    /// No drive mode specified
    None,
}

/// Configuration for a pin group in the device tree source.
///
/// A pin group corresponds to a DTS node containing one or more multiplexed pins
/// with shared electrical properties (bias, drive mode, slew rate).
#[derive(Debug, Clone)]
pub struct Pin {
    /// Pin multiplexing configurations, paired with sleep mode flags.
    /// Each tuple is `(PeripheralPinMux, is_sleep_mode)`
    pub muxes: Vec<(PeripheralPinMux, bool)>,
    /// Enable bias-disable property in DTS
    pub bias_disable: bool,
    /// Output drive mode (push-pull, open-drain, or unspecified)
    pub drive: PinMode,
    /// Slew rate value; if set, emits `slew-rate = <value>;` in DTS
    pub slew_rate: Option<usize>,
}

impl Pin {
    /// Generate a DTS pin configuration node.
    ///
    /// Produces a DTS node containing pinmux definitions and electrical properties.
    /// The node name is `"pins"` (or `"pins{id}"` if `id > 0`).
    ///
    /// Example output:
    /// ```dts
    /// pins {
    ///     pinmux = <STM32_PINMUX('A', 5, AF1)>, /* I2C1_SDA */
    ///             <STM32_PINMUX('A', 6, AF1)>; /* I2C1_SCL */
    ///     bias-disable;
    ///     drive-push-pull;
    ///     slew-rate = <3>;
    /// };
    /// ```
    pub fn generate_dts(&self, id: usize) -> String {
        let mut lines = Vec::new();

        // Node name
        let node_name = if id == 0 {
            "pins".to_string()
        } else {
            format!("pins{}", id)
        };
        lines.push(format!("{} {{", node_name));

        // Pinmux property
        if !self.muxes.is_empty() {
            lines.push("    pinmux =".to_string());
            for (idx, (mux, sleep)) in self.muxes.iter().enumerate() {
                let mux_line = mux.generate_dts_pinmux(*sleep);
                if idx == self.muxes.len() - 1 {
                    // Last item: end with semicolon
                    lines.push(format!("            {};", mux_line));
                } else {
                    // Not last: end with comma
                    lines.push(format!("            {},", mux_line));
                }
            }
        }

        // Bias configuration
        if self.bias_disable {
            lines.push("    bias-disable;".to_string());
        }

        // Drive mode
        match self.drive {
            PinMode::PushPull => lines.push("    drive-push-pull;".to_string()),
            PinMode::OpenDrain => lines.push("    drive-open-drain;".to_string()),
            PinMode::None => {}
        }

        // Slew rate
        if let Some(slew_rate) = self.slew_rate {
            lines.push(format!("    slew-rate = <{}>;", slew_rate));
        }

        lines.push("};".to_string());

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_mode_derives() {
        let _push_pull = PinMode::PushPull;
        let _open_drain = PinMode::OpenDrain;
        let _none = PinMode::None;

        // Test equality
        assert_eq!(PinMode::PushPull, PinMode::PushPull);
        assert_ne!(PinMode::PushPull, PinMode::OpenDrain);
    }

    #[test]
    fn test_generate_dts_empty_pins() {
        let pin = Pin {
            muxes: vec![],
            bias_disable: false,
            drive: PinMode::None,
            slew_rate: None,
        };

        let dts = pin.generate_dts(0);
        assert_eq!(dts, "pins {\n};");
    }

    #[test]
    fn test_generate_dts_with_id() {
        let pin = Pin {
            muxes: vec![],
            bias_disable: false,
            drive: PinMode::None,
            slew_rate: None,
        };

        let dts = pin.generate_dts(1);
        assert!(dts.starts_with("pins1 {"));
    }

    #[test]
    fn test_generate_dts_with_bias_and_drive() {
        let pin = Pin {
            muxes: vec![],
            bias_disable: true,
            drive: PinMode::PushPull,
            slew_rate: None,
        };

        let dts = pin.generate_dts(0);
        assert!(dts.contains("bias-disable;"));
        assert!(dts.contains("drive-push-pull;"));
    }

    #[test]
    fn test_generate_dts_with_slew_rate() {
        let pin = Pin {
            muxes: vec![],
            bias_disable: false,
            drive: PinMode::OpenDrain,
            slew_rate: Some(3),
        };

        let dts = pin.generate_dts(0);
        assert!(dts.contains("drive-open-drain;"));
        assert!(dts.contains("slew-rate = <3>;"));
    }

    #[test]
    fn test_generate_dts_multiline_formatting() {
        let pin = Pin {
            muxes: vec![],
            bias_disable: true,
            drive: PinMode::PushPull,
            slew_rate: Some(2),
        };

        let dts = pin.generate_dts(0);
        let lines: Vec<&str> = dts.lines().collect();

        assert_eq!(lines[0], "pins {");
        assert!(lines[1..].iter().any(|l| l.contains("bias-disable")));
        assert!(lines[1..].iter().any(|l| l.contains("drive-push-pull")));
        assert!(lines[1..].iter().any(|l| l.contains("slew-rate")));
        assert_eq!(lines[lines.len() - 1], "};");
    }
}
