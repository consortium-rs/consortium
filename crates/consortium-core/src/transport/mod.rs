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

//! The transport trait family: how a subsystem moves frames to another core.
//!
//! Three flavors of the same idea, each a separate trait so a transport implements
//! exactly the ones it can honor:
//!
//! | Trait                                     | Waits?                     | Typical implementors         |
//! | ----------------------------------------- | -------------------------- | ---------------------------- |
//! | [`SendTransport`] / [`RecvTransport`]     | async                      | every IPC transport          |
//! | [`TrySendTransport`] / [`TryRecvTransport`] | never                    | frame-atomic memory windows  |
//! | [`AckSendTransport`]                      | async, until the peer took it | shared memory with ack lines |
//!
//! plus [`Connect`], the one-shot readiness rendezvous run before any traffic.
//!
//! # Why a non-blocking pair
//!
//! A transport over a shared-memory window hands a frame over in one synchronous
//! step: claim a buffer the peer can see, copy, publish, ring. Either the whole
//! frame lands or the buffer is still occupied — there is nothing in between to
//! wait for. [`TrySendTransport`] and [`TryRecvTransport`] name that step, which
//! is what lets very different protocols share it: the `CIPC` IPC slots, an SCMI
//! shared-memory channel, and a `CDBG` log ring all report "occupied" as
//! [`TryError::WouldBlock`] instead of each inventing its own busy error.
//!
//! The async and blocking forms of such a transport are then written once, over
//! the try step, in [`wait`](crate::wait): [`park_until`](crate::wait::park_until)
//! parks on a doorbell line between attempts, and
//! [`spin_until`](crate::wait::spin_until) spins with a caller-supplied deadline
//! for code that runs without an executor.
//!
//! Wired links (UART, CAN) do **not** implement the try pair: a frame longer than
//! the peripheral's FIFO cannot be handed over atomically, so the only honest
//! synchronous send on them waits for the last byte. They implement the async
//! traits only.

mod ack;
mod connect;

pub use {ack::AckSendTransport, connect::Connect};

use crate::MaybeSend;

/// Provides the `Error` associated type shared by every transport trait.
pub trait TransportError {
    type Error;
}

/// Send half of a transport — moves bytes to the remote side.
///
/// The returned future is an opaque `impl Future` (RPITIT), so implementations
/// may be plain `async fn`s — including ones that await unnameable futures such
/// as `embedded_io_async::Write::write` — or return hand-rolled named futures.
pub trait SendTransport: TransportError {
    /// Write `data` to the transport and ring the doorbell.
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Maximum bytes transferable in a single send.
    fn max_send_size(&self) -> usize;
}

/// Receive half of a transport — awaits bytes from the remote side.
///
/// The returned future is an opaque `impl Future` (RPITIT); see
/// [`SendTransport`] for the implementation latitude this allows.
pub trait RecvTransport: TransportError {
    /// Await the doorbell and read bytes into `buf`.
    /// Returns the number of bytes written into `buf`.
    fn recv(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend;

    /// Maximum bytes transferable in a single recv.
    fn max_recv_size(&self) -> usize;
}

/// Why a [`TrySendTransport::try_send`] or [`TryRecvTransport::try_recv`] did not
/// complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TryError<E> {
    /// Nothing went wrong, but the attempt cannot complete now: the send buffer
    /// still holds a frame the peer has not taken, or there is nothing to receive.
    /// Nothing was transferred; try again later.
    WouldBlock,
    /// The attempt failed; retrying will not help on its own.
    Fail(E),
}

impl<E> TryError<E> {
    /// Map the failure, keeping [`WouldBlock`](Self::WouldBlock).
    pub fn map<F>(self, f: impl FnOnce(E) -> F) -> TryError<F> {
        match self {
            Self::WouldBlock => TryError::WouldBlock,
            Self::Fail(error) => TryError::Fail(f(error)),
        }
    }
}

impl<E> From<E> for TryError<E> {
    fn from(error: E) -> Self {
        Self::Fail(error)
    }
}

impl<E: core::fmt::Display> core::fmt::Display for TryError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WouldBlock => f.write_str("transport would block"),
            Self::Fail(error) => error.fmt(f),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for TryError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::WouldBlock => None,
            Self::Fail(error) => Some(error),
        }
    }
}

/// Send half of a transport whose hand-off never waits.
///
/// # Contract
///
/// - **Never waits.** No spinning, no parking, no executor: `try_send` may be
///   called from an interrupt handler, a `defmt` logger, or a panic handler.
/// - **Frame-atomic.** On `Ok` the whole frame was published and the peer
///   signalled (if the transport signals). On any `Err` nothing was published.
///   Implementations must never truncate.
/// - **Occupied is [`TryError::WouldBlock`].** A transport whose buffer still
///   holds an untaken frame reports that, rather than overwriting it.
/// - A frame longer than [`max_send_size`](Self::max_send_size) is a
///   [`TryError::Fail`], never `WouldBlock`: no amount of waiting makes it fit.
pub trait TrySendTransport: TransportError {
    /// Publish `frame` to the peer if the transport can take it now.
    fn try_send(&mut self, frame: &[u8]) -> Result<(), TryError<Self::Error>>;

    /// Maximum bytes transferable in a single `try_send`.
    fn max_send_size(&self) -> usize;
}

/// Receive half of a transport whose take never waits.
///
/// # Contract
///
/// - **Never waits**, as for [`TrySendTransport`].
/// - On `Ok(n)` one whole frame of `n` bytes was copied into `buf[..n]` and
///   released back to the peer. [`TryError::WouldBlock`] means nothing is
///   readable yet, or a frame is mid-update and should be read again.
/// - `buf` must be at least [`max_recv_size`](Self::max_recv_size) bytes for every
///   frame to fit; what a transport does with a frame that does not is its own
///   documented choice (drop it and report, typically).
pub trait TryRecvTransport: TransportError {
    /// Copy the next frame into `buf` if one is readable now.
    fn try_recv(&mut self, buf: &mut [u8]) -> Result<usize, TryError<Self::Error>>;

    /// Maximum bytes transferable in a single `try_recv`.
    fn max_recv_size(&self) -> usize;
}
