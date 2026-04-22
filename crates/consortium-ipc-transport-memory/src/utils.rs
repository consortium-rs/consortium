
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { (base.add(offset) as *const u32).read_volatile() }
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u8(base: *const u8, offset: usize) -> u8 {
    unsafe { base.add(offset).read_volatile() }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u32(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: the caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
    unsafe {
        (base.add(offset) as *mut u32).write_volatile(value);
    }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u8(base: *mut u8, offset: usize, value: u8) {
    // SAFETY: the caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
    unsafe {
        base.add(offset).write_volatile(value);
    }
}
