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

//! Per-core peripheral and interrupt maps, embedded at compile time from the
//! generated chip dumps (`database/chip/*.json`).
//!
//! Each JSON file is a single core's view of a chip: the peripheral MMIO bases
//! and the NVIC interrupt numbers as extracted from the vendor CMSIS headers.
//! Because there is one file per core, the set of peripherals in a core's file
//! *is* the set that core can reach — this is the filter used when a
//! `[peripheral.<name>]` is assigned to a coprocessor.

use std::collections::HashMap;

use crate::error::DataError;

/// A chip dump bundled into the binary, paired with the family prefix and core
/// name that select it.
struct Embedded {
    /// Lowercase part-number prefix that selects this family (e.g. `stm32mp25`).
    family: &'static str,
    /// Lowercase core name this dump describes (e.g. `cm33`, `cm7`, `cm0p`).
    core: &'static str,
    /// Whether the chip PAC's shipped `Interrupt` table is generated from *this*
    /// core's dump (see `scripts/pac/gen_interrupts.py`). A PAC carries one
    /// table per chip, so only one core per family is covered; code generation
    /// must not spell `Interrupt::<name>` for any other core, because the
    /// variant exists but carries the covered core's number.
    pac: bool,
    /// The raw JSON dump.
    json: &'static str,
}

/// All bundled per-core chip dumps.
///
/// Ordered most-specific family prefix first, mirroring the chip-family
/// database resolution.
const CHIPS: &[Embedded] = &[
    Embedded {
        family: "stm32mp25",
        core: "cm33",
        pac: true,
        json: include_str!("../database/chip/stm32mp25_cm33.json"),
    },
    Embedded {
        family: "stm32mp25",
        core: "cm0p",
        // The CM0+ has its own, differently numbered NVIC table, and the PAC
        // ships the CM33 one.
        pac: false,
        json: include_str!("../database/chip/stm32mp25_cm0p.json"),
    },
    Embedded {
        family: "stm32mp23",
        core: "cm33",
        // `stm32mp23x` selects the PAC's `stm32mp257` table, whose CM33 lines
        // are numbered identically.
        pac: true,
        json: include_str!("../database/chip/stm32mp23_cm33.json"),
    },
    Embedded {
        family: "stm32mp21",
        core: "cm33",
        pac: true,
        json: include_str!("../database/chip/stm32mp21_cm33.json"),
    },
    Embedded {
        family: "mimx95",
        core: "cm7",
        pac: true,
        json: include_str!("../database/chip/mimx95_cm7.json"),
    },
    Embedded {
        family: "mimx93",
        core: "cm33",
        pac: true,
        json: include_str!("../database/chip/mimx93_cm33.json"),
    },
];

/// Raw JSON shape of a chip dump (only the fields we consume).
#[derive(serde::Deserialize)]
struct RawChip {
    irqs: Vec<RawIrq>,
    peripherals: Vec<RawPeripheral>,
}

#[derive(serde::Deserialize)]
struct RawIrq {
    name: String,
    value: u16,
}

#[derive(serde::Deserialize)]
struct RawPeripheral {
    name: String,
    base_address: String,
    block: String,
}

/// A resolved peripheral: its MMIO base and register-block reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeripheralEntry {
    /// Instance name exactly as the dump spells it, e.g. `LPI2C1`. Lookups are
    /// case-insensitive, so this is what code generation must use when it needs
    /// to name the matching HAL peripheral singleton.
    pub name: String,
    /// Physical base address as seen by the owning core.
    pub base: u32,
    /// Register-block reference from the dump, e.g. `lpi2c/LPI2C` or `gpio/GPIO`.
    pub block: String,
}

impl PeripheralEntry {
    /// The block *kind* — the part before the `/`, lowercased (e.g. `lpi2c`).
    pub fn kind(&self) -> &str {
        self.block.split('/').next().unwrap_or(&self.block)
    }
}

/// A resolved interrupt: its NVIC number and canonical name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrqEntry {
    /// External interrupt number (exception number minus 16).
    pub number: u16,
    /// Canonical interrupt name from the dump (e.g. `LPI2C1`). The PAC's
    /// `Interrupt` variants are generated from these same names, so this is what
    /// a `#[interrupt] fn <name>()` vector is called.
    pub name: String,
}

/// A single core's peripheral and interrupt map.
#[derive(Debug, Clone)]
pub struct ChipMap {
    /// Peripheral entries keyed by lowercased instance name.
    peripherals: HashMap<String, PeripheralEntry>,
    /// Interrupt entries keyed by lowercased interrupt name.
    irqs: HashMap<String, IrqEntry>,
    /// Whether the chip PAC's `Interrupt` enum describes *this* core.
    pac_interrupts: bool,
}

impl ChipMap {
    /// Look up a peripheral instance by name (case-insensitive).
    pub fn peripheral(&self, name: &str) -> Option<&PeripheralEntry> {
        self.peripherals.get(&name.to_ascii_lowercase())
    }

    /// Whether this core can reach the named peripheral instance.
    pub fn contains_peripheral(&self, name: &str) -> bool {
        self.peripherals.contains_key(&name.to_ascii_lowercase())
    }

    /// Every peripheral instance name this core can reach, as the dump spells
    /// them. Order is unspecified; callers that emit code must sort.
    pub fn peripheral_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.peripherals.values().map(|p| p.name.clone()).collect();
        names.sort();
        names
    }

    /// Look up an interrupt by name (case-insensitive).
    pub fn irq(&self, name: &str) -> Option<&IrqEntry> {
        self.irqs.get(&name.to_ascii_lowercase())
    }

    /// Look up an interrupt by its external interrupt number.
    ///
    /// The hardware doorbell database records only the raw NVIC position for a
    /// party, so this is how code generation recovers the line's canonical name.
    /// Numbers are unique within a dump; if one ever repeats, the entry with the
    /// lowest name wins so the result stays deterministic.
    pub fn irq_by_number(&self, number: u16) -> Option<&IrqEntry> {
        self.irqs
            .values()
            .filter(|entry| entry.number == number)
            .min_by(|a, b| a.name.cmp(&b.name))
    }

    /// Whether the chip PAC's `Interrupt` enum is generated from this core's
    /// dump, and therefore whether generated code may name a line as
    /// `Interrupt::<name>` instead of by number.
    ///
    /// A PAC ships one interrupt table per chip (see
    /// `scripts/pac/gen_interrupts.py`), taken from the primary controller core.
    /// On a secondary core such as STM32MP2's CM0+, a shared line name like
    /// `IPCC2_RX` exists in both tables under *different* numbers, so spelling
    /// the variant there would enable the wrong vector.
    pub fn has_pac_interrupts(&self) -> bool {
        self.pac_interrupts
    }
}

/// Resolve and parse the per-core chip map for a chip part number and core name.
///
/// `chip` is the `[profile] chip` part number; `core` is the coprocessor name
/// (e.g. `cm7`). The lookup is a case-insensitive family-prefix match, with the
/// `imx*` aliases rewritten to the `mimx*` prefixes, mirroring the chip-family
/// database.
///
/// # Errors
///
/// Returns [`DataError::ChipMapUnavailable`] when no bundled dump matches, or
/// [`DataError::ChipMapParse`] when the dump fails to deserialize.
pub fn load(chip: &str, core: &str) -> Result<ChipMap, DataError> {
    let needle = normalize(chip);
    let core_key = core.to_ascii_lowercase();
    let entry = CHIPS
        .iter()
        .find(|entry| needle.starts_with(entry.family) && entry.core == core_key)
        .ok_or_else(|| DataError::ChipMapUnavailable {
            chip: chip.to_string(),
            core: core.to_string(),
        })?;

    let raw: RawChip = serde_json::from_str(entry.json).map_err(|err| DataError::ChipMapParse {
        chip: chip.to_string(),
        core: core.to_string(),
        detail: err.to_string(),
    })?;

    let mut peripherals = HashMap::with_capacity(raw.peripherals.len());
    for p in raw.peripherals {
        let base = parse_hex(&p.base_address).ok_or_else(|| DataError::ChipMapParse {
            chip: chip.to_string(),
            core: core.to_string(),
            detail: format!(
                "peripheral '{}' has invalid base '{}'",
                p.name, p.base_address
            ),
        })?;
        peripherals.insert(
            p.name.to_ascii_lowercase(),
            PeripheralEntry {
                name: p.name,
                base,
                block: p.block,
            },
        );
    }

    let irqs = raw
        .irqs
        .into_iter()
        .map(|irq| {
            (
                irq.name.to_ascii_lowercase(),
                IrqEntry {
                    number: irq.value,
                    name: irq.name,
                },
            )
        })
        .collect();

    Ok(ChipMap {
        peripherals,
        irqs,
        pac_interrupts: entry.pac,
    })
}

/// Parse a `0x`-prefixed (or bare) hexadecimal address.
fn parse_hex(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    let digits = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    u32::from_str_radix(digits, 16).ok()
}

/// Lowercases the part number and rewrites `imx` aliases to the `mimx` prefix.
fn normalize(chip: &str) -> String {
    let lowered = chip.to_ascii_lowercase();
    match lowered.strip_prefix("imx") {
        Some(rest) => format!("mimx{rest}"),
        None => lowered,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_imx95_cm7_lpi2c_and_irq() {
        let map = load("imx95", "cm7").expect("imx95 cm7 map should resolve");
        let lpi2c1 = map.peripheral("LPI2C1").expect("LPI2C1 should exist");
        assert_ne!(lpi2c1.base, 0);
        assert_eq!(lpi2c1.kind(), "lpi2c");
        // LPI2C instance name maps 1:1 to its interrupt name.
        assert!(map.irq("LPI2C1").is_some());
    }

    #[test]
    fn resolves_stm32mp25_cm33_gpio() {
        let map = load("stm32mp257fak3", "cm33").expect("stm32mp25 cm33 map should resolve");
        let gpioa = map.peripheral("GPIOA").expect("GPIOA should exist");
        assert_eq!(gpioa.kind(), "gpio");
        assert!(map.contains_peripheral("i2c1"));
    }

    #[test]
    fn rejects_unknown_core() {
        assert!(matches!(
            load("stm32mp257fak3", "cm4"),
            Err(DataError::ChipMapUnavailable { .. })
        ));
    }
}
