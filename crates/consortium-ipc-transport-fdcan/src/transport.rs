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

//! FDCAN-backed [`SendTransport`]/[`RecvTransport`] implementations.
//!
//! CAN is a *wired* transport with an implicit, transport-coupled (Type-B)
//! doorbell: the arrival of a frame is itself the notification, so unlike the
//! shared-memory transport there is no [`Doorbell`](consortium_ipc::Doorbell)
//! type parameter. There is no `Side` parameter either — direction is carried
//! by the identifier each half is bound to.
//!
//! Unlike the UART transport there is only one impl family here, because the
//! [`CanTx`]/[`CanRx`] seam is this crate's own and its futures are declared
//! `MaybeSend` directly. The `std` feature changes what `MaybeSend` requires,
//! not which code is compiled.

use core::future::Future;

use consortium_ipc::{
    Chan, Connect, MaybeSend, RecvTransport, SendTransport, Transport,
    TransportError as TransportErrorTrait,
};
use embedded_can::Id;

use crate::can::{CanFrame, CanRx, CanTx};
use crate::error::Error;
use crate::frame::{self, FD_PAYLOAD, Push, Reassembler, Segmenter};

/// Send half of an FDCAN transport: segments payloads and transmits them under
/// this endpoint's identifier. Pass it to `Channel::<Tx, …>`.
pub struct FdcanTransportTx<T> {
    chan: Chan,
    id: Id,
    io: T,
    mtu: usize,
}

/// Receive half of an FDCAN transport: filters by identifier and reassembles
/// segmented messages. Pass it to `Channel::<Rx, …>`.
pub struct FdcanTransportRx<R> {
    chan: Chan,
    id: Id,
    io: R,
    /// Reassembly accumulator, sized by [`frame::rx_buf_len`].
    buf: &'static mut [u8],
    mtu: usize,
    reasm: Reassembler,
}

/// Full-duplex FDCAN transport pairing a send and a receive half.
///
/// The halves carry independent controller types so a split HAL driver's halves
/// fit directly. [`split`](Self::split) yields them for simultaneous
/// `Channel<Tx, …>` / `Channel<Rx, …>` endpoints.
pub struct FdcanTransport<T, R> {
    tx: FdcanTransportTx<T>,
    rx: FdcanTransportRx<R>,
}

/// Reject an MTU the protocol cannot describe, shared by every constructor.
fn check_mtu<E>(mtu: usize) -> Result<(), Error<E>> {
    if mtu > frame::MAX_MTU {
        return Err(Error::PayloadTooLarge {
            payload_len: mtu,
            mtu: frame::MAX_MTU,
        });
    }
    Ok(())
}

impl<T: CanTx> FdcanTransportTx<T> {
    /// Construct a send half transmitting under `id`.
    ///
    /// Every frame this half emits carries `id`; the peer's receive half must be
    /// bound to the same value, and the reverse direction must use a different
    /// one so neither end reassembles its own traffic.
    pub fn new(
        chan: Chan,
        id: Id,
        io: T,
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        // A `CanFrame` impl this small could never make progress on a segmented
        // message; that is a bug in the impl, not a runtime condition.
        assert!(
            T::Frame::MAX_PAYLOAD >= frame::MIN_FRAME_PAYLOAD,
            "frame payload too small to carry a segmented message",
        );
        check_mtu(mtu)?;
        Ok(Self { chan, id, io, mtu })
    }

    /// The [`Chan`] this endpoint is bookkeeping for.
    pub fn chan(&self) -> Chan {
        self.chan
    }

    /// The CAN identifier every outgoing frame carries.
    pub fn id(&self) -> Id {
        self.id
    }

    /// Segment `payload` and transmit each frame in order.
    ///
    /// Ordering is what makes reassembly work, and it is the controller's to
    /// keep: [`CanTx::transmit`] resolves once the frame is on the bus, so the
    /// next segment is only loaded after the previous one left.
    async fn send_segmented(&mut self, payload: &[u8]) -> Result<(), Error<T::Error>> {
        if payload.len() > self.mtu {
            return Err(Error::PayloadTooLarge {
                payload_len: payload.len(),
                mtu: self.mtu,
            });
        }
        let cap = T::Frame::MAX_PAYLOAD;
        let mut scratch = [0u8; FD_PAYLOAD];
        let mut seg = Segmenter::new(payload, cap);
        let mut frames = 0usize;
        while let Some(len) = seg.next(&mut scratch[..cap]) {
            // Pad up to a length the medium can describe. On classic CAN every
            // length is representable and this is a no-op; on FD the DLC is
            // quantized above 8 bytes, so a 10-byte segment has to go out as
            // 12. The receiver ignores the difference — a `Single` carries its
            // own length and a `Continue` is bounded by the declared total.
            let padded = frame::padded_len(len).min(cap);
            scratch[len..padded].fill(0);
            let f = T::Frame::build(self.id, &scratch[..padded])
                .ok_or(Error::FrameRejected { len: padded })?;
            self.io.transmit(&f).await.map_err(Error::Can)?;
            frames += 1;
        }
        consortium_log::trace!(
            "fdcan send: {} payload bytes in {} frames on {:?}",
            payload.len(),
            frames,
            self.chan
        );
        Ok(())
    }
}

impl<R: CanRx> FdcanTransportRx<R> {
    /// Construct a receive half listening on `id`.
    ///
    /// `buf` is caller-provided scratch of at least
    /// [`frame::rx_buf_len`]`(mtu)` bytes (use
    /// `consortium-runtime-mcu::static_buf!` in firmware).
    pub fn new(
        chan: Chan,
        id: Id,
        io: R,
        buf: &'static mut [u8],
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        // See the note on the transmit half: an impl this small is a bug.
        assert!(
            R::Frame::MAX_PAYLOAD >= frame::MIN_FRAME_PAYLOAD,
            "frame payload too small to carry a segmented message",
        );
        check_mtu(mtu)?;
        let needed = frame::rx_buf_len(mtu);
        if buf.len() < needed {
            return Err(Error::BufferTooSmall {
                needed,
                got: buf.len(),
            });
        }
        Ok(Self {
            chan,
            id,
            io,
            buf,
            mtu,
            reasm: Reassembler::default(),
        })
    }

    /// The [`Chan`] this endpoint is bookkeeping for.
    pub fn chan(&self) -> Chan {
        self.chan
    }

    /// The CAN identifier this half accepts frames from.
    pub fn id(&self) -> Id {
        self.id
    }

    /// Await frames until one completes a message, copying it into `out`.
    ///
    /// Frames that are not ours, or that break the segmentation protocol, are
    /// logged and dropped: a shared bus is expected to carry other traffic, and
    /// a lost frame should cost one message rather than the endpoint.
    async fn recv_message(&mut self, out: &mut [u8]) -> Result<usize, Error<R::Error>> {
        loop {
            let f = self.io.receive().await.map_err(Error::Can)?;

            let outcome = if f.id() != self.id {
                Err(frame::FrameError::ForeignId)
            } else if f.is_remote() {
                Err(frame::FrameError::Remote)
            } else {
                self.reasm.push(self.buf, self.mtu, f.data())
            };

            match outcome {
                Ok(Push::Consumed) => {}
                Ok(Push::Message { len }) => {
                    consortium_log::trace!("fdcan recv: {} byte message on {:?}", len, self.chan);
                    out[..len].copy_from_slice(&self.buf[..len]);
                    return Ok(len);
                }
                Err(e) => {
                    consortium_log::warn!("fdcan recv: dropped frame on {:?}: {:?}", self.chan, e);
                }
            }
        }
    }
}

impl<T: CanTx, R: CanRx> FdcanTransport<T, R> {
    /// Construct a full-duplex transport.
    ///
    /// `tx_id` and `rx_id` must differ, and equality is rejected with
    /// [`Error::SameIdentifier`]: both halves sit on the same bus, so a shared
    /// identifier would make this endpoint reassemble its own frames. That is
    /// not hypothetical — a controller configured for loopback or
    /// self-reception, as bring-up self-tests use, echoes every frame back.
    ///
    /// Two halves on genuinely separate buses may share an identifier; build
    /// them with [`FdcanTransportTx::new`] and [`FdcanTransportRx::new`]
    /// directly, which do not pair the two directions.
    pub fn new(
        chan: Chan,
        tx_id: Id,
        rx_id: Id,
        tx_io: T,
        rx_io: R,
        rx_buf: &'static mut [u8],
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        if tx_id == rx_id {
            return Err(Error::SameIdentifier);
        }
        Ok(Self {
            tx: FdcanTransportTx::new(chan, tx_id, tx_io, mtu)?,
            rx: FdcanTransportRx::new(chan, rx_id, rx_io, rx_buf, mtu)?,
        })
    }

    /// The [`Chan`] both halves are bookkeeping for.
    pub fn chan(&self) -> Chan {
        self.tx.chan
    }

    /// Split into the halves for simultaneous typed endpoints.
    pub fn split(self) -> (FdcanTransportTx<T>, FdcanTransportRx<R>) {
        (self.tx, self.rx)
    }
}

impl<T: CanTx> TransportErrorTrait for FdcanTransportTx<T> {
    type Error = Error<T::Error>;
}

impl<T: CanTx> SendTransport for FdcanTransportTx<T> {
    async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.send_segmented(data).await
    }

    fn max_send_size(&self) -> usize {
        self.mtu
    }
}

impl<R: CanRx> TransportErrorTrait for FdcanTransportRx<R> {
    type Error = Error<R::Error>;
}

impl<R: CanRx> RecvTransport for FdcanTransportRx<R> {
    async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < self.mtu {
            return Err(Error::BufferTooSmall {
                needed: self.mtu,
                got: buf.len(),
            });
        }
        self.recv_message(buf).await
    }

    fn max_recv_size(&self) -> usize {
        self.mtu
    }
}

// The full-duplex impls require both halves to agree on the controller's error
// type, so the transport as a whole has one.
impl<T: CanTx, R: CanRx<Error = T::Error>> TransportErrorTrait for FdcanTransport<T, R> {
    type Error = Error<T::Error>;
}

impl<T: CanTx, R: CanRx<Error = T::Error>> SendTransport for FdcanTransport<T, R> {
    // Spelled as an explicit RPITIT rather than an `async fn` so delegation is a
    // plain forward instead of another future layer.
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.tx.send(data)
    }

    fn max_send_size(&self) -> usize {
        self.tx.max_send_size()
    }
}

impl<T: CanTx, R: CanRx<Error = T::Error>> RecvTransport for FdcanTransport<T, R> {
    fn recv(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        self.rx.recv(buf)
    }

    fn max_recv_size(&self) -> usize {
        self.rx.max_recv_size()
    }
}

impl<T: CanTx, R: CanRx<Error = T::Error>> Transport for FdcanTransport<T, R> {}

impl<T: CanTx, R: CanRx<Error = T::Error>> Connect for FdcanTransport<T, R> {
    type Params = ();

    /// CAN has nothing to rendezvous over: the bus is a shared pair of wires,
    /// frame arrival is itself the notification, and neither end publishes a
    /// descriptor the other must validate. `connect` is therefore an immediate
    /// no-op, provided so bring-up code can drive every transport through the
    /// same call. A future link-level hello (version or MTU negotiation over the
    /// bus) would slot in here without touching callers.
    async fn connect(&mut self, _params: Self::Params) -> Result<(), Self::Error> {
        Ok(())
    }
}
