//! Single-slot buffer layout within the shared memory region.
//!
//! ```text
//! [0..4]        len    : u32   — valid payload bytes
//! [4..8]        flags  : u32   — FLAG_VALID | FLAG_CONSUMED
//! [8..8+mtu]    payload: [u8]
//! ```
//!
//! Write order: payload → len → flags (ensures receiver sees consistent data).
//! Read order:  check flags → read len → copy payload → write FLAG_CONSUMED.

pub const SLOT_HEADER: usize = 8;

pub const FLAG_VALID: u32 = 1 << 0;
pub const FLAG_CONSUMED: u32 = 1 << 1;

/// Write `payload` into the slot and mark it VALID.
///
/// # Safety
/// `base` must point to a slot of at least `SLOT_HEADER + payload.len()` bytes.
pub unsafe fn write(base: *mut u8, payload: &[u8]) {
    // SAFETY: the caller must ensure that `base` points to a valid slot and is large enough for the payload.
    unsafe {
        let dst = base.add(SLOT_HEADER);
        core::ptr::copy_nonoverlapping(payload.as_ptr(), dst, payload.len());
        (base as *mut u32).write_volatile(payload.len() as u32);
        (base.add(4) as *mut u32).write_volatile(FLAG_VALID);
    }
}

/// Copy the slot payload into `buf` and mark it CONSUMED.
/// Returns the number of bytes written, or `None` if the slot is not VALID.
///
/// # Safety
/// `base` must point to a valid slot of at least `SLOT_HEADER + buf.len()` bytes.
pub unsafe fn try_read(base: *mut u8, buf: &mut [u8]) -> Option<usize> {
    // SAFETY: the caller must ensure that `base` points to a valid slot and `buf` is large enough.
    unsafe {
        let flags = (base.add(4) as *const u32).read_volatile();
        if flags & FLAG_VALID == 0 {
            return None;
        }
        let len = (base as *const u32).read_volatile() as usize;
        let n = len.min(buf.len());
        core::ptr::copy_nonoverlapping(base.add(SLOT_HEADER), buf.as_mut_ptr(), n);
        (base.add(4) as *mut u32).write_volatile(FLAG_CONSUMED);
        Some(n)
    }
}

/// True if the slot is free for the sender to reuse (consumed or never written).
///
/// # Safety
/// `base` must point to a valid slot.
#[allow(dead_code)]
pub unsafe fn is_consumed(base: *const u8) -> bool {
    // SAFETY: the sender and receiver must synchronise access to the slot via the FLAG_VALID and FLAG_CONSUMED flags.  If this function is called when the sender is allowed to write, then either the slot is currently consumed (flags == 0) or the previous message has been consumed (flags & FLAG_CONSUMED != 0), so it's safe for the sender to reuse.
    unsafe {
        let flags = (base.add(4) as *const u32).read_volatile();
        flags == 0 || (flags & FLAG_CONSUMED != 0)
    }
}
