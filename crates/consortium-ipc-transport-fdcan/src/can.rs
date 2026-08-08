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

//! The controller seam: what this crate needs from a CAN driver.
//!
//! The UART transport rides on `embedded-io-async`, the ecosystem's async byte
//! stream. CAN has no equivalent: `embedded-can` 0.4 offers only blocking and
//! `nb` traits, and a bus carries discrete frames rather than bytes. So this
//! crate declares its own three-trait seam.
//!
//! Owning the seam buys something the UART crate does not have. Its futures are
//! declared `impl Future<..> + MaybeSend` — the same RPITIT shape the upstream
//! [`SendTransport`](consortium_ipc::SendTransport) uses — so one set of
//! transport impls satisfies both firmware and `std`. The UART crate needs two
//! parallel impl families only because `embedded_io_async`'s unnameable futures
//! cannot be proven `Send` generically.
//!
//! [`CanFrame`] is blanket-implemented for every [`embedded_can::Frame`], so
//! both chip HAL frame types work with no adapter, and an FD frame type with a
//! 64-byte payload slots in by raising `MAX_PAYLOAD`.

use core::future::Future;

use consortium_ipc::MaybeSend;
use embedded_can::Id;

/// A CAN data frame this transport can carry a payload in.
///
/// Implementors report [`MAX_PAYLOAD`](CanFrame::MAX_PAYLOAD) so the framing
/// layer can size its segmentation to the medium: 8 bytes for classic CAN, 64
/// for CAN FD.
pub trait CanFrame: Sized {
    /// Largest data payload this frame type carries.
    const MAX_PAYLOAD: usize;

    /// Build a data frame, or `None` if `data` exceeds
    /// [`MAX_PAYLOAD`](CanFrame::MAX_PAYLOAD) (or is not a valid length for the
    /// medium).
    fn build(id: Id, data: &[u8]) -> Option<Self>;

    /// This frame's identifier.
    fn id(&self) -> Id;

    /// This frame's data bytes.
    fn data(&self) -> &[u8];

    /// Whether this is a remote-transmission-request frame, which carries no
    /// data and is dropped by the receiver.
    fn is_remote(&self) -> bool;
}

impl<F: embedded_can::Frame> CanFrame for F {
    // `embedded_can::Frame` describes classic CAN: `Frame::new` rejects
    // anything longer than eight bytes. An FD frame type implements `CanFrame`
    // directly rather than going through this blanket impl.
    const MAX_PAYLOAD: usize = 8;

    fn build(id: Id, data: &[u8]) -> Option<Self> {
        embedded_can::Frame::new(id, data)
    }

    fn id(&self) -> Id {
        embedded_can::Frame::id(self)
    }

    fn data(&self) -> &[u8] {
        embedded_can::Frame::data(self)
    }

    fn is_remote(&self) -> bool {
        embedded_can::Frame::is_remote_frame(self)
    }
}

/// The error a CAN half reports. Split out so [`CanTx`] and [`CanRx`] can be
/// implemented separately for the two halves of a split driver while agreeing
/// on one error type.
pub trait CanErrorType {
    /// Driver-reported bus or transfer error.
    type Error;
}

/// Transmit half of a CAN controller.
///
/// The `MaybeSend` bounds are what let the transport satisfy
/// [`SendTransport`](consortium_ipc::SendTransport)'s `MaybeSend` future: under
/// `std` they require `Send`, and under `no_std` they are vacuous. Carrying them
/// here keeps every impl site in the transport free of repetition.
pub trait CanTx: CanErrorType + MaybeSend {
    /// Frame type this controller transmits.
    type Frame: CanFrame + MaybeSend;

    /// Transmit `frame`, resolving once the controller has handed it to the bus.
    ///
    /// Back-pressure is the implementor's: a driver with one transmit buffer
    /// should await completion rather than overwrite a pending frame, because
    /// the segmentation layer sends a message's frames back to back and relies
    /// on them arriving in order.
    fn transmit(
        &mut self,
        frame: &Self::Frame,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}

/// Receive half of a CAN controller.
///
/// See [`CanTx`] for why the `MaybeSend` bounds are here.
pub trait CanRx: CanErrorType + MaybeSend {
    /// Frame type this controller receives.
    type Frame: CanFrame + MaybeSend;

    /// Await the next frame from the bus.
    ///
    /// Implementors deliver every frame the hardware accepts; identifier
    /// filtering is the transport's job unless the controller was programmed
    /// with hardware acceptance filters.
    fn receive(&mut self) -> impl Future<Output = Result<Self::Frame, Self::Error>> + MaybeSend;
}
