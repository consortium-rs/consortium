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

//! The `Transport` trait and related types.
use crate::future::MaybeSend;

/// Moves raw bytes between cores over a specific medium.
/// Knows nothing about message types — that's Channel's job.
///
/// Owns a Doorbell internally — callers never ring/wait directly.
/// The Transport decides when to notify and when to wait.
pub trait Transport {
    type Error;
    type SendFut<'a>: Future<Output = Result<(), Self::Error>> + MaybeSend
    where
        Self: 'a;
    type RecvFut<'a>: Future<Output = Result<usize, Self::Error>> + MaybeSend
    where
        Self: 'a;

    /// Send bytes to the remote side.
    /// Internally handles doorbell ring after write.
    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a>;

    /// Receive bytes from the remote side into buf.
    /// Internally awaits doorbell before reading.
    /// Returns number of bytes written into buf.
    fn recv<'a>(&mut self, buf: &mut [u8]) -> Self::RecvFut<'a>;

    /// Maximum bytes transferable in a single send.
    /// Channel uses this to size its serialization scratch buffer.
    fn max_send_size(&self) -> usize;

    /// Maximum bytes transferable in a single recv.
    /// Channel uses this to size its deserialization scratch buffer.
    fn max_recv_size(&self) -> usize;
}
