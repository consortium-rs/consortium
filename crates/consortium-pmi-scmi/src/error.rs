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

//! Errors a [`Client`](crate::Client) call can return.

use crate::{header::MessageHeader, status::Status};

/// A failed SCMI call. `E` is the doorbell's error type.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// The platform answered with a non-success status.
    Status(Status),
    /// Ringing or waiting on the doorbell failed.
    Doorbell(E),
    /// The channel was not free when the call started: the platform still owns
    /// it, most likely from a cancelled earlier call.
    ChannelBusy,
    /// The platform set the channel's error bit instead of answering.
    ChannelError,
    /// The exchange broke the protocol.
    Protocol(ProtocolError),
}

/// A malformed exchange.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtocolError {
    /// The response header does not echo the request's protocol, message, and token.
    HeaderMismatch {
        /// The header sent.
        sent: MessageHeader,
        /// The header that came back.
        received: MessageHeader,
    },
    /// The response is shorter than the header and status word, or than the
    /// return values the command defines.
    ShortResponse {
        /// Bytes of return values received.
        len: usize,
    },
    /// The response length runs past the channel.
    OversizedResponse {
        /// The length field the platform wrote.
        len: usize,
    },
    /// The request does not fit the channel's payload area.
    RequestTooLarge {
        /// Request payload bytes.
        len: usize,
        /// Payload bytes the channel holds.
        capacity: usize,
    },
}

impl<E> From<ProtocolError> for Error<E> {
    fn from(e: ProtocolError) -> Self {
        Self::Protocol(e)
    }
}

impl<E: core::fmt::Debug> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Status(s) => write!(f, "SCMI platform returned {s:?}"),
            Self::Doorbell(e) => write!(f, "SCMI doorbell failed: {e:?}"),
            Self::ChannelBusy => f.write_str("SCMI channel is still owned by the platform"),
            Self::ChannelError => f.write_str("SCMI platform flagged a channel error"),
            Self::Protocol(e) => write!(f, "SCMI protocol violation: {e:?}"),
        }
    }
}

impl<E: core::fmt::Debug> core::error::Error for Error<E> {}
