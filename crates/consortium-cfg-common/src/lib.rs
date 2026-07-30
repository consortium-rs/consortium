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

//! The `Consortium.toml` schema and the chip-database schema it is validated
//! against — types and `serde` derives only, no logic.
//!
//! This crate is the vocabulary two otherwise-unrelated consumers must share, and
//! that is the whole reason it exists separately from `consortium-cfg`:
//!
//! - `consortium-cfg` parses, validates, lowers, and generates code from a
//!   manifest.
//! - `consortium-data` embeds the chip-family YAML databases and resolves a part
//!   number to one, so it needs [`database`]'s types — but must not depend on the
//!   codegen crate, which depends on *it*.
//!
//! Splitting the schema out breaks that cycle. Anything with behaviour belongs in
//! `consortium-cfg`; anything a database file has to deserialize into belongs here.
//!
//! # [`config`] — the manifest schema
//!
//! [`Config`](config::Config) is the root of `Consortium.toml`. **This is the
//! authoritative definition of the current schema** — prefer it over older example
//! manifests, which drift.
//!
//! | Section              | Type                                             |
//! | -------------------- | ------------------------------------------------ |
//! | `[profile]`          | [`Profile`](config::Profile)                      |
//! | `[dts]`              | [`Dts`](config::Dts), [`DtsMode`](config::DtsMode) |
//! | `[endpoints]`        | [`Endpoints`](config::Endpoints)                   |
//! | `[dbg.<core>]`       | [`Dbg`](config::Dbg), [`DbgCore`](config::DbgCore) |
//! | `[[ipc.channel]]`    | [`Ipc`](config::Ipc), [`IpcChannel`](config::IpcChannel) |
//! | `[tee]`              | [`Tee`](config::Tee)                              |
//! | `[hmi]`              | [`Hmi`](config::Hmi)                              |
//! | `[peripheral.<name>]`| [`Peripheral`](config::Peripheral)                 |
//!
//! Core names are newtypes rather than strings — [`AcoreName`](config::AcoreName),
//! [`McoreName`](config::McoreName), [`RcoreName`](config::RcoreName),
//! [`RvCoreName`](config::RvCoreName) — so a manifest cannot quietly name a core
//! kind the chip does not have. [`VisibleCore`](config::VisibleCore) is the
//! resolved, validated form the rest of the pipeline uses.
//!
//! Most structs carry `#[serde(deny_unknown_fields)]`, so a typo or a stale field
//! from an older schema is an error rather than a silently ignored key.
//!
//! # [`database`] — the chip-database schema
//!
//! What a chip *offers*, against which a manifest's requests are checked:
//! [`ChipDatabase`](database::ChipDatabase) with its coprocessors, memory regions
//! and aliases, peripherals, and the doorbell peripherals
//! ([`HardwareDoorbellPeripheral`](database::HardwareDoorbellPeripheral)) whose
//! channel counts and parties bound what `[[ipc.channel]]` may ask for.
//!
//! The YAML and JSON that populate these types live in `consortium-data`.
//!
//! # Changing the schema
//!
//! Work outward in this order, because each step's tests depend on the previous:
//!
//! 1. types here,
//! 2. validation in `consortium-cfg`'s `Session`,
//! 3. lowering into the `components` IR,
//! 4. generated code and build artifacts — then update the snapshot tests under
//!    `crates/consortium-cfg/tests/`.

pub mod config;
pub mod database;
