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

//! Error type shared by the UART transport halves.

/// Errors surfaced by [`UartTransportTx`](crate::UartTransportTx) and
/// [`UartTransportRx`](crate::UartTransportRx).
///
/// Frame-level corruption (bad COBS encoding, CRC mismatch, foreign channel
/// byte, oversized frame) is *not* an error: the receiver drops the frame,
/// logs it, and keeps waiting for the next delimiter. Only conditions the
/// caller can act on are reported here.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// The underlying serial device failed.
    #[error("serial io error: {0}")]
    Io(E),
    /// The underlying serial device reached end-of-stream (e.g. the peer
    /// closed a pseudo-terminal). A real UART never reports this.
    #[error("serial stream ended")]
    Eof,
    /// The payload handed to `send` exceeds the configured MTU.
    #[error("payload of {payload_len} bytes exceeds mtu {mtu}")]
    PayloadTooLarge { payload_len: usize, mtu: usize },
    /// A caller-provided buffer is smaller than the configured MTU requires.
    #[error("buffer of {got} bytes is smaller than the required {needed}")]
    BufferTooSmall { needed: usize, got: usize },
}
