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
//! The wire protocol and transport live in `consortium-ipc-transport-uart` and
//! are generic over [`embedded_io_async::Read`]/[`Write`](embedded_io_async::Write).
//! This crate wires that transport to the chip HALs' interrupt-buffered UART
//! drivers (`BufferedUart`), selected by a chip feature:
//!
//! - `mimx93` / `mimx95` re-export [`consortium_hal_imx9::uart`] as [`imx9`];
//! - `stm32mp21x` / `stm32mp23x` / `stm32mp25x` re-export
//!   [`consortium_hal_stm32mp2::uart`] as [`stm32mp2`].
//!
//! With no chip feature the crate is chip-agnostic: use any driver that
//! implements the `embedded-io-async` traits (Embassy, a custom HAL) directly
//! with the re-exported transport types.
//!
//! The `BufferedUart` receiver is interrupt-driven: its IRQ handler drains the
//! receive FIFO into a ring so no bytes are lost between the wake and the next
//! poll of the transport's `recv`. Per repository convention, this crate does
//! **not** install `#[interrupt]` handlers — the application owns the vector
//! table and forwards the UART interrupt into `on_buffered_interrupt::<T>`,
//! parameterized with the same peripheral singleton the driver took. That is
//! the same wake pattern the doorbell crates use.
//!
//! # End-to-end (STM32MP2, `stm32mp25x` + the HAL's `rt` feature)
//!
//! ```ignore
//! use consortium_ipc::Chan;
//! use consortium_ipc_transport_uart_embedded::{
//!     UartTransport, rx_buf_len, tx_buf_len,
//!     stm32mp2::{BufferedUart, Config, on_buffered_interrupt},
//! };
//! use consortium_runtime_mcu::static_buf;
//!
//! const MTU: usize = 256;
//!
//! // Ring storage for the IRQ handler, plus the transport scratch buffers.
//! // `static_buf!(NAME, LEN)` declares a `StaticCell<[u8; LEN]>`; `.init(..)`
//! // yields the `&'static mut [u8]` each constructor wants.
//! static_buf!(RX_RING, 512);
//! static_buf!(RX_SCRATCH, rx_buf_len(MTU));
//! static_buf!(TX_SCRATCH, tx_buf_len(MTU));
//!
//! // The peripheral singleton carries the register base and the driver's
//! // interrupt state; the platform must leave USART2 clocked and pinned out.
//! let p = consortium_hal_stm32mp2::init();
//! let uart = BufferedUart::new(
//!     p.USART2,
//!     KERNEL_HZ,
//!     Config::default(),
//!     RX_RING.init([0; 512]),
//! );
//! let (tx_wire, rx_wire) = uart.split();
//!
//! let transport = UartTransport::new(
//!     Chan::new(0)?,
//!     rx_wire,
//!     tx_wire,
//!     RX_SCRATCH.init([0; rx_buf_len(MTU)]),
//!     TX_SCRATCH.init([0; tx_buf_len(MTU)]),
//!     MTU,
//! )?;
//! // `tx` / `rx` are `SendTransport` / `RecvTransport`; wrap them in
//! // `Channel<Tx, ..>` / `Channel<Rx, ..>` with a codec to send typed messages.
//! let (tx, rx) = transport.split();
//!
//! #[interrupt]
//! fn USART2() {
//!     on_buffered_interrupt::<consortium_hal_stm32mp2::peripherals::USART2>();
//! }
//! ```
//!
//! The i.MX9 flow is identical with [`imx9::BufferedUart`] and an `LPUARTn`
//! vector. See the melt-pot `mcu/src/main.rs` files for full worked examples of
//! the vector-table forwarding convention.

pub use consortium_ipc_transport_uart::{
    Error, FRAME_OVERHEAD, UartTransport, UartTransportRx, UartTransportTx, max_frame_len,
    rx_buf_len, tx_buf_len,
};
pub use embedded_io_async;

/// i.MX9 LPUART driver, including the interrupt-buffered `BufferedUart`.
#[cfg(any(feature = "mimx93", feature = "mimx95"))]
pub mod imx9 {
    pub use consortium_hal_imx9::uart::*;
}

/// STM32MP2 USART driver, including the interrupt-buffered `BufferedUart`.
#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub mod stm32mp2 {
    pub use consortium_hal_stm32mp2::uart::*;
}
