#![cfg_attr(not(feature = "std"), no_std)]
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

//! MU (Messaging Unit) doorbell driver for Consortium IPC.
//!
//! A [`MuDoorbell`] drives one MU instance from one processor [`side`], with the
//! instance number and side selected at the type level (e.g. `MuDoorbell<Primary,
//! 7>` is the CA55 view of MU7, the i.MX95 CA55 <-> CM7 default route). Its
//! channels span two line kinds of that instance: channels `0..4` are the
//! general-purpose interrupt (GI) lines and channels `4..8` are the
//! receive-full (RF) lines.
//!
//! # Two line kinds in one channel space
//!
//! An MU instance offers two independent signaling mechanisms, and this crate packs
//! both into the single flat channel space the [`Doorbell`](consortium_ipc::Doorbell) trait expects:
//!
//! | [`Chan`] range | Line kind                     | Registers                  |
//! | -------------- | ----------------------------- | -------------------------- |
//! | `0..4`         | general-purpose interrupt (GI) | `GCR` / `GSR` / `GIER`      |
//! | `4..8`         | receive-full (RF)              | transmit/receive + `RSR` / `RCR` |
//!
//! GI lines are pure interrupts. RF lines work by writing a token word to a transmit
//! register, which sets the peer's receive-full flag — a doorbell that happens to
//! carry four bytes this crate does not use. Both kinds signal; which to pick is a
//! board-integration choice, so both are exposed rather than one being hidden.
//!
//! Each instance's side-B (MUB) window sits one register-window length above its
//! side-A (MUA) base, so one base table plus a length describes both processor views
//! and [`Side`] selects between them at the type level.
//!
//! [`Chan`]: consortium_ipc::Chan
//! [`Side`]: consortium_ipc::Side
//!
//! # Construction
//!
//! - **Firmware** — [`MuDoorbell::new`] consumes the chip HAL's `MUn_MUx` peripheral
//!   singleton. A const assertion checks the singleton's side against `S`, so
//!   `MuDoorbell<Secondary, 7>` cannot be built from the MUA singleton. The window
//!   address still comes from this crate's [`registers::base`] table so the security
//!   alias does not change underfoot.
//! - **Linux/UIO** — [`MuDoorbell::new_mapped`] is `unsafe` and takes an `mmap`'d
//!   virtual base, because a mapped window has no singleton. Passing a physical
//!   address is undefined behaviour.
//!
//! The doorbell is [`Clone`] and clones share one wake source, so
//! `SharedMemoryTransport::split` can give a copy to each half.
//!
//! # Interrupts, and why MU needs masking
//!
//! This crate installs **no** `#[interrupt]` handlers. The application owns the vector
//! table and forwards each MU vector a core drives into
//! [`notify_gi_interrupt::<S, N>()`](notify_gi_interrupt), parameterized with that
//! instance's side and number — i.MX93 `MU1_B`/`MU2_B`, i.MX95 CM7
//! `MU5_A`/`MU7_B`/`MU8_B`. Enable the *HAL's* `rt` feature
//! (`consortium-hal-imx9/rt`) so the `#[interrupt]` macro and PAC vector table are
//! available there.
//!
//! ```rust,ignore
//! #[interrupt]
//! fn MU7_B() {
//!     consortium_ipc_doorbell_mu::notify_gi_interrupt::<consortium_ipc::Secondary, 7>();
//! }
//! ```
//!
//! In practice firmware does not write that by hand: `consortium-cfg` emits it into
//! the generated `consortium.gen.rs` vector module from the manifest, which already
//! names the doorbell, its instance, and each party's side.
//!
//! MU lines are **level-sensitive**, so *both* sides mask before waking, through the
//! same internal `service_gi`: every asserted GI and RF line is disabled **before**
//! the wake. Disabling only masks the interrupt and leaves `GSR`/`RSR` set, so the
//! woken `wait` future still observes its flag, acknowledges it, and re-arms.
//!
//! On firmware, an ISR that only woke would return with the line still asserted and
//! re-enter immediately, spinning without ever letting the executor run the waiter.
//! On Linux, re-arming the UIO IRQ while a status flag is still set fires the line
//! again and the loop spins — so the doorbell is bound there as a *service* rather
//! than a plain notifier:
//!
//! ```rust,ignore
//! // Linux: bind the masking service, not the bare notifier.
//! let db = doorbell.clone();
//! uio.bind_service(std::sync::Arc::new(move || db.notify_gi_interrupt()));
//! ```
//!
//! `notifier()` is still exposed for integrators who mask
//! elsewhere, but the service is the correct default.
//!
//! # Features
//!
//! | Feature  | Effect                                                              |
//! | -------- | ------------------------------------------------------------------- |
//! | `mimx95` | Chip selection (default). Eight MU instances.                        |
//! | `mimx93` | Chip selection. Two MU instances.                                    |
//! | `std`    | Tokio `Notify` wake path and the masking service; host-testable.      |
//! | `defmt`  | Derive `defmt::Format` on [`MuError`].                               |
//!
//! Enabling both chip features is a compile error — the instance count and base table
//! differ. A dependent selecting the non-default chip must set
//! `default-features = false`, or the HAL's default chip is pulled in too and the
//! PAC's `Interrupt` enum becomes ambiguous.
//!
//! [`side`]: consortium_ipc::Side

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Mu;
pub use chan::{
    MU_CHAN_COUNT, MU_CHAN_OCCUPIED, MU_GI_CHAN_COUNT, MU_INSTANCE_COUNT, MU_RF_CHAN_COUNT,
    MuChanValidate,
};
pub use consortium_ipc::{Primary, Secondary, Side};
pub use doorbell::MuDoorbell;
#[cfg(not(feature = "std"))]
pub use doorbell::notify_gi_interrupt;
pub use error::MuError;
