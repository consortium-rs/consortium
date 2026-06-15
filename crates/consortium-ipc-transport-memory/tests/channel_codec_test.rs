mod common;

use common::doorbell::HostDoorbell;
use consortium_codec::{CodecFor, PostcardCodec, RkyvCodec};
use consortium_ipc::{Chan, Channel, Rx, SramRegion, Tx};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport,
    channel_table_entry_offset, descriptor_init, slot,
};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = 0x100;
const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct PostcardMessage {
    sequence: u32,
    ready: bool,
}

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

fn scratch<const N: usize>() -> &'static mut [u8] {
    Box::leak(Box::new([0; N])).as_mut_slice()
}

#[test]
fn postcard_channel_sends_typed_message_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Tx, PostcardMessage, _, PostcardCodec>::new(ch, transport, scratch::<MTU>());

        channel
            .send(&PostcardMessage {
                sequence: 42,
                ready: true,
            })
            .await
            .expect("typed postcard send should succeed");

        let tx = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut raw = [0; MTU];
        let n = unsafe { slot::try_read(tx.slot_addr(base), &mut raw) }
            .expect("remote reads raw postcard payload");
        let decoded = <PostcardCodec as CodecFor<PostcardMessage>>::decode(&raw[..n])
            .expect("remote decodes postcard payload");

        assert_eq!(
            decoded,
            PostcardMessage {
                sequence: 42,
                ready: true
            }
        );
    });
}

#[test]
fn postcard_channel_receives_typed_message_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let mut encoded = [0; MTU];
    let n = PostcardCodec::encode(
        &PostcardMessage {
            sequence: 7,
            ready: false,
        },
        &mut encoded,
    )
    .expect("encode remote postcard payload");
    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), &encoded[..n]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Rx, PostcardMessage, _, PostcardCodec>::new(ch, transport, scratch::<MTU>());

        let decoded = channel
            .recv()
            .await
            .expect("typed postcard recv should succeed")
            .into_inner();

        assert_eq!(
            decoded,
            PostcardMessage {
                sequence: 7,
                ready: false
            }
        );
    });
}

#[test]
fn rkyv_channel_sends_archived_primitive_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Tx, u32, _, RkyvCodec<64>>::new(ch, transport, scratch::<MTU>());

        channel
            .send(&0xCAFE_BABEu32)
            .await
            .expect("typed rkyv send should succeed");

        let tx = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut raw = [0; MTU];
        let n = unsafe { slot::try_read(tx.slot_addr(base), &mut raw) }
            .expect("remote reads raw rkyv payload");
        let decoded =
            <RkyvCodec<64> as CodecFor<u32>>::decode(&raw[..n]).expect("remote decodes rkyv");

        assert_eq!(*decoded, 0xCAFE_BABEu32);
    });
}

#[test]
fn rkyv_channel_receives_archived_primitive_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let remote_value = 0x1234_5678u32;
    let mut encoded = [0; MTU];
    let n = RkyvCodec::<64>::encode(&remote_value, &mut encoded).expect("encode rkyv payload");
    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), &encoded[..n]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Rx, u32, _, RkyvCodec<64>>::new(ch, transport, scratch::<MTU>());

        let decoded = channel
            .recv()
            .await
            .expect("typed rkyv recv should succeed")
            .into_inner();

        assert_eq!(*decoded, remote_value);
    });
}
