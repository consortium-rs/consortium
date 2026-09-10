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

//! Acknowledged sending — the optional half of [`SendTransport`].
//!
//! A separate module and a separate trait, following [`Connect`](crate::Connect): most
//! transports cannot implement this, and none should have to. The contract lives on
//! [`AckSendTransport`] itself, since this module is not public.
//!
//! [`SendTransport`]: crate::SendTransport
use crate::future::MaybeSend;
use crate::transport::TransportError;

/// A [`SendTransport`](crate::SendTransport) that can also wait for the peer to take the
/// message.
///
/// [`SendTransport::send`](crate::SendTransport::send) is fire-and-forget: it hands bytes
/// to the transport and returns. That is the right default, and it stays the default. But
/// a transport whose depth is one frame per direction has nothing to queue behind, so a
/// sender that outruns its peer can only be told "not now" — and it never learns whether a
/// message that *was* accepted arrived intact. `send_acked` answers both.
///
/// # Contract
///
/// - It resolves `Ok` only after the peer has taken the message out of the transport. What
///   "taken" means is the transport's to define, but it must be an observation of the peer,
///   not of the local side.
/// - It may resolve `Err` when the peer took the message and rejected it. This is the main
///   thing an acknowledgement buys over a plain send.
/// - It has no timeout. Callers own the deadline, exactly as with
///   [`Connect::connect`](crate::Connect::connect); wrap it in `tokio::time::timeout` or
///   `embassy_time::with_timeout`.
/// - **Delivery is at-least-once; acknowledgement is not.** Dropping the future after the
///   message goes out loses the confirmation, not the message.
/// - A transport **may refuse**, returning an error when the link was not configured for
///   acknowledgement. Refusing is required; hanging is not acceptable.
///
/// # Deadlock
///
/// Two peers that both call this with nothing draining the other direction will wait on
/// each other forever. That is inherent to a blocking acknowledgement rather than a defect
/// in any particular transport: the acknowledgement is produced by the peer's *receive*
/// path, so that path has to be running. Use
/// [`send`](crate::SendTransport::send) — which reports rather than waits — for lockstep
/// single-task designs, and keep the receive half driven concurrently when using this one.
pub trait AckSendTransport: TransportError {
    /// Send `data`, then resolve once the peer has taken it.
    ///
    /// The returned future is an opaque `impl Future` (RPITIT), so implementations may be
    /// plain `async fn`s or hand-rolled named futures.
    fn send_acked(
        &mut self,
        data: &[u8],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}
