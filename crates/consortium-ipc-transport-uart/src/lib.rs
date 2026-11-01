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
//! # Why the framing looks like this
//!
//! A UART hands over bytes, not messages, and it will happily hand over the wrong
//! ones: a peer that reboots mid-frame, a line that glitches at power-up, a baud
//! mismatch. Each layer of the frame answers one of those.
//!
//! - **COBS** removes every `0x00` from the frame body, which reserves `0x00` as a
//!   delimiter that cannot occur inside a frame. A receiver that has lost sync only
//!   has to hunt for the next zero byte to be aligned again — no length prefix to
//!   mistrust, no escape-state machine to get stuck in.
//! - **CRC32** over `chan | payload` catches the corruption that framing cannot:
//!   flipped bits, a dropped byte, a peer transmitting at the wrong rate.
//! - **`chan`** carries the [`Chan`](consortium_ipc::Chan) contract for serial
//!   links. One transport instance owns one logical channel and drops frames
//!   stamped with anybody else's byte.
//!
//! Frame-level corruption is deliberately **not** an error: the receiver logs the
//! bad frame, discards it, and keeps waiting for the next delimiter. [`Error`]
//! reports only what a caller can act on — IO failure, EOF, an oversized payload, an
//! under-sized buffer.
//!
//! # No doorbell, no side, no handshake
//!
//! Byte arrival *is* the notification, so there is no [`Doorbell`] type parameter to
//! supply and no interrupt to forward. Direction is physical — the TX and RX wires —
//! so there is no [`Side`] either. [`Connect`] is implemented as a documented no-op,
//! which keeps generated bring-up code uniform across transports: it emits the same
//! `connect(..)` call whether the link underneath is shared memory or a wire.
//!
//! [`Doorbell`]: consortium_ipc::Doorbell
//! [`Side`]: consortium_ipc::Side
//! [`Connect`]: consortium_ipc::Connect
//!
//! # Buffers
//!
//! Like every Consortium transport this one does not allocate; the caller supplies
//! `&'static mut [u8]` scratch. UART needs *two* buffers, and their sizes are not
//! the MTU — COBS expansion and the frame accumulator both add overhead. Derive them
//! rather than guessing:
//!
//! | Half                 | Size                | Holds                                  |
//! | -------------------- | ------------------- | -------------------------------------- |
//! | [`UartTransportTx`]  | [`tx_buf_len(mtu)`] | frame body, then the encoded frame      |
//! | [`UartTransportRx`]  | [`rx_buf_len(mtu)`] | a read chunk, then a frame accumulator  |
//!
//! Both constructors verify the buffer against the MTU and return
//! [`Error::BufferTooSmall`] rather than trusting the caller.
//!
//! [`tx_buf_len(mtu)`]: tx_buf_len
//! [`rx_buf_len(mtu)`]: rx_buf_len
//!
//! # Backend selection
//!
//! The IO bounds are chosen by the `std` feature, and this is the crate's one real
//! wrinkle:
//!
//! - **without `std` (firmware)** — generic over [`embedded_io_async::Read`] and
//!   [`Write`](embedded_io_async::Write), the async serial traits Embassy and modern
//!   HAL UART drivers implement.
//! - **with `std` (Linux/host)** — generic over Tokio `AsyncRead`/`AsyncWrite`.
//!
//! The split exists because under `std` transport futures must be `Send` (see
//! `consortium_ipc::MaybeSend`), and `embedded_io_async`'s unnameable `async fn`
//! futures cannot guarantee that generically. So the two families are separate impl
//! blocks over the same types rather than one generic implementation.
//!
//! Optional modules supply the concrete IO: `unix` (`unix` feature) opens
//! `/dev/tty*` devices for the Tokio family, and `imx9` / `stm32mp2` (chip
//! features) re-export a chip HAL's interrupt-buffered driver for the
//! `embedded-io-async` family. With no chip feature, any driver implementing the
//! `embedded-io-async` traits works directly.
//!
//! # Firmware: interrupt-buffered receive
//!
//! The chip HALs' `BufferedUart` receiver is interrupt-driven: its IRQ handler
//! drains the receive FIFO into a ring so no bytes are lost between the wake and
//! the next poll of the transport's `recv`. Per repository convention, this crate
//! does **not** install `#[interrupt]` handlers — the application owns the vector
//! table and forwards the UART interrupt into `on_buffered_interrupt::<T>`,
//! parameterized with the same peripheral singleton the driver took. Enable the
//! *HAL's* `rt` feature for the `#[interrupt]` macro and PAC vector table.
//!
//! ```ignore
//! // `stm32mp25x` feature, plus `consortium-hal-stm32mp2/rt`.
//! use consortium_ipc::Chan;
//! use consortium_ipc_transport_uart::{
//!     UartTransport, rx_buf_len, tx_buf_len,
//!     stm32mp2::{BufferedUart, Config, on_buffered_interrupt},
//! };
//! use consortium_runtime_mcu::static_buf;
//!
//! const MTU: usize = 256;
//!
//! // Ring storage for the IRQ handler, plus the transport scratch buffers.
//! static_buf!(RX_RING, 512);
//! static_buf!(RX_SCRATCH, rx_buf_len(MTU));
//! static_buf!(TX_SCRATCH, tx_buf_len(MTU));
//!
//! // The platform must leave USART2 clocked and pinned out.
//! let p = consortium_hal_stm32mp2::init();
//! let uart = BufferedUart::new(p.USART2, KERNEL_HZ, Config::default(), RX_RING.init([0; 512]));
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
//! let (tx, rx) = transport.split();
//!
//! #[interrupt]
//! fn USART2() {
//!     on_buffered_interrupt::<consortium_hal_stm32mp2::peripherals::USART2>();
//! }
//! ```
//!
//! The i.MX9 flow is identical with `imx9::BufferedUart` and an `LPUARTn` vector.
//!
//! # Example
//!
//! ```rust,ignore
//! use consortium_ipc_transport_uart::{UartTransport, rx_buf_len, tx_buf_len};
//!
//! const MTU: usize = 256;
//! static mut TX: [u8; tx_buf_len(MTU)] = [0; tx_buf_len(MTU)];
//! static mut RX: [u8; rx_buf_len(MTU)] = [0; rx_buf_len(MTU)];
//!
//! let transport = UartTransport::new(chan, reader, writer, rx_buf, tx_buf, MTU)?;
//! let (tx, rx) = transport.split();
//! ```
//!
//! # Features
//!
//! Defaults to `std`. Firmware builds want `default-features = false`.
//!
//! - `std` — `alloc`, Tokio IO bounds, `Send` futures, `tracing` diagnostics.
//! - `alloc` — link `alloc` through `consortium-ipc`.
//! - `defmt` — route diagnostics to `defmt` and derive `defmt::Format` on [`Error`]
//!   (and forward `defmt` to a selected chip HAL).
//! - `unix` — implies `std`; adds the `unix` module (`open`, `pty_pair`) over
//!   `tokio-serial`.
//! - `mimx93` / `mimx95` — re-export `consortium_hal_imx9::uart` as `imx9`.
//! - `stm32mp21x` / `stm32mp23x` / `stm32mp25x` — re-export
//!   `consortium_hal_stm32mp2::uart` as `stm32mp2`.
//!
//! # Testing
//!
//! ```bash
//! cargo test -p consortium-ipc-transport-uart                        # Tokio family
//! cargo test -p consortium-ipc-transport-uart --no-default-features  # embedded-io family
//! cargo test -p consortium-ipc-transport-uart --features unix        # pseudo-terminals (Linux)
//! ```
//!
//! Run all three when touching the protocol: the two feature paths are distinct impl
//! blocks, so a change to one is not covered by the other's tests.

#[cfg(feature = "alloc")]
extern crate alloc;

pub(crate) mod error;
pub(crate) mod frame;
pub(crate) mod transport;

#[cfg(feature = "unix")]
pub mod unix;

pub use {
    embedded_io_async,
    error::Error,
    frame::{FRAME_OVERHEAD, max_frame_len, rx_buf_len, tx_buf_len},
    transport::{UartTransport, UartTransportRx, UartTransportTx},
};

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
