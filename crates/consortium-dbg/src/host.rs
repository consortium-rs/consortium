//! A-core shared-memory reader for the CDBG debug ring.
//!
//! The caller (A-core runtime or board integration) maps the MCU's debug SRAM region
//! and passes the base pointer and length to [`HostRing::from_raw_parts`].  This module
//! does not perform any mapping itself.
//!
//! [`HostRing`] is the allocating convenience over [`RingReader`]: it validates the
//! ring the same way and takes frames through the same
//! [`try_recv`](TryRecvTransport::try_recv), returning each as a `Vec`.
#![cfg(feature = "host")]

use consortium_core::{TryError, TryRecvTransport, UncachedRegion};

use crate::{
    descriptor::{Descriptor, MAX_FRAME_LEN},
    ring::{RingError, RingReader},
};

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
    /// The ring became inconsistent after it was opened (a cursor or capacity the
    /// writer left out of range).
    #[error("{0}")]
    Ring(RingError),
}

impl From<RingError> for HostError {
    fn from(error: RingError) -> Self {
        match error {
            RingError::MagicMismatch => Self::MagicMismatch,
            RingError::VersionMismatch => Self::VersionMismatch,
            RingError::CapacityMismatch { descriptor, mapped } => {
                Self::CapacityMismatch { descriptor, mapped }
            }
            RingError::PointerOutOfBounds { ptr, capacity } => {
                Self::PointerOutOfBounds { ptr, capacity }
            }
            other => Self::Ring(other),
        }
    }
}

/// A-core reader over an already-mapped CDBG debug ring.
///
/// Only the A-core process modifies `read_ptr`; the MCU only writes `write_ptr` and
/// packet data. The mapping is treated as uncached, as UIO maps it.
pub struct HostRing {
    reader: RingReader<UncachedRegion>,
}

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
        consortium_log::trace!(
            "HostRing::from_raw_parts(base = {:#x}, len = {:#x})",
            base as usize,
            len
        );
        // SAFETY: the caller guarantees `base` is valid for reads and writes for `len`
        // bytes while the ring lives.
        let region = unsafe { UncachedRegion::new(base, len) };
        // SAFETY: the caller guarantees no other code writes `read_ptr`, so this is
        // the ring's only reader.
        let reader = unsafe { RingReader::new(region) }?;
        let desc = reader.snapshot();
        consortium_log::trace!(
            "HostRing::from_raw_parts: descriptor capacity = {:#x}, read_ptr = {:#x}, write_ptr = {:#x}",
            desc.capacity,
            desc.read_ptr,
            desc.write_ptr
        );
        Ok(Self { reader })
    }

    /// Returns the number of bytes in the mapped region supplied at construction time.
    pub fn mapped_len(&self) -> usize {
        use consortium_core::Region;
        self.reader.region().len()
    }

    /// Returns a point-in-time snapshot of the descriptor fields read from shared memory.
    pub fn snapshot(&self) -> Descriptor {
        self.reader.snapshot()
    }

    /// Returns the next available packet payload, or `Ok(None)` when the ring is empty.
    pub fn read_packet(&mut self) -> Result<Option<Vec<u8>>, HostError> {
        let mut buf = [0u8; MAX_FRAME_LEN];
        match self.reader.try_recv(&mut buf) {
            Ok(len) => Ok(Some(buf[..len].to_vec())),
            Err(TryError::WouldBlock) => Ok(None),
            Err(TryError::Fail(error)) => Err(error.into()),
        }
    }

    /// Drains all available packets from the ring, returning them in arrival order.
    pub fn drain_packets(&mut self) -> Result<Vec<Vec<u8>>, HostError> {
        let mut packets = Vec::new();
        while let Some(packet) = self.read_packet()? {
            packets.push(packet);
        }
        Ok(packets)
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
        consortium_log::trace!("parsing defmt table from ELF bytes ({} bytes)", bytes.len());
        let table = defmt_decoder::Table::parse(bytes)
            .map_err(|e| {
                consortium_log::error!("defmt table parse failed: {:?}", e);
                HostError::ElfParse
            })?
            .ok_or_else(|| {
                consortium_log::error!("defmt table missing from ELF");
                HostError::ElfParse
            })?;
        consortium_log::info!(
            "defmt table parsed: encoding = {:?}, has_timestamp = {}",
            table.encoding(),
            table.has_timestamp()
        );
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
