//! M-core descriptor initialization and state management.
//!
//! The M-core writes the descriptor header and channel table to the shared DMA-coherent region,
//! then transitions the state through `MReady` to signal A-core readiness.

/// Descriptor layout in the DMA-coherent shared region.
/// (Must match consortium-ipc-transport-uio/src/descriptor.rs)
pub const MAGIC: u32 = 0x434F_4E53;
pub const VERSION: u8 = 1;
pub const CHAN_TABLE_BASE: usize = 0x20;
pub const CHAN_ENTRY_SIZE: usize = 16;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescState {
    KernelInit = 0,
    Proposed = 1,
    MReady = 2,
    Ready = 3,
    Suspended = 4,
    Error = 5,
}

/// Channel descriptor written to the descriptor table.
#[derive(Clone, Copy)]
pub struct ChanDesc {
    pub tx_offset: u32,
    pub tx_size: u32,
    pub rx_offset: u32,
    pub rx_size: u32,
}

/// Write the descriptor header and channel table to the shared memory region.
///
/// # Arguments
///
/// * `base` — pointer to the start of the shared DMA-coherent region
/// * `num_channels` — total number of channels in the table
/// * `proposed_mtu` — MTU proposed by kernel/A-core (written to offset 0x08)
/// * `accepted_mtu` — MTU accepted by M-core (written to offset 0x0C)
/// * `channels` — slice of channel descriptors (must match `num_channels`)
///
/// # Safety
///
/// Caller must ensure:
/// - `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes
/// - Memory is DMA-coherent and not accessed concurrently
/// - All values in `channels` do not cause out-of-bounds offsets
pub unsafe fn write_descriptor(
    base: *mut u8,
    num_channels: u8,
    proposed_mtu: u32,
    accepted_mtu: u32,
    channels: &[ChanDesc],
) {
    debug_assert!(
        channels.len() == num_channels as usize,
        "channel count mismatch"
    );

    // Write magic and version
    unsafe {
        wv_u32(base, 0x00, MAGIC);
        wv_u8(base, 0x04, VERSION);
        wv_u8(base, 0x05, num_channels);

        // Write MTU fields
        wv_u32(base, 0x08, proposed_mtu);
        wv_u32(base, 0x0C, accepted_mtu);

        // Channel table
        for (i, ch) in channels.iter().enumerate() {
            let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            wv_u32(base, off, ch.tx_offset);
            wv_u32(base, off + 4, ch.tx_size);
            wv_u32(base, off + 8, ch.rx_offset);
            wv_u32(base, off + 12, ch.rx_size);
        }
    }
}

/// Set the descriptor state field.
///
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn set_state(base: *mut u8, state: DescState) {
    unsafe {
        wv_u32(base, 0x14, state as u32);
    }
}

/// Read the descriptor state field.
///
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn get_state(base: *const u8) -> DescState {
    let val = unsafe { rv_u32(base, 0x14) };
    match val {
        0 => DescState::KernelInit,
        1 => DescState::Proposed,
        2 => DescState::MReady,
        3 => DescState::Ready,
        4 => DescState::Suspended,
        _ => DescState::Error,
    }
}

unsafe fn wv_u32(base: *mut u8, offset: usize, value: u32) {
    unsafe {
        (base.add(offset) as *mut u32).write_volatile(value);
    }
}

unsafe fn wv_u8(base: *mut u8, offset: usize, value: u8) {
    unsafe {
        base.add(offset).write_volatile(value);
    }
}

unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    unsafe {
        (base.add(offset) as *const u32).read_volatile()
    }
}
