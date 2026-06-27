use crate::codec_support::{
    MTU, Message, REGION_LEN, block_on_current_thread, init_two_channel_descriptor, scratch,
};
use crate::common::doorbell::HostDoorbell;
use consortium_codec::{CodecFor, RkyvCodec};
use consortium_ipc::{Chan, Channel, Rx, Tx};
use consortium_ipc_transport_memory::{
    ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport, channel_table_entry_offset, slot,
};
use consortium_shared_memory::SramRegion;

#[test]
fn rkyv_channel_sends_archived_message_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    block_on_current_thread(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Tx, Message, _, RkyvCodec<128>>::new(ch, transport, scratch::<MTU>());

        channel
            .send(&Message {
                sequence: 0xCAFE_BABE,
                ready: true,
            })
            .await
            .expect("typed rkyv send should succeed");

        let tx = unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut raw = [0; MTU];
        let n = unsafe { slot::try_read(tx.slot_addr(base), &mut raw) }
            .expect("remote reads raw rkyv payload");
        let decoded =
            <RkyvCodec<128> as CodecFor<Message>>::decode(&raw[..n]).expect("remote decodes rkyv");

        assert_eq!(decoded.sequence, 0xCAFE_BABE);
        assert!(decoded.ready);
    });
}

#[test]
fn rkyv_channel_receives_archived_message_over_shared_memory_transport() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let remote_value = Message {
        sequence: 0x1234_5678,
        ready: false,
    };
    let mut encoded = [0; MTU];
    let n = RkyvCodec::<128>::encode(&remote_value, &mut encoded).expect("encode rkyv payload");
    unsafe {
        let rx = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx.slot_addr(base), &encoded[..n]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    block_on_current_thread(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };
        let mut channel =
            Channel::<Rx, Message, _, RkyvCodec<128>>::new(ch, transport, scratch::<MTU>());

        let decoded = channel
            .recv()
            .await
            .expect("typed rkyv recv should succeed")
            .into_inner();

        assert_eq!(decoded.sequence, remote_value.sequence);
        assert_eq!(decoded.ready, remote_value.ready);
    });
}
