//! Byte-at-a-time volatile accessors for the shared debug region.
//!
//! Every multi-byte read and write here is decomposed into individual volatile
//! byte accesses and reassembled little-endian, rather than issuing one
//! `read_volatile::<u16>()`. That is deliberate: the descriptor is
//! `#[repr(C, packed)]` and its fields sit at unaligned offsets, so a wider
//! volatile access would be an unaligned access — undefined behaviour in Rust, and
//! a genuine fault on some of the AArch64 device-memory mappings this region is
//! reached through on the Linux side.
//!
//! `volatile` rather than plain reads because the far side of this memory is
//! another processor: the compiler must not cache, reorder, or elide these
//! accesses. Ordering *between* accesses is the descriptor protocol's job (see
//! [`descriptor`](crate::descriptor)); these helpers only guarantee each access
//! actually happens.
//!
//! All functions are `unsafe` and crate-internal. The caller owes validity of
//! `base + offset` for the access width, and cache coherency for the region.

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
