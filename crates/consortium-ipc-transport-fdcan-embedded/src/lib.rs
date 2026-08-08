#![no_std]
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

//! Firmware-side glue for the Consortium FDCAN IPC transport.
//!
//! Re-exports [`consortium-ipc-transport-fdcan`](consortium_ipc_transport_fdcan)
//! and, when a chip feature is selected, that chip HAL's CAN driver plus the
//! adapters that let its split halves drive the transport.
//!
//! With **no** chip feature the crate is chip-agnostic: the transport works with
//! any type implementing [`CanTx`]/[`CanRx`], so an Embassy or vendor driver
//! needs only those two impls. That is why `default = []`.
//!
//! # Interrupts
//!
//! This crate installs no `#[interrupt]` handlers, per the repository-wide
//! convention: the crate exports the entry point, the application decides which
//! vector reaches it. The chip HAL's `on_interrupt` is parameterized by the
//! peripheral singleton, so the application writes:
//!
//! ```ignore
//! use consortium_hal_stm32mp2::peripherals;
//! use consortium_ipc_transport_fdcan_embedded::stm32mp2::on_interrupt;
//!
//! #[interrupt]
//! fn FDCAN1_IT0() {
//!     on_interrupt::<peripherals::FDCAN1>();
//! }
//! ```
//!
//! The driver enables interrupt line 0 on STM32MP2 (`FDCANx_IT0`); i.MX9 raises
//! one line per instance. Without that handler the async paths never wake — the
//! blocking `embedded_can::nb::Can` paths still work.
//!
//! # Bring-up
//!
//! Clocks, pin muxing and the bit rate are `consortium-cfg`'s job via
//! `[peripheral.*]`; this crate only adapts an already-configured controller.
//!
//! ```ignore
//! use consortium_ipc_transport_fdcan::{FdcanTransport, embedded_can::{Id, StandardId}};
//! use consortium_ipc_transport_fdcan_embedded::stm32mp2;
//!
//! let (tx, rx) = stm32mp2::split(can);
//! let mut transport = FdcanTransport::new(
//!     chan,
//!     Id::Standard(StandardId::new(0x101).unwrap()), // we transmit here
//!     Id::Standard(StandardId::new(0x102).unwrap()), // the peer transmits here
//!     tx,
//!     rx,
//!     RX_BUF.init([0; rx_buf_len(MTU)]),
//!     MTU,
//! )?;
//! ```
//!
//! The two directions must use different identifiers; see the core crate's docs.
//!
//! # Features
//!
//! | Feature | Effect |
//! | ------- | ------ |
//! | none (default) | Chip-agnostic; bring your own [`CanTx`]/[`CanRx`]. |
//! | `mimx93`, `mimx95` | i.MX9 FlexCAN via [`imx9`]. |
//! | `stm32mp21x`, `stm32mp23x`, `stm32mp25x` | STM32MP2 FDCAN via [`stm32mp2`]. |
//! | `defmt` | Routes the transport's logging to `defmt`. |
//!
//! Choose exactly one chip feature; the HAL rejects several at once.

#[cfg(any(feature = "mimx93", feature = "mimx95"))]
pub mod imx9;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub mod stm32mp2;

pub use consortium_ipc_transport_fdcan::{
    CLASSIC_PAYLOAD, CanErrorType, CanFrame, CanRx, CanTx, Error, FD_PAYLOAD, FIRST_EXTRA,
    FdcanTransport, FdcanTransportRx, FdcanTransportTx, HEADER_LEN, MAX_MTU, MIN_FRAME_PAYLOAD,
    continue_max, embedded_can, first_max, frames_for, padded_len, rx_buf_len, single_max,
};
