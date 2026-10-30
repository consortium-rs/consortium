use std::sync::{Arc, Barrier};
use std::thread;

use crate::common::doorbell::HostDoorbell;
use consortium_codec::CodecFor;
use consortium_ipc::{Chan, Channel, Rx, Tx};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, DescState, MAGIC, Primary, RX_UPDATED_ID_OFFSET, Secondary,
    SharedMemoryTransport, VERSION, channel_table_entry_offset, slot, slot_offset, slot_stride,
};
use consortium_shared_memory::UncachedRegion;
use consortium_shared_memory::descriptor::DescriptorHeader;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{self, DecodeContext, WireType};
use serde::{Deserialize, Serialize};

pub const MTU: usize = 128;
pub const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
pub const A_TO_M_SLOT: usize = slot_offset(0, MTU);
pub const M_TO_A_SLOT: usize = slot_offset(1, MTU);
pub const REGION_LEN: usize = M_TO_A_SLOT + slot_stride(MTU);

#[derive(
    Clone, Debug, Default, Deserialize, PartialEq, rkyv::Archive, rkyv::Serialize, Serialize,
)]
pub struct Message {
    pub sequence: u32,
    pub ready: bool,
}

impl prost::Message for Message {
    fn encode_raw(&self, buf: &mut impl BufMut)
    where
        Self: Sized,
    {
        encoding::uint32::encode(1, &self.sequence, buf);
        encoding::bool::encode(2, &self.ready, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        match tag {
            1 => encoding::uint32::merge(wire_type, &mut self.sequence, buf, ctx),
            2 => encoding::bool::merge(wire_type, &mut self.ready, buf, ctx),
            _ => encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        encoding::uint32::encoded_len(1, &self.sequence)
            + encoding::bool::encoded_len(2, &self.ready)
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

pub fn channel(offset: usize, direction: ChannelDirection) -> ChannelLayout {
    let mut layout = ChannelLayout::default();
    layout.offset = offset as u32;
    layout.size = SLOT_SIZE as u32;
    layout.direction = direction;
    layout
}

pub fn init_two_channel_descriptor(base: *mut u8) {
    let a_to_m = channel(A_TO_M_SLOT, ChannelDirection::PrimaryToSecondary);
    let m_to_a = channel(M_TO_A_SLOT, ChannelDirection::SecondaryToPrimary);

    unsafe {
        DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(base);
        base.add(0x06).write_volatile(2);
        (base.add(0x10) as *mut u32).write_volatile(MTU as u32);
        (base.add(0x18) as *mut u32).write_volatile(DescState::Ready as u32);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);

        for (index, channel) in [a_to_m, m_to_a].iter().enumerate() {
            let entry = base.add(channel_table_entry_offset(index));
            (entry as *mut u32).write_volatile(channel.offset);
            (entry.add(4) as *mut u32).write_volatile(channel.size);
            entry.add(8).write_volatile(channel.direction as u8);
        }
    }
}

pub fn scratch<const N: usize>() -> &'static mut [u8] {
    Box::leak(Box::new([0; N])).as_mut_slice()
}

pub fn block_on_current_thread(fut: impl Future<Output = ()>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");
    rt.block_on(fut);
}

pub fn round_trip_between_threads<C>(a_to_m: Message, m_to_a: Message)
where
    C: CodecFor<Message> + Send + 'static,
    for<'buf> C::Decoded<'buf>: MessageView + Send,
{
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    init_two_channel_descriptor(base_addr as *mut u8);
    let (a_doorbell, m_doorbell) = HostDoorbell::pair();
    let a_expected = a_to_m.clone();
    let m_expected = m_to_a.clone();
    let a_sent = Arc::new(Barrier::new(2));
    let m_sent = Arc::new(Barrier::new(2));

    thread::scope(|scope| {
        let a_sent_for_a = a_sent.clone();
        let m_sent_for_a = m_sent.clone();
        let a_thread = scope.spawn(move || {
            block_on_current_thread(async move {
                let base = base_addr as *mut u8;
                let ch_tx = unsafe { Chan::new_unchecked(0) };
                let ch_rx = unsafe { Chan::new_unchecked(1) };
                let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
                let transport = unsafe {
                    SharedMemoryTransport::<Primary, _, _>::new(ch_tx, ch_rx, a_doorbell, region)
                };
                let (tx_half, rx_half) = transport.split();
                let mut tx = Channel::<Tx, Message, _, C>::new(ch_tx, tx_half, scratch::<MTU>());
                let mut rx = Channel::<Rx, Message, _, C>::new(ch_rx, rx_half, scratch::<MTU>());

                tx.send(&a_to_m)
                    .await
                    .expect("A-side channel send should succeed");

                a_sent_for_a.wait();
                m_sent_for_a.wait();

                let decoded = rx
                    .recv()
                    .await
                    .expect("A-side channel recv should succeed")
                    .into_inner();

                assert_message_eq(&decoded, &m_expected);
            });
        });

        let a_sent_for_m = a_sent;
        let m_sent_for_m = m_sent;
        let m_thread = scope.spawn(move || {
            block_on_current_thread(async move {
                let base = base_addr as *mut u8;
                let ch_rx = unsafe { Chan::new_unchecked(0) };
                let ch_tx = unsafe { Chan::new_unchecked(1) };
                let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
                let transport = unsafe {
                    SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, m_doorbell, region)
                };
                let (tx_half, rx_half) = transport.split();
                let mut tx = Channel::<Tx, Message, _, C>::new(ch_tx, tx_half, scratch::<MTU>());
                let mut rx = Channel::<Rx, Message, _, C>::new(ch_rx, rx_half, scratch::<MTU>());

                a_sent_for_m.wait();

                let decoded = rx
                    .recv()
                    .await
                    .expect("M-side channel recv should succeed")
                    .into_inner();

                assert_message_eq(&decoded, &a_expected);

                tx.send(&m_to_a)
                    .await
                    .expect("M-side channel send should succeed");

                m_sent_for_m.wait();
            });
        });

        a_thread.join().expect("A-side thread should finish");
        m_thread.join().expect("M-side thread should finish");
    });
}

pub trait MessageView {
    fn sequence(&self) -> u32;
    fn ready(&self) -> bool;
}

impl MessageView for Message {
    fn sequence(&self) -> u32 {
        self.sequence
    }

    fn ready(&self) -> bool {
        self.ready
    }
}

impl MessageView for ArchivedMessage {
    fn sequence(&self) -> u32 {
        self.sequence.into()
    }

    fn ready(&self) -> bool {
        self.ready
    }
}

impl<T: MessageView + ?Sized> MessageView for &T {
    fn sequence(&self) -> u32 {
        (**self).sequence()
    }

    fn ready(&self) -> bool {
        (**self).ready()
    }
}

pub fn assert_message_eq(actual: &impl MessageView, expected: &Message) {
    assert_eq!(actual.sequence(), expected.sequence);
    assert_eq!(actual.ready(), expected.ready);
}
