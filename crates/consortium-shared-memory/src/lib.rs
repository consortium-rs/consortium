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

#![no_std]

//! Shared-memory building blocks for Consortium: memory regions with explicit
//! cache semantics, and the descriptor header every Consortium shared-memory
//! structure begins with.
//!
//! This crate deliberately knows nothing about IPC. It answers two narrow
//! questions that several subsystems need answered the same way:
//!
//! 1. *How is this window mapped, and what does it cost to make a write visible
//!    to another core?* — the [`Region`] trait and its implementations.
//! 2. *Whose structure is this, and what version?* — the [`descriptor`] header.
//!
//! Protocol lives one level up: `consortium-ipc-transport-memory` owns the `CIPC`
//! descriptor, channel table, and slot protocol; `consortium-dbg` owns `CDBG` and
//! its log rings. Both share the header and the regions defined here.
//!
//! # Regions and coherency
//!
//! On a heterogeneous SoC, cache maintenance follows the mapping attributes,
//! not whether SRAM or DDR physically backs a window. A non-cacheable mapping
//! cannot retain dirty or stale lines. A cacheable mapping needs the writer to
//! clean dirty lines to the point of coherency and the reader to invalidate
//! stale ones — the difference between a working link and one that reads its
//! own stale cache forever.
//!
//! [`Region`] makes that difference explicit rather than assumed:
//!
//! | Type              | Backing        | `flush` / `invalidate`        | `is_coherent` |
//! | ----------------- | -------------- | ----------------------------- | ------------- |
//! | [`UncachedRegion`]| non-cacheable  | no-ops                        | `true`        |
//! | [`CachedRegion`]  | cacheable      | AArch64 / ARMv7-M maintenance | `false`      |
//!
//! Transports are generic over the region, so `is_coherent` and the no-op cache
//! methods are resolved at monomorphization: an uncached link compiles to code
//! with no cache maintenance in it at all, rather than branching at runtime.
//!
//! `CachedRegion` performs maintenance on AArch64 and cache-bearing ARMv7-M
//! cores, and compiles to no-ops on cacheless and host targets.
//!
//! # Safety model
//!
//! Every region constructor is `unsafe` and takes a raw pointer and length. This
//! is the one place the obligation is stated: the caller guarantees the window is
//! valid for reads and writes for `len` bytes for as long as the region lives, and
//! that no conflicting Rust alias to it exists. In exchange, `UncachedRegion` and
//! `CachedRegion` are `Send + Sync` — genuinely two processors touch these bytes, so
//! there is no single-owner story to tell — and everything built on top of them can
//! be safe.
//!
//! In practice the pointer comes from a `mmap` of `/dev/uioN` on Linux or from a
//! linker-placed symbol on firmware, and `consortium-cfg` has already validated the
//! window's bounds, overlap, and MPU alignment against `Consortium.toml`.
//!
//! # Descriptor header
//!
//! Every Consortium shared-memory structure opens with a [`descriptor::DescriptorHeader`]: a
//! 4-byte [`descriptor::DescriptorMagic`] naming the structure and a version byte. Reading it
//! is how a secondary core distinguishes "the primary has not published yet" from
//! "this is somebody else's memory" from "this is a `CIPC` region I am too old to
//! parse", and [`descriptor::DescriptorMagic::check`] turns each of those into a distinct
//! error rather than a generic parse failure.
//!
//! # Features
//!
//! - `defmt` — derive `defmt::Format` on the descriptor error and magic types, for
//!   firmware builds that surface them through `defmt` logging.

pub mod descriptor;
pub mod region;
pub mod regions;

pub use {
    region::Region,
    regions::{CachedRegion, UncachedRegion},
};
