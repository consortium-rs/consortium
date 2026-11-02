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

//! The byte-IO seam the transport is written against.
//!
//! [`UartRead`]/[`UartWrite`] exist so the transport has **one** impl family.
//! Under `std` the transport's futures must be `Send` (`MaybeSend`), and
//! `embedded_io_async`'s unnameable `async fn` futures cannot be proven `Send`
//! generically — so the transport cannot be generic over those traits directly
//! without a second, `std`-only copy over Tokio's. Instead the `MaybeSend`
//! bounds live here, as in the FDCAN transport's `CanTx`/`CanRx`, and the
//! backend is chosen by which blanket impl the `std` feature compiles:
//!
//! - without `std`: every [`embedded_io_async::Read`]/[`Write`](embedded_io_async::Write);
//! - with `std`: every Tokio `AsyncRead`/`AsyncWrite` that is `Unpin + Send`.
//!
//! The two blanket impls cannot coexist (a type may implement both trait
//! families), which is why the choice is a feature rather than both at once.
//! Implement the traits directly for a driver that fits neither.

use consortium_ipc::MaybeSend;

/// Receive wire of a UART: a byte stream with no message boundaries.
pub trait UartRead: MaybeSend {
    /// Driver-reported IO error.
    type Error;

    /// Read at least one byte into `buf`, returning how many were read.
    ///
    /// `Ok(0)` means end-of-stream, which the transport reports as
    /// [`Error::Eof`](crate::Error::Eof).
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend;
}

/// Transmit wire of a UART.
pub trait UartWrite: MaybeSend {
    /// Driver-reported IO error.
    type Error;

    /// Write all of `buf`.
    fn write_all(
        &mut self,
        buf: &[u8],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Resolve once every written byte has left the driver's buffers.
    fn flush(&mut self) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}

#[cfg(not(feature = "std"))]
impl<T: embedded_io_async::Read> UartRead for T {
    type Error = T::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        embedded_io_async::Read::read(self, buf).await
    }
}

#[cfg(not(feature = "std"))]
impl<T: embedded_io_async::Write> UartWrite for T {
    type Error = T::Error;

    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        embedded_io_async::Write::write_all(self, buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        embedded_io_async::Write::flush(self).await
    }
}

#[cfg(feature = "std")]
impl<T: tokio::io::AsyncRead + Unpin + Send> UartRead for T {
    type Error = std::io::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        tokio::io::AsyncReadExt::read(self, buf).await
    }
}

#[cfg(feature = "std")]
impl<T: tokio::io::AsyncWrite + Unpin + Send> UartWrite for T {
    type Error = std::io::Error;

    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        tokio::io::AsyncWriteExt::write_all(self, buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        tokio::io::AsyncWriteExt::flush(self).await
    }
}
