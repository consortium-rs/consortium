use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{
    CountingRegion, MTU, REGION_LEN, SLOT_SIZE, channel, init_two_channel_descriptor,
};
use consortium_ipc::{Chan, SendTransport, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, Error, SharedMemoryTransport, channel_table_entry_offset,
    descriptor_init, slot,
};

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
