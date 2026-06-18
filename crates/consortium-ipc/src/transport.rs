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

//! The `Transport` trait family and related types.
use crate::future::MaybeSend;

/// Provides the `Error` associated type shared by [`SendTransport`] and [`RecvTransport`].
pub trait TransportError {
    type Error;
}

/// Send half of a transport — moves bytes to the remote side.
pub trait SendTransport: TransportError {
    type SendFut<'a>: Future<Output = Result<(), Self::Error>> + MaybeSend
    where
        Self: 'a;

    /// Write `data` to the transport and ring the doorbell.
    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a>;

    /// Maximum bytes transferable in a single send.
    fn max_send_size(&self) -> usize;
}

/// Receive half of a transport — awaits bytes from the remote side.
pub trait RecvTransport: TransportError {
    type RecvFut<'a>: Future<Output = Result<usize, Self::Error>> + MaybeSend
    where
        Self: 'a;

    /// Await the doorbell and read bytes into `buf`.
    /// Returns the number of bytes written into `buf`.
    fn recv<'a>(&mut self, buf: &mut [u8]) -> Self::RecvFut<'a>;

    /// Maximum bytes transferable in a single recv.
    fn max_recv_size(&self) -> usize;
}

/// Combined transport — covers both send and receive directions.
///
/// Implement this for full-duplex types; use [`SendTransport`] / [`RecvTransport`]
/// directly when only one direction is needed (e.g. after a [`split`]).
///
/// [`split`]: crate::channel::Channel
pub trait Transport: SendTransport + RecvTransport {}
