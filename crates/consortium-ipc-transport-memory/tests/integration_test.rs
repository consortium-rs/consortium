mod common;

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    task::Poll,
};

use common::doorbell::HostDoorbell;
use consortium_ipc::{Chan, Doorbell, RecvTransport, Region, SendTransport, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, Error, RX_UPDATED_ID_OFFSET, SharedMemoryTransport,
    SharedMemoryTransportRx, SharedMemoryTransportTx, SharedMemoryTransportable,
    channel_table_entry_offset, descriptor_init, slot,
};
use futures::{pin_mut, poll};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = 0x100;
const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;

// Paired-descriptor layout: two separate descriptor headers share two slots.
const DESCRIPTOR_LEN: usize = 0x300;
const A_DESCRIPTOR_OFFSET: usize = 0;
const M_DESCRIPTOR_OFFSET: usize = DESCRIPTOR_LEN;
const SHARED_A_TO_M_SLOT: usize = DESCRIPTOR_LEN * 2;
const SHARED_M_TO_A_SLOT: usize = SHARED_A_TO_M_SLOT + SLOT_SIZE;
const PAIRED_REGION_LEN: usize = SHARED_M_TO_A_SLOT + SLOT_SIZE;

fn channel(offset: usize, direction: ChannelDirection) -> ChannelLayout {
    let mut layout = ChannelLayout::default();
    layout.offset = offset as u32;
    layout.size = SLOT_SIZE as u32;
    layout.direction = direction;
    layout
}

fn init_two_channel_descriptor(base: *mut u8) {
    let a_to_m = channel(A_TO_M_SLOT, ChannelDirection::AToM);
    let m_to_a = channel(M_TO_A_SLOT, ChannelDirection::MToA);

    unsafe {
        descriptor_init(base, 2, MTU, &[a_to_m, m_to_a]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }
}

#[test]
fn shared_memory_transport_round_trips_over_heap_region_between_host_threads() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();

    init_two_channel_descriptor(base);

    let backing = Box::leak(backing);
    let base_addr = backing.as_mut_ptr() as usize;
    let (app_doorbell, remote_doorbell) = HostDoorbell::pair();
    let ch = unsafe { Chan::new_unchecked(0) };
    let (reply_written_tx, reply_written_rx) = mpsc::channel();

    let remote_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("create remote host runtime");

        rt.block_on(async move {
            remote_doorbell
                .wait(ch)
                .await
                .expect("remote waits for app doorbell");

            let base = base_addr as *mut u8;
            let app_to_remote =
                unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
            let remote_to_app =
                unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1))) };

            let mut rx = [0; MTU];
            let n = unsafe { slot::try_read(app_to_remote.slot_addr(base), &mut rx) }
                .expect("remote reads app payload");
            assert_eq!(&rx[..n], b"ping");

            unsafe {
                slot::write(remote_to_app.slot_addr(base), b"pong");
                base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
            }

            reply_written_tx
                .send(())
                .expect("signal reply is in shared memory");
            remote_doorbell.ring(ch).expect("ring app doorbell");
        });
    });

    let app_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("create app host runtime");

        rt.block_on(async move {
            let base = base_addr as *mut u8;
            let region = unsafe { SramRegion::new(base, REGION_LEN) };
            let mut transport =
                unsafe { SharedMemoryTransport::new(ch, app_doorbell, region, base) };

            transport.send(b"ping").await.expect("send app payload");

            reply_written_rx
                .recv()
                .expect("remote writes reply before app receive");

            let mut rx = [0; MTU];
            let n = transport.recv(&mut rx).await.expect("receive reply");
            assert_eq!(&rx[..n], b"pong");
        });
    });

    app_thread.join().expect("app thread completes");
    remote_thread.join().expect("remote thread completes");
}

#[test]
fn send_returns_no_channel_when_all_tx_slots_are_unconsumed() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let tx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0)));
        slot::write(tx.slot_addr(base), b"still-pending");
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let err = transport
            .send(b"new-payload")
            .await
            .expect_err("occupied tx slot should reject send");
        assert!(matches!(err, Error::NoChannel));
    });
}

#[test]
fn send_accepts_payload_at_channel_capacity() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);
    let payload = [0xA5; MTU];

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        transport
            .send(&payload)
            .await
            .expect("MTU-sized payload should fit");

        let tx = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut rx = [0; MTU];
        let n = unsafe { slot::try_read(tx.slot_addr(base), &mut rx) }
            .expect("remote reads MTU-sized payload");
        assert_eq!(n, MTU);
        assert_eq!(rx, payload);
    });
}

#[test]
fn send_rejects_payload_larger_than_channel_capacity() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);
    let payload = [0x5A; MTU + 1];

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let err = transport
            .send(&payload)
            .await
            .expect_err("oversized payload should be rejected");
        assert!(matches!(
            err,
            Error::PayloadTooLarge {
                payload_len,
                capacity: MTU
            } if payload_len == MTU + 1
        ));

        let tx = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        assert!(unsafe { slot::is_consumed(tx.slot_addr(base)) });
    });
}

#[test]
fn send_uses_later_tx_channel_when_first_tx_slot_is_pending() {
    const TX0_SLOT: usize = 0x100;
    const TX1_SLOT: usize = TX0_SLOT + SLOT_SIZE;
    const RX_SLOT: usize = TX1_SLOT + SLOT_SIZE;
    const MULTI_REGION_LEN: usize = RX_SLOT + SLOT_SIZE;

    let mut backing = vec![0; MULTI_REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    let channels = [
        channel(TX0_SLOT, ChannelDirection::AToM),
        channel(TX1_SLOT, ChannelDirection::AToM),
        channel(RX_SLOT, ChannelDirection::MToA),
    ];

    unsafe {
        descriptor_init(base, channels.len() as u8, MTU, &channels);
        let tx0 = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0)));
        slot::write(tx0.slot_addr(base), b"still-pending");
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, MULTI_REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        transport
            .send(b"second-slot")
            .await
            .expect("second tx slot should be selected");

        let tx1 = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1))) };
        let mut rx = [0; MTU];
        let n = unsafe { slot::try_read(tx1.slot_addr(base), &mut rx) }
            .expect("remote reads second tx slot");
        assert_eq!(&rx[..n], b"second-slot");
    });
}

#[test]
fn recv_prefers_updated_rx_channel_over_fallback_order() {
    const TX_SLOT: usize = 0x100;
    const RX0_SLOT: usize = TX_SLOT + SLOT_SIZE;
    const RX1_SLOT: usize = RX0_SLOT + SLOT_SIZE;
    const MULTI_REGION_LEN: usize = RX1_SLOT + SLOT_SIZE;

    let mut backing = vec![0; MULTI_REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    let channels = [
        channel(TX_SLOT, ChannelDirection::AToM),
        channel(RX0_SLOT, ChannelDirection::MToA),
        channel(RX1_SLOT, ChannelDirection::MToA),
    ];

    unsafe {
        descriptor_init(base, channels.len() as u8, MTU, &channels);
        let rx0 = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        let rx1 = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(2)));
        slot::write(rx0.slot_addr(base), b"fallback-first");
        slot::write(rx1.slot_addr(base), b"updated-id-wins");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(2);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, MULTI_REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let mut rx = [0; MTU];
        let n = transport.recv(&mut rx).await.expect("receive updated slot");
        assert_eq!(&rx[..n], b"updated-id-wins");

        let rx0_layout =
            unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1))) };
        let mut fallback = [0; MTU];
        let n = unsafe { slot::try_read(rx0_layout.slot_addr(base), &mut fallback) }
            .expect("fallback slot should still be unread");
        assert_eq!(&fallback[..n], b"fallback-first");
    });
}

#[test]
fn recv_falls_back_when_updated_id_is_wrong_direction() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), b"fallback");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let mut rx = [0; MTU];
        let n = transport
            .recv(&mut rx)
            .await
            .expect("receive fallback slot");
        assert_eq!(&rx[..n], b"fallback");
    });
}

#[test]
fn recv_falls_back_when_updated_id_is_out_of_range() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), b"out-of-range-fallback");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let mut rx = [0; MTU];
        let n = transport
            .recv(&mut rx)
            .await
            .expect("receive fallback slot");
        assert_eq!(&rx[..n], b"out-of-range-fallback");
    });
}

#[test]
fn recv_is_pending_when_no_rx_slot_contains_data() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut rx = [0; MTU];

        let recv = transport.recv(&mut rx);
        pin_mut!(recv);
        assert!(matches!(poll!(recv.as_mut()), Poll::Pending));
    });
}

#[test]
fn recv_future_becomes_ready_after_pending_poll_then_doorbell_ring() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (app_doorbell, remote_doorbell) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, app_doorbell, region, base) };
        let mut rx = [0; MTU];

        let recv = transport.recv(&mut rx);
        pin_mut!(recv);
        assert!(matches!(poll!(recv.as_mut()), Poll::Pending));

        unsafe {
            let rx_layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
            slot::write(rx_layout.slot_addr(base), b"woken");
            base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
        }
        remote_doorbell.ring(ch).expect("wake app doorbell");

        match poll!(recv.as_mut()) {
            Poll::Ready(Ok(n)) => assert_eq!(&rx[..n], b"woken"),
            other => panic!("recv future should be ready after doorbell ring: {other:?}"),
        }
    });
}

fn init_paired_descriptors(base: *mut u8) {
    let a_base = unsafe { base.add(A_DESCRIPTOR_OFFSET) };
    let m_base = unsafe { base.add(M_DESCRIPTOR_OFFSET) };

    let a_to_m_from_a = SHARED_A_TO_M_SLOT - A_DESCRIPTOR_OFFSET;
    let m_to_a_from_a = SHARED_M_TO_A_SLOT - A_DESCRIPTOR_OFFSET;
    let a_to_m_from_m = SHARED_A_TO_M_SLOT - M_DESCRIPTOR_OFFSET;
    let m_to_a_from_m = SHARED_M_TO_A_SLOT - M_DESCRIPTOR_OFFSET;

    unsafe {
        descriptor_init(
            a_base,
            2,
            MTU,
            &[
                channel(a_to_m_from_a, ChannelDirection::AToM),
                channel(m_to_a_from_a, ChannelDirection::MToA),
            ],
        );
        descriptor_init(
            m_base,
            2,
            MTU,
            &[
                channel(a_to_m_from_m, ChannelDirection::MToA),
                channel(m_to_a_from_m, ChannelDirection::AToM),
            ],
        );
        a_base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
        m_base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }
}

/// Demonstrate that `split()` produces two independent halves that can coexist
/// in the same scope.  With a combined [`SharedMemoryTransport`] the borrow
/// checker would not allow holding a send future and a receive future at the same
/// time; the split halves remove that restriction.
#[test]
fn split_into_halves_allows_tx_and_rx_to_coexist() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    // Pre-fill the rx slot so the recv half returns immediately.
    unsafe {
        let rx_layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx_layout.slot_addr(base), b"from-remote");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        // `split` consumes the combined transport and yields two owned halves.
        // Both live in this scope simultaneously — impossible with a single &mut transport.
        let (mut tx, mut rx): (
            SharedMemoryTransportTx<HostDoorbell, SramRegion>,
            SharedMemoryTransportRx<HostDoorbell, SramRegion>,
        ) = transport.split();

        tx.send(b"to-remote").await.expect("tx half sends");

        let mut buf = [0; MTU];
        let n = rx.recv(&mut buf).await.expect("rx half receives");
        assert_eq!(&buf[..n], b"from-remote");

        // Confirm the tx payload reached its slot.
        let tx_layout =
            unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut raw = [0; MTU];
        let n = unsafe { slot::try_read(tx_layout.slot_addr(base), &mut raw) }
            .expect("tx slot is readable");
        assert_eq!(&raw[..n], b"to-remote");
    });
}

#[derive(Clone)]
struct CountingRegion {
    ptr: *mut u8,
    len: usize,
    flushes: Arc<AtomicUsize>,
    invalidations: Arc<AtomicUsize>,
}

// SAFETY: tests use heap-backed shared memory and synchronize access through the futures under test.
unsafe impl Send for CountingRegion {}
// SAFETY: counters are atomic, and the raw memory aliasing discipline matches SramRegion test usage.
unsafe impl Sync for CountingRegion {}

impl CountingRegion {
    fn new(ptr: *mut u8, len: usize) -> Self {
        Self {
            ptr,
            len,
            flushes: Arc::new(AtomicUsize::new(0)),
            invalidations: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn flush_count(&self) -> usize {
        self.flushes.load(Ordering::SeqCst)
    }

    fn invalidate_count(&self) -> usize {
        self.invalidations.load(Ordering::SeqCst)
    }
}

impl Region for CountingRegion {
    unsafe fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    fn flush(&self) {
        self.flushes.fetch_add(1, Ordering::SeqCst);
    }

    fn invalidate(&self) {
        self.invalidations.fetch_add(1, Ordering::SeqCst);
    }

    fn is_coherent(&self) -> bool {
        false
    }
}

impl SharedMemoryTransportable for CountingRegion {}

#[test]
fn send_flushes_region_after_writing() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = CountingRegion::new(base, REGION_LEN);
        let observed_region = region.clone();
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        transport.send(b"flush-me").await.expect("send payload");

        assert_eq!(observed_region.flush_count(), 1);
        assert_eq!(observed_region.invalidate_count(), 0);
    });
}

#[test]
fn pending_recv_invalidates_region_once() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        let slot_base = rx.slot_addr(base);
        slot::write(slot_base, b"corrupted-before-read");
        *slot_base.add(slot::SLOT_HEADER) ^= 0xAA;
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = CountingRegion::new(base, REGION_LEN);
        let observed_region = region.clone();
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut rx = [0; MTU];

        let recv = transport.recv(&mut rx);
        pin_mut!(recv);
        assert!(matches!(poll!(recv.as_mut()), Poll::Pending));

        assert_eq!(observed_region.flush_count(), 0);
        assert_eq!(observed_region.invalidate_count(), 1);
    });
}

#[test]
fn immediate_recv_does_not_invalidate_region() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), b"already-there");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = CountingRegion::new(base, REGION_LEN);
        let observed_region = region.clone();
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let mut transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let mut rx = [0; MTU];
        let n = transport
            .recv(&mut rx)
            .await
            .expect("receive ready payload");
        assert_eq!(&rx[..n], b"already-there");

        assert_eq!(observed_region.flush_count(), 0);
        assert_eq!(observed_region.invalidate_count(), 0);
    });
}

/// Two-thread multidirectional demo: A and M each split their transport and use
/// `tokio::join!` to send and receive **at the same time** within a single thread.
///
/// This is the scenario that motivated the split design.  A combined
/// `&mut SharedMemoryTransport` cannot back two concurrent futures (`send` and `recv`
/// both need exclusive access), so without `split` you would need a sequential
/// send-then-recv loop or two separately constructed transports.  The split halves
/// each hold exclusive ownership of their direction, so both can be polled
/// concurrently with no conflict.
#[test]
fn split_halves_allow_full_duplex_between_two_threads() {
    let mut backing = vec![0u8; PAIRED_REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    init_paired_descriptors(base_addr as *mut u8);

    let (a_doorbell, m_doorbell) = HostDoorbell::pair();

    std::thread::scope(|scope| {
        // ── A-side thread ────────────────────────────────────────────────────
        let a_thread = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create A-side runtime");

            rt.block_on(async move {
                let a_base = (base_addr + A_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(a_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, a_doorbell, region, a_base) };

                let (mut tx, mut rx): (
                    SharedMemoryTransportTx<HostDoorbell, SramRegion>,
                    SharedMemoryTransportRx<HostDoorbell, SramRegion>,
                ) = transport.split();

                let mut buf = [0u8; MTU];
                eprintln!("[A] entering join");
                let (send_res, recv_res) =
                    futures::join!(tx.send(b"ping"), rx.recv(&mut buf));
                eprintln!("[A] join completed");

                send_res.expect("A sends ping");
                let n = recv_res.expect("A receives pong");
                assert_eq!(&buf[..n], b"pong");
                eprintln!("[A] done");
            });
        });

        // ── M-side thread ────────────────────────────────────────────────────
        let m_thread = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create M-side runtime");

            rt.block_on(async move {
                let m_base = (base_addr + M_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(m_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, m_doorbell, region, m_base) };

                let (mut tx, mut rx): (
                    SharedMemoryTransportTx<HostDoorbell, SramRegion>,
                    SharedMemoryTransportRx<HostDoorbell, SramRegion>,
                ) = transport.split();

                let mut buf = [0u8; MTU];
                eprintln!("[M] before join: about to futures::join!(rx.recv, tx.send(pong))");
                let (recv_res, send_res) =
                    futures::join!(rx.recv(&mut buf), tx.send(b"pong"));
                eprintln!("[M] join completed");

                let n = recv_res.expect("M receives ping");
                assert_eq!(&buf[..n], b"ping");
                send_res.expect("M sends pong");
                eprintln!("[M] done");
            });
        });

        a_thread.join().expect("A-side thread completes");
        m_thread.join().expect("M-side thread completes");
    });
}
