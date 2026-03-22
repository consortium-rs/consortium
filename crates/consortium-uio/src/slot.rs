/// Single-slot buffer layout within the DMA-coherent region.
///
/// ```text
/// [0..4]        len    : u32   — valid payload bytes
/// [4..8]        flags  : u32   — FLAG_VALID | FLAG_CONSUMED
/// [8..8+mtu]    payload: [u8]
/// ```
///
/// Write order: payload → len → flags (ensures receiver sees consistent data).
/// Read order:  check flags → read len → copy payload → write FLAG_CONSUMED.

pub const SLOT_HEADER: usize = 8;

pub const FLAG_VALID: u32 = 1 << 0;
pub const FLAG_CONSUMED: u32 = 1 << 1;

/// Write `payload` into the slot and mark it VALID.
///
/// # Safety
/// `base` must point to a slot of at least `SLOT_HEADER + payload.len()` bytes.
pub unsafe fn write(base: *mut u8, payload: &[u8]) {
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
pub unsafe fn is_consumed(base: *const u8) -> bool {
    unsafe {
        let flags = (base.add(4) as *const u32).read_volatile();
        flags == 0 || (flags & FLAG_CONSUMED != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_slot(mtu: usize) -> Vec<u8> {
        vec![0u8; SLOT_HEADER + mtu]
    }

    #[test]
    fn write_then_read_roundtrip() {
        let payload = b"hello world";
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), payload) };

        let mut buf = [0u8; 64];
        let n = unsafe { try_read(slot.as_mut_ptr(), &mut buf) };
        assert_eq!(n, Some(payload.len()));
        assert_eq!(&buf[..payload.len()], payload);
    }

    #[test]
    fn try_read_returns_none_on_fresh_slot() {
        let mut slot = make_slot(64);
        let mut buf = [0u8; 64];
        // All zeros → FLAG_VALID not set
        assert_eq!(unsafe { try_read(slot.as_mut_ptr(), &mut buf) }, None);
    }

    #[test]
    fn try_read_marks_slot_consumed() {
        let payload = b"test";
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), payload) };

        let mut buf = [0u8; 64];
        unsafe { try_read(slot.as_mut_ptr(), &mut buf) };

        assert!(unsafe { is_consumed(slot.as_ptr()) });
    }

    #[test]
    fn is_consumed_true_on_fresh_slot() {
        let slot = make_slot(64);
        // flags == 0 → considered free for reuse
        assert!(unsafe { is_consumed(slot.as_ptr()) });
    }

    #[test]
    fn is_consumed_false_after_write() {
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), b"data") };
        // flags == FLAG_VALID → not yet consumed
        assert!(!unsafe { is_consumed(slot.as_ptr()) });
    }

    #[test]
    fn try_read_truncates_to_buf_length() {
        let payload = b"abcdefgh"; // 8 bytes
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), payload) };

        let mut buf = [0u8; 4];
        let n = unsafe { try_read(slot.as_mut_ptr(), &mut buf) };
        assert_eq!(n, Some(4));
        assert_eq!(&buf[..], b"abcd");
    }

    #[test]
    fn write_zero_length_payload() {
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), &[]) };

        let mut buf = [0u8; 64];
        let n = unsafe { try_read(slot.as_mut_ptr(), &mut buf) };
        assert_eq!(n, Some(0));
    }

    #[test]
    fn double_read_returns_none_second_time() {
        let payload = b"once";
        let mut slot = make_slot(64);
        unsafe { write(slot.as_mut_ptr(), payload) };

        let mut buf = [0u8; 64];
        // First read succeeds
        assert!(unsafe { try_read(slot.as_mut_ptr(), &mut buf) }.is_some());
        // Second read sees CONSUMED flag, not VALID → None
        assert_eq!(unsafe { try_read(slot.as_mut_ptr(), &mut buf) }, None);
    }
}
