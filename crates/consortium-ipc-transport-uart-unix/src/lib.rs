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

//! `/dev/tty*` backend for the Consortium UART IPC transport.
//!
//! Opens a serial device in raw mode via `tokio-serial` and hands its Tokio
//! `AsyncRead`/`AsyncWrite` halves to
//! [`UartTransport`](consortium_ipc_transport_uart::UartTransport)'s `std`
//! implementation family:
//!
//! ```no_run
//! use consortium_ipc_transport_uart::{UartTransport, rx_buf_len, tx_buf_len};
//! use consortium_ipc_transport_uart_unix::open;
//!
//! # fn scratch(n: usize) -> &'static mut [u8] { Box::leak(vec![0; n].into_boxed_slice()) }
//! # async fn demo(chan: consortium_ipc::Chan) -> Result<(), Box<dyn std::error::Error>> {
//! const MTU: usize = 256;
//! let (rx_wire, tx_wire) = open("/dev/ttySTM1", 115_200)?;
//! let transport = UartTransport::new(
//!     chan,
//!     rx_wire,
//!     tx_wire,
//!     scratch(rx_buf_len(MTU)),
//!     scratch(tx_buf_len(MTU)),
//!     MTU,
//! )?;
//! let (tx, rx) = transport.split();
//! # Ok(()) }
//! ```

use tokio::io::{ReadHalf, WriteHalf};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

pub use consortium_ipc_transport_uart::{
    Error, UartTransport, UartTransportRx, UartTransportTx, rx_buf_len, tx_buf_len,
};
pub use tokio_serial;

/// Receive wire half of an opened serial device.
pub type UartRx = ReadHalf<SerialStream>;
/// Transmit wire half of an opened serial device.
pub type UartTx = WriteHalf<SerialStream>;

/// Errors from [`open`].
#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("open {path}: {source}")]
    Open {
        path: String,
        source: tokio_serial::Error,
    },
}

/// Open `path` (e.g. `/dev/ttySTM1`, `/dev/ttyUSB0`) at `baud` in raw 8N1
/// mode and split it into `(rx, tx)` wire halves for
/// [`UartTransport::new`].
///
/// Must be called from within a Tokio runtime (the stream registers with the
/// current reactor).
pub fn open(path: &str, baud: u32) -> Result<(UartRx, UartTx), OpenError> {
    let stream = tokio_serial::new(path, baud)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()
        .map_err(|source| OpenError::Open {
            path: path.to_owned(),
            source,
        })?;
    Ok(tokio::io::split(stream))
}

/// Create a connected pseudo-terminal pair for host-side testing, each split
/// into `(rx, tx)` wire halves.
///
/// Bytes written to one side's `tx` arrive on the other side's `rx`, mimicking
/// a crossed UART cable.
#[cfg(unix)]
pub fn pty_pair() -> tokio_serial::Result<((UartRx, UartTx), (UartRx, UartTx))> {
    let (a, b) = SerialStream::pair()?;
    Ok((tokio::io::split(a), tokio::io::split(b)))
}
