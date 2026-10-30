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

//! Shared-memory transport for Consortium IPC: the `CIPC` descriptor, its channel
//! table, the slot protocol, and [`SharedMemoryTransport`].
//!
//! This is the primary Consortium link — the one an A-core and an M-core on the same
//! SoC use. It implements `consortium-ipc`'s [`SendTransport`], [`RecvTransport`], and
//! [`Connect`] over a window of memory both cores can address, using a
//! [`Doorbell`] for wakeups because shared memory on its own cannot notify anybody.
//!
//! Memory primitives come from `consortium-shared-memory` ([`Region`] and its cache
//! semantics, the descriptor header). Hardware signaling comes from the Type-A
//! doorbells in [`bell`] (IPCC, MU, HSEM); GPIO stays in `consortium-ipc-doorbell-gpio`.
//! What this crate adds is the protocol.
//!
//! [`SendTransport`]: consortium_ipc::SendTransport
//! [`RecvTransport`]: consortium_ipc::RecvTransport
//! [`Connect`]: consortium_ipc::Connect
//! [`Doorbell`]: consortium_ipc::Doorbell
//! [`Region`]: consortium_shared_memory::Region
//!
//! # Layout
//!
//! One window holds everything. A fixed-size descriptor at offset 0 describes a
//! channel table, and each table entry points at a slot elsewhere in the same
//! window:
//!
//! ```text
//! 0x00  ┌────────────────────────────────┐
//!       │ header: "CIPC", version        │  consortium-shared-memory
//!       │ num_channels, updated ids      │
//!       │ tx/rx_transfer_size            │
//!       │ state (DescState)              │
//! 0x20  ├────────────────────────────────┤
//!       │ channel table: N × 12 bytes    │  offset, size, direction
//!       ├────────────────────────────────┤
//!       │ cache-line padding             │
//!       ├────────────────────────────────┤
//!       │ slot 0  [len|flags|crc32|data] │  ← one direction, aligned
//!       │ padding to cache-line boundary │
//!       │ slot 1  [len|flags|crc32|data] │  ← the other, aligned
//!       └────────────────────────────────┘
//! ```
//!
//! [`Descriptor`] and [`ChannelLayout`] give the exact field offsets, [`slot`] the
//! slot header. Channel directions in the table are **absolute**
//! ([`ChannelDirection::PrimaryToSecondary`] / [`SecondaryToPrimary`]), and each
//! side resolves them to TX or RX through its own [`Side`] type parameter. One
//! descriptor therefore describes the link identically from both ends, and neither
//! end needs a runtime branch on "am I the primary".
//!
//! [`SecondaryToPrimary`]: ChannelDirection::SecondaryToPrimary
//!
//! # Protocol
//!
//! **Bring-up** is [`Connect`], implemented once per [`Side`]:
//!
//! | Side        | `connect` does                                                        |
//! | ----------- | --------------------------------------------------------------------- |
//! | [`Primary`] | publish the descriptor, `flush`, ring, await the ack                   |
//! | [`Secondary`] | await/observe the descriptor, validate it, `flush`, ring the ack     |
//!
//! The header magic is the readiness gate: the primary writes it last, so a
//! secondary that sees `CIPC` knows the whole descriptor landed. That makes the
//! doorbell an optimization rather than a requirement — a secondary that boots late
//! finds the descriptor already published and skips the wait.
//!
//! [`DescriptorLayout`] is the negotiated part, and only that: `num_channels` plus a
//! proposed `mtu`. Everything identifying the link — channel ids, doorbell, region,
//! base — is fixed at construction. `mtu` is a *proposal*; after `connect` returns,
//! both sides report the resulting slot capacity through `max_send_size` /
//! `max_recv_size`, which is what `send` enforces.
//!
//! **Steady state** is the slot protocol, and its ordering is the whole design:
//!
//! ```text
//! send: pick a consumed TX slot → write payload → len → crc32 → FLAG_VALID
//!       → region.flush()  → doorbell.ring()
//! recv: doorbell.wait()   → region.invalidate()
//!       → check FLAG_VALID → read len → copy payload → verify crc32
//!       → FLAG_CONSUMED
//! ```
//!
//! `FLAG_VALID` is written last on the sending side and read first on the receiving
//! side, so a peer either sees a complete frame or no frame — never a half-written
//! one. `flush` precedes every `ring` and `invalidate` follows every wake, which is
//! what makes a `CachedRegion` link work; on an [`UncachedRegion`] both compile away.
//!
//! A CRC32 over the payload catches what ordering cannot: a truncated DMA, a
//! misconfigured MPU window, a peer writing with the wrong cache attributes. A
//! mismatch sets `FLAG_INTEGRITY_ERROR` in the slot and reports no message rather
//! than handing up corrupt bytes.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary
//! [`Side`]: consortium_ipc::Side
//! [`UncachedRegion`]: consortium_shared_memory::UncachedRegion
//!
//! # Example
//!
//! `new` is `unsafe`: the caller guarantees `region` covers the descriptor, channel table,
//! and every slot, and that nothing else aliases the window.
//!
//! ```rust,ignore
//! use consortium_ipc::{Connect, Secondary};
//! use consortium_ipc_transport_memory::{DescriptorLayout, SharedMemoryTransport};
//!
//! let mut transport = unsafe {
//!     SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, doorbell, region)
//! };
//!
//! // Same call on both sides; Side selects the protocol role.
//! transport.connect(DescriptorLayout::new(2, 256)).await?;
//!
//! let (tx, rx) = transport.split();
//! ```
//!
//! [`split`](SharedMemoryTransport::split) yields
//! [`SharedMemoryTransportTx`] and [`SharedMemoryTransportRx`], each carrying one
//! channel id and a clone of the doorbell and region handles, so a `Tx` and an `Rx`
//! `Channel` can be driven concurrently over one window.
//!
//! # Acknowledged sends
//!
//! [`send`] is fire-and-forget: it writes a slot, rings, and returns. That stays the
//! default on every channel. A channel built with [`new_acked`] also offers
//! [`send_acked`], which resolves only once the peer has drained *that* slot — and which
//! reports [`Error::PeerIntegrityError`] when the peer took the message and rejected its
//! CRC, something a plain send can never tell you.
//!
//! Two separate flags carry this, at two levels, because they answer different questions:
//!
//! - **Is acknowledgement wired on this link?** A capability, agreed once in the
//!   descriptor's channel entry ([`FLAG_CHAN_ACK_CAPABLE`]) so a half-configured pair
//!   fails at [`Connect::connect`] instead of hanging on the first acknowledged send.
//! - **Does *this message* want acknowledging?** The sender's per-call decision, recorded
//!   in the message's own slot header ([`slot::FLAG_ACK_REQUESTED`]). `send` never sets it,
//!   `send_acked` always does, so a capable channel taxes only the messages that ask.
//!
//! Acknowledgement gets its **own pair of doorbell lines**, one per core, rather than
//! reusing the data lines. Each line carries exactly one waiter — the receive half is
//! already parked on the data line, and the hardware doorbells hold one waker per line,
//! arm and disarm it unrefcounted, and *consume* the status on the first poll — so an
//! acknowledgement arriving there would have nowhere to land.
//!
//! Three things to know before using it:
//!
//! - **Two peers both awaiting an acknowledgement, with neither draining the other
//!   direction, deadlock.** The acknowledgement is produced by the peer's *receive* path,
//!   so that path has to be running. `send` reports instead of waiting, which is why it
//!   remains the default for lockstep single-task designs.
//! - **Delivery is at-least-once; acknowledgement is not.** Dropping the future after the
//!   ring loses the confirmation, not the message.
//! - **A plain `send` in flight blocks an acknowledged one.** An unacknowledged message
//!   frees its slot silently, so while one is outstanding there is no edge to wait for and
//!   `send_acked` returns [`Error::NoChannel`] — the same answer `send` would give, with
//!   the retry left to the caller. Hanging would be the alternative.
//!
//! [`send`]: consortium_ipc::SendTransport::send
//! [`send_acked`]: consortium_ipc::AckSendTransport::send_acked
//! [`new_acked`]: SharedMemoryTransport::new_acked
//! [`Connect::connect`]: consortium_ipc::Connect::connect
//!
//! # Considerations and limitations
//!
//! - **Depth is one frame per direction.** A slot holds a single message, so a
//!   sender whose peer has not set `FLAG_CONSUMED` gets [`Error::NoChannel`] rather
//!   than queueing. Backpressure is the caller's to handle, or delegated to
//!   [`send_acked`] on a channel wired for it.
//! - **At most [`CHANNEL_NUM_MAX`] channels** per descriptor.
//! - **`new` is `unsafe`.** The caller guarantees the window covers the descriptor,
//!   channel table, and every slot, and that nothing else aliases it. On Linux the
//!   pointer comes from a `mmap` of `/dev/uioN`; on firmware from a linker-placed
//!   symbol. `consortium-cfg` validates the bounds, overlap, and MPU alignment
//!   against `Consortium.toml` before either is generated.
//! - **The descriptor is not a security boundary.** CRC32 detects accident, not
//!   tampering; a malicious peer sharing the window is outside the threat model.
//! - **`connect` has no timeout**, per the [`Connect`] contract. Wrap it in
//!   `tokio::time::timeout` or `embassy_time::with_timeout` for bounded bring-up.
//!
//! # Features
//!
//! Defaults to `std`. Firmware builds want `default-features = false`.
//!
//! | Feature                          | Effect                                          |
//! | -------------------------------- | ----------------------------------------------- |
//! | `std`                            | `alloc`, Tokio support, UIO errors, `tracing` logs |
//! | `alloc`                          | link `alloc` through `consortium-ipc`             |
//! | `defmt`                          | route protocol tracing to `defmt`; derive `Format` |
//! | `stm32mp21x` / `23x` / `25x`     | select the STM32MP2 chip; enables `bell::ipcc` (and `bell::hsem` on 23x/25x) |
//! | `mimx93` / `mimx95`              | select the i.MX 9 chip; enables `bell::mu`        |
//!
//! The transport itself is generic over any [`Doorbell`] and needs none of them; the
//! chip features only bring in the Type-A bells and their chip HAL.
//!
//! # Testing
//!
//! Tests under `tests/` run the real protocol against heap-backed regions and fake
//! doorbells, so they need no hardware:
//!
//! ```bash
//! cargo test -p consortium-ipc-transport-memory
//! cargo test -p consortium-ipc-transport-memory --no-default-features \
//!     --test async_atomic_waker_test    # no_std-shaped future path
//! ```
//!
//! Coverage spans sync descriptor layout, Tokio futures, split halves,
//! codec-backed channels, Embassy-style polling, and an end-to-end
//! primary/secondary exchange. Prefer extending these before requiring hardware.

#[cfg(feature = "alloc")]
extern crate alloc;

/// Type-A doorbells: the generic [`BellA`](bell::BellA) over a per-block
/// [`BellHw`](bell::BellHw) register model, and the blocks themselves behind their chip
/// features (IPCC, MU, HSEM).
pub mod bell;

mod descriptor;
mod endpoint;
mod error;
#[path = "slot.rs"]
mod slot_internal;
mod transport;
pub(crate) mod utils;

pub use consortium_ipc::{Primary, Secondary, Side};
pub use descriptor::{
    CHANNEL_NUM_MAX, ChannelDirection, ChannelLayout, DESCRIPTOR_LENGTH, DescState, Descriptor,
    DescriptorLayout, FLAG_CHAN_ACK_CAPABLE, MAGIC, RX_UPDATED_ID_OFFSET, SLOT_ALIGNMENT, VERSION,
    channel_table_entry_offset, descriptor_complete, get_state, required_region_len, set_state,
    slot_offset, slot_stride,
};

pub use embassy_hal_internal::{Peri, PeripheralType};
pub use endpoint::{
    ConnectError, Connected, Endpoint, EndpointState, Instance, InstanceTransport, SealedInstance,
    SingletonGuard, Transceiver, install,
};
pub use error::{DescriptorError, Error};

/// Low-level slot layout operations over the DMA-coherent shared region.
///
/// A slot is one message buffer: a 12-byte header ([`SLOT_HEADER`](slot::SLOT_HEADER)) of
/// `len | flags | crc32`, followed by the payload.
///
/// The ordering rules are the protocol. [`write`](slot::write) lays down the payload, then
/// `len`, then `crc32`, and sets [`FLAG_VALID`](slot::FLAG_VALID) **last**; [`try_read`](slot::try_read) checks
/// `FLAG_VALID` **first** and sets [`FLAG_CONSUMED`](slot::FLAG_CONSUMED) only after it has copied and
/// verified the payload. A peer therefore observes either a complete frame or no
/// frame, never a partially written one. A CRC mismatch sets
/// [`FLAG_INTEGRITY_ERROR`](slot::FLAG_INTEGRITY_ERROR) and reports no message rather than surfacing corrupt
/// bytes.
///
/// [`FLAG_ACK_REQUESTED`](slot::FLAG_ACK_REQUESTED) rides in that same final store. It is the
/// sender's per-message request to be told when the slot drains, and [`try_read`](slot::try_read)
/// hands it back through [`ReadOutcome`](slot::ReadOutcome) — including on the integrity-error
/// path, because a sender parked on an acknowledgement must be woken whether or not its
/// message survived the CRC.
///
/// These functions are `unsafe` and take a raw slot pointer: they perform no bounds
/// checks and assume the caller resolved the slot from a validated channel-table
/// entry. Cache maintenance is *not* their job — the transport calls
/// [`Region::flush`](consortium_shared_memory::Region::flush) before ringing and
/// [`invalidate`](consortium_shared_memory::Region::invalidate) after waking.
/// Prefer [`SharedMemoryTransport`], which does all of that; this module is exposed
/// for tests and for tooling that inspects a live window.
pub mod slot {
    pub use super::slot_internal::{
        FLAG_ACK_REQUESTED, FLAG_CONSUMED, FLAG_INTEGRITY_ERROR, FLAG_VALID, ReadOutcome,
        SLOT_HEADER, ack_requested, flags, is_consumed, try_read, write,
    };
}

pub use transport::{
    RegionSharedMemoryTransportable, SharedMemoryTransport, SharedMemoryTransportRx,
    SharedMemoryTransportTx,
};
