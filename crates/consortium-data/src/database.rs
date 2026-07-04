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

//! Chip-family databases, embedded at compile time and resolved from a chip
//! part number declared in `[profile] chip`.

pub use consortium_cfg_common::database::*;

use crate::error::DataError;

/// A chip-family database bundled into the binary, paired with the part-number
/// prefix that selects it.
struct Embedded {
    /// Lowercase part-number prefix that selects this family (e.g. `stm32mp25`).
    prefix: &'static str,
    /// The raw YAML definition.
    yaml: &'static str,
}

/// All bundled chip-family databases.
///
/// Ordered most-specific prefix first so that, for example, `stm32mp25` is
/// preferred over a hypothetical shorter `stm32mp` prefix.
const DATABASES: &[Embedded] = &[
    Embedded {
        prefix: "stm32mp25",
        yaml: include_str!("../database/memory/stm32mp25.yaml"),
    },
    Embedded {
        prefix: "stm32mp23",
        yaml: include_str!("../database/memory/stm32mp23.yaml"),
    },
    Embedded {
        prefix: "stm32mp21",
        yaml: include_str!("../database/memory/stm32mp21.yaml"),
    },
    Embedded {
        prefix: "mimx95",
        yaml: include_str!("../database/memory/mimx95.yaml"),
    },
    Embedded {
        prefix: "mimx93",
        yaml: include_str!("../database/memory/mimx93.yaml"),
    },
];

/// Resolves and parses the chip-family database for a chip part number.
///
/// The lookup is a case-insensitive prefix match: `stm32mp257fak3` resolves to
/// the `stm32mp25` family. Common i.MX aliases (`imx95`, `imx93`) are accepted
/// in addition to the canonical `mimx*` names.
///
/// # Errors
///
/// Returns [`DataError::UnknownChip`] if no family matches, or
/// [`DataError::DatabaseParse`] if the bundled YAML fails to deserialize.
pub fn load(chip: &str) -> Result<ChipDatabase, DataError> {
    let needle = normalize(chip);
    let entry = DATABASES
        .iter()
        .find(|entry| needle.starts_with(entry.prefix))
        .ok_or_else(|| DataError::UnknownChip(chip.to_string()))?;

    serde_yaml::from_str(entry.yaml)
        .map_err(|err| DataError::DatabaseParse(chip.to_string(), err.to_string()))
}

/// Lowercases the part number and rewrites `imx` aliases to the `mimx`
/// database prefix.
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
    fn resolves_stm32mp25_from_full_part_number() {
        let database = load("stm32mp257fak3").expect("stm32mp25 family should resolve");
        assert_eq!(database.name, "STM32MP25");
    }

    #[test]
    fn accepts_imx_alias() {
        assert!(load("imx95").is_ok());
    }

    #[test]
    fn rejects_unknown_chip() {
        assert!(matches!(load("acme9000"), Err(DataError::UnknownChip(_))));
    }
}
