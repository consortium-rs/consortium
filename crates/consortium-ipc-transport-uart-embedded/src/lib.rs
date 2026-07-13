#![no_std]
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

//! Firmware-side entry point for the Consortium UART IPC transport.
//!
//! The transport itself lives in `consortium-ipc-transport-uart` and is
//! generic over [`embedded_io_async::Read`]/[`Write`](embedded_io_async::Write),
//! which Embassy and modern HAL UART drivers implement directly:
//!
//! ```ignore
//! let (tx_wire, rx_wire) = hal_uart.split();
//! let transport = UartTransport::new(
//!     chan,
//!     rx_wire,
//!     tx_wire,
//!     static_buf!(rx_buf_len(MTU)),
//!     static_buf!(tx_buf_len(MTU)),
//!     MTU,
//! )?;
//! let (tx, rx) = transport.split();
//! ```
//!
//! Per repository convention, no crate in this family installs `#[interrupt]`
//! handlers: the application owns the vector table. For PAC-level UARTs
//! without an async HAL driver, the planned (not yet implemented) addition
//! here is an IRQ-fed RX ring implementing [`embedded_io_async::Read`], whose
//! `notify()`-style feed function the application calls from its UART RX/idle
//! interrupt handler — the same wake pattern the doorbell crates use.

pub use consortium_ipc_transport_uart::{
    Error, FRAME_OVERHEAD, UartTransport, UartTransportRx, UartTransportTx, max_frame_len,
    rx_buf_len, tx_buf_len,
};
pub use embedded_io_async;
