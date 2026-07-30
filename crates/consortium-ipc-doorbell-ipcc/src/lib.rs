#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
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

//! STM32 inter-processor communication controller (IPCC) doorbell support.
//!
//! The IPCC carries no payload. It only exposes per-channel status flags used
//! to transfer ownership of shared memory between processor 1 and processor 2.
//! This crate uses simplex signaling: `IpccDoorbell::ring` sets the local
//! transmit channel occupied, and `IpccDoorbell::wait` waits for the remote
//! transmit channel to become occupied before clearing it.
//!
//! The processor side and IPCC instance are selected at the type level:
//! [`Primary`] drives processor 1, [`Secondary`] processor 2; `N` is the instance
//! (`IpccDoorbell<Primary, 1>` is processor 1 on IPCC1). Channels are the 16
//! IPCC channels.
//!
//! This is the **recommended STM32MP2 doorbell**. IPCC is present across
//! STM32MP21/23/25 and has the better-supported ST integration path; HSEM is absent
//! on STM32MP21 and has shown surprising bus-error access failures in the field.
//!
//! # Signaling model
//!
//! IPCC exposes a pair of directional channel banks and a status flag per channel.
//! This crate uses the simplex half of the protocol:
//!
//! | Operation | IPCC action |
//! | --------- | ----------- |
//! | `ring`        | set the local transmit channel *occupied* (`CnSCR.CHnS`) |
//! | `wait`        | await the remote transmit channel becoming occupied, then clear it |
//! | `pending`  | read the incoming status flag non-destructively     |
//!
//! Because occupancy is a flag rather than a queue, `ring` returns
//! [`IpccError::ChannelOccupied`] when the previous signal has not been consumed. That
//! is a real condition, not an internal retry: the peer has not caught up.
//!
//! # Construction
//!
//! - **Firmware** — [`IpccDoorbell::new`] consumes the chip HAL's `IPCCn` peripheral
//!   singleton; a const assertion checks it against the doorbell's type parameters.
//!   The window address still comes from this crate's [`registers::base`] table so the
//!   security alias does not change underfoot.
//! - **Linux/UIO** — [`IpccDoorbell::new_mapped`] is `unsafe` and takes an `mmap`'d
//!   virtual base, because a mapped window has no singleton. Passing a physical
//!   address is undefined behaviour.
//!
//! The doorbell is [`Clone`] and clones share one wake source, so
//! `SharedMemoryTransport::split` can give a copy to each half.
//!
//! # Interrupts, and why IPCC needs masking
//!
//! This crate installs **no** `#[interrupt]` handlers. The application owns the
//! vector table and forwards the non-secure `IPCC1_RX` / `IPCC2_RX` vectors into
//! `notify_rx_occupied_interrupt`. Enable the *HAL's* `rt` feature
//! (`consortium-hal-stm32mp2/rt`) so the `#[interrupt]` macro and PAC vector table are
//! available there.
//!
//! ```rust,ignore
//! #[interrupt]
//! fn IPCC1_RX() {
//!     consortium_ipc_doorbell_ipcc::notify_rx_occupied_interrupt();
//! }
//! ```
//!
//! The IPCC line is **level-sensitive**, which matters on Linux: if the UIO IRQ loop
//! re-arms the interrupt while a channel is still occupied, the line fires again
//! immediately and the loop spins. So on `std` the doorbell is bound as a *service*
//! rather than a plain notifier — `IpccDoorbell::notify_rx_occupied_interrupt` masks
//! every asserted channel **before** waking, and the woken `wait`
//! future acknowledges and re-arms its own channel:
//!
//! ```rust,ignore
//! // Linux: bind the masking service, not the bare notifier.
//! let db = doorbell.clone();
//! uio.bind_service(std::sync::Arc::new(move || db.notify_rx_occupied_interrupt()));
//! ```
//!
//! `notifier()` is still exposed for integrators who mask
//! elsewhere, but the service is the correct default.
//!
//! # Features
//!
//! | Feature      | Effect                                                          |
//! | ------------ | --------------------------------------------------------------- |
//! | `stm32mp25x` | Chip selection (default). IPCC1 plus the SmartRun IPCC2.          |
//! | `stm32mp23x` | Chip selection. IPCC1 plus the SmartRun IPCC2.                    |
//! | `stm32mp21x` | Chip selection. IPCC1 only.                                       |
//! | `std`        | Tokio `Notify` wake path and the masking service; host-testable.   |
//! | `defmt`      | Derive `defmt::Format` on [`IpccError`].                          |
//! | `tracing`    | Route diagnostics to `consortium-log`'s `tracing` backend.          |
//!
//! A dependent selecting a non-default chip must set `default-features = false`, or
//! the HAL's default chip is pulled in too and the PAC's `Interrupt` enum becomes
//! ambiguous.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Ipcc;
pub use chan::{IPCC_CHAN_COUNT, IpccChanValidate};
pub use consortium_ipc::{Primary, Secondary, Side};
#[cfg(not(feature = "std"))]
pub use doorbell::notify_rx_occupied_interrupt;
pub use doorbell::{IpccDoorbell, IpccWaitFut};
pub use error::IpccError;
