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

//! Peripheral configuration for resource partitioning and access control.
//!
//! Peripherals are configured in `Consortium.toml` under `[peripheral.<name>]` sections,
//! where `<name>` must match the peripheral name in the device tree source, hardware-specific
//! RPI configuration (RIF for STM32MP, TRDC for i.MX), and the PAC crate.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for a single peripheral.
///
/// Specifies ownership and security domain for a peripheral resource.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct Peripheral {
    /// Owner of the peripheral (e.g., "cm33", "cm7", "linux", "optee").
    /// On the application core, "linux" is always non-secure, while "optee" is secure by default.
    pub owner: String,
    /// Whether this peripheral is in the secure domain.
    /// If not specified, defaults based on the owner (linux → false, optee → true).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub secure: Option<bool>,
    /// DTS user code attached to the DTS node for this peripheral, which can be used to generate additional code or configuration.
    /// This field is optional and can be used for any custom user data that needs to be associated with the peripheral in the device tree source (DTS) generation process.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dts: Option<String>,
}

impl Peripheral {
    /// Create a new peripheral configuration with an owner.
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            secure: None,
            dts: None,
        }
    }

    /// Create a new peripheral configuration with owner and security domain.
    pub fn with_secure(owner: String, secure: bool) -> Self {
        Self {
            owner,
            secure: Some(secure),
            dts: None,
        }
    }

    /// Determine if this peripheral is secure, using defaults based on owner if not explicitly set.
    pub fn is_secure(&self) -> bool {
        self.secure.unwrap_or_else(|| {
            // Default: linux is non-secure, optee and mcu cores are secure
            !self.owner.to_lowercase().contains("linux")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peripheral_creation() {
        let p1 = Peripheral::new("cm33".to_string());
        assert_eq!(p1.owner, "cm33");
        assert_eq!(p1.secure, None);

        let p2 = Peripheral::with_secure("linux".to_string(), false);
        assert_eq!(p2.owner, "linux");
        assert_eq!(p2.secure, Some(false));
    }

    #[test]
    fn test_peripheral_is_secure_defaults() {
        let linux_periph = Peripheral::new("linux".to_string());
        assert!(!linux_periph.is_secure());

        let optee_periph = Peripheral::new("optee".to_string());
        assert!(optee_periph.is_secure());

        let cm33_periph = Peripheral::new("cm33".to_string());
        assert!(cm33_periph.is_secure());
    }

    #[test]
    fn test_peripheral_is_secure_explicit() {
        let linux_secure = Peripheral::with_secure("linux".to_string(), true);
        assert!(linux_secure.is_secure());

        let cm33_nonsecure = Peripheral::with_secure("cm33".to_string(), false);
        assert!(!cm33_nonsecure.is_secure());
    }
}
