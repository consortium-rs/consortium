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

#![cfg_attr(not(feature = "std"), no_std)]

//! The cross-core primitives every Consortium subsystem builds on: shared memory
//! regions, the descriptor header, doorbells, and the transport traits.
//!
//! On a heterogeneous SoC, two cores that share a window of memory need exactly
//! two things before they can agree on anything: a way to make writes visible to
//! each other, and a way to say "look now". This crate owns both, and no protocol
//! on top of them:
//!
//! 1. *How is this window mapped, and what does it cost to make a write visible
//!    to another core?* — the [`Region`] trait and its implementations.
//! 2. *Whose structure is this, and what version?* — the [`descriptor`] header.
//! 3. *How does one core wake another?* — the [`Doorbell`] trait, its [`Chan`] and
//!    [`Side`] vocabulary, the Type-A hardware bells in `bell` (IPCC, MU, HSEM,
//!    behind their chip features), and the GPIO doorbell (feature `gpio`).
//! 4. *What does handing a frame to the other core look like?* — the transport
//!    traits ([`SendTransport`], [`TrySendTransport`], …) and the [`wait`] helpers
//!    that turn a non-waiting attempt into an async or a blocking one.
//!
//! The transport traits are vocabulary, not protocol: they say what a send or
//! receive promises, and nothing about the bytes. Protocol lives one level up, and
//! each subsystem combines these pieces its own way:
//! `consortium-ipc-transport-memory` owns the `CIPC` descriptor and slot protocol
//! (region + doorbell), `consortium-pmi-scmi` the SCMI shared-memory channel
//! (region + doorbell), and `consortium-dbg` `CDBG` and its log rings (region
//! only). `consortium-ipc` re-exports the doorbell and transport vocabulary as part
//! of its trait stack.
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
//! # Doorbells
//!
//! [`Doorbell`] is the signaling primitive: a synchronous `ring` and an async
//! `wait`, each on a [`Chan`]. A Type-A bell is one protocol written once in
//! `bell` over a per-block register model (`bell::BellHw`); read that module's
//! documentation before changing any block. [`Side`] names which window or bank of
//! a block this core drives, which is not necessarily the role a transport above
//! it plays.
//!
//! # Transports
//!
//! A frame-atomic transport — one whose hand-off either lands a whole frame in a
//! peer-visible buffer or does nothing — implements [`TrySendTransport`] /
//! [`TryRecvTransport`], which never wait and report an occupied buffer as
//! [`TryError::WouldBlock`]. [`wait::park_until`] then gives it an async form
//! (park on a doorbell line between attempts, with the lost-wakeup ordering every
//! doorbell needs) and [`wait::spin_until`] a blocking one (spin against a
//! caller-supplied deadline). The async [`SendTransport`] / [`RecvTransport`],
//! [`Connect`], and [`AckSendTransport`] are the traits `consortium-ipc`'s channels
//! are generic over; wired transports implement only those.
//!
//! # Features
//!
//! - `std` — makes [`MaybeSend`] require `Send` and selects the `std` bell
//!   interrupt path (`BellA::service_interrupt`).
//! - `defmt` — derive `defmt::Format` on the descriptor, bell and GPIO error types,
//!   and require it of every [`Doorbell::Error`].
//! - `bell` — the generic Type-A bell protocol (`bell::BellA`, `bell::BellHw`). Every
//!   chip feature implies it; it is off otherwise so a plain IPC build needs no atomic
//!   compare-and-swap.
//! - `gpio` — the single-channel GPIO doorbell (`gpio`).
//! - `mimx93` / `mimx95` — the i.MX 9 MU bell (`bell::mu`). Mutually exclusive.
//! - `stm32mp21x` / `stm32mp23x` / `stm32mp25x` — the STM32MP2 IPCC bell
//!   (`bell::ipcc`), plus HSEM (`bell::hsem`) on MP23/MP25.

#[cfg(all(test, not(feature = "std")))]
extern crate std;

#[cfg(feature = "bell")]
pub mod bell;
pub mod descriptor;
#[cfg(feature = "gpio")]
pub mod gpio;
pub mod region;
pub mod regions;
pub mod wait;

mod chan;
mod doorbell;
mod future;
mod sealed;
mod side;
mod transport;

pub use {
    chan::{Chan, ChanValidate},
    doorbell::Doorbell,
    future::MaybeSend,
    region::Region,
    regions::{CachedRegion, UncachedRegion},
    side::{Primary, Secondary, Side, Tertiary},
    transport::{
        AckSendTransport, Connect, RecvTransport, SendTransport, TransportError, TryError,
        TryRecvTransport, TrySendTransport,
    },
};
