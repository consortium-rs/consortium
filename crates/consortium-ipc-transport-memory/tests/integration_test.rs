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
use consortium_ipc::{Chan, Doorbell, Region, SramRegion, Transport};
use consortium_ipc_transport_memory::{
    descriptor::{
        ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, channel_table_entry_offset,
        descriptor_init,
    },
    error::Error,
    slot,
    transport::{SharedMemoryTransport, SharedMemoryTransportable},
};
use futures::{pin_mut, poll};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = 0x100;
const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;

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
