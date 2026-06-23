use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use consortium_ipc::{Region, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransportable,
    descriptor_init, slot,
};

pub const MTU: usize = 64;
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

pub fn init_paired_descriptors(base: *mut u8) {
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

#[derive(Clone)]
pub struct CountingRegion {
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
    pub fn new(ptr: *mut u8, len: usize) -> Self {
        Self {
            ptr,
            len,
            flushes: Arc::new(AtomicUsize::new(0)),
            invalidations: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn flush_count(&self) -> usize {
        self.flushes.load(Ordering::SeqCst)
    }

    pub fn invalidate_count(&self) -> usize {
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

pub type HostSramTransportTx = consortium_ipc_transport_memory::SharedMemoryTransportTx<
    crate::common::doorbell::HostDoorbell,
    SramRegion,
>;
pub type HostSramTransportRx = consortium_ipc_transport_memory::SharedMemoryTransportRx<
    crate::common::doorbell::HostDoorbell,
    SramRegion,
>;
