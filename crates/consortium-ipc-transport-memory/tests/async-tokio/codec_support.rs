use std::sync::{Arc, Barrier};
use std::thread;

use crate::common::doorbell::HostDoorbell;
use consortium_codec::CodecFor;
use consortium_ipc::{Chan, Channel, Rx, SramRegion, Tx};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport, descriptor_init,
    slot,
};
use prost::bytes::{Buf, BufMut};
use prost::encoding::{self, DecodeContext, WireType};
use serde::{Deserialize, Serialize};

pub const MTU: usize = 128;
pub const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
pub const A_TO_M_SLOT: usize = 0x100;
pub const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
pub const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;
pub const DESCRIPTOR_LEN: usize = 0x300;
pub const A_DESCRIPTOR_OFFSET: usize = 0;
pub const M_DESCRIPTOR_OFFSET: usize = DESCRIPTOR_LEN;
pub const SHARED_A_TO_M_SLOT: usize = DESCRIPTOR_LEN * 2;
pub const SHARED_M_TO_A_SLOT: usize = SHARED_A_TO_M_SLOT + SLOT_SIZE;
pub const PAIRED_REGION_LEN: usize = SHARED_M_TO_A_SLOT + SLOT_SIZE;

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
    let a_to_m = channel(A_TO_M_SLOT, ChannelDirection::AToM);
    let m_to_a = channel(M_TO_A_SLOT, ChannelDirection::MToA);

    unsafe {
        descriptor_init(base, 2, MTU, &[a_to_m, m_to_a]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }
}

pub fn scratch<const N: usize>() -> &'static mut [u8] {
    Box::leak(Box::new([0; N])).as_mut_slice()
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
    let mut backing = vec![0; PAIRED_REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    init_paired_descriptors(base_addr as *mut u8);
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
                let a_base = (base_addr + A_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(a_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, a_doorbell, region, a_base) };
                let (tx_half, rx_half) = transport.split();
                let mut tx = Channel::<Tx, Message, _, C>::new(ch, tx_half, scratch::<MTU>());
                let mut rx = Channel::<Rx, Message, _, C>::new(ch, rx_half, scratch::<MTU>());

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
                let m_base = (base_addr + M_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(m_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, m_doorbell, region, m_base) };
                let (tx_half, rx_half) = transport.split();
                let mut tx = Channel::<Tx, Message, _, C>::new(ch, tx_half, scratch::<MTU>());
                let mut rx = Channel::<Rx, Message, _, C>::new(ch, rx_half, scratch::<MTU>());

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
