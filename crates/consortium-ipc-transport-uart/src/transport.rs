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

//! UART-backed [`SendTransport`]/[`RecvTransport`] implementations.
//!
//! UART is a *wired* transport with an implicit, transport-coupled (Type-B)
//! doorbell: the arrival of bytes is itself the notification, so unlike the
//! shared-memory transport there is no [`Doorbell`](consortium_ipc::Doorbell)
//! type parameter. There is no `Side` parameter either — direction is
//! physical (the TX and RX wires).
//!
//! The generic IO bounds are selected by the `std` feature:
//!
//! - without `std` (firmware): `embedded_io_async::Read`/`Write`, the async
//!   serial traits implemented by Embassy and modern HAL UART drivers;
//! - with `std` (Linux/host): Tokio `AsyncRead`/`AsyncWrite`, because under
//!   `std` transport futures must be `Send` (`MaybeSend`) and
//!   `embedded_io_async`'s `async fn` futures cannot guarantee that
//!   generically. The `consortium-ipc-transport-uart-unix` crate supplies
//!   `/dev/tty*`-backed halves for this family.

use consortium_ipc::Chan;

use crate::error::Error;
use crate::frame::{self, Deframer, Push, RX_CHUNK};

/// Send half of a UART transport: frames payloads and writes them to the
/// UART TX wire. Pass it to `Channel::<Tx, …>`.
pub struct UartTransportTx<W> {
    chan: Chan,
    io: W,
    /// Staging space: `[frame body | encoded frame]`, sized by
    /// [`frame::tx_buf_len`].
    buf: &'static mut [u8],
    mtu: usize,
}

/// Receive half of a UART transport: accumulates wire bytes, deframes and
/// validates them. Pass it to `Channel::<Rx, …>`.
pub struct UartTransportRx<R> {
    chan: Chan,
    io: R,
    /// `[read chunk | frame accumulator]`, sized by [`frame::rx_buf_len`].
    buf: &'static mut [u8],
    mtu: usize,
    deframer: Deframer,
    /// Unprocessed region of the read chunk: `buf[chunk_pos..chunk_len]`.
    chunk_len: usize,
    chunk_pos: usize,
}

/// Full-duplex UART transport pairing a send and a receive half.
///
/// The halves carry independent IO types so split HAL UART halves (or split
/// Tokio halves) fit directly. [`split`](Self::split) yields the halves for
/// simultaneous `Channel<Tx, …>` / `Channel<Rx, …>` endpoints.
pub struct UartTransport<R, W> {
    tx: UartTransportTx<W>,
    rx: UartTransportRx<R>,
}

impl<W> UartTransportTx<W> {
    /// Construct a send half over the UART TX wire `io`.
    ///
    /// `buf` is caller-provided scratch of at least
    /// [`frame::tx_buf_len`]`(mtu)` bytes (use
    /// `consortium-runtime-mcu::static_buf!` in firmware).
    pub fn new(
        chan: Chan,
        io: W,
        buf: &'static mut [u8],
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        let needed = frame::tx_buf_len(mtu);
        if buf.len() < needed {
            return Err(Error::BufferTooSmall {
                needed,
                got: buf.len(),
            });
        }
        Ok(Self { chan, io, buf, mtu })
    }

    /// The [`Chan`] whose byte is stamped into every frame header.
    pub fn chan(&self) -> Chan {
        self.chan
    }

    /// Frame `payload` into the staging buffer.
    ///
    /// Returns the `(offset, len)` of the encoded frame within `self.buf`.
    fn stage<E>(&mut self, payload: &[u8]) -> Result<(usize, usize), Error<E>> {
        if payload.len() > self.mtu {
            return Err(Error::PayloadTooLarge {
                payload_len: payload.len(),
                mtu: self.mtu,
            });
        }
        let inner_cap = frame::inner_len(self.mtu);
        let (inner, out) = self.buf.split_at_mut(inner_cap);
        let encoded = frame::encode_frame(self.chan.into(), payload, inner, out);
        consortium_log::trace!(
            "uart send: {} payload bytes framed to {} wire bytes on {:?}",
            payload.len(),
            encoded,
            self.chan
        );
        Ok((inner_cap, encoded))
    }
}

impl<R> UartTransportRx<R> {
    /// Construct a receive half over the UART RX wire `io`.
    ///
    /// `buf` is caller-provided scratch of at least
    /// [`frame::rx_buf_len`]`(mtu)` bytes.
    pub fn new(
        chan: Chan,
        io: R,
        buf: &'static mut [u8],
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        let needed = frame::rx_buf_len(mtu);
        if buf.len() < needed {
            return Err(Error::BufferTooSmall {
                needed,
                got: buf.len(),
            });
        }
        Ok(Self {
            chan,
            io,
            buf,
            mtu,
            deframer: Deframer::default(),
            chunk_len: 0,
            chunk_pos: 0,
        })
    }

    /// The [`Chan`] accepted from incoming frame headers.
    pub fn chan(&self) -> Chan {
        self.chan
    }

    /// End of the frame accumulator region within `self.buf`.
    fn acc_end(&self) -> usize {
        RX_CHUNK + frame::max_frame_len(self.mtu) - 1
    }

    /// Feed buffered wire bytes through the deframer.
    ///
    /// Returns the payload length once a valid frame has been copied into
    /// `out`; `None` means the chunk was exhausted first. Invalid frames are
    /// logged and skipped so a corrupt peer cannot wedge the receiver.
    fn drain(&mut self, out: &mut [u8]) -> Option<usize> {
        let acc_end = self.acc_end();
        while self.chunk_pos < self.chunk_len {
            let byte = self.buf[self.chunk_pos];
            self.chunk_pos += 1;
            let acc = &mut self.buf[RX_CHUNK..acc_end];
            match self.deframer.push(acc, byte) {
                Push::Consumed => {}
                Push::Skipped => {
                    consortium_log::warn!("uart recv: dropped oversized or empty frame");
                }
                Push::Frame { len } => {
                    match frame::decode_frame(&mut acc[..len], self.chan.into()) {
                        Ok(n) if n <= out.len() => {
                            out[..n].copy_from_slice(&acc[1..1 + n]);
                            consortium_log::trace!(
                                "uart recv: {} payload bytes on {:?}",
                                n,
                                self.chan
                            );
                            return Some(n);
                        }
                        Ok(n) => {
                            consortium_log::warn!(
                                "uart recv: dropped {}-byte payload exceeding receive buffer",
                                n
                            );
                        }
                        Err(err) => {
                            consortium_log::warn!("uart recv: dropped invalid frame: {:?}", err);
                        }
                    }
                }
            }
        }
        None
    }
}

impl<R, W> UartTransport<R, W> {
    /// Construct a full-duplex transport from independent RX/TX wire halves.
    ///
    /// `rx_buf` and `tx_buf` are caller-provided scratch sized by
    /// [`frame::rx_buf_len`] and [`frame::tx_buf_len`].
    pub fn new(
        chan: Chan,
        rx_io: R,
        tx_io: W,
        rx_buf: &'static mut [u8],
        tx_buf: &'static mut [u8],
        mtu: usize,
    ) -> Result<Self, Error<core::convert::Infallible>> {
        Ok(Self {
            tx: UartTransportTx::new(chan, tx_io, tx_buf, mtu)?,
            rx: UartTransportRx::new(chan, rx_io, rx_buf, mtu)?,
        })
    }

    /// The [`Chan`] both halves are bound to.
    pub fn chan(&self) -> Chan {
        self.tx.chan
    }

    /// Consume this transport and return a `(send, recv)` half-pair, for
    /// simultaneous `Channel<Tx, …>` and `Channel<Rx, …>` endpoints.
    pub fn split(self) -> (UartTransportTx<W>, UartTransportRx<R>) {
        (self.tx, self.rx)
    }
}

/// `embedded-io-async` implementation family (firmware-side).
#[cfg(not(feature = "std"))]
mod impl_embedded {
    use consortium_ipc::{
        Connect, MaybeSend, RecvTransport, SendTransport, Transport,
        TransportError as TransportErrorTrait,
    };
    use embedded_io_async::{ErrorType, Read, Write};

    use super::{UartTransport, UartTransportRx, UartTransportTx};
    use crate::error::Error;
    use crate::frame::RX_CHUNK;

    impl<W: Write> TransportErrorTrait for UartTransportTx<W> {
        type Error = Error<W::Error>;
    }

    impl<W: Write> SendTransport for UartTransportTx<W> {
        async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            let (start, len) = self.stage(data)?;
            self.io
                .write_all(&self.buf[start..start + len])
                .await
                .map_err(Error::Io)?;
            self.io.flush().await.map_err(Error::Io)
        }

        fn max_send_size(&self) -> usize {
            self.mtu
        }
    }

    impl<R: Read> TransportErrorTrait for UartTransportRx<R> {
        type Error = Error<R::Error>;
    }

    impl<R: Read> RecvTransport for UartTransportRx<R> {
        async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            if buf.len() < self.mtu {
                return Err(Error::BufferTooSmall {
                    needed: self.mtu,
                    got: buf.len(),
                });
            }
            loop {
                if let Some(n) = self.drain(buf) {
                    return Ok(n);
                }
                let chunk = &mut self.buf[..RX_CHUNK];
                let n = self.io.read(chunk).await.map_err(Error::Io)?;
                if n == 0 {
                    return Err(Error::Eof);
                }
                self.chunk_pos = 0;
                self.chunk_len = n;
            }
        }

        fn max_recv_size(&self) -> usize {
            self.mtu
        }
    }

    impl<R: Read, W: Write + ErrorType<Error = R::Error>> TransportErrorTrait for UartTransport<R, W> {
        type Error = Error<R::Error>;
    }

    impl<R: Read, W: Write + ErrorType<Error = R::Error>> SendTransport for UartTransport<R, W> {
        fn send(
            &mut self,
            data: &[u8],
        ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
            self.tx.send(data)
        }

        fn max_send_size(&self) -> usize {
            self.tx.max_send_size()
        }
    }

    impl<R: Read, W: Write + ErrorType<Error = R::Error>> RecvTransport for UartTransport<R, W> {
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

    impl<R: Read, W: Write + ErrorType<Error = R::Error>> Transport for UartTransport<R, W> {}

    /// UART has nothing to rendezvous over: the link is a pair of wires, byte
    /// arrival is itself the notification, and neither end publishes state the
    /// other must validate. `connect` is therefore an immediate no-op, provided
    /// so bring-up code can drive every transport through the same call.
    ///
    /// A future link-level hello (version or MTU negotiation over the wire)
    /// would slot in here without touching callers.
    impl<R: Read, W: Write + ErrorType<Error = R::Error>> Connect for UartTransport<R, W> {
        type Params = ();

        async fn connect(&mut self, _params: ()) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}

/// Tokio implementation family (Linux/host-side).
#[cfg(feature = "std")]
mod impl_tokio {
    use consortium_ipc::{
        Connect, MaybeSend, RecvTransport, SendTransport, Transport,
        TransportError as TransportErrorTrait,
    };
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

    use super::{UartTransport, UartTransportRx, UartTransportTx};
    use crate::error::Error;
    use crate::frame::RX_CHUNK;

    impl<W: AsyncWrite + Unpin + Send> TransportErrorTrait for UartTransportTx<W> {
        type Error = Error<std::io::Error>;
    }

    impl<W: AsyncWrite + Unpin + Send> SendTransport for UartTransportTx<W> {
        async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            let (start, len) = self.stage(data)?;
            self.io
                .write_all(&self.buf[start..start + len])
                .await
                .map_err(Error::Io)?;
            self.io.flush().await.map_err(Error::Io)
        }

        fn max_send_size(&self) -> usize {
            self.mtu
        }
    }

    impl<R: AsyncRead + Unpin + Send> TransportErrorTrait for UartTransportRx<R> {
        type Error = Error<std::io::Error>;
    }

    impl<R: AsyncRead + Unpin + Send> RecvTransport for UartTransportRx<R> {
        async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            if buf.len() < self.mtu {
                return Err(Error::BufferTooSmall {
                    needed: self.mtu,
                    got: buf.len(),
                });
            }
            loop {
                if let Some(n) = self.drain(buf) {
                    return Ok(n);
                }
                let chunk = &mut self.buf[..RX_CHUNK];
                let n = self.io.read(chunk).await.map_err(Error::Io)?;
                if n == 0 {
                    return Err(Error::Eof);
                }
                self.chunk_pos = 0;
                self.chunk_len = n;
            }
        }

        fn max_recv_size(&self) -> usize {
            self.mtu
        }
    }

    impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> TransportErrorTrait
        for UartTransport<R, W>
    {
        type Error = Error<std::io::Error>;
    }

    impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> SendTransport
        for UartTransport<R, W>
    {
        fn send(
            &mut self,
            data: &[u8],
        ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
            self.tx.send(data)
        }

        fn max_send_size(&self) -> usize {
            self.tx.max_send_size()
        }
    }

    impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> RecvTransport
        for UartTransport<R, W>
    {
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

    impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> Transport for UartTransport<R, W> {}

    /// UART has nothing to rendezvous over: the link is a pair of wires, byte
    /// arrival is itself the notification, and neither end publishes state the
    /// other must validate. `connect` is therefore an immediate no-op, provided
    /// so bring-up code can drive every transport through the same call.
    ///
    /// A future link-level hello (version or MTU negotiation over the wire)
    /// would slot in here without touching callers.
    impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> Connect for UartTransport<R, W> {
        type Params = ();

        async fn connect(&mut self, _params: ()) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}
