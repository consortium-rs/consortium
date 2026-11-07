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

//! The SMT channel as a Consortium transport.
//!
//! [`SmtTransport`] pairs an [`Smt`] window with the doorbell that announces a
//! request and the [`Completion`] that announces the reply, and exposes one
//! exchange as the non-waiting pair every memory-backed Consortium link shares:
//!
//! - [`try_send_message`](SmtTransport::try_send_message) writes a request into a
//!   free channel and rings, or reports [`TryError::WouldBlock`] while the platform
//!   still owns the channel;
//! - [`try_recv_message`](SmtTransport::try_recv_message) reads the reply once the
//!   platform has freed the channel, or reports `WouldBlock` until then.
//!
//! The waiting forms are written once over that attempt, with the
//! `consortium-core` helpers: [`recv_message`](SmtTransport::recv_message) parks on
//! the completion line through [`park_until`] (or yields between polls), and
//! [`blocking_recv_message`](SmtTransport::blocking_recv_message) spins through
//! [`spin_until`] against a caller deadline, for code with no executor.
//!
//! The message API keeps the header apart from the payload, so a request is written
//! straight from the caller's buffer into the window with no staging copy. The
//! byte-frame traits ([`TrySendTransport`] / [`TryRecvTransport`]) view the same
//! exchange as one frame — the header word followed by the payload — for code
//! generic over any try transport.
//!
//! # One message in flight
//!
//! SMT has one buffer for both directions, so a channel carries one exchange at a
//! time. The transport remembers that a request is outstanding: until one is, the
//! free bit means "nothing sent yet", not "replied", and a receive reports
//! `WouldBlock` rather than reading a stale reply. A request abandoned before its
//! reply is read (a cancelled call) is simply superseded by the next send once the
//! platform frees the channel.

use core::cell::Cell;

use consortium_core::{
    Chan, Doorbell, Region, TransportError, TryError, TryRecvTransport, TrySendTransport,
    wait::{BlockingError, Readiness, park_until, spin_until, yield_now},
};

use crate::{
    client::Completion,
    error::{Error, ProtocolError},
    smt::{Response, STATUS_ERROR, STATUS_FREE, Smt},
};

/// Bytes of a frame before its payload: the message header.
const HEADER_LEN: usize = 4;
/// Bytes of a received frame before its return values: header and status word.
const REPLY_PREFIX: usize = HEADER_LEN + 4;

/// An SMT channel with its doorbell: the agent's end of one SCMI transport.
pub struct SmtTransport<R, D> {
    smt: Smt<R>,
    doorbell: D,
    a2p: Chan,
    completion: Completion,
    /// A request has been handed to the platform and its reply not yet read.
    ///
    /// A `Cell` because the readiness check [`park_until`] drives sees the
    /// transport by shared reference; it is `Send`, so a future holding
    /// `&mut SmtTransport` stays `Send` too.
    outstanding: Cell<bool>,
}

impl<R: Region, D: Doorbell> SmtTransport<R, D> {
    /// A transport that writes requests into `smt`, rings `a2p` on `doorbell`, and
    /// learns of completion through `completion`.
    pub fn new(smt: Smt<R>, doorbell: D, a2p: Chan, completion: Completion) -> Self {
        Self {
            smt,
            doorbell,
            a2p,
            completion,
            outstanding: Cell::new(false),
        }
    }

    /// Take the channel and doorbell back.
    pub fn into_parts(self) -> (Smt<R>, D) {
        (self.smt, self.doorbell)
    }

    /// The window backing this transport.
    pub fn smt(&self) -> &Smt<R> {
        &self.smt
    }

    /// How this transport learns of completion.
    pub fn completion(&self) -> Completion {
        self.completion
    }

    /// Hand a request to the platform if the channel is free now, and ring.
    ///
    /// [`TryError::WouldBlock`] means the platform still owns the channel — an
    /// earlier exchange it has not finished — and nothing was written. A payload
    /// longer than the channel is [`ProtocolError::RequestTooLarge`].
    pub fn try_send_message(
        &mut self,
        header: u32,
        payload: &[u8],
    ) -> Result<(), TryError<Error<D::Error>>> {
        if self.smt.status() & STATUS_FREE == 0 {
            return Err(TryError::WouldBlock);
        }
        let irq = matches!(self.completion, Completion::Doorbell(_));
        self.smt
            .write_request(header, payload, irq)
            .map_err(|e| TryError::Fail(e.into()))?;
        // The channel is the platform's from here, whether or not the ring lands.
        self.outstanding.set(true);
        self.doorbell
            .ring(self.a2p)
            .map_err(|e| TryError::Fail(Error::Doorbell(e)))
    }

    /// Read the reply to the outstanding request if the platform has answered,
    /// copying up to `out.len()` return-value bytes into `out`.
    ///
    /// [`TryError::WouldBlock`] means no request is outstanding, or the platform
    /// still owns the channel. A channel the platform flagged with its error bit is
    /// [`Error::ChannelError`]; either way the exchange is over.
    pub fn try_recv_message(&self, out: &mut [u8]) -> Result<Response, TryError<Error<D::Error>>> {
        if !self.outstanding.get() {
            return Err(TryError::WouldBlock);
        }
        let status = self.smt.status();
        if status & STATUS_ERROR != 0 {
            self.outstanding.set(false);
            return Err(TryError::Fail(Error::ChannelError));
        }
        if status & STATUS_FREE == 0 {
            return Err(TryError::WouldBlock);
        }
        self.outstanding.set(false);
        self.smt
            .read_response(out)
            .map_err(|e| TryError::Fail(e.into()))
    }

    /// Wait for the reply to the outstanding request.
    ///
    /// With [`Completion::Doorbell`] this parks on the completion line through
    /// [`park_until`], which re-checks the channel after arming so a reply that
    /// lands in between is not missed. With [`Completion::Poll`] it re-checks after
    /// every executor yield. No timeout: callers own the deadline. With no request
    /// outstanding it never resolves.
    pub async fn recv_message(&mut self, out: &mut [u8]) -> Result<Response, Error<D::Error>> {
        match self.completion {
            Completion::Doorbell(line) => {
                park_until(
                    self,
                    |transport| &transport.doorbell,
                    line,
                    Error::Doorbell,
                    |transport| Readiness::from_try(transport.try_recv_message(&mut *out)),
                )
                .await
            }
            Completion::Poll => loop {
                match self.try_recv_message(out) {
                    Ok(reply) => return Ok(reply),
                    Err(TryError::WouldBlock) => yield_now().await,
                    Err(TryError::Fail(error)) => return Err(error),
                }
            },
        }
    }

    /// Spin until the reply to the outstanding request arrives, or `expired`
    /// reports the deadline passed ([`Error::TimedOut`]).
    ///
    /// For code with no executor, such as a synchronous `init()`. The channel's
    /// free bit is polled whatever [`Completion`] says; if the platform also rings
    /// the completion line, that edge is left latched and a later async wait wakes
    /// once, harmlessly.
    pub fn blocking_recv_message(
        &mut self,
        out: &mut [u8],
        expired: impl FnMut() -> bool,
    ) -> Result<Response, Error<D::Error>> {
        spin_until(
            || Readiness::from_try(self.try_recv_message(&mut *out)),
            expired,
        )
        .map_err(|error| match error {
            BlockingError::TimedOut => Error::TimedOut,
            BlockingError::Fail(error) => error,
        })
    }

    /// Bytes of request payload the channel holds.
    pub fn payload_capacity(&self) -> usize {
        self.smt.payload_capacity()
    }
}

impl<R: Region, D: Doorbell> TransportError for SmtTransport<R, D> {
    type Error = Error<D::Error>;
}

/// The frame is the header word (little-endian) followed by the request payload.
impl<R: Region, D: Doorbell> TrySendTransport for SmtTransport<R, D> {
    fn try_send(&mut self, frame: &[u8]) -> Result<(), TryError<Self::Error>> {
        let Some((header, payload)) = frame.split_first_chunk::<HEADER_LEN>() else {
            return Err(TryError::Fail(
                ProtocolError::FrameTooShort { len: frame.len() }.into(),
            ));
        };
        self.try_send_message(u32::from_le_bytes(*header), payload)
    }

    fn max_send_size(&self) -> usize {
        HEADER_LEN + self.smt.payload_capacity()
    }
}

/// The frame is the header word, the status word, then the return values, all
/// little-endian as on the wire — the reply exactly as the platform wrote it.
///
/// `buf` must hold at least the header and status word; a shorter one is refused
/// with [`ProtocolError::FrameTooShort`] before the reply is consumed. Return
/// values beyond `buf` are dropped, as with [`SmtTransport::try_recv_message`], and
/// the length returned counts only the bytes copied.
impl<R: Region, D: Doorbell> TryRecvTransport for SmtTransport<R, D> {
    fn try_recv(&mut self, buf: &mut [u8]) -> Result<usize, TryError<Self::Error>> {
        if buf.len() < REPLY_PREFIX {
            return Err(TryError::Fail(
                ProtocolError::FrameTooShort { len: buf.len() }.into(),
            ));
        }
        let (prefix, values) = buf.split_at_mut(REPLY_PREFIX);
        let reply = self.try_recv_message(values)?;
        prefix[..HEADER_LEN].copy_from_slice(&reply.header.to_le_bytes());
        prefix[HEADER_LEN..].copy_from_slice(&reply.status.to_le_bytes());
        Ok(REPLY_PREFIX + reply.len.min(values.len()))
    }

    fn max_recv_size(&self) -> usize {
        // The payload area holds the status word and the return values.
        HEADER_LEN + self.smt.payload_capacity()
    }
}
