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

//! Error type shared by the FDCAN transport halves.

/// Errors surfaced by [`FdcanTransportTx`](crate::FdcanTransportTx) and
/// [`FdcanTransportRx`](crate::FdcanTransportRx).
///
/// Message-level corruption (a frame carrying a foreign identifier, a remote
/// frame, a reserved header kind, a sequence gap, a continuation with no
/// preceding first frame, or a declared length past the MTU) is *not* an error:
/// the receiver drops the partial message, logs it, and resynchronizes on the
/// next first or single frame. Only conditions the caller can act on are
/// reported here.
///
/// There is deliberately no end-of-stream variant. A CAN bus has no peer to
/// close it: a link with nothing on the other end is quiet, not finished.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// The underlying CAN controller failed.
    #[error("can error: {0}")]
    Can(E),
    /// The payload handed to `send` exceeds the configured MTU.
    #[error("payload of {payload_len} bytes exceeds mtu {mtu}")]
    PayloadTooLarge { payload_len: usize, mtu: usize },
    /// A caller-provided buffer is smaller than the configured MTU requires.
    #[error("buffer of {got} bytes is smaller than the required {needed}")]
    BufferTooSmall { needed: usize, got: usize },
    /// The frame type's payload could not hold a segment the framing layer
    /// produced. Only reachable from a [`CanFrame`](crate::CanFrame) impl whose
    /// `build` rejects a length at or under its own `MAX_PAYLOAD`.
    #[error("controller rejected a {len}-byte frame")]
    FrameRejected { len: usize },
}
