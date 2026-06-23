/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    // SAFETY: caller guarantees `base.add(offset)` is in-bounds for a u32 read.
    let ptr = unsafe { base.add(offset) } as *const u32;
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.read_volatile() }
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u8(base: *const u8, offset: usize) -> u8 {
    // SAFETY: caller guarantees `base.add(offset)` is in-bounds for a u8 read.
    let ptr = unsafe { base.add(offset) };
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.read_volatile() }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u32(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: caller guarantees `base.add(offset)` is in-bounds for a u32 write.
    let ptr = unsafe { base.add(offset) } as *mut u32;
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.write_volatile(value) };
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u8(base: *mut u8, offset: usize, value: u8) {
    // SAFETY: caller guarantees `base.add(offset)` is in-bounds for a u8 write.
    let ptr = unsafe { base.add(offset) };
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.write_volatile(value) };
}
