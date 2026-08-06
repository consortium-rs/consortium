use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, DescState, MAGIC, RX_UPDATED_ID_OFFSET,
    RegionSharedMemoryTransportable, VERSION, channel_table_entry_offset, slot, slot_offset,
    slot_stride,
};
use consortium_shared_memory::descriptor::DescriptorHeader;
use consortium_shared_memory::{Region, UncachedRegion};

pub const MTU: usize = 64;
pub const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
pub const SLOT_STRIDE: usize = slot_stride(MTU);
pub const A_TO_M_SLOT: usize = slot_offset(0, MTU);
pub const M_TO_A_SLOT: usize = slot_offset(1, MTU);
pub const REGION_LEN: usize = M_TO_A_SLOT + SLOT_STRIDE;

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

    init_descriptor_with_channels(base, MTU, &[a_to_m, m_to_a]);
}

pub fn init_descriptor_with_channels(base: *mut u8, mtu: usize, channels: &[ChannelLayout]) {
    unsafe {
        DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(base);
        base.add(0x06).write_volatile(channels.len() as u8);
        (base.add(0x10) as *mut u32).write_volatile(mtu as u32);
        (base.add(0x18) as *mut u32).write_volatile(DescState::Ready as u32);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);

        for (index, channel) in channels.iter().enumerate() {
            let entry = base.add(channel_table_entry_offset(index));
            (entry as *mut u32).write_volatile(channel.offset);
            (entry.add(4) as *mut u32).write_volatile(channel.size);
            entry.add(8).write_volatile(channel.direction as u8);
        }
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
// SAFETY: counters are atomic, and the raw memory aliasing discipline matches UncachedRegion test usage.
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

impl RegionSharedMemoryTransportable for CountingRegion {}

pub type HostUncachedTransportTx<S> = consortium_ipc_transport_memory::SharedMemoryTransportTx<
    S,
    crate::common::doorbell::HostDoorbell,
    UncachedRegion,
>;
pub type HostUncachedTransportRx<S> = consortium_ipc_transport_memory::SharedMemoryTransportRx<
    S,
    crate::common::doorbell::HostDoorbell,
    UncachedRegion,
>;
