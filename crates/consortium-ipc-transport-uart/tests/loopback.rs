#![cfg(feature = "std")]
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

//! In-memory loopback tests for the Tokio (`std`) implementation family.

use consortium_codec::PostcardCodec;
use consortium_ipc::{Chan, Channel, Connect, RecvTransport, Rx, SendTransport, Tx};
use consortium_ipc_transport_uart::{Error, UartTransport, rx_buf_len, tx_buf_len};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncWriteExt, DuplexStream, ReadHalf, WriteHalf};

const MTU: usize = 96;

fn scratch(n: usize) -> &'static mut [u8] {
    Box::leak(vec![0u8; n].into_boxed_slice())
}

fn chan(id: u8) -> Chan {
    // SAFETY: test-local logical channel byte; the UART transport accepts any id.
    unsafe { Chan::new_unchecked(id) }
}

type Wire = (ReadHalf<DuplexStream>, WriteHalf<DuplexStream>);

/// Two crossed wire ends, like a null-modem cable.
fn wires() -> (Wire, Wire) {
    let (a, b) = tokio::io::duplex(4096);
    (tokio::io::split(a), tokio::io::split(b))
}

fn transport(
    ch: Chan,
    (rx, tx): Wire,
) -> UartTransport<ReadHalf<DuplexStream>, WriteHalf<DuplexStream>> {
    UartTransport::new(
        ch,
        rx,
        tx,
        scratch(rx_buf_len(MTU)),
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("buffers sized by rx_buf_len/tx_buf_len")
}

/// Bring-up code that knows only the trait — the same helper shape the
/// shared-memory tests use, showing UART fits the uniform call.
async fn bring_up<T: Connect>(transport: &mut T, params: T::Params) -> Result<(), T::Error> {
    transport.connect(params).await
}

/// UART has no rendezvous: `connect` resolves immediately and leaves the link
/// exactly as constructed, so it can sit in generic bring-up without effect.
#[tokio::test]
async fn connect_is_an_immediate_no_op() {
    let (wire_a, wire_b) = wires();
    let mut a = transport(chan(9), wire_a);
    let mut b = transport(chan(9), wire_b);

    bring_up(&mut a, ()).await.expect("connect a");
    bring_up(&mut b, ()).await.expect("connect b");

    // Unlike shared memory, the MTU is fixed at construction, not negotiated.
    assert_eq!(a.max_send_size(), MTU);
    assert_eq!(b.max_recv_size(), MTU);

    a.send(b"after connect").await.expect("send a->b");
    let mut buf = [0u8; MTU];
    let n = b.recv(&mut buf).await.expect("recv on b");
    assert_eq!(&buf[..n], b"after connect");
}

#[tokio::test]
async fn bytes_roundtrip_both_directions() {
    let (wire_a, wire_b) = wires();
    let mut a = transport(chan(1), wire_a);
    let mut b = transport(chan(1), wire_b);

    a.send(b"ping from a").await.expect("send a->b");
    let mut buf = [0u8; MTU];
    let n = b.recv(&mut buf).await.expect("recv on b");
    assert_eq!(&buf[..n], b"ping from a");

    b.send(b"pong from b").await.expect("send b->a");
    let n = a.recv(&mut buf).await.expect("recv on a");
    assert_eq!(&buf[..n], b"pong from b");
}

#[tokio::test]
async fn back_to_back_frames_arrive_in_order() {
    let (wire_a, wire_b) = wires();
    let mut a = transport(chan(2), wire_a);
    let mut b = transport(chan(2), wire_b);

    a.send(b"first").await.expect("send first");
    a.send(b"second").await.expect("send second");
    a.send(&[]).await.expect("send empty");

    let mut buf = [0u8; MTU];
    let n = b.recv(&mut buf).await.expect("recv first");
    assert_eq!(&buf[..n], b"first");
    let n = b.recv(&mut buf).await.expect("recv second");
    assert_eq!(&buf[..n], b"second");
    let n = b.recv(&mut buf).await.expect("recv empty");
    assert_eq!(n, 0);
}

#[tokio::test]
async fn receiver_resynchronizes_after_line_noise() {
    let (a, b) = tokio::io::duplex(4096);
    let (rx_a, mut wa) = tokio::io::split(a);
    let (rx_b, wb) = tokio::io::split(b);

    // Inject a burst of line noise terminated by a frame delimiter: the 0x00
    // lets the receiver discard the partial garbage and resynchronize onto the
    // next clean frame.
    wa.write_all(&[0xDE, 0xAD, 0x17, 0x42, 0x00])
        .await
        .expect("inject line noise");

    let mut tx = consortium_ipc_transport_uart::UartTransportTx::new(
        chan(3),
        wa,
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("tx buffer sized");
    let mut rx = consortium_ipc_transport_uart::UartTransportRx::new(
        chan(3),
        rx_b,
        scratch(rx_buf_len(MTU)),
        MTU,
    )
    .expect("rx buffer sized");
    let _keep_other_direction_open = (rx_a, wb);

    tx.send(b"survives noise").await.expect("send after noise");
    let mut buf = [0u8; MTU];
    let n = rx.recv(&mut buf).await.expect("recv resynchronized");
    assert_eq!(&buf[..n], b"survives noise");
}

#[tokio::test]
async fn split_halves_feed_typed_channels() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Telemetry {
        sequence: u32,
        temperature_mc: i32,
    }

    let (wire_a, wire_b) = wires();
    let (a_tx, _a_rx) = transport(chan(4), wire_a).split();
    let (_b_tx, b_rx) = transport(chan(4), wire_b).split();

    let mut tx = Channel::<Tx, Telemetry, _, PostcardCodec>::new(chan(4), a_tx, scratch(MTU));
    let mut rx = Channel::<Rx, Telemetry, _, PostcardCodec>::new(chan(4), b_rx, scratch(MTU));

    let msg = Telemetry {
        sequence: 7,
        temperature_mc: -12_500,
    };
    tx.send(&msg).await.expect("typed send");
    let got = rx.recv().await.expect("typed recv").into_inner();
    assert_eq!(got, msg);
}

#[tokio::test]
async fn payload_over_mtu_is_rejected() {
    let (wire_a, _wire_b) = wires();
    let mut a = transport(chan(5), wire_a);
    let err = a
        .send(&[0u8; MTU + 1])
        .await
        .expect_err("payload too large");
    assert!(matches!(
        err,
        Error::PayloadTooLarge {
            payload_len,
            mtu: MTU
        } if payload_len == MTU + 1
    ));
}

#[tokio::test]
async fn undersized_recv_buffer_is_rejected() {
    let (wire_a, _wire_b) = wires();
    let mut a = transport(chan(6), wire_a);
    let mut small = [0u8; MTU - 1];
    let err = a.recv(&mut small).await.expect_err("buffer too small");
    assert!(matches!(
        err,
        Error::BufferTooSmall {
            needed: MTU,
            got,
        } if got == MTU - 1
    ));
}

#[tokio::test]
async fn closed_peer_reports_eof() {
    let (wire_a, wire_b) = wires();
    let a = transport(chan(7), wire_a);
    let mut b = transport(chan(7), wire_b);

    drop(a);
    let mut buf = [0u8; MTU];
    assert!(matches!(
        b.recv(&mut buf).await.expect_err("peer closed"),
        Error::Eof
    ));
}
