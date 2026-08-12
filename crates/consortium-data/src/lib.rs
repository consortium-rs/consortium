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

//! Static chip data for the Consortium project, embedded at compile time.
//!
//! This crate owns the chip-family databases (`database/memory/*.yaml`), the
//! generated per-core chip dumps (`database/chip/*.json`), and the extracted
//! peripheral register definitions (`database/peri/`).
//! It also owns the resolvers that turn a `[profile] chip` part number into a
//! parsed [`database::ChipDatabase`] or a per-core [`chipmap::ChipMap`].
//!
//! # Why it is embedded rather than read from disk
//!
//! `csti` is installed as a binary and run from a user's project directory, which
//! has no `database/` in it. Chip YAML and JSON are therefore compiled in through
//! `include_str!`. Linux device-tree sources are intentionally not bundled; the
//! build reads them from the checkout passed to `csti build --linux`.
//!
//! # What each module resolves
//!
//! | Module       | Source                       | Answers                              |
//! | ------------ | ---------------------------- | ------------------------------------ |
//! | [`database`] | `database/memory/*.yaml`      | What does this chip *family* offer — coprocessors, memory regions and aliases, peripherals, doorbells? |
//! | [`chipmap`]  | `database/chip/*.json`        | What can *one core* reach — peripheral MMIO bases and NVIC numbers? |
//!
//! The split between the first two matters. A family database says the chip *has* an
//! LPUART2; a chip map says *this core* can see it. Because there is one JSON file
//! per core, the set of peripherals in a core's file **is** the set that core can
//! reach — which is exactly the filter `consortium-cfg` applies when validating
//! `[peripheral.<name>] owner`.
//!
//! The same chip dumps also feed `scripts/pac/gen_interrupts.py` and
//! `scripts/pac/gen_peripherals.py`, which generate the PAC's interrupt tables and
//! the chip HAL's singleton map. One source for all three is what keeps a HAL
//! singleton, a PAC interrupt number, and a validated manifest in agreement.
//!
//! # Generated and imported data
//!
//! Treat everything under `database/` as **generated or vendored**, not authored.
//! Change the generation workflow (the `just pac-*` recipes and `scripts/pac/`)
//! rather than editing a large data file by hand.
//!
//! Schema types come from `consortium-cfg-common`, which exists partly so this crate
//! can deserialize into them without depending on the codegen crate that depends on
//! it.
//!
pub mod chipmap;
pub mod database;
mod error;

pub use error::DataError;
