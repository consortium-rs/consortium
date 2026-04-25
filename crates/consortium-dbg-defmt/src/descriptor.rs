//! Shared memory defmt descriptor definitions.
//!
//!
//! ```text
//! Offset  Size  Owner   Field
//! 0x00    6     N/A     header (refer to `DescriptorHeader`)
//! 0x06    2     mcu     write_ptr
//! 0x08    2     app     read_ptr
//! 0x0A    2     mcu     capacity
//! 0x0C    2     mcu     flags (error flags, etc.)
//! 0x0E    2     n/a     reserved
//! 0x10    vary  mcu     data (circular buffer of log data)
//! ```

use bitflags::bitflags;
use consortium_common::descriptor::{DescriptorHeader, DescriptorMagic};

use crate::packet::{PACKET_HEADER_SIZE, Packet};
use crate::utils::{rv_u16, wv_u16};
#[repr(C)]
pub struct Descriptor {
    pub header: DescriptorHeader,
    pub write_ptr: u16,
    pub read_ptr: u16,
    pub capacity: u16,
    pub flags: Flags,
    _reserved1: [u8; 2],
}

bitflags! {
    pub struct Flags: u16 {
        const Writing = 0b0000_0000_0000_0001;
        const Error = 0b0000_0000_0001_0000;
        const Overflow = 0b0000_0000_0010_0000;
    }
}

pub const MAGIC: DescriptorMagic = DescriptorMagic::new(b"CDBG");
pub const VERSION: u8 = 0;

pub const WRITE_PTR_OFFSET: usize = 0x06;
pub const READ_PTR_OFFSET: usize = 0x08;
pub const CAPACITY_OFFSET: usize = 0x0A;
pub const FLAGS_OFFSET: usize = 0x0C;

pub const DESCRIPTOR_SIZE: usize = core::mem::size_of::<Descriptor>();

impl Descriptor {
    pub const fn new(capacity: u16) -> Self {
        Self {
            header: DescriptorHeader::new(MAGIC, VERSION),
            write_ptr: 0,
            read_ptr: 0,
            capacity,
            flags: Flags::empty(),
            _reserved1: [0; 2],
        }
    }

    pub fn check(&self) -> bool {
        self.header.check(MAGIC, VERSION).is_ok()
    }

    pub unsafe fn from_raw_ptr(ptr: *const u8) -> Self {
        let header = unsafe { DescriptorHeader::from_raw_ptr(ptr) };
        let write_ptr = unsafe { rv_u16(ptr, WRITE_PTR_OFFSET) };
        let read_ptr = unsafe { rv_u16(ptr, READ_PTR_OFFSET) };
        let capacity = unsafe { rv_u16(ptr, CAPACITY_OFFSET) };
        let flags = Flags::from_bits_retain(unsafe { rv_u16(ptr, FLAGS_OFFSET) });
        Self {
            header,
            write_ptr,
            read_ptr,
            capacity,
            flags,
            _reserved1: [0; 2],
        }
    }

    pub unsafe fn write_to_raw_ptr(&self, ptr: *mut u8) {
        unsafe {
            DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(ptr);
            wv_u16(ptr, WRITE_PTR_OFFSET, self.write_ptr);
            wv_u16(ptr, READ_PTR_OFFSET, self.read_ptr);
            wv_u16(ptr, CAPACITY_OFFSET, self.capacity);
            wv_u16(ptr, FLAGS_OFFSET, self.flags.bits());
        }
    }

    #[cfg(feature = "host")]
    pub unsafe fn read_one_frame(&mut self, base: *mut u8) -> Option<Vec<u8>> {
        if self.read_ptr == self.write_ptr {
            return None;
        }
        let Some(mut frame_header) =
            (unsafe { Packet::try_from_raw_ptr(base.add(self.read_ptr as usize)) })
        else {
            return None;
        };
        // Find the next valid frame header, in case the current one is corrupted (e.g. due to an overflow)
        while self.read_ptr != DESCRIPTOR_SIZE as u16 && !frame_header.is_valid() {
            self.read_ptr = unsafe { self.next_read_ptr(base) };
            match unsafe { Packet::try_from_raw_ptr(base.add(self.read_ptr as usize)) } {
                Some(header) => frame_header = header,
                None => return None,
            }
        }

        if self.read_ptr == DESCRIPTOR_SIZE as u16 {
            None
        } else {
            let mut buffer = vec![0u8; frame_header.len as usize];
            unsafe {
                frame_header.read_payload(
                    base.add(self.read_ptr as usize + PACKET_HEADER_SIZE),
                    &mut buffer,
                );
            }
            Some(buffer)
        }
    }

    #[cfg(feature = "host")]
    pub unsafe fn read_bulk_frames(&mut self, base: *mut u8) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        while let Some(frame) = unsafe { self.read_one_frame(base) } {
            frames.push(frame);
            self.read_ptr = unsafe { self.next_read_ptr(base) };
        }
        frames
    }

    // the return value indicates whether the frame was successfully written (i.e. it fits in the buffer and doesn't overwrite unread data)
    // if it returns false, it will set the Overflow flag; otherwise it will clear the Overflow flag and write the frame.
    pub unsafe fn write_one_frame(&mut self, base: *mut u8, frame: &[u8]) -> bool {
        match unsafe { self.allocate_write_ptr(base) } {
            Some(p) => {
                unsafe { Packet::write_full_packet_to_raw_ptr(base.add(p as usize), frame) };
                self.flags.remove(Flags::Overflow);
                self.write_ptr = p;
                true
            }
            None => {
                self.flags.insert(Flags::Overflow);
                false
            }
        }
    }

    pub(crate) unsafe fn next_read_ptr(&mut self, base: *const u8) -> u16 {
        let cursor = unsafe { base.add(self.read_ptr as usize) };
        let Some(frame_header) = (unsafe { Packet::try_from_raw_ptr(cursor) }) else {
            return DESCRIPTOR_SIZE as u16;
        };
        let next_padding = frame_header.len as usize + PACKET_HEADER_SIZE;
        let next_ptr = self.read_ptr.wrapping_add(next_padding as u16);
        if next_ptr >= self.capacity {
            DESCRIPTOR_SIZE as u16
        } else {
            next_ptr
        }
    }

    pub(crate) unsafe fn next_write_ptr(&mut self, base: *mut u8) -> u16 {
        let cursor = unsafe { base.add(self.write_ptr as usize) };
        let Some(frame_header) = (unsafe { Packet::try_from_raw_ptr(cursor) }) else {
            return self.write_ptr;
        };
        let next_padding = frame_header.len as usize + PACKET_HEADER_SIZE;
        let next_ptr = self.write_ptr.wrapping_add(next_padding as u16);
        if next_ptr >= self.capacity {
            DESCRIPTOR_SIZE as u16
        } else {
            next_ptr
        }
    }

    pub(crate) unsafe fn allocate_write_ptr(&mut self, base: *mut u8) -> Option<u16> {
        let cursor = unsafe { base.add(self.write_ptr as usize) };
        let Some(mut frame_header) = (unsafe { Packet::try_from_raw_ptr(cursor) }) else {
            return Some(self.write_ptr);
        };
        while frame_header.is_valid() && self.write_ptr != DESCRIPTOR_SIZE as u16 {
            self.write_ptr = unsafe { self.next_write_ptr(base) };
            match unsafe { Packet::try_from_raw_ptr(base.add(self.write_ptr as usize)) } {
                Some(header) => frame_header = header,
                None => return Some(self.write_ptr),
            }
        }

        if frame_header.is_valid() {
            None
        } else {
            Some(DESCRIPTOR_SIZE as u16)
        }
    }

    /// Block until host has read all pending data. (defmt's Logger trait)
    pub(crate) unsafe fn flush(&mut self, base: *mut u8) {
        let cursor = unsafe { base.add(self.read_ptr as usize) };
        let Some(mut frame_header) = (unsafe { Packet::try_from_raw_ptr(cursor) }) else {
            return;
        };
        while self.read_ptr != self.write_ptr && self.read_ptr != DESCRIPTOR_SIZE as u16 {
            while frame_header.is_valid() {
                core::hint::spin_loop();
            }
            self.read_ptr = unsafe { self.next_read_ptr(base) };
            match unsafe { Packet::try_from_raw_ptr(base.add(self.read_ptr as usize)) } {
                Some(header) => frame_header = header,
                None => return,
            }
        }
    }
}
