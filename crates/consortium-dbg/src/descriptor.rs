//! Shared memory defmt descriptor definitions.
//!
//! This module defines the `CDBG` descriptor: a fixed-size header placed at the
//! start of a shared-memory region, immediately followed by a circular buffer of
//! defmt log frames. The MCU (firmware) side is the *writer* and the application
//! (Linux/host) side is the *reader*; the two communicate purely through the
//! descriptor fields and the ring data, with no locks.
//!
//! Ownership of each field is split so that, in steady state, each side only
//! mutates fields it owns: the writer owns `write_ptr`, `capacity`, and `flags`;
//! the reader owns `read_ptr`. `write_ptr == read_ptr` means the ring is empty.
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
use consortium_shared_memory::descriptor::{DescriptorHeader, DescriptorMagic};

use crate::packet::{PACKET_HEADER_SIZE, Packet};
use crate::utils::{rv_u16, wv_u16};

/// In-memory mirror of the shared `CDBG` descriptor.
///
/// `#[repr(C)]` keeps the field order matching the on-wire layout documented in
/// the module header, but reads/writes still go through the explicit offset
/// constants below (via volatile helpers) rather than relying on struct layout.
#[repr(C)]
pub struct Descriptor {
    /// Magic + version stamp shared by all Consortium descriptors.
    pub header: DescriptorHeader,
    /// Writer-owned cursor: byte offset where the next frame will be written.
    pub write_ptr: u16,
    /// Reader-owned cursor: byte offset of the next frame to be read.
    /// `read_ptr == write_ptr` means the ring is empty.
    pub read_ptr: u16,
    /// Total byte length of the shared region (header + data). Acts as the
    /// wrap-around bound for the circular buffer; usable data is
    /// `[DATA_START, capacity)`.
    pub capacity: u16,
    /// Writer-owned status flags (e.g. overflow).
    pub flags: Flags,
    _reserved1: [u8; 2],
}

bitflags! {
    pub struct Flags: u16 {
        /// Writer is mid-write; readers should not consume the in-flight frame.
        const Writing = 0b0000_0000_0000_0001;
        /// A non-recoverable error occurred on the writer side.
        const Error = 0b0000_0000_0001_0000;
        /// The last frame was dropped because it did not fit without
        /// overwriting unread data.
        const Overflow = 0b0000_0000_0010_0000;
    }
}

/// Identifies a Consortium defmt debug descriptor.
pub const MAGIC: DescriptorMagic = DescriptorMagic::new(b"CDBG");
/// On-wire layout version; bump when the descriptor/packet format changes.
pub const VERSION: u8 = 0;

// Byte offsets of each field within the shared region. All field access goes
// through these constants so the volatile reads/writes stay layout-agnostic.
pub const WRITE_PTR_OFFSET: usize = 0x06;
pub const READ_PTR_OFFSET: usize = 0x08;
pub const CAPACITY_OFFSET: usize = 0x0A;
pub const FLAGS_OFFSET: usize = 0x0C;

/// Size of the descriptor header in bytes.
pub const DESCRIPTOR_SIZE: usize = core::mem::size_of::<Descriptor>();
/// First byte usable for ring data, i.e. just past the descriptor header.
/// Both cursors wrap back to this offset rather than to 0.
const DATA_START: u16 = DESCRIPTOR_SIZE as u16;

impl Descriptor {
    /// Builds a fresh descriptor for a region of `capacity` total bytes, with
    /// both cursors parked at `DATA_START` (empty ring) and no flags set.
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
        // SAFETY: caller guarantees the descriptor header is present at `ptr`.
        let header = unsafe { DescriptorHeader::from_raw_ptr(ptr) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        let write_ptr = unsafe { rv_u16(ptr, WRITE_PTR_OFFSET) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        let read_ptr = unsafe { rv_u16(ptr, READ_PTR_OFFSET) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        let capacity = unsafe { rv_u16(ptr, CAPACITY_OFFSET) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
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
        // SAFETY: caller guarantees `ptr` covers the descriptor header.
        unsafe { DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(ptr) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        unsafe { wv_u16(ptr, WRITE_PTR_OFFSET, self.write_ptr) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        unsafe { wv_u16(ptr, READ_PTR_OFFSET, self.read_ptr) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        unsafe { wv_u16(ptr, CAPACITY_OFFSET, self.capacity) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
        unsafe { wv_u16(ptr, FLAGS_OFFSET, self.flags.bits()) };
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
        // SAFETY: caller guarantees the ring buffer covers `self.read_ptr`.
        let packet_ptr = unsafe { base.add(self.read_ptr as usize) };
        // SAFETY: `packet_ptr` points into the shared ring buffer.
        let mut frame_header = (unsafe { Packet::try_from_raw_ptr(packet_ptr) })?;

        if !frame_header.is_valid() {
            return None;
        }

        let mut buffer = vec![0u8; frame_header.len as usize];
        // SAFETY: `packet_ptr` points to the valid frame described by `frame_header`.
        unsafe { frame_header.read_payload(packet_ptr, &mut buffer) };
        // SAFETY: `packet_ptr` points to the valid frame described by `frame_header`.
        unsafe { frame_header.mark_as_consumed(packet_ptr) };
        // SAFETY: `base` points to the descriptor-owned ring buffer.
        self.read_ptr = unsafe { self.next_read_ptr(base) };
        // SAFETY: caller guarantees the descriptor fields are valid and coherent.
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
        // SAFETY: caller guarantees `base` points to this descriptor`s ring buffer.
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
    #[cfg(feature = "logger")]
    pub unsafe fn write_one_frame(&mut self, base: *mut u8, frame: &[u8]) -> bool {
        // SAFETY: caller guarantees `base` points to this descriptor`s ring buffer.
        match unsafe { self.allocate_write_ptr(base, frame.len()) } {
            Some(p) => {
                // SAFETY: `p` was allocated within the ring buffer.
                let packet_ptr = unsafe { base.add(p as usize) };
                // SAFETY: `packet_ptr` points to enough space for the full packet.
                unsafe { Packet::write_full_packet_to_raw_ptr(packet_ptr, frame) };
                self.flags.remove(Flags::Overflow);
                self.write_ptr = self.advance_ptr(p, frame.len());
                // SAFETY: caller guarantees the descriptor fields are valid and coherent.
                unsafe { wv_u16(base, WRITE_PTR_OFFSET, self.write_ptr) };
                // SAFETY: caller guarantees the descriptor fields are valid and coherent.
                unsafe { wv_u16(base, FLAGS_OFFSET, self.flags.bits()) };
                true
            }
            None => {
                self.flags.insert(Flags::Overflow);
                // SAFETY: caller guarantees the descriptor fields are valid and coherent.
                unsafe { wv_u16(base, FLAGS_OFFSET, self.flags.bits()) };
                false
            }
        }
    }

    /// Computes the `read_ptr` value after consuming the frame at the current
    /// `read_ptr`, wrapping back to `DATA_START` once it reaches `capacity`.
    ///
    /// If the current slot does not hold a valid packet header, the cursor is
    /// reset to the start of the data area.
    ///
    /// # Safety
    /// `base` must point to this descriptor's ring buffer, valid for at least
    /// `self.capacity` bytes.
    #[cfg(feature = "logger")]
    pub(crate) unsafe fn next_read_ptr(&mut self, base: *const u8) -> u16 {
        // SAFETY: caller guarantees the ring buffer covers `self.read_ptr`.
        let cursor = unsafe { base.add(self.read_ptr as usize) };
        // SAFETY: `cursor` points into the shared ring buffer.
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

    /// Reserves a write position for a packet of `frame_len` payload bytes.
    ///
    /// Returns the offset to write at, or `None` if the frame is rejected:
    /// the payload exceeds the per-frame limit (`u8::MAX`), the packet cannot
    /// fit in the data area at all, or wrapping would overwrite an unread frame
    /// (back-pressure). When the remaining tail is too small, the write wraps to
    /// `DATA_START`.
    ///
    /// # Safety
    /// `base` must point to this descriptor's ring buffer, valid for at least
    /// `self.capacity` bytes.
    #[cfg(feature = "logger")]
    pub(crate) unsafe fn allocate_write_ptr(
        &mut self,
        base: *mut u8,
        frame_len: usize,
    ) -> Option<u16> {
        // Packet length field is a single byte, so payloads larger than u8::MAX
        // cannot be represented.
        if frame_len > u8::MAX as usize {
            return None;
        }

        let packet_len = PACKET_HEADER_SIZE + frame_len;
        // Reject frames that can never fit, even in an empty ring.
        if packet_len > self.capacity as usize - DATA_START as usize {
            return None;
        }

        let mut ptr = self.write_ptr;
        // Not enough room before the end of the region: wrap to the data start.
        if ptr as usize + packet_len > self.capacity as usize {
            ptr = DATA_START;
        }

        // SAFETY: `ptr` has been wrapped into the ring buffer range.
        let cursor = unsafe { base.add(ptr as usize) };
        // SAFETY: `cursor` points into the shared ring buffer.
        // A still-valid packet at the target slot means the reader has not yet
        // consumed it; refuse to clobber it.
        if unsafe { Packet::try_from_raw_ptr(cursor) }.is_some_and(|packet| packet.is_valid()) {
            None
        } else {
            Some(ptr)
        }
    }

    /// Returns the `write_ptr` value after a packet of `frame_len` payload bytes
    /// is written at `ptr`, wrapping to `DATA_START` at the end of the region.
    #[cfg(feature = "logger")]
    fn advance_ptr(&self, ptr: u16, frame_len: usize) -> u16 {
        let next_ptr = ptr.wrapping_add((PACKET_HEADER_SIZE + frame_len) as u16);
        if next_ptr >= self.capacity {
            DATA_START
        } else {
            next_ptr
        }
    }

    /// Block until host has read all pending data. (defmt's Logger trait)
    ///
    /// Spins on each pending frame until the reader marks it consumed
    /// (invalidates its header), advancing `read_ptr` until the ring drains.
    ///
    /// # Safety
    /// `base` must point to this descriptor's ring buffer, valid for at least
    /// `self.capacity` bytes.
    #[cfg(feature = "logger")]
    pub(crate) unsafe fn flush(&mut self, base: *mut u8) {
        // SAFETY: caller guarantees the ring buffer covers `self.read_ptr`.
        let cursor = unsafe { base.add(self.read_ptr as usize) };
        // SAFETY: `cursor` points into the shared ring buffer.
        let Some(mut frame_header) = (unsafe { Packet::try_from_raw_ptr(cursor) }) else {
            return;
        };
        while self.read_ptr != self.write_ptr && self.read_ptr != DESCRIPTOR_SIZE as u16 {
            while frame_header.is_valid() {
                core::hint::spin_loop();
            }
            // SAFETY: `base` points to the descriptor-owned ring buffer.
            self.read_ptr = unsafe { self.next_read_ptr(base) };
            // SAFETY: caller guarantees the ring buffer covers `self.read_ptr`.
            let cursor = unsafe { base.add(self.read_ptr as usize) };
            // SAFETY: `cursor` points into the shared ring buffer.
            match unsafe { Packet::try_from_raw_ptr(cursor) } {
                Some(header) => frame_header = header,
                None => return,
            }
        }
    }
}
