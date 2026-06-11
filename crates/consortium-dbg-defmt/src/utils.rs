#![allow(dead_code)]

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    let bytes = [
        unsafe { base.add(offset).read_volatile() },
        unsafe { base.add(offset + 1).read_volatile() },
        unsafe { base.add(offset + 2).read_volatile() },
        unsafe { base.add(offset + 3).read_volatile() },
    ];
    u32::from_le_bytes(bytes)
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn rv_u16(base: *const u8, offset: usize) -> u16 {
    let bytes = [unsafe { base.add(offset).read_volatile() }, unsafe {
        base.add(offset + 1).read_volatile()
    }];
    u16::from_le_bytes(bytes)
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
    let bytes = value.to_le_bytes();
    unsafe {
        base.add(offset).write_volatile(bytes[0]);
        base.add(offset + 1).write_volatile(bytes[1]);
        base.add(offset + 2).write_volatile(bytes[2]);
        base.add(offset + 3).write_volatile(bytes[3]);
    }
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub(crate) unsafe fn wv_u16(base: *mut u8, offset: usize, value: u16) {
    // SAFETY: the caller must ensure that `base` is valid for at least `offset + 2` bytes and that the memory is DMA-coherent.
    let bytes = value.to_le_bytes();
    unsafe {
        base.add(offset).write_volatile(bytes[0]);
        base.add(offset + 1).write_volatile(bytes[1]);
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
