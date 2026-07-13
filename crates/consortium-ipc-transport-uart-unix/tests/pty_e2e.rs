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

//! End-to-end exchange over a real pseudo-terminal pair — the closest
//! host-side stand-in for two cores linked by a UART cable.
//!
//! Linux only. macOS/BSD `kqueue` does not deliver read-readiness events for
//! pty file descriptors, so `tokio-serial`'s async reads never wake and the
//! exchange stalls. This is a host-test limitation, not a transport one: the
//! `/dev/tty*` deployment target is the Linux A-core, and the portable
//! in-memory duplex loopback tests in `consortium-ipc-transport-uart` cover
//! the same transport logic on every host. Each test still wraps its awaits
//! in a wall-clock timeout so a regression fails fast instead of hanging.
#![cfg(target_os = "linux")]

use std::time::Duration;

use consortium_codec::PostcardCodec;
use consortium_ipc::{Chan, Channel, RecvTransport, Rx, SendTransport, Tx};
use consortium_ipc_transport_uart_unix::{
    UartTransport, UartTransportRx, UartTransportTx, pty_pair, rx_buf_len, tx_buf_len,
};
use serde::{Deserialize, Serialize};

const MTU: usize = 128;
const TIMEOUT: Duration = Duration::from_secs(5);

fn scratch(n: usize) -> &'static mut [u8] {
    Box::leak(vec![0u8; n].into_boxed_slice())
}

fn chan(id: u8) -> Chan {
    // SAFETY: test-local logical channel byte; the UART transport accepts any id.
    unsafe { Chan::new_unchecked(id) }
}

async fn within<T>(fut: impl Future<Output = T>) -> T {
    tokio::time::timeout(TIMEOUT, fut)
        .await
        .expect("pty exchange should complete well within the timeout")
}

#[tokio::test]
async fn raw_bytes_roundtrip_over_pty() {
    let ((rx_a, tx_a), (rx_b, tx_b)) = pty_pair().expect("create pty pair");
    let mut a = UartTransport::new(
        chan(1),
        rx_a,
        tx_a,
        scratch(rx_buf_len(MTU)),
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("buffers sized");
    let mut b = UartTransport::new(
        chan(1),
        rx_b,
        tx_b,
        scratch(rx_buf_len(MTU)),
        scratch(tx_buf_len(MTU)),
        MTU,
    )
    .expect("buffers sized");

    within(a.send(b"over a real tty")).await.expect("send a->b");
    let mut buf = [0u8; MTU];
    let n = within(b.recv(&mut buf)).await.expect("recv on b");
    assert_eq!(&buf[..n], b"over a real tty");

    within(b.send(b"and back")).await.expect("send b->a");
    let n = within(a.recv(&mut buf)).await.expect("recv on a");
    assert_eq!(&buf[..n], b"and back");
}

#[tokio::test]
async fn typed_channels_exchange_both_directions_over_pty() {
    // Distinct logical channels per direction, so even a pty configured with
    // echo would only feed each receiver foreign (dropped) frames.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SensorReading {
        sequence: u32,
        millivolts: u32,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Setpoint {
        valve: u8,
        percent: u8,
    }

    let ((rx_a, tx_a), (rx_b, tx_b)) = pty_pair().expect("create pty pair");
    let a_to_b = chan(1);
    let b_to_a = chan(2);

    // "Firmware" side A: sends readings, receives setpoints.
    let a_tx = UartTransportTx::new(a_to_b, tx_a, scratch(tx_buf_len(MTU)), MTU).expect("sized");
    let a_rx = UartTransportRx::new(b_to_a, rx_a, scratch(rx_buf_len(MTU)), MTU).expect("sized");
    let mut a_readings =
        Channel::<Tx, SensorReading, _, PostcardCodec>::new(a_to_b, a_tx, scratch(MTU));
    let mut a_setpoints =
        Channel::<Rx, Setpoint, _, PostcardCodec>::new(b_to_a, a_rx, scratch(MTU));

    // "Linux" side B: receives readings, sends setpoints.
    let b_tx = UartTransportTx::new(b_to_a, tx_b, scratch(tx_buf_len(MTU)), MTU).expect("sized");
    let b_rx = UartTransportRx::new(a_to_b, rx_b, scratch(rx_buf_len(MTU)), MTU).expect("sized");
    let mut b_readings =
        Channel::<Rx, SensorReading, _, PostcardCodec>::new(a_to_b, b_rx, scratch(MTU));
    let mut b_setpoints =
        Channel::<Tx, Setpoint, _, PostcardCodec>::new(b_to_a, b_tx, scratch(MTU));

    for sequence in 0..8u32 {
        let reading = SensorReading {
            sequence,
            millivolts: 3_300 + sequence,
        };
        within(a_readings.send(&reading))
            .await
            .expect("send reading");
        let got = within(b_readings.recv()).await.expect("recv reading");
        assert_eq!(got.into_inner(), reading);

        let setpoint = Setpoint {
            valve: (sequence % 4) as u8,
            percent: (sequence * 10) as u8,
        };
        within(b_setpoints.send(&setpoint))
            .await
            .expect("send setpoint");
        let got = within(a_setpoints.recv()).await.expect("recv setpoint");
        assert_eq!(got.into_inner(), setpoint);
    }
}
