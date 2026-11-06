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

//! The SCMI agent.
//!
//! [`Client::call`] is the one exchange every command goes through:
//!
//! 1. invalidate and check the channel is free (else [`Error::ChannelBusy`]);
//! 2. write header + parameters, clear the free bit, flush
//!    ([`Smt::write_request`]);
//! 3. ring the agent-to-platform line;
//! 4. wait for completion — park on the completion line, or poll the free bit
//!    ([`Completion`]);
//! 5. read the response, check it echoes the request header, decode the status.
//!
//! One call is outstanding at a time, which `&mut self` enforces. A cancelled
//! call leaves the channel with the platform; the next call then reports
//! [`Error::ChannelBusy`] until the platform frees it.
//!
//! The per-protocol modules ([`base`], [`clock`], [`pinctrl`], [`power`],
//! [`reset`]) encode parameters, decode return values, and implement
//! `consortium-pmi`'s provider traits on [`Client`].

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use consortium_core::{Chan, Doorbell, Region};

use crate::{
    error::{Error, ProtocolError},
    header::{MessageHeader, MessageType, ProtocolId, TOKEN_MAX},
    smt::{STATUS_ERROR, STATUS_FREE, Smt},
    status::Status,
};

pub mod base;
pub mod clock;
pub mod pinctrl;
pub mod power;
pub mod reset;

/// How the agent learns the platform has answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Completion {
    /// Poll the channel's free bit, yielding to the executor between polls.
    Poll,
    /// Park on this doorbell line, which the platform rings on completion; the
    /// request asks for it by setting the SMT completion-interrupt flag.
    Doorbell(Chan),
}

/// The common message every protocol implements: its version.
pub(crate) const PROTOCOL_VERSION: u8 = 0x0;

/// An SCMI agent on one SMT channel.
pub struct Client<R, D> {
    smt: Smt<R>,
    doorbell: D,
    a2p: Chan,
    completion: Completion,
    token: u16,
    clock_major: Option<u16>,
}

impl<R: Region, D: Doorbell> Client<R, D> {
    /// An agent that writes requests into `smt`, rings `a2p` on `doorbell`, and
    /// learns of completion through `completion`.
    pub fn new(smt: Smt<R>, doorbell: D, a2p: Chan, completion: Completion) -> Self {
        Self {
            smt,
            doorbell,
            a2p,
            completion,
            token: 0,
            clock_major: None,
        }
    }

    /// Take the channel and doorbell back.
    pub fn into_parts(self) -> (Smt<R>, D) {
        (self.smt, self.doorbell)
    }

    /// Send command `message` of `protocol` with parameters `request`, and copy
    /// its return values into `response`.
    ///
    /// Returns the number of return-value bytes the platform wrote, which may
    /// exceed `response.len()` (the excess is not copied). A non-success status
    /// is [`Error::Status`].
    pub async fn call(
        &mut self,
        protocol: ProtocolId,
        message: u8,
        request: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error<D::Error>> {
        if self.smt.status() & STATUS_FREE == 0 {
            return Err(Error::ChannelBusy);
        }
        let token = self.token;
        self.token = (self.token + 1) & TOKEN_MAX;
        let header = MessageHeader::new(protocol, message, MessageType::Command, token);
        let irq = matches!(self.completion, Completion::Doorbell(_));
        self.smt.write_request(header.raw(), request, irq)?;
        self.doorbell.ring(self.a2p).map_err(Error::Doorbell)?;

        loop {
            if let Completion::Doorbell(line) = self.completion {
                self.doorbell.wait(line).await.map_err(Error::Doorbell)?;
            }
            let status = self.smt.status();
            if status & STATUS_ERROR != 0 {
                return Err(Error::ChannelError);
            }
            if status & STATUS_FREE != 0 {
                break;
            }
            if self.completion == Completion::Poll {
                YieldNow(false).await;
            }
        }

        let reply = self.smt.read_response(response)?;
        let received = MessageHeader::from_raw(reply.header);
        if received.protocol() != protocol
            || received.message() != message
            || received.token() != token
        {
            return Err(ProtocolError::HeaderMismatch {
                sent: header,
                received,
            }
            .into());
        }
        match Status::from_raw(reply.status) {
            Status::Success => Ok(reply.len),
            status => Err(Error::Status(status)),
        }
    }

    /// [`call`](Self::call), requiring at least `response.len()` bytes back.
    pub(crate) async fn call_exact(
        &mut self,
        protocol: ProtocolId,
        message: u8,
        request: &[u8],
        response: &mut [u8],
    ) -> Result<(), Error<D::Error>> {
        let len = self.call(protocol, message, request, response).await?;
        if len < response.len() {
            return Err(ProtocolError::ShortResponse { len }.into());
        }
        Ok(())
    }

    /// The payload bytes one request can carry.
    pub(crate) fn payload_capacity(&self) -> usize {
        self.smt.payload_capacity()
    }
}

/// Little-endian parameter encoding: `N` words into a byte array.
pub(crate) fn words<const B: usize>(values: &[u32]) -> [u8; B] {
    debug_assert_eq!(values.len() * 4, B);
    let mut out = [0u8; B];
    for (chunk, value) in out.chunks_exact_mut(4).zip(values) {
        chunk.copy_from_slice(&value.to_le_bytes());
    }
    out
}

/// Little-endian word `index` of a return-value buffer.
pub(crate) fn word(bytes: &[u8], index: usize) -> u32 {
    let mut le = [0u8; 4];
    le.copy_from_slice(&bytes[index * 4..index * 4 + 4]);
    u32::from_le_bytes(le)
}

/// Yields once, so a polling wait does not starve the executor.
struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
