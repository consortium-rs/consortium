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

//! Readiness rendezvous with the remote end of a transport.
//!
//! [`Connect`] covers exactly one thing: **waiting until the far side of a
//! link is ready to carry traffic.** A transport implements it when a peer
//! must agree — on a memory layout, a protocol version, an MTU — before the
//! first message can be sent.
//!
//! # What belongs here
//!
//! The defining property is that there is *another party*, and that the
//! operation can fail because that party never showed up or disagreed. That is
//! why [`connect`](Connect::connect) is async: the wait is genuine, not
//! incidental. Concretely:
//!
//! - the shared-memory transport, where the primary publishes a descriptor and
//!   the secondary validates and acks it;
//! - a hypothetical link needing PHY training or a name-service announce —
//!   still a rendezvous, even though the "peer" is hardware.
//!
//! # What does not belong here
//!
//! Local bring-up is *not* a rendezvous, and forcing it through this trait
//! would make simple register writes pretend to await something:
//!
//! - **Peripherals.** Enabling a clock, muxing pins and programming registers
//!   has no peer and no protocol. Its real complexity is the ordering graph
//!   between peripherals, which a per-object method cannot express; in this
//!   workspace that sequencing is emitted by `consortium-cfg` into the
//!   generated `init()`. Peripheral drivers should keep the ecosystem
//!   convention instead: constructor plus config struct, with typestate where
//!   readiness is worth encoding in the type.
//! - **Doorbells.** They have local setup (unmasking an interrupt, binding to
//!   the Linux UIO fan-out) but no handshake, which is why that stays plain
//!   inherent API (`bind_notifier` / `bind_service`).
//!
//! Waiting on *someone else* is `Connect`; configuring *your own* silicon is a
//! constructor.
//!
//! # Contract
//!
//! - Call `connect` **once**, after construction and before any
//!   [`send`](crate::SendTransport::send) or [`recv`](crate::RecvTransport::recv),
//!   and before splitting a full-duplex transport into halves — the handshake
//!   is a property of the whole link, not of one direction.
//! - Implementations are side-asymmetric where the protocol is: the two ends of
//!   one link may run different code, selected by the transport's type (e.g. its
//!   [`Side`](crate::Side)), not by a runtime branch in the caller.
//! - `connect` carries **no timeout**. A peer that never answers parks the
//!   caller forever, so callers that need bounded bring-up must impose their own
//!   deadline (`tokio::time::timeout`, `embassy_time::with_timeout`).
//! - `connect` is safe. Transports whose construction is `unsafe` (raw shared
//!   memory, say) discharge that obligation in their constructor; `connect`
//!   then relies on the invariant already established there.

use crate::future::MaybeSend;
use crate::transport::TransportError;

/// A transport that must rendezvous with its remote peer before carrying
/// traffic.
///
/// The trait covers exactly one thing: **waiting until the far side of a link is
/// ready.** Implement it when a peer must agree — on a memory layout, a protocol
/// version, an MTU — before the first message can be sent. The defining property
/// is that there is *another party*, and that the operation can fail because that
/// party never showed up or disagreed. That is why `connect` is async: the wait is
/// genuine, not incidental.
///
/// # Contract
///
/// - Call [`connect`](Connect::connect) **once**, after construction and before any
///   [`send`](crate::SendTransport::send) or [`recv`](crate::RecvTransport::recv),
///   and before splitting a full-duplex transport into halves — the handshake is a
///   property of the whole link, not of one direction.
/// - Implementations are side-asymmetric where the protocol is. The two ends of one
///   link may run different code, selected by the transport's type (typically its
///   [`Side`](crate::Side)) rather than by a runtime branch in the caller. Generated
///   bring-up code can then emit one `connect` call for both sides.
/// - `connect` carries **no timeout**. A peer that never answers parks the caller
///   forever, so callers needing bounded bring-up impose their own deadline
///   (`tokio::time::timeout`, `embassy_time::with_timeout`).
/// - `connect` is safe. A transport whose construction is `unsafe` (raw shared
///   memory, say) discharges that obligation in its constructor; `connect` relies on
///   the invariant already established there.
///
/// # What does not belong here
///
/// Local bring-up is not a rendezvous, and routing it through this trait would make
/// plain register writes pretend to await something.
///
/// - **Peripherals.** Enabling a clock, muxing pins and programming registers has no
///   peer and no protocol. Its real complexity is the ordering graph between
///   peripherals, which a per-object method cannot express; in this workspace that
///   sequencing is emitted by `consortium-cfg` into the generated `init()`.
///   Peripheral drivers keep the ecosystem convention instead: constructor plus
///   config struct, with typestate where readiness is worth encoding in the type.
/// - **Doorbells.** They have local setup (unmasking an interrupt, binding to the
///   Linux UIO fan-out) but no handshake, which is why that stays plain inherent API
///   (`bind_notifier` / `bind_service`).
///
/// Waiting on *someone else* is `Connect`; configuring *your own* silicon is a
/// constructor.
///
/// # Example
///
/// The uniform shape lets bring-up code drive any link the same way:
///
/// ```ignore
/// async fn bring_up<T: Connect>(transport: &mut T, params: T::Params) -> Result<(), T::Error> {
///     transport.connect(params).await
/// }
/// ```
pub trait Connect: TransportError {
    /// Handshake inputs that are negotiated rather than owned by the transport.
    ///
    /// Anything that identifies the link — channel ids, the doorbell, the
    /// region base — belongs in the constructor. What lands here is what the
    /// two ends must *agree on*, and whose agreed value the transport learns
    /// only by completing the handshake (for shared memory: the channel count
    /// and the proposed MTU). Transports with nothing to negotiate use `()`.
    type Params;

    /// Run the readiness handshake with the remote end.
    ///
    /// Resolves once the peer is ready; the transport may record what the
    /// handshake settled (a negotiated MTU, say) before returning. Call once,
    /// before any traffic and before splitting into halves.
    fn connect(
        &mut self,
        params: Self::Params,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}
