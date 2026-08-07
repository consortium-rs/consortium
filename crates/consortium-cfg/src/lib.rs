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

//! Turns a `Consortium.toml` manifest into validated configuration, generated Rust,
//! and Linux integration artifacts.
//!
//! This is the crate that makes Consortium's "explicit cross-core contracts" idea
//! pay off. A manifest declares what the system *is* — which cores exist, which
//! memory windows they share, which doorbell rings which — and this crate checks
//! that against the chip's real capabilities, then emits the bring-up code and
//! device-tree nodes that make it so. Firmware and application authors never write
//! the address arithmetic, the UIO ids, or the doorbell wiring by hand.
//!
//! # Pipeline
//!
//! ```text
//! Consortium.toml
//!   └─ session   parse + resolve the chip database + validate       → Session
//!        └─ bridge    lower into the components IR                  → ShuHan, BellA, …
//!             └─ components   each IR node generates its own output
//!                  ├─ Rust tokens ─ render ─→ src/consortium.gen.rs
//!                  ├─ DTS nodes   ────────→ merged tree or .dtso
//!                  └─ Kconfig / boot / systemd fragments
//! ```
//!
//! | Stage           | Module         | Produces                                    |
//! | --------------- | -------------- | ------------------------------------------- |
//! | Parse, validate | [`session`]    | [`Session`] — manifest bound to its chip database |
//! | Lower           | [`bridge`]     | The [`components`] IR, with addresses and UIO ids resolved |
//! | Generate        | [`components`] | Rust tokens, DTS nodes, UIO/Kconfig fragments  |
//! | Format          | [`render()`](render())   | Formatted Rust source                         |
//!
//! [`consortium-builder`] drives this and adds compilation, staging, and packaging.
//!
//! [`consortium-builder`]: https://docs.rs/consortium-builder
//!
//! # What validation catches
//!
//! [`Session::from_toml`] resolves `[profile] chip` through the embedded databases
//! in `consortium-data` and then rejects manifests that ask for things the chip
//! cannot do — the errors in [`CfgError`] are the interesting part of this crate's
//! contract:
//!
//! - a doorbell the chip does not have, or a doorbell channel out of its range;
//! - a channel whose party is not a visible core, or has no declared endpoint;
//! - two channels claiming the same doorbell channel;
//! - memory regions out of bounds, overlapping, or misaligned for the MPU;
//! - debug regions that collide with IPC regions;
//! - a peripheral owned by a core that cannot reach it — checked against the
//!   per-core chip map, so the filter is the peripherals that core's CMSIS view
//!   actually contains;
//! - `[peripheral.*]` with `secure = true`, since the chip HAL's singletons carry
//!   only the non-secure alias.
//!
//! Catching these at manifest-parse time is the point: each would otherwise be a
//! silent bus error or a link that never wakes.
//!
//! # Where the fixed decisions live
//!
//! [`bridge`] pins the AP-side address layout so the firmware, NVIC, and Linux DTS
//! outputs cannot disagree:
//!
//! - every device — the aggregate IPC shared-memory device, one per distinct
//!   doorbell, and the aggregate debug-ring device — gets its `uioN` id by
//!   ascending mapped base, so ids follow the memory map the kernel enumerates;
//! - within the IPC device a region's `mmap_id` is its index in channel-declaration
//!   order, and a doorbell always exposes its register window as `mmap_id` 0.
//!
//! # Generated code
//!
//! The output is `src/consortium.gen.rs` in each endpoint crate, which the crate
//! `include!`s. It defines `Config`, endpoint-local `Peripherals` and `Channels`,
//! and synchronous `init(config)`. Init performs local platform bring-up and
//! installs unsplit transports; application code connects each channel token
//! independently before use.
//!
//! Because a generated crate cannot be `cargo check`ed before the pipeline has run,
//! the melt-pot endpoint crates are excluded from the workspace.
//!
//! # Adding to the schema
//!
//! Work outward, because each step's tests depend on the previous: schema types in
//! `consortium-cfg-common` → validation in [`session`] → lowering in [`bridge`] →
//! codegen in [`components`]. Then update the snapshot tests under `tests/`.
//!
//! `Consortium.example.*.toml` and [`session`]'s own tests are the best references
//! for current syntax — older melt-pot manifests drift.

pub mod bridge;
pub mod components;
pub mod error;
pub mod render;
pub mod session;

pub use render::render;

// The chip-family database and per-core chip maps live in `consortium-data`.
// Re-export them under their historical paths so existing `crate::database` /
// `crate::chipmap` references keep resolving.
pub use consortium_data::{chipmap, database, dts};

pub use consortium_cfg_common::config;
pub use error::CfgError;
pub use session::Session;
