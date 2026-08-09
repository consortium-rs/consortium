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

//! Doorbell API for signaling events between cores. This is a low-level primitive used by higher-level channel and transport implementations.
use crate::{chan::Chan, future::MaybeSend};

/// Notifies the remote core that something has happened.
/// Carries no data - pure signaling primitive.
///
/// The meaning of Chan:
///   HSEM:   semaphore gate ID (0..=31)
///   MU:     transmit register index (0..=3)
///   Serial: logical channel byte in frame header
///   Host:   Unix socket / TCP port logical channel
///
/// # Ordering contract
///
/// `ring` is the publication edge for data written before signaling the peer.
/// Implementations must ensure those writes happen-before the corresponding
/// `wait` completion or `pending` observation on the receiving side. For
/// software pending bits this means a release operation in `ring` (or in the
/// interrupt producer) paired with an acquire operation in `wait`/`pending`, or
/// stronger ordering. For hardware-backed doorbells, the interrupt or MMIO
/// signal must not be observed by the peer before the shared-memory writes that
/// the signal publishes are visible.
pub trait Doorbell {
    #[cfg(feature = "defmt")]
    type Error: defmt::Format + core::error::Error + Send + Sync + 'static;
    #[cfg(not(feature = "defmt"))]
    type Error: core::error::Error + Send + Sync + 'static;
    /// Signal the remote core on the given channel.
    /// Sync - notification dispatch must never need to await.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error>;

    /// Await a signal from the remote core.
    /// Returns which channel fired - relevant when one
    /// Doorbell covers multiple channels (e.g. MU has 4).
    ///
    /// The returned future is an opaque `impl Future` (RPITIT), matching
    /// [`SendTransport`](crate::SendTransport) and
    /// [`RecvTransport`](crate::RecvTransport): implementations may be plain
    /// `async fn`s or hand-rolled named futures with their own `Drop`-time
    /// disarming. It borrows `self`, so callers must hold it somewhere that is
    /// dropped before the doorbell — an `.await` in an `async` body does this
    /// naturally, whereas a hand-written `poll` needs the future stored in the
    /// same pinned struct that owns (or outlives) the doorbell.
    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend;

    /// Non-blocking check - has a signal arrived?
    fn pending(&self, ch: Chan) -> bool;
}
