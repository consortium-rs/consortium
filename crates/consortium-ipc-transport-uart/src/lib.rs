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

//! COBS-framed UART transport for Consortium IPC.
//!
//! UART is the *wired* transport category with a Type-B (transport-coupled)
//! doorbell: byte arrival is itself the notification, so there is no
//! `Doorbell` and no `Side` — direction is physical.
//!
//! Wire format, per frame:
//!
//! ```text
//! COBS( chan: u8 | payload[0..=MTU] | crc32: u32 LE )  0x00
//! ```
//!
//! Backend selection:
//!
//! - default / `no_std`: transports are generic over
//!   [`embedded_io_async::Read`]/[`Write`](embedded_io_async::Write)
//!   (Embassy and modern HAL UART drivers implement these);
//! - `std`: transports are generic over Tokio `AsyncRead`/`AsyncWrite`; the
//!   `consortium-ipc-transport-uart-unix` crate opens `/dev/tty*` devices
//!   for this family.

#[cfg(feature = "alloc")]
extern crate alloc;

pub(crate) mod error;
pub(crate) mod frame;
pub(crate) mod transport;

pub use {
    error::Error,
    frame::{FRAME_OVERHEAD, max_frame_len, rx_buf_len, tx_buf_len},
    transport::{UartTransport, UartTransportRx, UartTransportTx},
};
