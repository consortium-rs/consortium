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

//! STM32MP2 FDCAN (Bosch M_CAN) glue.
//!
//! The whole HAL `can` module is re-exported, so `Can`, `Config`, `Frame`,
//! `Error`, `REGION_BYTES` and `on_interrupt` are reachable from here without a
//! second dependency. [`split`] is the entry point: it takes a configured
//! [`Can`] and returns halves the transport accepts.
//!
//! # Message RAM
//!
//! All FDCAN instances share one `CAN_SRAM` window, and `Can::new` takes this
//! instance's byte offset within it. Leave at least [`REGION_BYTES`] between the
//! offsets handed to different instances; the driver lays out an eight-element
//! receive FIFO followed by one transmit buffer at each.

use consortium_hal_stm32mp2::can;
use consortium_ipc_transport_fdcan as fdcan;

pub use consortium_hal_stm32mp2::can::*;

/// Transmit half of an FDCAN instance, adapted to [`fdcan::CanTx`].
///
/// A newtype rather than an impl on the HAL type directly: both the trait and
/// the HAL half are foreign to this crate, so the orphan rule requires a local
/// type to hang the impl on.
pub struct CanBusTx(can::CanTx);

/// Receive half of an FDCAN instance, adapted to [`fdcan::CanRx`].
pub struct CanBusRx(can::CanRx);

/// Split a configured controller into halves the FDCAN transport accepts.
///
/// ```ignore
/// // SAFETY: CAN_SRAM base, this instance's reserved region, exclusively owned.
/// let can = unsafe { Can::new(p.FDCAN1, CAN_SRAM, 0, 80_000_000, Config::default()) };
/// let (tx, rx) = consortium_ipc_transport_fdcan_embedded::stm32mp2::split(can);
/// ```
pub fn split(can: can::Can) -> (CanBusTx, CanBusRx) {
    let (tx, rx) = can.split();
    (CanBusTx(tx), CanBusRx(rx))
}

impl CanBusTx {
    /// Wrap an already-split transmit half.
    pub fn new(inner: can::CanTx) -> Self {
        Self(inner)
    }
}

impl CanBusRx {
    /// Wrap an already-split receive half.
    pub fn new(inner: can::CanRx) -> Self {
        Self(inner)
    }
}

impl fdcan::CanErrorType for CanBusTx {
    type Error = can::Error;
}

impl fdcan::CanTx for CanBusTx {
    type Frame = can::Frame;

    async fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        self.0.transmit(frame).await
    }
}

impl fdcan::CanErrorType for CanBusRx {
    type Error = can::Error;
}

impl fdcan::CanRx for CanBusRx {
    type Frame = can::Frame;

    async fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        self.0.receive().await
    }
}
