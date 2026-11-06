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

//! Part-number to chip-family matching, shared by every resolver in this crate.
//!
//! A plain prefix match is not enough. NXP names separate dies with longer
//! prefixes of an existing family: i.MX 952 parts are `MIMX952x` and i.MX 937
//! parts are `MIMX937x`, so `mimx9529…` starts with `mimx95` and `mimx9375…`
//! with `mimx93`. Both have their own memory maps — i.MX 952 moves the WAKEUP
//! domain (LPUART3–8, CAN2/3, MU7/8, LPIT2, TPM3–6) — so a prefix match would
//! hand them another die's addresses without an error.
//!
//! [`matches`] therefore also checks the character after the family prefix
//! against the sub-family digits that family actually uses.

/// For a family whose prefix is a prefix of other families' part numbers, the
/// characters allowed right after it. A part number may also end at the prefix
/// (`mimx95`, the bare family name).
///
/// The digit sets are the variant directories `mcux-devices-imx` ships under
/// `i.MX93/` (`MIMX93[01235]x`), `i.MX95/` (`MIMX95[0345789N]x`), and
/// `i.MX952/` (`MIMX952[234589]x`). Anything else after `mimx93`/`mimx95`
/// belongs to a different die (`i.MX937/`, `i.MX952/`) and must not resolve to
/// these families.
const SUBFAMILIES: &[(&str, &str)] = &[
    ("mimx93", "01235"),
    ("mimx95", "0345789n"),
    ("mimx952", "234589"),
];

/// Lowercases the part number and rewrites `imx` aliases to the `mimx` prefix.
pub(crate) fn normalize(chip: &str) -> String {
    let lowered = chip.to_ascii_lowercase();
    match lowered.strip_prefix("imx") {
        Some(rest) => format!("mimx{rest}"),
        None => lowered,
    }
}

/// Whether the normalized part number `needle` belongs to `family`.
pub(crate) fn matches(needle: &str, family: &str) -> bool {
    let Some(rest) = needle.strip_prefix(family) else {
        return false;
    };
    let Some((_, allowed)) = SUBFAMILIES.iter().find(|(name, _)| *name == family) else {
        return true;
    };
    rest.chars()
        .next()
        .is_none_or(|next| allowed.contains(next))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every i.MX 93 and i.MX 95 variant directory in `mcux-devices-imx`.
    const IMX93: &[&str] = &[
        "mimx9301", "mimx9302", "mimx9311", "mimx9312", "mimx9321", "mimx9322", "mimx9331",
        "mimx9332", "mimx9351", "mimx9352",
    ];
    const IMX95: &[&str] = &[
        "mimx9506", "mimx9534", "mimx9536", "mimx9546", "mimx9554", "mimx9556", "mimx9574",
        "mimx9576", "mimx9586", "mimx9594", "mimx9596", "mimx95n4", "mimx95n6",
    ];
    /// Every i.MX 952 variant directory in `mcux-devices-imx`.
    const IMX952: &[&str] = &[
        "mimx9522", "mimx9523", "mimx9524", "mimx9525", "mimx9528", "mimx9529",
    ];
    /// The i.MX 937 variant directories: a die with no bundled data yet.
    const IMX937: &[&str] = &["mimx9371", "mimx9373", "mimx9375"];

    #[test]
    fn accepts_every_variant_of_its_family() {
        for part in IMX93 {
            assert!(matches(part, "mimx93"), "{part}");
        }
        for part in IMX95 {
            assert!(matches(part, "mimx95"), "{part}");
        }
        for part in IMX952 {
            assert!(matches(part, "mimx952"), "{part}");
        }
    }

    #[test]
    fn accepts_full_part_numbers_and_bare_family() {
        assert!(matches("mimx9352dvvxm", "mimx93"));
        assert!(matches("mimx9596xvt", "mimx95"));
        assert!(matches("mimx95n6cvyxn", "mimx95"));
        assert!(matches("mimx95", "mimx95"));
        assert!(matches(&normalize("iMX93"), "mimx93"));
    }

    #[test]
    fn rejects_other_dies_sharing_a_prefix() {
        for part in IMX952.iter().chain(IMX937) {
            assert!(!matches(part, "mimx93"), "{part} matched mimx93");
            assert!(!matches(part, "mimx95"), "{part} matched mimx95");
        }
        for part in IMX937 {
            assert!(!matches(part, "mimx952"), "{part} matched mimx952");
        }
        assert!(!matches("mimx9529avzxn", "mimx95"));
        assert!(!matches(&normalize("IMX952"), "mimx95"));
        assert!(matches(&normalize("IMX952"), "mimx952"));
        assert!(matches("mimx9529avzxn", "mimx952"));
        assert!(!matches("mimx9521", "mimx952"), "no i.MX 952 segment 1");
    }

    #[test]
    fn families_without_subfamily_rules_stay_prefix_matches() {
        assert!(matches("stm32mp257fak3", "stm32mp25"));
        assert!(!matches("stm32mp235fak3", "stm32mp25"));
    }
}
