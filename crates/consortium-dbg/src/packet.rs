use crate::utils::{rv_u8, rv_u16, wv_u8, wv_u16};

pub struct Packet {
    pub magic: u16, // Consortium Debug Packet Magic, shall not be ASCII, [0xC0, 0xDE] = 0xC0DE
    pub flag: u8,
    pub len: u8,
}

pub const MAGIC: u16 = 0xC0DE;

pub const FLAG_OFFSET: usize = 2;
pub const LEN_OFFSET: usize = 3;

pub const FLAG_VALID: u8 = 0b0000_0001;
pub const FLAG_CONSUMED: u8 = 0b0000_0010;

pub const PACKET_HEADER_SIZE: usize = core::mem::size_of::<Packet>();

impl Packet {
    pub const fn new(len: u8) -> Self {
        Self {
            magic: MAGIC,
            flag: FLAG_VALID,
            len,
        }
    }

    /// Reads a `Packet` header from `ptr`, returning `None` if the magic does not match.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `PACKET_HEADER_SIZE` bytes.
    pub unsafe fn try_from_raw_ptr(ptr: *const u8) -> Option<Self> {
        // SAFETY: caller guarantees `ptr` covers the packet header.
        let magic = unsafe { rv_u16(ptr, 0) };
        if magic != MAGIC {
            return None;
        }
        // SAFETY: caller guarantees `ptr` covers the packet header.
        let flag = unsafe { rv_u8(ptr, FLAG_OFFSET) };
        // SAFETY: caller guarantees `ptr` covers the packet header.
        let len = unsafe { rv_u8(ptr, LEN_OFFSET) };
        Some(Self { magic, flag, len })
    }

    /// Reads the packet payload from `base` into `payload`.
    ///
    /// Each byte is read with volatile semantics because host-side UIO commonly
    /// maps the shared ring as Device-nGnRnE memory. A compiler-generated bulk
    /// copy may use unaligned or SIMD loads, which are not valid for that memory
    /// type and can raise `SIGBUS` on AArch64.
    ///
    /// # Safety
    /// `base` must be valid for at least `self.len` bytes past `PACKET_HEADER_SIZE`.
    pub unsafe fn read_payload(&mut self, base: *mut u8, payload: &mut [u8]) {
        // SAFETY: caller guarantees the packet payload begins after the header.
        let src = unsafe { base.add(PACKET_HEADER_SIZE) };
        for (offset, byte) in payload.iter_mut().take(self.len as usize).enumerate() {
            // SAFETY: caller guarantees `src` covers `self.len` bytes; a u8
            // volatile load has no alignment requirement and is valid for UIO
            // Device memory.
            *byte = unsafe { rv_u8(src, offset) };
        }
    }

    /// Writes this packet header to `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `PACKET_HEADER_SIZE` bytes.
    pub unsafe fn write_to_raw_ptr(&self, ptr: *mut u8) {
        // SAFETY: caller guarantees `ptr` covers the packet header.
        unsafe { wv_u16(ptr, 0, self.magic) };
        // SAFETY: caller guarantees `ptr` covers the packet header.
        unsafe { wv_u8(ptr, FLAG_OFFSET, self.flag) };
        // SAFETY: caller guarantees `ptr` covers the packet header.
        unsafe { wv_u8(ptr, LEN_OFFSET, self.len) };
    }

    /// Writes `payload` into the buffer at `ptr + 4`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `4 + self.len` bytes.
    pub unsafe fn write_payload_to_raw_ptr(&self, ptr: *mut u8, payload: &[u8]) {
        // SAFETY: caller guarantees the packet payload begins after the header.
        let dst = unsafe { ptr.add(PACKET_HEADER_SIZE) };
        // SAFETY: `dst` points to `self.len` bytes and does not overlap `payload`.
        unsafe { core::ptr::copy_nonoverlapping(payload.as_ptr(), dst, self.len as usize) };
    }

    /// Writes a complete packet (header + payload) to `ptr` in one call.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `PACKET_HEADER_SIZE + payload.len()` bytes.
    pub unsafe fn write_full_packet_to_raw_ptr(ptr: *mut u8, payload: &[u8]) {
        let packet = Self::new(payload.len() as u8);
        // SAFETY: caller guarantees `ptr` covers the full packet.
        unsafe { packet.write_to_raw_ptr(ptr) };
        // SAFETY: caller guarantees `ptr` covers the full packet.
        unsafe { packet.write_payload_to_raw_ptr(ptr, payload) };
    }

    /// Sets the `FLAG_CONSUMED` bit in the on-disk flag byte at `ptr`.
    ///
    /// # Safety
    /// `ptr` must point to the start of this packet in shared memory, valid for
    /// at least `PACKET_HEADER_SIZE` bytes.
    pub unsafe fn mark_as_consumed(&mut self, ptr: *mut u8) {
        self.flag |= FLAG_CONSUMED;
        // SAFETY: caller guarantees `ptr` covers the packet header.
        unsafe { wv_u8(ptr, FLAG_OFFSET, self.flag) };
    }

    /// Returns `true` if the packet has the correct magic, is flagged valid, and has not been consumed.
    pub fn is_valid(&self) -> bool {
        self.magic == MAGIC && (self.flag & FLAG_VALID != 0) && (self.flag & FLAG_CONSUMED == 0)
    }
}
