use std::task::Poll;

use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{
    CountingRegion, MTU, REGION_LEN, SLOT_SIZE, channel, init_descriptor_with_channels,
    init_two_channel_descriptor,
};
use consortium_ipc::{Chan, Doorbell, Primary, RecvTransport};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport,
    channel_table_entry_offset, slot,
};
use consortium_shared_memory::SramRegion;
use futures::{pin_mut, poll};

#[test]
fn recv_prefers_updated_rx_channel_over_fallback_order() {
    const TX_SLOT: usize = 0x100;
    const RX0_SLOT: usize = TX_SLOT + SLOT_SIZE;
    const RX1_SLOT: usize = RX0_SLOT + SLOT_SIZE;
    const MULTI_REGION_LEN: usize = RX1_SLOT + SLOT_SIZE;

    let mut backing = vec![0; MULTI_REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    let channels = [
        channel(TX_SLOT, ChannelDirection::PrimaryToSecondary),
        channel(RX0_SLOT, ChannelDirection::SecondaryToPrimary),
        channel(RX1_SLOT, ChannelDirection::SecondaryToPrimary),
    ];

    init_descriptor_with_channels(base, MTU, &channels);
    unsafe {
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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };

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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };

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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };

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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };
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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, app_doorbell, region, base)
        };
        let mut rx = [0; MTU];

        let recv = transport.recv(&mut rx);
        pin_mut!(recv);
        assert!(matches!(poll!(recv.as_mut()), Poll::Pending));

        unsafe {
            let rx_layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
            slot::write(rx_layout.slot_addr(base), b"woken");
            base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
        }
        remote_doorbell.ring(ch2).expect("wake app doorbell");

        match poll!(recv.as_mut()) {
            Poll::Ready(Ok(n)) => assert_eq!(&rx[..n], b"woken"),
            other => panic!("recv future should be ready after doorbell ring: {other:?}"),
        }
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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };
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
        let ch1 = unsafe { Chan::new_unchecked(0) };
        let ch2 = unsafe { Chan::new_unchecked(1) };
        let mut transport = unsafe {
            SharedMemoryTransport::<Primary, _, _>::new(ch1, ch2, doorbell, region, base)
        };

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
