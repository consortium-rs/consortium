//! Acknowledged sends: `send_acked` resolves only once the peer has drained the slot.
//!
//! Both cores run over one shared descriptor and slot region, as in `end_to_end`, and both
//! drive real transports through the real `Connect` handshake — the acknowledgement
//! capability is negotiated there, so bypassing it would test a transport that could never
//! exist on hardware.
use std::time::Duration;

use consortium_ipc::{AckSendTransport, Chan, Connect, RecvTransport, SendTransport};
use consortium_ipc_transport_memory::{
    DescriptorLayout, Error, Primary, Secondary, SharedMemoryTransport, required_region_len, slot,
};
use consortium_shared_memory::UncachedRegion;

use crate::common::doorbell::LinedDoorbell;
use crate::transport_support::MTU;

/// Data lines are 0 (primary→secondary) and 1 (secondary→primary); acknowledgement lines
/// are 2 (rung by the primary) and 3 (rung by the secondary).
const DATA_P2S: u8 = 0;
const DATA_S2P: u8 = 1;
const ACK_BY_P: u8 = 2;
const ACK_BY_S: u8 = 3;

const REGION_LEN: usize = required_region_len(2, MTU);

/// Long enough that a healthy run never reaches it, short enough that a genuine hang fails
/// the test instead of wedging the suite.
const PATIENCE: Duration = Duration::from_secs(5);

fn ch(id: u8) -> Chan {
    // SAFETY: ids are within the 4-line bus these tests construct.
    unsafe { Chan::new_unchecked(id) }
}

type Transport<S> = SharedMemoryTransport<S, LinedDoorbell, UncachedRegion>;

/// Build the primary's transport with acknowledgement lines wired.
///
/// # Safety
/// `base` must point to a `REGION_LEN` region shared with the secondary and nothing else.
unsafe fn primary(base: *mut u8, doorbell: LinedDoorbell) -> Transport<Primary> {
    // SAFETY: forwarded to the caller.
    unsafe {
        let region = UncachedRegion::new(base, REGION_LEN);
        SharedMemoryTransport::<Primary, _, _>::new_acked(
            ch(DATA_P2S),
            ch(DATA_S2P),
            ch(ACK_BY_P),
            ch(ACK_BY_S),
            doorbell,
            region,
            base,
        )
    }
}

/// Build the secondary's transport with acknowledgement lines wired.
///
/// # Safety
/// Same contract as [`primary`].
unsafe fn secondary(base: *mut u8, doorbell: LinedDoorbell) -> Transport<Secondary> {
    // SAFETY: forwarded to the caller.
    unsafe {
        let region = UncachedRegion::new(base, REGION_LEN);
        SharedMemoryTransport::<Secondary, _, _>::new_acked(
            ch(DATA_S2P),
            ch(DATA_P2S),
            ch(ACK_BY_S),
            ch(ACK_BY_P),
            doorbell,
            region,
            base,
        )
    }
}

fn acked_layout() -> DescriptorLayout {
    DescriptorLayout::new(2, MTU).with_ack(true)
}

/// Run one closure per core on its own current-thread runtime, failing rather than hanging.
fn two_cores<P, S>(primary_side: P, secondary_side: S)
where
    P: FnOnce(*mut u8) -> std::pin::Pin<Box<dyn Future<Output = ()>>> + Send,
    S: FnOnce(*mut u8) -> std::pin::Pin<Box<dyn Future<Output = ()>>> + Send,
{
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;

    std::thread::scope(|scope| {
        let p = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .expect("primary runtime");
            rt.block_on(async move {
                tokio::time::timeout(PATIENCE, primary_side(base_addr as *mut u8))
                    .await
                    .expect("primary side should not stall");
            });
        });
        let s = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .expect("secondary runtime");
            rt.block_on(async move {
                tokio::time::timeout(PATIENCE, secondary_side(base_addr as *mut u8))
                    .await
                    .expect("secondary side should not stall");
            });
        });
        p.join().expect("primary thread");
        s.join().expect("secondary thread");
    });
}

#[test]
fn send_acked_resolves_only_after_the_peer_consumes() {
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus) = (bus.clone(), bus.clone());

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe { primary(base, p_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("primary connect");

                transport
                    .send_acked(b"confirm-me")
                    .await
                    .expect("acked send");

                // Resolving means the peer drained the slot: the slot is reusable and the
                // acknowledgement line was rung exactly once for this one message.
                assert_eq!(p_bus_rings(&transport), 1);
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe { secondary(base, s_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("secondary connect");

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("secondary receives");
                assert_eq!(&buf[..n], b"confirm-me");
            })
        },
    );

    fn p_bus_rings<S: consortium_ipc::Side>(t: &Transport<S>) -> usize {
        t.doorbell().rings(ch(ACK_BY_S))
    }
}

#[test]
fn send_acked_waits_for_a_slot_the_peer_still_holds() {
    // Two acknowledged sends back to back over a depth-one channel.  The second cannot
    // start until the first is drained, and it must wait rather than report `NoChannel` —
    // the first message asked to be acknowledged, so an edge is guaranteed to arrive.
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus) = (bus.clone(), bus.clone());

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe { primary(base, p_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("primary connect");

                transport
                    .send_acked(b"first")
                    .await
                    .expect("first acked send");
                transport
                    .send_acked(b"second")
                    .await
                    .expect("second acked send");

                // Two messages asked to be confirmed, so the peer rang twice — the second
                // send could only proceed because the first ring freed the slot.
                assert_eq!(transport.doorbell().rings(ch(ACK_BY_S)), 2);
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe { secondary(base, s_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("secondary connect");

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("first");
                assert_eq!(&buf[..n], b"first");
                let n = transport.recv(&mut buf).await.expect("second");
                assert_eq!(&buf[..n], b"second");
            })
        },
    );
}

#[test]
fn plain_send_never_rings_the_acknowledgement_line() {
    // The payoff of putting the request in the slot rather than the channel: a capable
    // channel taxes only the messages that actually ask to be confirmed, so a
    // fire-and-forget stream over the same channel pays nothing for the capability.
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus, probe) = (bus.clone(), bus.clone(), bus.clone());

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe { primary(base, p_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("primary connect");

                // Depth is one frame, so pace the plain sends against the peer's drain the
                // way an application without acknowledgement already has to.
                for message in [b"one".as_slice(), b"two", b"three"] {
                    while let Err(error) = transport.send(message).await {
                        assert!(matches!(error, Error::NoChannel));
                        tokio::task::yield_now().await;
                    }
                }
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe { secondary(base, s_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("secondary connect");

                let mut buf = [0u8; MTU];
                for expected in [b"one".as_slice(), b"two", b"three"] {
                    let n = transport.recv(&mut buf).await.expect("receives");
                    assert_eq!(&buf[..n], expected);
                }
            })
        },
    );

    assert_eq!(
        probe.rings(ch(ACK_BY_S)),
        0,
        "a capable channel must stay silent for messages that did not ask to be confirmed"
    );
    // The control: the data line did carry traffic, so the silent acknowledgement line
    // above is a real observation rather than a doorbell that never fired.  One ring per
    // message, plus the one the primary's handshake sends to publish the descriptor.
    assert_eq!(probe.rings(ch(DATA_P2S)), 1 + 3);
}

#[test]
fn send_acked_reports_rather_than_stalling_behind_a_plain_send() {
    // The one sharp edge of per-message acknowledgement, pinned deliberately.  A plain
    // `send` frees its slot silently, so while one is in flight there is no edge to park
    // on — `send_acked` must report `NoChannel` exactly as `send` would, and the caller
    // retries.  Parking here would be a hang.
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus) = (bus.clone(), bus.clone());
    // The peer runs on its own thread, so without this it can drain the plain message
    // before the primary probes, and the probe then rightly succeeds.  Hold the peer back
    // until the probe has seen the in-flight slot.
    let (probed_tx, probed_rx) = tokio::sync::oneshot::channel::<()>();

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe { primary(base, p_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("primary connect");

                transport.send(b"unconfirmed").await.expect("plain send");

                // The peer has not drained it yet, and nothing will announce when it does.
                let error = transport
                    .send_acked(b"confirmed")
                    .await
                    .expect_err("must not park behind an unacknowledged message");
                assert!(matches!(error, Error::NoChannel));
                probed_tx.send(()).expect("secondary still running");

                // Retrying is the caller's job, and it succeeds once the slot frees.
                loop {
                    match transport.send_acked(b"confirmed").await {
                        Ok(()) => break,
                        Err(Error::NoChannel) => tokio::task::yield_now().await,
                        Err(other) => panic!("unexpected error: {other:?}"),
                    }
                }
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe { secondary(base, s_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("secondary connect");

                // If the primary panicked before probing, its failure is the one to report.
                let _ = probed_rx.await;
                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("plain message");
                assert_eq!(&buf[..n], b"unconfirmed");
                let n = transport.recv(&mut buf).await.expect("acked message");
                assert_eq!(&buf[..n], b"confirmed");
            })
        },
    );
}

#[test]
fn send_acked_reports_a_payload_the_peer_rejected() {
    // A CRC failure frees the slot without delivering anything.  The receiver still has to
    // ring, or the sender waits forever for an edge nobody will produce — and the sender
    // has to learn that the message did not land, which is the thing an acknowledgement
    // buys over a plain send.
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus) = (bus.clone(), bus.clone());

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe { primary(base, p_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("primary connect");

                let error = transport
                    .send_acked(b"corrupted-in-flight")
                    .await
                    .expect_err("a rejected payload must surface");
                assert!(matches!(error, Error::PeerIntegrityError));
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe { secondary(base, s_bus) };
                transport
                    .connect(acked_layout())
                    .await
                    .expect("secondary connect");

                // Corrupt the payload after the primary published it but before we read,
                // standing in for a truncated DMA or a mis-mapped window.
                let slot_base = wait_for_valid_slot(base).await;
                // SAFETY: the slot payload starts after the header and is at least a byte.
                unsafe { slot_base.add(slot::SLOT_HEADER).write_volatile(0x00) };

                let mut buf = [0u8; MTU];
                let recv =
                    tokio::time::timeout(Duration::from_millis(200), transport.recv(&mut buf));
                assert!(
                    recv.await.is_err(),
                    "a corrupt frame must not be delivered upwards"
                );
            })
        },
    );
}

/// Spin until the primary publishes its slot, so the corruption lands on a real frame.
async fn wait_for_valid_slot(base: *mut u8) -> *mut u8 {
    let addr = base as usize;
    loop {
        // SAFETY: the descriptor is published by the handshake both sides just completed,
        // and channel 0 is the primary→secondary slot.
        let slot_base = unsafe {
            let base = addr as *mut u8;
            base.add(consortium_ipc_transport_memory::slot_offset(0, MTU))
        };
        // SAFETY: `slot_base` addresses a valid slot in the shared region.
        if unsafe { slot::flags(slot_base) } & slot::FLAG_VALID != 0 {
            return slot_base;
        }
        tokio::task::yield_now().await;
    }
}

#[test]
fn send_acked_fails_fast_when_the_link_has_no_acknowledgement() {
    // Failing is required; hanging is not acceptable.  A channel built without
    // acknowledgement lines has nothing to park on, so `send_acked` must say so rather
    // than wait on a line nobody rings.
    let bus = LinedDoorbell::bus(4);
    let (p_bus, s_bus) = (bus.clone(), bus.clone());

    two_cores(
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the secondary and with nothing else.
                let mut transport = unsafe {
                    let region = UncachedRegion::new(base, REGION_LEN);
                    SharedMemoryTransport::<Primary, _, _>::new(
                        ch(DATA_P2S),
                        ch(DATA_S2P),
                        p_bus,
                        region,
                        base,
                    )
                };
                transport
                    .connect(DescriptorLayout::new(2, MTU))
                    .await
                    .expect("primary connect");

                let error = transport
                    .send_acked(b"nowhere-to-wait")
                    .await
                    .expect_err("an unacknowledged link must refuse");
                assert!(matches!(error, Error::AckNotEnabled));

                // ...and the ordinary path is untouched by the refusal.
                transport.send(b"still-fine").await.expect("plain send");
            })
        },
        move |base| {
            Box::pin(async move {
                // SAFETY: the region is shared with the primary and with nothing else.
                let mut transport = unsafe {
                    let region = UncachedRegion::new(base, REGION_LEN);
                    SharedMemoryTransport::<Secondary, _, _>::new(
                        ch(DATA_S2P),
                        ch(DATA_P2S),
                        s_bus,
                        region,
                        base,
                    )
                };
                transport
                    .connect(DescriptorLayout::new(2, MTU))
                    .await
                    .expect("secondary connect");

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("receives");
                assert_eq!(&buf[..n], b"still-fine");
            })
        },
    );
}

#[test]
fn plain_send_still_reports_a_full_slot_rather_than_waiting() {
    // The regression guard for "purely additive": wiring acknowledgement lines must not
    // change what `send` does when the peer has not caught up.
    let bus = LinedDoorbell::bus(4);
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime");
    rt.block_on(async {
        // No peer here: the descriptor is laid down directly so the test can fill the slot
        // and observe what happens next, without a secondary to drain it.
        crate::transport_support::init_two_channel_descriptor(base);
        // SAFETY: this test owns the region outright; nothing else maps it.
        let mut transport = unsafe { primary(base, bus) };

        transport.send(b"first").await.expect("first send");
        let error = transport
            .send(b"second")
            .await
            .expect_err("depth is one frame per direction");
        assert!(matches!(error, Error::NoChannel));
    });
}
