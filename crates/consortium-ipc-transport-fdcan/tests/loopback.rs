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

//! End-to-end tests over a fake CAN bus.
//!
//! This one file runs under **both** feature sets:
//!
//! ```bash
//! cargo test -p consortium-ipc-transport-fdcan
//! cargo test -p consortium-ipc-transport-fdcan --no-default-features
//! ```
//!
//! That is the point, and the difference from the UART transport, which needs a
//! separate `loopback_embedded.rs` for its second impl family. Here the
//! `CanTx`/`CanRx` seam belongs to the crate, so one set of transport impls
//! serves firmware and host alike — and the `std` run additionally proves the
//! futures are `Send`, since `MaybeSend` requires it there.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use consortium_codec::PostcardCodec;
use consortium_ipc::{Chan, Channel, Connect, RecvTransport, Rx, SendTransport, Tx};
use consortium_ipc_transport_fdcan::embedded_can::{Frame as _, Id, StandardId};
use consortium_ipc_transport_fdcan::{
    CanErrorType, CanRx, CanTx, Error, FdcanTransport, FdcanTransportRx, FdcanTransportTx,
    rx_buf_len,
};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};

const MTU: usize = 128;
/// Identifier node A transmits under; B listens here.
const ID_A: u16 = 0x101;
/// Identifier node B transmits under; A listens here.
const ID_B: u16 = 0x102;
/// An identifier belonging to unrelated traffic on the same bus.
const ID_OTHER: u16 = 0x200;

// --- fake bus ---------------------------------------------------------------

/// A classic CAN frame. Implementing `embedded_can::Frame` exercises the
/// blanket `CanFrame` impl, which is how the real chip HAL frames arrive too.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestFrame {
    id: Id,
    remote: bool,
    data: Vec<u8>,
}

impl consortium_ipc_transport_fdcan::embedded_can::Frame for TestFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        (data.len() <= 8).then(|| Self {
            id: id.into(),
            remote: false,
            data: data.to_vec(),
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        (dlc <= 8).then(|| Self {
            id: id.into(),
            remote: true,
            data: Vec::new(),
        })
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        self.remote
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

/// The fake bus reports this instead of parking forever when a test reads past
/// the last frame. A real controller would simply stay quiet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Drained;

/// One direction of a fake bus: a frame queue shared between a transmit end and
/// a receive end.
#[derive(Clone, Default)]
struct Wire(Arc<Mutex<VecDeque<TestFrame>>>);

impl Wire {
    fn frames(&self) -> Vec<TestFrame> {
        self.0.lock().unwrap().iter().cloned().collect()
    }

    /// Inject a frame directly, standing in for another node on the bus.
    fn inject(&self, frame: TestFrame) {
        self.0.lock().unwrap().push_back(frame);
    }

    /// Drop the frame at `index`, as a FIFO overrun or a bus error would.
    fn drop_frame(&self, index: usize) {
        self.0.lock().unwrap().remove(index).expect("frame exists");
    }
}

impl CanErrorType for Wire {
    type Error = Drained;
}

impl CanTx for Wire {
    type Frame = TestFrame;

    async fn transmit(&mut self, frame: &TestFrame) -> Result<(), Drained> {
        self.0.lock().unwrap().push_back(frame.clone());
        Ok(())
    }
}

impl CanRx for Wire {
    type Frame = TestFrame;

    async fn receive(&mut self) -> Result<TestFrame, Drained> {
        self.0.lock().unwrap().pop_front().ok_or(Drained)
    }
}

// --- helpers ----------------------------------------------------------------

fn scratch(n: usize) -> &'static mut [u8] {
    Box::leak(vec![0u8; n].into_boxed_slice())
}

fn chan(id: u8) -> Chan {
    // SAFETY: test-local logical channel byte, used only for bookkeeping; the
    // FDCAN transport puts the CAN identifier on the wire, not this.
    unsafe { Chan::new_unchecked(id) }
}

fn id(raw: u16) -> Id {
    Id::Standard(StandardId::new(raw).expect("11-bit identifier"))
}

/// Two endpoints on one bus, each transmitting under its own identifier and
/// listening for the other's. Returns the wires so tests can tamper with them.
fn endpoints(
    ch: Chan,
) -> (
    FdcanTransport<Wire, Wire>,
    FdcanTransport<Wire, Wire>,
    (Wire, Wire),
) {
    let a_to_b = Wire::default();
    let b_to_a = Wire::default();
    let a = FdcanTransport::new(
        ch,
        id(ID_A),
        id(ID_B),
        a_to_b.clone(),
        b_to_a.clone(),
        scratch(rx_buf_len(MTU)),
        MTU,
    )
    .expect("buffer sized by rx_buf_len");
    let b = FdcanTransport::new(
        ch,
        id(ID_B),
        id(ID_A),
        b_to_a.clone(),
        a_to_b.clone(),
        scratch(rx_buf_len(MTU)),
        MTU,
    )
    .expect("buffer sized by rx_buf_len");
    (a, b, (a_to_b, b_to_a))
}

/// A payload whose bytes depend on their index, so a misordered or truncated
/// reassembly is visible.
fn ramp(len: usize, start: u8) -> Vec<u8> {
    (0..len).map(|i| start.wrapping_add(i as u8)).collect()
}

/// Drive any transport through the uniform bring-up call, the same helper shape
/// the shared-memory and UART tests use.
async fn bring_up<T: Connect<Params = ()>>(t: &mut T) -> Result<(), T::Error> {
    t.connect(()).await
}

// --- tests ------------------------------------------------------------------

#[test]
fn connect_is_an_immediate_no_op() {
    let (mut a, mut b, _) = endpoints(chan(1));
    block_on(async {
        // Neither side publishes anything, so both resolve with no peer action.
        bring_up(&mut a).await.expect("connect a");
        bring_up(&mut b).await.expect("connect b");
    });
}

#[test]
fn single_frame_message_round_trips() {
    let (mut a, mut b, (wire, _)) = endpoints(chan(1));
    block_on(async {
        a.send(b"ping").await.expect("send a->b");
        // Short enough to ride in one frame, under A's identifier.
        let frames = wire.frames();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].id(), id(ID_A));

        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(&buf[..n], b"ping");
    });
}

#[test]
fn multi_frame_message_round_trips_both_directions() {
    let (mut a, mut b, _) = endpoints(chan(1));
    block_on(async {
        let out = ramp(100, 0);
        a.send(&out).await.expect("send a->b");
        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(&buf[..n], &out[..]);

        let back = ramp(80, 200);
        b.send(&back).await.expect("send b->a");
        let n = a.recv(&mut buf).await.expect("recv on a");
        assert_eq!(&buf[..n], &back[..]);
    });
}

#[test]
fn full_mtu_message_round_trips() {
    let (mut a, mut b, _) = endpoints(chan(1));
    block_on(async {
        let out = ramp(MTU, 7);
        a.send(&out).await.expect("send a->b");
        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(n, MTU);
        assert_eq!(&buf[..n], &out[..]);
    });
}

#[test]
fn back_to_back_messages_arrive_in_order() {
    let (mut a, mut b, _) = endpoints(chan(1));
    block_on(async {
        // A mix of single-frame, multi-frame and empty messages queued up
        // before the receiver runs, as a burst on a real bus would be.
        let sent = [ramp(3, 0), ramp(40, 10), Vec::new(), ramp(12, 60)];
        for msg in &sent {
            a.send(msg).await.expect("send");
        }
        let mut buf = [0u8; MTU];
        for msg in &sent {
            let n = b.recv(&mut buf).await.expect("recv");
            assert_eq!(&buf[..n], &msg[..]);
        }
    });
}

#[test]
fn foreign_identifiers_are_dropped() {
    let (mut a, mut b, (a_to_b, _)) = endpoints(chan(1));
    block_on(async {
        // Unrelated traffic from a third node, before and after our message.
        a_to_b.inject(TestFrame::new(id(ID_OTHER), b"noise").expect("frame"));
        a.send(b"real").await.expect("send a->b");
        a_to_b.inject(TestFrame::new(id(ID_OTHER), b"more").expect("frame"));

        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(&buf[..n], b"real");
    });
}

#[test]
fn remote_frames_are_dropped() {
    let (mut a, mut b, (a_to_b, _)) = endpoints(chan(1));
    block_on(async {
        // A remote-transmission request carrying our identifier but no payload.
        a_to_b.inject(TestFrame::new_remote(id(ID_A), 4).expect("frame"));
        a.send(b"real").await.expect("send a->b");

        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(&buf[..n], b"real");
    });
}

#[test]
fn a_lost_frame_costs_one_message_not_the_endpoint() {
    let (mut a, mut b, (a_to_b, _)) = endpoints(chan(1));
    block_on(async {
        // Send a segmented message, then lose one of its continuations.
        a.send(&ramp(40, 0)).await.expect("send doomed message");
        a_to_b.drop_frame(2);
        // A second message follows on the same bus and must still arrive.
        a.send(b"after").await.expect("send follow-up");

        let mut buf = [0u8; MTU];
        let n = b.recv(&mut buf).await.expect("recv on b");
        assert_eq!(
            &buf[..n],
            b"after",
            "receiver should resynchronize on the next message"
        );
    });
}

#[test]
fn payload_over_mtu_is_rejected() {
    let (mut a, _b, _) = endpoints(chan(1));
    block_on(async {
        let err = a.send(&ramp(MTU + 1, 0)).await.expect_err("over mtu");
        assert_eq!(
            err,
            Error::PayloadTooLarge {
                payload_len: MTU + 1,
                mtu: MTU
            }
        );
    });
}

#[test]
fn undersized_recv_buffer_is_rejected() {
    let (_a, mut b, _) = endpoints(chan(1));
    block_on(async {
        let mut buf = [0u8; MTU - 1];
        let err = b.recv(&mut buf).await.expect_err("undersized buffer");
        assert_eq!(
            err,
            Error::BufferTooSmall {
                needed: MTU,
                got: MTU - 1
            }
        );
    });
}

#[test]
fn undersized_reassembly_buffer_is_rejected_at_construction() {
    // The transports hold a controller, which need not be `Debug`, so match
    // rather than `expect_err`.
    let Err(err) = FdcanTransportRx::new(chan(1), id(ID_A), Wire::default(), scratch(MTU - 1), MTU)
    else {
        panic!("buffer smaller than rx_buf_len should be rejected");
    };
    assert_eq!(
        err,
        Error::BufferTooSmall {
            needed: MTU,
            got: MTU - 1
        }
    );
}

#[test]
fn mtu_beyond_the_length_field_is_rejected() {
    let too_big = u16::MAX as usize + 1;
    let Err(err) = FdcanTransportTx::new(chan(1), id(ID_A), Wire::default(), too_big) else {
        panic!("an mtu past the u16 total-length field should be rejected");
    };
    assert!(matches!(err, Error::PayloadTooLarge { .. }));
}

#[test]
fn split_halves_feed_typed_channels() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Command {
        motor: u8,
        rpm: u16,
        waypoints: [u16; 16],
    }

    let (a, b, _) = endpoints(chan(2));
    let (a_tx, _a_rx) = a.split();
    let (_b_tx, b_rx) = b.split();

    let mut tx = Channel::<Tx, Command, _, PostcardCodec>::new(chan(2), a_tx, scratch(MTU));
    let mut rx = Channel::<Rx, Command, _, PostcardCodec>::new(chan(2), b_rx, scratch(MTU));

    block_on(async {
        // Long enough that the encoded message must be segmented.
        let msg = Command {
            motor: 2,
            rpm: 950,
            waypoints: [
                10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160,
            ],
        };
        tx.send(&msg).await.expect("typed send");
        let got = rx.recv().await.expect("typed recv").into_inner();
        assert_eq!(got, msg);
    });
}
