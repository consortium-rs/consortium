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
const DATA_START: u16 = DESCRIPTOR_SIZE as u16;

impl Descriptor {
    pub const fn new(capacity: u16) -> Self {
        Self {
            header: DescriptorHeader::new(MAGIC, VERSION),
            write_ptr: DATA_START,
            read_ptr: DATA_START,
            capacity,
            flags: Flags::empty(),
            _reserved1: [0; 2],
        }
    }

    /// Returns `true` if the descriptor header has the expected magic and version.
    pub fn check(&self) -> bool {
        self.header.check(MAGIC, VERSION).is_ok()
    }

    /// Reads a `Descriptor` from shared memory at `ptr`.
    ///
    /// # Safety
    /// `ptr` must point to a valid, readable shared-memory region of at least
    /// `DESCRIPTOR_SIZE` bytes with consistent volatile layout.
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

    /// Writes this descriptor to shared memory at `ptr`.
    ///
    /// # Safety
    /// `ptr` must point to a valid, writable shared-memory region of at least
    /// `DESCRIPTOR_SIZE` bytes.
    pub unsafe fn write_to_raw_ptr(&self, ptr: *mut u8) {
        // SAFETY: caller guarantees ptr is valid for DESCRIPTOR_SIZE bytes.
        unsafe {
            DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(ptr);
            wv_u16(ptr, WRITE_PTR_OFFSET, self.write_ptr);
            wv_u16(ptr, READ_PTR_OFFSET, self.read_ptr);
            wv_u16(ptr, CAPACITY_OFFSET, self.capacity);
            wv_u16(ptr, FLAGS_OFFSET, self.flags.bits());
        }
    }

    /// Reads the next available defmt frame from the circular buffer, returning its payload.
    ///
    /// Returns `None` if no frame is available or if the current frame header is invalid.
    ///
    /// # Safety
    /// `base` must point to the start of the shared-memory region owning this descriptor,
    /// valid for at least `self.capacity` bytes.
    #[cfg(feature = "host")]
    pub unsafe fn read_one_frame(&mut self, base: *mut u8) -> Option<Vec<u8>> {
        if self.read_ptr == self.write_ptr {
            return None;
        }
        let packet_ptr = unsafe { base.add(self.read_ptr as usize) };
        let Some(mut frame_header) = (unsafe { Packet::try_from_raw_ptr(packet_ptr) }) else {
            return None;
        };

        if !frame_header.is_valid() {
            return None;
        }

        let mut buffer = vec![0u8; frame_header.len as usize];
        unsafe {
            frame_header.read_payload(packet_ptr, &mut buffer);
            frame_header.mark_as_consumed(packet_ptr);
        }
        self.read_ptr = unsafe { self.next_read_ptr(base) };
        unsafe { wv_u16(base, READ_PTR_OFFSET, self.read_ptr) };
        Some(buffer)
    }

    /// Drains all available defmt frames from the circular buffer, returning them as a `Vec`.
    ///
    /// # Safety
    /// Same requirements as [`read_one_frame`]: `base` must be valid for at least
    /// `self.capacity` bytes.
    ///
    /// [`read_one_frame`]: Self::read_one_frame
    #[cfg(feature = "host")]
    pub unsafe fn read_bulk_frames(&mut self, base: *mut u8) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        while let Some(frame) = unsafe { self.read_one_frame(base) } {
            frames.push(frame);
        }
        frames
    }

    /// Writes a single defmt frame into the circular buffer.
    ///
    /// Returns `true` on success; returns `false` and sets the `Overflow` flag if the
    /// frame does not fit without overwriting unread data.
    ///
    /// # Safety
    /// `base` must point to the start of the shared-memory region owning this descriptor,
    /// valid for at least `self.capacity` bytes.
    pub unsafe fn write_one_frame(&mut self, base: *mut u8, frame: &[u8]) -> bool {
        match unsafe { self.allocate_write_ptr(base, frame.len()) } {
            Some(p) => {
                unsafe { Packet::write_full_packet_to_raw_ptr(base.add(p as usize), frame) };
                self.flags.remove(Flags::Overflow);
                self.write_ptr = self.advance_ptr(p, frame.len());
                unsafe {
                    wv_u16(base, WRITE_PTR_OFFSET, self.write_ptr);
                    wv_u16(base, FLAGS_OFFSET, self.flags.bits());
                }
                true
            }
            None => {
                self.flags.insert(Flags::Overflow);
                unsafe { wv_u16(base, FLAGS_OFFSET, self.flags.bits()) };
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
            DATA_START
        } else {
            next_ptr
        }
    }

    pub(crate) unsafe fn allocate_write_ptr(
        &mut self,
        base: *mut u8,
        frame_len: usize,
    ) -> Option<u16> {
        if frame_len > u8::MAX as usize {
            return None;
        }

        let packet_len = PACKET_HEADER_SIZE + frame_len;
        if packet_len > self.capacity as usize - DATA_START as usize {
            return None;
        }

        let mut ptr = self.write_ptr;
        if ptr as usize + packet_len > self.capacity as usize {
            ptr = DATA_START;
        }

        let cursor = unsafe { base.add(ptr as usize) };
        if unsafe { Packet::try_from_raw_ptr(cursor) }.is_some_and(|packet| packet.is_valid()) {
            None
        } else {
            Some(ptr)
        }
    }

    fn advance_ptr(&self, ptr: u16, frame_len: usize) -> u16 {
        let next_ptr = ptr.wrapping_add((PACKET_HEADER_SIZE + frame_len) as u16);
        if next_ptr >= self.capacity {
            DATA_START
        } else {
            next_ptr
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
