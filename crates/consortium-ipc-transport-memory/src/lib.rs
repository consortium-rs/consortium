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
//! semantics, the descriptor header); hardware signaling comes from the
//! `consortium-ipc-doorbell-*` crates. What this crate adds is the protocol.
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
//!       │ slot 0  [len|flags|crc32|data] │  ← one direction
//!       │ slot 1  [len|flags|crc32|data] │  ← the other
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
//! what makes a `DdrRegion` link work; on an [`SramRegion`] both compile away.
//!
//! A CRC32 over the payload catches what ordering cannot: a truncated DMA, a
//! misconfigured MPU window, a peer writing with the wrong cache attributes. A
//! mismatch sets `FLAG_INTEGRITY_ERROR` in the slot and reports no message rather
//! than handing up corrupt bytes.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary
//! [`Side`]: consortium_ipc::Side
//! [`SramRegion`]: consortium_shared_memory::SramRegion
//!
//! # Example
//!
//! ```rust,ignore
//! use consortium_ipc::{Connect, Secondary};
//! use consortium_ipc_transport_memory::{DescriptorLayout, SharedMemoryTransport};
//!
//! // SAFETY: `base` covers the descriptor, channel table, and slots, and nothing
//! // else aliases the window.
//! let mut transport = unsafe {
//!     SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, doorbell, region, base)
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
//! # Considerations and limitations
//!
//! - **Depth is one frame per direction.** A slot holds a single message, so a
//!   sender whose peer has not set `FLAG_CONSUMED` gets [`Error::NoChannel`] rather
//!   than queueing. Backpressure is the caller's to handle.
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
//! | `doorbell-hsem` / `-ipcc` / `-mu` | re-export a doorbell crate under [`doorbell`]     |
//! | `stm32mp21x` / `23x` / `25x`     | select the STM32MP2 chip and its doorbells        |
//! | `mimx93` / `mimx95`              | select the i.MX 9 chip and its MU doorbell        |
//!
//! Chip features exist so [`doorbell::SharedMemoryTransportDoorbell`] can report a
//! doorbell peripheral's window address, which generated Linux bring-up needs in
//! order to map it.
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

/// Doorbells commonly paired with the shared-memory transport.
///
/// Each `doorbell-*` or chip feature re-exports one `consortium-ipc-doorbell-*`
/// crate here (as `hsem`, `ipcc`, or `mu`) and implements
/// `SharedMemoryTransportDoorbell` for its driver type. That trait reports
/// whether the doorbell is a real peripheral and where its register window lives —
/// which is what generated Linux bring-up needs in order to `mmap` it, and which
/// the doorbell crate itself keeps in its own base table so a security alias does
/// not change.
///
/// Nothing here is required to use the transport: it is generic over any
/// [`Doorbell`](consortium_ipc::Doorbell), including the fake ones the tests use.
pub mod doorbell;

mod descriptor;
mod error;
#[path = "slot.rs"]
mod slot_internal;
mod transport;
pub(crate) mod utils;

pub use consortium_ipc::{Primary, Secondary, Side};
pub use descriptor::{
    CHANNEL_NUM_MAX, ChannelDirection, ChannelLayout, DESCRIPTOR_LENGTH, DescState, Descriptor,
    DescriptorLayout, MAGIC, RX_UPDATED_ID_OFFSET, VERSION, channel_table_entry_offset,
    descriptor_complete, get_state, set_state,
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
/// These functions are `unsafe` and take a raw slot pointer: they perform no bounds
/// checks and assume the caller resolved the slot from a validated channel-table
/// entry. Cache maintenance is *not* their job — the transport calls
/// [`Region::flush`](consortium_shared_memory::Region::flush) before ringing and
/// [`invalidate`](consortium_shared_memory::Region::invalidate) after waking.
/// Prefer [`SharedMemoryTransport`], which does all of that; this module is exposed
/// for tests and for tooling that inspects a live window.
pub mod slot {
    pub use super::slot_internal::{
        FLAG_CONSUMED, FLAG_INTEGRITY_ERROR, FLAG_VALID, SLOT_HEADER, is_consumed, try_read, write,
    };
}

pub use transport::{
    RecvAccess, RecvFut, RegionSharedMemoryTransportable, SendFut, SharedMemoryTransport,
    SharedMemoryTransportRx, SharedMemoryTransportTx,
};
