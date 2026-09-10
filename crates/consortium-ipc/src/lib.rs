#![cfg_attr(not(feature = "std"), no_std)]
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

//! Core inter-processor communication primitives for Consortium.
//!
//! This crate is the abstraction layer that every Consortium link is built from.
//! It defines the trait stack, the typed endpoints application and firmware code
//! call, and the marker types that keep a message meaningful on both sides of a
//! core boundary. It performs no hardware access, no allocation, and hosts no
//! executor; concrete links live in `consortium-ipc-transport-*`, concrete
//! signaling in `consortium-ipc-doorbell-*`, and serialization in
//! `consortium-codec`.
//!
//! # Architecture
//!
//! Six traits compose into two user-facing types:
//!
//! ```text
//!   Doorbell                       wake the remote processor (no data)
//!   SendTransport / RecvTransport  byte halves; ring or await the doorbell
//!   Transport                      marker for full-duplex links
//!   Connect                        one-shot readiness rendezvous with the peer
//!   CodecFor<T>                    serialization family for a message type
//!   Channel<D, T, Tr, C>           typed endpoint over one transport half
//!   Transceiver<..>                typed endpoint pairing a Tx and an Rx channel
//! ```
//!
//! - [`Doorbell`] is pure signaling. Its ordering contract — writes before `ring`
//!   happen-before the peer's `wait`/`pending` — is what makes shared-memory
//!   payloads safe to read on the far side.
//! - [`SendTransport`] and [`RecvTransport`] move bytes and report their MTU. Both
//!   return `impl Future<..> + MaybeSend`, so implementations may be plain
//!   `async fn`s awaiting unnameable futures.
//! - [`Channel`] adds a message type and a [`consortium_codec::CodecFor`] to one
//!   transport half.
//!   It never allocates: the caller supplies a `&'static mut [u8]` scratch buffer
//!   sized for the transport MTU.
//! - [`Transceiver`] pairs a [`Tx`] and an [`Rx`] channel, each over its own
//!   transport half, into one bidirectional endpoint.
//!
//! Supporting types: [`Chan`] (validated channel id, with per-platform
//! [`ChanValidate`]), [`Side`] ([`Primary`] / [`Secondary`] / [`Tertiary`], the
//! register view of a multi-ported peripheral), [`IpcSafe`] (ABI-portable message
//! types), and [`MaybeSend`] (`Send` under `std`, vacuous under `no_std`).
//!
//! # Example
//!
//! ```rust,ignore
//! use consortium_codec::PostcardCodec;
//! use consortium_ipc::{Chan, Channel, Connect, Rx, Transceiver, Tx};
//!
//! // Rendezvous once, before any traffic and before splitting.
//! transport.connect(params).await?;
//!
//! let (tx_half, rx_half) = transport.split();
//! let tx = Channel::<Tx, Command, _, PostcardCodec>::new(chan, tx_half, tx_buf);
//! let rx = Channel::<Rx, Telemetry, _, PostcardCodec>::new(chan, rx_half, rx_buf);
//!
//! let mut link = Transceiver::new(tx, rx);
//! link.send(&Command::Start).await?;
//! let reply = link.recv().await?;
//! ```
//!
//! [`recv`](Channel::recv) yields a [`ReceivedMessage`] rather than a bare `T`, so
//! owned codecs (`Decoded<'buf> = T`) and future zero-copy codecs
//! (`Decoded<'buf> = &'buf T::Archived`) share one call-site shape.
//!
//! # Implementing a transport
//!
//! Implement [`TransportError`] to name the error type, then [`SendTransport`]
//! and/or [`RecvTransport`], plus [`Transport`] as a marker for full-duplex links.
//! Implement [`Connect`] when a peer must agree before traffic flows, using
//! `Params = ()` if nothing is negotiated. Report honest `max_send_size` /
//! `max_recv_size`: callers size their scratch buffers from them. If the link owns
//! a channel-id namespace, implement [`ChanValidate`] for a platform marker so
//! `Chan::new::<P>(id)` rejects out-of-range ids.
//!
//! # Features
//!
//! The crate is `no_std` unless `std` is enabled.
//!
//! - `alloc` — link `alloc`.
//! - `futures` — futures-oriented helpers.
//! - `tracing` — route internal logging to `consortium-log`'s `tracing` backend.
//! - `defmt` — route internal logging to `defmt` and derive `defmt::Format`.
//! - `std` — `alloc + futures + tracing`, and makes [`MaybeSend`] require `Send`
//!   (which Tokio's multi-threaded executor needs). Leave it off for Embassy.

#[cfg(feature = "alloc")]
extern crate alloc;

pub(crate) mod ack;
pub(crate) mod chan;
pub(crate) mod channel;
pub(crate) mod connect;
pub(crate) mod doorbell;
pub(crate) mod future;
pub(crate) mod sealed;
pub(crate) mod side;
pub(crate) mod transceiver;
pub(crate) mod transport;
pub(crate) mod wire;

pub use {
    ack::AckSendTransport,
    chan::{Chan, ChanValidate},
    channel::{Channel, ChannelError, Direction, ReceivedMessage, Rx, Tx},
    connect::Connect,
    doorbell::Doorbell,
    future::MaybeSend,
    side::{Primary, Secondary, Side, Tertiary},
    transceiver::Transceiver,
    transport::{RecvTransport, SendTransport, Transport, TransportError},
    wire::IpcSafe,
};

pub use consortium_ipc_macros::IpcSafe;
