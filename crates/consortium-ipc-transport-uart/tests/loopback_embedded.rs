#![cfg(not(feature = "std"))]
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

//! Loopback tests for the `embedded-io-async` (firmware-side) implementation
//! family. Run with `cargo test -p consortium-ipc-transport-uart
//! --no-default-features --test loopback_embedded`.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use consortium_codec::PostcardCodec;
use consortium_ipc::{Chan, Channel, RecvTransport, Rx, SendTransport, Tx};
use consortium_ipc_transport_uart::{Error, UartTransport, rx_buf_len, tx_buf_len};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};

const MTU: usize = 96;

/// One direction of a fake UART wire: a byte queue shared between a write
/// end and a read end, standing in for the peripheral's shift register path.
#[derive(Clone, Default)]
struct FakeWire(Arc<Mutex<VecDeque<u8>>>);

impl embedded_io_async::ErrorType for FakeWire {
    type Error = core::convert::Infallible;
}

impl embedded_io_async::Write for FakeWire {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.lock().unwrap().extend(buf);
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_io_async::Read for FakeWire {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut queue = self.0.lock().unwrap();
        // An empty queue reads as end-of-stream; tests write before reading.
        let n = buf.len().min(queue.len());
        for slot in &mut buf[..n] {
            *slot = queue.pop_front().expect("length checked");
        }
        Ok(n)
    }
}

fn scratch(n: usize) -> &'static mut [u8] {
    Box::leak(vec![0u8; n].into_boxed_slice())
}

fn chan(id: u8) -> Chan {
    // SAFETY: test-local logical channel byte; the UART transport accepts any id.
    unsafe { Chan::new_unchecked(id) }
}

/// Two crossed wire ends, like a null-modem cable.
fn transports(
    ch: Chan,
) -> (
    UartTransport<FakeWire, FakeWire>,
    UartTransport<FakeWire, FakeWire>,
) {
    let a_to_b = FakeWire::default();
    let b_to_a = FakeWire::default();
    let a = UartTransport::new(
        ch,
        b_to_a.clone(),
        a_to_b.clone(),
        scratch(rx_buf_len(MTU)),
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("buffers sized by rx_buf_len/tx_buf_len");
    let b = UartTransport::new(
        ch,
        a_to_b,
        b_to_a,
        scratch(rx_buf_len(MTU)),
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("buffers sized by rx_buf_len/tx_buf_len");
    (a, b)
}

#[test]
fn bytes_roundtrip_both_directions() {
    let (mut a, mut b) = transports(chan(1));
    block_on(async {
        a.send(b"ping from a").await.expect("send a->b");
        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(&buf[..n], b"ping from a");

        b.send(b"pong from b").await.expect("send b->a");
        let n = a.recv(&mut buf).await.expect("recv on a");
        assert_eq!(&buf[..n], b"pong from b");
    });
}

#[test]
fn split_halves_feed_typed_channels() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Command {
        motor: u8,
        rpm: u16,
    }

    let (a, b) = transports(chan(2));
    let (a_tx, _a_rx) = a.split();
    let (_b_tx, b_rx) = b.split();

    let mut tx = Channel::<Tx, Command, _, PostcardCodec>::new(chan(2), a_tx, scratch(MTU));
    let mut rx = Channel::<Rx, Command, _, PostcardCodec>::new(chan(2), b_rx, scratch(MTU));

    block_on(async {
        let msg = Command { motor: 2, rpm: 950 };
        tx.send(&msg).await.expect("typed send");
        let got = rx.recv().await.expect("typed recv").into_inner();
        assert_eq!(got, msg);
    });
}

#[test]
fn empty_wire_reads_as_eof() {
    let (_a, mut b) = transports(chan(3));
    block_on(async {
        let mut buf = [0u8; MTU];
        assert_eq!(b.recv(&mut buf).await.expect_err("empty wire"), Error::Eof);
    });
}
