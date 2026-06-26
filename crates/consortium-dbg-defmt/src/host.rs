//! A-core shared-memory reader for the CDBG debug ring.
//!
//! The caller (A-core runtime or board integration) maps the MCU's debug SRAM region
//! and passes the base pointer and length to [`HostRing::from_raw_parts`].  This module
//! does not perform any mapping itself.

use consortium_shared_memory::descriptor::DescriptorHeaderError;

use crate::descriptor::{DESCRIPTOR_SIZE, Descriptor, MAGIC, VERSION};

/// Errors produced by the A-core host reader.
#[derive(thiserror::Error, Debug)]
pub enum HostError {
    /// CDBG magic bytes did not match the expected `b"CDBG"` value.
    #[error("CDBG magic mismatch")]
    MagicMismatch,
    /// CDBG version field did not match the expected version.
    #[error("CDBG version mismatch")]
    VersionMismatch,
    /// Descriptor `capacity` field is incompatible with the mapped region length.
    #[error("descriptor capacity {descriptor} exceeds mapped length {mapped}")]
    CapacityMismatch {
        /// Capacity recorded in the descriptor (0 when the region is too small to hold a header).
        descriptor: u16,
        /// Number of bytes that were actually mapped.
        mapped: usize,
    },
    /// A ring pointer lies outside the valid `[DATA_START, capacity)` range.
    #[error("ring pointer {ptr} is out of bounds (capacity {capacity})")]
    PointerOutOfBounds {
        /// The offending pointer value.
        ptr: u16,
        /// The descriptor's stated capacity.
        capacity: u16,
    },
    /// ELF bytes could not be parsed or contained no defmt table.
    #[cfg(feature = "decode")]
    #[error("ELF parsing failed or missing defmt table")]
    ElfParse,
    /// defmt stream decoding failed with a non-recoverable error.
    #[cfg(feature = "decode")]
    #[error("defmt decode error: {0:?}")]
    Decode(defmt_decoder::DecodeError),
}

/// A-core reader over an already-mapped CDBG debug ring.
///
/// Wraps the raw shared-memory pointer validated during construction. Only the A-core
/// process modifies `read_ptr`; the MCU only writes `write_ptr` and packet data.
pub struct HostRing {
    base: *mut u8,
    /// Mapped byte length (>= `desc.capacity` at construction time).
    len: usize,
}

// SAFETY: HostRing holds a raw pointer into shared SRAM. The MCU never writes read_ptr,
// and the A-core has exclusive ownership of the HostRing handle, so it is safe to send
// across threads.
unsafe impl Send for HostRing {}

impl HostRing {
    /// Wraps an already-mapped CDBG debug region.
    ///
    /// Reads the descriptor at `base` and validates the CDBG magic, version, capacity
    /// against `len`, and both ring pointers before returning.
    ///
    /// # Safety
    /// - `base` must be valid for reads and writes for at least `len` bytes.
    /// - No other code may write to `read_ptr` (descriptor offset `0x08`) for the
    ///   lifetime of the returned `HostRing`.
    pub unsafe fn from_raw_parts(base: *mut u8, len: usize) -> Result<Self, HostError> {
        if len < DESCRIPTOR_SIZE {
            return Err(HostError::CapacityMismatch {
                descriptor: 0,
                mapped: len,
            });
        }

        // SAFETY: caller guarantees base is valid for at least len >= DESCRIPTOR_SIZE bytes.
        let desc = unsafe { Descriptor::from_raw_ptr(base) };

        desc.header.check(MAGIC, VERSION).map_err(|e| match e {
            DescriptorHeaderError::BadVersion(..) => HostError::VersionMismatch,
            _ => HostError::MagicMismatch,
        })?;

        if desc.capacity as usize > len {
            return Err(HostError::CapacityMismatch {
                descriptor: desc.capacity,
                mapped: len,
            });
        }

        let data_start = DESCRIPTOR_SIZE as u16;

        if desc.read_ptr < data_start || desc.read_ptr >= desc.capacity {
            return Err(HostError::PointerOutOfBounds {
                ptr: desc.read_ptr,
                capacity: desc.capacity,
            });
        }
        if desc.write_ptr < data_start || desc.write_ptr >= desc.capacity {
            return Err(HostError::PointerOutOfBounds {
                ptr: desc.write_ptr,
                capacity: desc.capacity,
            });
        }

        Ok(Self { base, len })
    }

    /// Returns the number of bytes in the mapped region supplied at construction time.
    pub fn mapped_len(&self) -> usize {
        self.len
    }

    /// Returns a point-in-time snapshot of the descriptor fields read from shared memory.
    pub fn snapshot(&self) -> Descriptor {
        // SAFETY: base was validated in from_raw_parts to be valid for at least DESCRIPTOR_SIZE bytes.
        unsafe { Descriptor::from_raw_ptr(self.base) }
    }

    /// Returns the next available packet payload, or `Ok(None)` when the ring is empty.
    pub fn read_packet(&mut self) -> Result<Option<Vec<u8>>, HostError> {
        // SAFETY: base was validated in from_raw_parts to be valid for at least DESCRIPTOR_SIZE bytes.
        let mut desc = unsafe { Descriptor::from_raw_ptr(self.base) };
        // SAFETY: base is valid for self.len bytes; desc.capacity <= self.len (checked in from_raw_parts).
        let payload = unsafe { desc.read_one_frame(self.base) };
        Ok(payload)
    }

    /// Drains all available packets from the ring, returning them in arrival order.
    pub fn drain_packets(&mut self) -> Result<Vec<Vec<u8>>, HostError> {
        // SAFETY: base was validated in from_raw_parts to be valid for at least DESCRIPTOR_SIZE bytes.
        let mut desc = unsafe { Descriptor::from_raw_ptr(self.base) };
        // SAFETY: base is valid for self.len bytes; desc.capacity <= self.len (checked in from_raw_parts).
        let payloads = unsafe { desc.read_bulk_frames(self.base) };
        Ok(payloads)
    }
}

/// Parsed defmt table extracted from a firmware ELF file.
///
/// Constructed once per firmware image; cheap to share across [`DefmtSession`]s.
#[cfg(feature = "decode")]
pub struct DefmtContext {
    table: defmt_decoder::Table,
}

#[cfg(feature = "decode")]
impl DefmtContext {
    /// Parses the defmt symbol table from firmware ELF bytes.
    ///
    /// Returns `Err(HostError::ElfParse)` when the bytes are not a valid ELF file or
    /// when the ELF contains no `.defmt` section.
    pub fn from_elf_bytes(bytes: &[u8]) -> Result<Self, HostError> {
        let table = defmt_decoder::Table::parse(bytes)
            .map_err(|_| HostError::ElfParse)?
            .ok_or(HostError::ElfParse)?;
        Ok(Self { table })
    }
}

/// Stateful defmt stream decoder tied to a [`DefmtContext`].
///
/// Feed raw packet payloads from a [`HostRing`] into [`push_packet`]; the session
/// accumulates bytes across calls and invokes the callback for each complete frame.
///
/// The lifetime `'ctx` is that of the backing [`DefmtContext`]; the session must not
/// outlive it.
///
/// [`push_packet`]: DefmtSession::push_packet
#[cfg(feature = "decode")]
pub struct DefmtSession<'ctx> {
    decoder: Box<dyn defmt_decoder::StreamDecoder + 'ctx>,
}

#[cfg(feature = "decode")]
impl<'ctx> DefmtSession<'ctx> {
    /// Creates a new decoding session backed by `ctx`.
    pub fn new(ctx: &'ctx DefmtContext) -> Self {
        Self {
            decoder: ctx.table.new_stream_decoder(),
        }
    }

    /// Feeds `bytes` into the stream decoder and invokes `on_frame` for each complete frame.
    ///
    /// A packet payload may contain a partial defmt frame; `UnexpectedEof` from the
    /// decoder is treated as "need more bytes" and returns `Ok(())`.  Any other decode
    /// error is returned as `Err(HostError::Decode(...))`.
    pub fn push_packet(
        &mut self,
        bytes: &[u8],
        mut on_frame: impl FnMut(&defmt_decoder::Frame<'_>),
    ) -> Result<(), HostError> {
        self.decoder.received(bytes);
        loop {
            match self.decoder.decode() {
                Ok(frame) => on_frame(&frame),
                Err(defmt_decoder::DecodeError::UnexpectedEof) => return Ok(()),
                Err(e) => return Err(HostError::Decode(e)),
            }
        }
    }
}
