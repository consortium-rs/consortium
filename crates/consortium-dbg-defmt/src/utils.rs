#![allow(dead_code)]

unsafe fn byte_ptr(base: *const u8, offset: usize) -> *const u8 {
    // SAFETY: caller guarantees the computed byte address is in-bounds.
    unsafe { base.add(offset) }
}

unsafe fn byte_mut_ptr(base: *mut u8, offset: usize) -> *mut u8 {
    // SAFETY: caller guarantees the computed byte address is in-bounds.
    unsafe { base.add(offset) }
}

unsafe fn read_byte(base: *const u8, offset: usize) -> u8 {
    // SAFETY: caller guarantees `base.add(offset)` is valid for a u8 read.
    let ptr = unsafe { byte_ptr(base, offset) };
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.read_volatile() }
}

unsafe fn write_byte(base: *mut u8, offset: usize, value: u8) {
    // SAFETY: caller guarantees `base.add(offset)` is valid for a u8 write.
    let ptr = unsafe { byte_mut_ptr(base, offset) };
    // SAFETY: caller guarantees the computed pointer is valid and DMA-coherent.
    unsafe { ptr.write_volatile(value) };
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least `offset + 4` bytes and DMA-coherent.
pub(crate) unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    // SAFETY: caller guarantees `base` is valid for the first byte.
    let b0 = unsafe { read_byte(base, offset) };
    // SAFETY: caller guarantees `base` is valid for the second byte.
    let b1 = unsafe { read_byte(base, offset + 1) };
    // SAFETY: caller guarantees `base` is valid for the third byte.
    let b2 = unsafe { read_byte(base, offset + 2) };
    // SAFETY: caller guarantees `base` is valid for the fourth byte.
    let b3 = unsafe { read_byte(base, offset + 3) };
    u32::from_le_bytes([b0, b1, b2, b3])
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least `offset + 2` bytes and DMA-coherent.
pub(crate) unsafe fn rv_u16(base: *const u8, offset: usize) -> u16 {
    // SAFETY: caller guarantees `base` is valid for the first byte.
    let b0 = unsafe { read_byte(base, offset) };
    // SAFETY: caller guarantees `base` is valid for the second byte.
    let b1 = unsafe { read_byte(base, offset + 1) };
    u16::from_le_bytes([b0, b1])
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least `offset + 1` byte and DMA-coherent.
pub(crate) unsafe fn rv_u8(base: *const u8, offset: usize) -> u8 {
    // SAFETY: caller guarantees `base` is valid for the requested byte.
    unsafe { read_byte(base, offset) }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u32(base: *mut u8, offset: usize, value: u32) {
    let [b0, b1, b2, b3] = value.to_le_bytes();
    // SAFETY: caller guarantees `base` is valid for the first byte.
    unsafe { write_byte(base, offset, b0) };
    // SAFETY: caller guarantees `base` is valid for the second byte.
    unsafe { write_byte(base, offset + 1, b1) };
    // SAFETY: caller guarantees `base` is valid for the third byte.
    unsafe { write_byte(base, offset + 2, b2) };
    // SAFETY: caller guarantees `base` is valid for the fourth byte.
    unsafe { write_byte(base, offset + 3, b3) };
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least `offset + 2` bytes and DMA-coherent.
pub(crate) unsafe fn wv_u16(base: *mut u8, offset: usize, value: u16) {
    let [b0, b1] = value.to_le_bytes();
    // SAFETY: caller guarantees `base` is valid for the first byte.
    unsafe { write_byte(base, offset, b0) };
    // SAFETY: caller guarantees `base` is valid for the second byte.
    unsafe { write_byte(base, offset + 1, b1) };
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
pub(crate) unsafe fn wv_u8(base: *mut u8, offset: usize, value: u8) {
    // SAFETY: caller guarantees `base` is valid for the requested byte.
    unsafe { write_byte(base, offset, value) };
}
