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
            flag: 0u8 | FLAG_VALID,
            len,
        }
    }

    pub unsafe fn try_from_raw_ptr(ptr: *const u8) -> Option<Self> {
        let magic = unsafe { rv_u16(ptr, 0) };
        if magic != MAGIC {
            return None;
        }
        let flag = unsafe { rv_u8(ptr, FLAG_OFFSET) };
        let len = unsafe { rv_u8(ptr, LEN_OFFSET) };
        Some(Self { magic, flag, len })
    }

    pub unsafe fn read_payload(&mut self, base: *mut u8, payload: &mut [u8]) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                base.add(PACKET_HEADER_SIZE),
                payload.as_mut_ptr(),
                self.len as usize,
            );
        }
    }

    pub unsafe fn write_to_raw_ptr(&self, ptr: *mut u8) {
        unsafe {
            wv_u16(ptr, 0, self.magic);
            wv_u8(ptr, FLAG_OFFSET, self.flag);
            wv_u8(ptr, LEN_OFFSET, self.len);
        }
    }

    pub unsafe fn write_payload_to_raw_ptr(&self, ptr: *mut u8, payload: &[u8]) {
        unsafe {
            std::ptr::copy_nonoverlapping(payload.as_ptr(), ptr.add(4), self.len as usize);
        }
    }

    pub unsafe fn write_full_packet_to_raw_ptr(ptr: *mut u8, payload: &[u8]) {
        let packet = Self::new(payload.len() as u8);
        unsafe {
            packet.write_to_raw_ptr(ptr);
            packet.write_payload_to_raw_ptr(ptr, payload);
        }
    }

    pub unsafe fn mark_as_consumed(&mut self, ptr: *mut u8) {
        self.flag |= FLAG_CONSUMED;
        unsafe {
            wv_u8(ptr, FLAG_OFFSET, self.flag);
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == MAGIC && (self.flag & FLAG_VALID != 0) && (self.flag & FLAG_CONSUMED == 0)
    }
}
