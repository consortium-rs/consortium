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
//! 1. build the header and hand the request to the [`SmtTransport`]
//!    ([`try_send_message`](SmtTransport::try_send_message)): it checks the
//!    channel is free (else [`Error::ChannelBusy`]), writes header + parameters,
//!    clears the free bit, flushes, and rings the agent-to-platform line;
//! 2. wait for the reply ([`recv_message`](SmtTransport::recv_message)) — park on
//!    the completion line, or poll the free bit ([`Completion`]);
//! 3. check the reply echoes the request header and decode the status.
//!
//! [`Client::blocking_call`] is the same exchange for code with no executor (a
//! synchronous `init()`), spinning against a caller deadline instead of parking.
//!
//! One call is outstanding at a time, which `&mut self` enforces. A cancelled
//! call leaves the channel with the platform; the next call then reports
//! [`Error::ChannelBusy`] until the platform frees it.
//!
//! The per-protocol modules ([`base`], [`clock`], [`pinctrl`], [`power`],
//! [`reset`]) encode parameters, decode return values, and implement
//! `consortium-pmi`'s provider traits on [`Client`].

use consortium_core::{Chan, Doorbell, Region, TryError};

use crate::{
    error::{Error, ProtocolError},
    header::{MessageHeader, MessageType, ProtocolId, TOKEN_MAX},
    smt::{Response, Smt},
    status::Status,
    transport::SmtTransport,
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
    transport: SmtTransport<R, D>,
    token: u16,
    clock_major: Option<u16>,
}

impl<R: Region, D: Doorbell> Client<R, D> {
    /// An agent that writes requests into `smt`, rings `a2p` on `doorbell`, and
    /// learns of completion through `completion`.
    pub fn new(smt: Smt<R>, doorbell: D, a2p: Chan, completion: Completion) -> Self {
        Self::from_transport(SmtTransport::new(smt, doorbell, a2p, completion))
    }

    /// An agent over an already-assembled transport.
    pub fn from_transport(transport: SmtTransport<R, D>) -> Self {
        Self {
            transport,
            token: 0,
            clock_major: None,
        }
    }

    /// The transport this agent exchanges messages over.
    pub fn transport(&self) -> &SmtTransport<R, D> {
        &self.transport
    }

    /// Take the channel and doorbell back.
    pub fn into_parts(self) -> (Smt<R>, D) {
        self.transport.into_parts()
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
        let header = self.send(protocol, message, request)?;
        let reply = self.transport.recv_message(response).await?;
        finish(header, reply)
    }

    /// [`call`](Self::call) without an executor: spin until the platform answers,
    /// or until `expired` reports the deadline passed ([`Error::TimedOut`]).
    ///
    /// For bring-up that runs before any executor, such as a synchronous
    /// generated `init()`. `expired` is clock-agnostic — an `embassy_time`
    /// deadline check, a cycle counter, or a spin budget — and is not consulted if
    /// the reply is already there. A timed-out request stays with the platform, so
    /// the next call reports [`Error::ChannelBusy`] until the platform frees it.
    pub fn blocking_call(
        &mut self,
        protocol: ProtocolId,
        message: u8,
        request: &[u8],
        response: &mut [u8],
        expired: impl FnMut() -> bool,
    ) -> Result<usize, Error<D::Error>> {
        let header = self.send(protocol, message, request)?;
        let reply = self.transport.blocking_recv_message(response, expired)?;
        finish(header, reply)
    }

    /// Hand one request to the platform, and advance the token once it is sent.
    fn send(
        &mut self,
        protocol: ProtocolId,
        message: u8,
        request: &[u8],
    ) -> Result<MessageHeader, Error<D::Error>> {
        let header = MessageHeader::new(protocol, message, MessageType::Command, self.token);
        match self.transport.try_send_message(header.raw(), request) {
            Ok(()) => {}
            Err(TryError::WouldBlock) => return Err(Error::ChannelBusy),
            Err(TryError::Fail(error)) => return Err(error),
        }
        self.token = (self.token + 1) & TOKEN_MAX;
        Ok(header)
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
        self.transport.payload_capacity()
    }
}

/// Check a reply answers `sent`, and decode its status.
fn finish<E>(sent: MessageHeader, reply: Response) -> Result<usize, Error<E>> {
    let received = MessageHeader::from_raw(reply.header);
    if received.protocol() != sent.protocol()
        || received.message() != sent.message()
        || received.token() != sent.token()
    {
        return Err(ProtocolError::HeaderMismatch { sent, received }.into());
    }
    match Status::from_raw(reply.status) {
        Status::Success => Ok(reply.len),
        status => Err(Error::Status(status)),
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
