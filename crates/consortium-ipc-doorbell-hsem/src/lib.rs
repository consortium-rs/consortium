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

//! Stale HSEM (Hardware Semaphore) doorbell driver for Consortium IPC.
//!
//! This crate is retained for experiments and existing STM32MP23/25 users, but
//! new STM32MP2 integrations should prefer `consortium-ipc-doorbell-ipcc`.
//! IPCC is present across STM32MP21/23/25 and has the better supported ST
//! integration path. HSEM is absent on STM32MP21, and field testing has shown
//! surprising access failures where the peripheral clock and RIF assignment
//! looked correct but reads still produced a bus error.
//!
//! A [`HsemDoorbell`] drives the HSEM peripheral from one processor [`side`],
//! with the side selected at the type level: [`Primary`] is core C1, [`Secondary`]
//! is C2 and [`Tertiary`] is C3. HSEM is single-instance, so the instance const
//! `N` is always `1` (e.g. `HsemDoorbell<Primary, 1>`). Its channels are the 16
//! hardware semaphores.
//!
//! # How the semaphore becomes a doorbell
//!
//! HSEM is a lock peripheral, not a mailbox, so the signaling is a side effect of
//! the locking protocol:
//!
//! | Operation | HSEM action |
//! | --------- | ----------- |
//! | `ring`     | 1-step read-lock, then immediately unlock — the *unlock* raises the remote core's HSEM interrupt |
//! | `wait`     | arm this core's `CnIER`, resolve when the semaphore appears in `CnMISR`, then clear it |
//! | `pending` | read `CnMISR` non-destructively                    |
//!
//! # Construction
//!
//! Two constructors, one per side of the system:
//!
//! - **Firmware** — [`HsemDoorbell::new`] consumes the chip HAL's `HSEM` peripheral
//!   singleton. A const assertion checks the singleton against the doorbell's type
//!   parameters, and the window address still comes from this crate's own
//!   [`registers::base`] table so the security alias it has always used does not
//!   change underfoot.
//! - **Linux/UIO** — [`HsemDoorbell::new_mapped`] is `unsafe` and takes an `mmap`'d
//!   virtual base, because a mapped window has no singleton to consume. Passing a
//!   physical address is undefined behaviour.
//!
//! The doorbell is [`Clone`], and clones share one wake source, so
//! `SharedMemoryTransport::split` can hand a copy to each half and an interrupt
//! reaches whichever half is waiting.
//!
//! # Interrupts are the application's
//!
//! Per repository convention this crate installs **no** `#[interrupt]` handlers. The
//! application owns the vector table and forwards the non-secure HSEM vector into the
//! crate's exported wake path. Enable the *HAL's* `rt` feature
//! (`consortium-hal-stm32mp2/rt`) so the `#[interrupt]` macro and PAC vector table
//! are available to that code.
//!
//! ```rust,ignore
//! // Firmware: forward the vector.
//! #[interrupt]
//! fn HSEM() {
//!     consortium_ipc_doorbell_hsem::notify();
//! }
//! ```
//!
//! On `std` the wake source is a Tokio `Notify` instead. `wait`
//! enables the line's `CnIER` while pending and disarms it on completion, so
//! userspace owns the mask and re-arming the UIO IRQ does not immediately re-fire a
//! level-sensitive line. Bind it through `consortium-runtime-app`'s UIO fan-out with
//! `bind_notifier(doorbell.notifier())`.
//!
//! # Features
//!
//! | Feature      | Effect                                                        |
//! | ------------ | ------------------------------------------------------------- |
//! | `stm32mp25x` | Chip selection (default). Forwarded to `consortium-hal-stm32mp2`. |
//! | `stm32mp23x` | Chip selection; shares the STM32MP25 HSEM layout.              |
//! | `std`        | Tokio `Notify` wake path, host-testable with fake MMIO.        |
//! | `defmt`      | Derive `defmt::Format` on [`HsemError`].                       |
//! | `tracing`    | Route diagnostics to `consortium-log`'s `tracing` backend.      |
//!
//! A dependent selecting a non-default chip must set `default-features = false`, or
//! the HAL's default chip is pulled in too and the PAC's `Interrupt` enum becomes
//! ambiguous.
//!
//! [`side`]: consortium_ipc::Side

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Hsem;
pub use chan::{HSEM_CHAN_COUNT, HSEM_CHAN_OCCUPIED, HsemChanValidate};
pub use consortium_ipc::{Primary, Secondary, Side, Tertiary};
pub use doorbell::HsemDoorbell;
#[cfg(not(feature = "std"))]
pub use doorbell::notify;
pub use error::HsemError;
