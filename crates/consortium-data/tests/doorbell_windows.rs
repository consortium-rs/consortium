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

//! Cross-checks the family database's MU doorbell description against the
//! per-core chip dumps.
//!
//! The two halves of `consortium-data` are generated from different sources —
//! `database/memory/*.yaml` is hand-authored from the reference manual, and
//! `database/chip/*.json` is machine-extracted from the vendor CMSIS headers —
//! and nothing forced them to agree. A party's `window` and `irq` are exactly
//! the fields a copy-paste gets wrong, and both fail silently downstream: the
//! window picks the register half the generated doorbell drives, and the `irq`
//! becomes an NVIC vector name or a device-tree `interrupts` cell.
//!
//! The dump is the oracle. A core's CMSIS header declares `MU<n>_MU<A|B>` for
//! the window that core can reach and an `MU<n>_<A|B>` NVIC line for the one
//! routed to it, so both fields are checkable for every coprocessor party.

use consortium_cfg_common::database::HardwareDoorbellPeripheralType;
use consortium_data::{chipmap, database};

/// Every bundled family database, paired with the one core whose dump is
/// bundled alongside it (see `chipmap::CHIPS`).
const COVERED: &[(&str, &str)] = &[("mimx95", "cm7"), ("mimx93", "cm33")];

#[test]
fn mu_party_windows_and_irqs_match_the_core_dump() {
    for (chip, core) in COVERED {
        let db = database::load(chip).expect("bundled family database should load");
        let map = chipmap::load(chip, core).expect("bundled core dump should load");

        // The dump's bases are absolute; the database stores them relative to a
        // peripheral alias, so lift them through the non-secure one.
        let alias = db
            .peripheral
            .aliases
            .iter()
            .find(|alias| !alias.secure)
            .expect("chip should declare a non-secure peripheral alias")
            .address;

        let mut checked = 0;
        for bell in &db.peripheral.doorbells {
            if bell.r#type != HardwareDoorbellPeripheralType::Mu {
                continue;
            }
            let instance = bell
                .name
                .strip_prefix("MU")
                .expect("MU doorbell should be named MU<n>");

            for party in &bell.parties {
                if party.core.to_string() != *core {
                    continue;
                }
                let side = match party.window {
                    consortium_cfg_common::database::DoorbellWindow::A => "A",
                    consortium_cfg_common::database::DoorbellWindow::B => "B",
                };

                // The window this party claims must be one this core can reach.
                let singleton = format!("MU{instance}_MU{side}");
                let entry = map.peripheral(&singleton).unwrap_or_else(|| {
                    panic!(
                        "{chip} {core}: `{}` party claims window {side}, but the \
                         {core} dump has no `{singleton}` -- it exposes: {:?}",
                        bell.name,
                        map.peripheral_names()
                            .into_iter()
                            .filter(|name| name.starts_with(&format!("MU{instance}_")))
                            .collect::<Vec<_>>()
                    )
                });
                assert_eq!(
                    bell.party_window_base(party) + alias,
                    entry.base,
                    "{chip} {core}: `{}` window {side} resolves to a different \
                     address than the dump's `{singleton}`",
                    bell.name
                );

                // ...and the NVIC line must be the one for that same window.
                let vector = format!("MU{instance}_{side}");
                let irq = map
                    .irq(&vector)
                    .unwrap_or_else(|| panic!("{chip} {core}: dump has no `{vector}` interrupt"));
                assert_eq!(
                    party.irq,
                    u32::from(irq.number),
                    "{chip} {core}: `{}` party is on window {side}, so its NVIC \
                     line is `{vector}` ({}), not {}",
                    bell.name,
                    irq.number,
                    party.irq
                );
                checked += 1;
            }
        }
        assert!(checked > 0, "{chip} {core}: no MU party was checked");
    }
}
