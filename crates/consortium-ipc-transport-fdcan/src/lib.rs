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

//! Segmenting FDCAN IPC transport.
//!
//! This is the second *wired*, Type-B transport in the workspace, after
//! [`consortium-ipc-transport-uart`]. Type-B means the link itself is the
//! notification: a frame arriving is the wakeup, so there is no
//! [`Doorbell`](consortium_ipc::Doorbell) and no `Side` — unlike shared memory,
//! where both are needed to coordinate two cores over one window.
//!
//! # Wire format
//!
//! A CAN controller already delimits every frame and protects it with a CRC, so
//! there is nothing here to frame or checksum — the UART transport's COBS and
//! CRC32 layers have no counterpart. What CAN lacks is a way to carry a message
//! longer than one frame, so that is all this crate adds:
//!
//! ```text
//! byte 0: [ kind:2 | low:6 ]
//!   00 Single   : low = payload length; the whole message follows
//!   01 First    : low = 0; bytes 1..3 = total length u16 LE, then the head
//!   10 Continue : low = sequence number, 1,2,3,… mod 64; then the body
//!   11 reserved
//! ```
//!
//! # Sizing
//!
//! One payload byte per frame goes to the header, and a `First` frame spends
//! two more on the total length:
//!
//! | Frame kind | Classic (8 B) | FD (64 B) |
//! | ---------- | ------------- | --------- |
//! | `Single`   | 7 B           | 63 B      |
//! | `First`    | 5 B           | 61 B      |
//! | `Continue` | 7 B           | 63 B      |
//!
//! [`frames_for`] reports what a full-MTU message costs in frames, which is the
//! number to reach for when budgeting bus time. A 128-byte message is 19
//! classic frames — about 2.5 ms at 500 kbit/s — or 3 FD frames.
//!
//! Segments are padded up to a length the medium can describe ([`padded_len`]).
//! On classic CAN that is a no-op, since every length `0..=8` is exact. On FD
//! the DLC jumps 8, 12, 16, 20, 24, 32, 48, 64, so a 10-byte segment goes out
//! as 12. The receiver ignores the difference — a `Single` carries its own
//! length and a `Continue` is bounded by the declared total.
//!
//! Only the receive half takes a buffer, sized by [`rx_buf_len`]. CAN frames are
//! fixed-size, so the transmit path stages each one in a stack array and slices
//! the caller's payload directly; there is no `tx_buf_len` to match the UART
//! crate's.
//!
//! # Identifiers
//!
//! [`Chan`](consortium_ipc::Chan) does not go on the wire. CAN frames already
//! carry an identifier, so each half is bound to one: the transmit half stamps
//! it into every frame, and the receive half drops anything else. The two
//! directions **must** use different identifiers, since both halves see every
//! frame on the bus and an endpoint must not reassemble its own traffic.
//! [`FdcanTransport::new`] rejects equality with [`Error::SameIdentifier`]
//! rather than leaving it to be discovered at runtime — a controller running
//! with loopback or self-reception, as bring-up self-tests use, echoes every
//! frame straight back.
//!
//! Filtering is currently done in software, because neither chip HAL programs
//! hardware acceptance filters yet. Frames for other identifiers therefore cost
//! a wakeup; a hardware filter is the natural optimization and needs no change
//! here.
//!
//! # Example
//!
//! ```ignore
//! use consortium_ipc_transport_fdcan::{FdcanTransport, rx_buf_len};
//! use embedded_can::{Id, StandardId};
//!
//! const MTU: usize = 128;
//! static RX: StaticCell<[u8; rx_buf_len(MTU)]> = StaticCell::new([0; rx_buf_len(MTU)]);
//!
//! let (tx, rx) = can.split();
//! let mut transport = FdcanTransport::new(
//!     chan,
//!     Id::Standard(StandardId::new(0x101).unwrap()), // we send here
//!     Id::Standard(StandardId::new(0x102).unwrap()), // the peer sends here
//!     tx,
//!     rx,
//!     RX.init([0; rx_buf_len(MTU)]),
//!     MTU,
//! )?;
//! transport.connect(()).await?;
//! let (tx, rx) = transport.split();
//! ```
//!
//! # Backends
//!
//! | Crate | Controller |
//! | ----- | ---------- |
//! | `consortium-ipc-transport-fdcan-embedded` | Chip HAL drivers for i.MX9 FlexCAN and STM32MP2 FDCAN |
//! | this crate, no backend | Any type implementing [`CanTx`]/[`CanRx`] |
//!
//! # Limitations
//!
//! - Classic 8-byte frames. The seam is FD-ready — a frame type reporting
//!   `MAX_PAYLOAD = 64` needs no change here — but neither chip HAL drives FD
//!   yet.
//! - No flow control. A receiver that is not already parked in `recv` can miss
//!   frames from a burst, and the segmentation layer will report the resulting
//!   sequence gap and drop that message. The i.MX9 driver's single receive
//!   mailbox makes this tighter than the STM32MP2's eight-element FIFO.
//! - One [`Chan`](consortium_ipc::Chan) per transport; no multiplexing.
//! - Losing exactly a multiple of 64 consecutive frames is invisible to the
//!   sequence counter. The codec's decode is the backstop.
//!
//! # Features
//!
//! | Feature | Effect |
//! | ------- | ------ |
//! | `std`   | Default. Widens `MaybeSend` to `Send` and routes logging to `tracing`. |
//! | `alloc` | Implied by `std`; enables `consortium-ipc`'s allocating paths. |
//! | `defmt` | Routes logging to `defmt` and derives `Format` on [`Error`]. |
//!
//! Unlike the UART transport, `std` does not select a second set of impls: the
//! [`CanTx`]/[`CanRx`] seam belongs to this crate, so its futures are declared
//! `MaybeSend` once and one impl family serves both firmware and host.
//!
//! # Testing
//!
//! ```bash
//! cargo test -p consortium-ipc-transport-fdcan
//! cargo test -p consortium-ipc-transport-fdcan --no-default-features
//! ```
//!
//! Both run the same sources, including the same `loopback` integration test —
//! which is the evidence for the single-impl-family claim above.
//!
//! [`consortium-ipc-transport-uart`]: https://docs.rs/consortium-ipc-transport-uart

pub(crate) mod can;
pub(crate) mod error;
pub(crate) mod frame;
pub(crate) mod transport;

/// Re-exported so callers can name [`Id`](embedded_can::Id) — which every
/// constructor here takes — without adding their own dependency and risking a
/// version mismatch on the trait this crate's blanket [`CanFrame`] impl keys on.
pub use embedded_can;

pub use {
    can::{CanErrorType, CanFrame, CanRx, CanTx},
    error::Error,
    frame::{
        CLASSIC_PAYLOAD, FD_PAYLOAD, FIRST_EXTRA, HEADER_LEN, MAX_MTU, MIN_FRAME_PAYLOAD,
        continue_max, first_max, frames_for, padded_len, rx_buf_len, single_max,
    },
    transport::{FdcanTransport, FdcanTransportRx, FdcanTransportTx},
};
