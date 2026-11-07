// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! The `CDBG` ring as a Consortium transport.
//!
//! A log ring is the simplest memory-backed link there is: one writer (firmware),
//! one reader (the host), no doorbell, and a hand-off that never waits — a frame
//! either fits in the ring or is dropped with [`Flags::Overflow`] latched. That is
//! exactly `consortium-core`'s non-waiting pair, so the ring implements it:
//!
//! - [`RingWriter`] is a [`TrySendTransport`]: [`try_send`](TrySendTransport::try_send)
//!   appends one frame, or reports [`TryError::WouldBlock`] when the reader has not
//!   yet freed enough room. It never waits, so the `defmt` logger and the panic
//!   handler can call it; [`ConsortiumLogger`](crate::logger::ConsortiumLogger)
//!   writes every frame through it.
//! - [`RingReader`] is a [`TryRecvTransport`]: [`try_recv`](TryRecvTransport::try_recv)
//!   copies the next frame into a caller buffer, allocation-free, so a `no_std`
//!   reader works as well as the host's [`HostRing`](crate::host::HostRing), which
//!   is built on it.
//!
//! With no doorbell there is nothing to park on; a reader that wants to wait polls,
//! or spins with `consortium_core::wait::spin_until`.
//!
//! Both sides keep the ring's descriptor discipline: the writer only writes
//! `write_ptr`, `capacity`, `flags` and packet data; the reader only writes
//! `read_ptr` and each packet's consumed bit. Both bracket their accesses with the
//! region's cache maintenance — invalidate before reading the other side's fields,
//! flush after writing their own — so a [`CachedRegion`](consortium_core::CachedRegion)
//! works as well as an [`UncachedRegion`](consortium_core::UncachedRegion).

use consortium_core::{
    Region, TransportError, TryError, TryRecvTransport, TrySendTransport,
    descriptor::DescriptorHeaderError,
};

use crate::{
    descriptor::{
        DESCRIPTOR_SIZE, Descriptor, FLAGS_OFFSET, Flags, MAGIC, MAX_FRAME_LEN, ReadOutcome,
        VERSION,
    },
    packet::PACKET_HEADER_SIZE,
    utils::wv_u16,
};

/// Why a ring could not be opened, or a frame could not be moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingError {
    /// The region does not start with the `CDBG` magic.
    MagicMismatch,
    /// The descriptor is a `CDBG` version this crate cannot read.
    VersionMismatch,
    /// The descriptor's `capacity` does not fit the region.
    CapacityMismatch {
        /// Capacity recorded in the descriptor (0 when the region is too small to
        /// hold a descriptor at all).
        descriptor: u16,
        /// The region's length.
        mapped: usize,
    },
    /// A ring cursor lies outside `[DATA_START, capacity)`.
    PointerOutOfBounds {
        /// The offending cursor.
        ptr: u16,
        /// The descriptor's capacity.
        capacity: u16,
    },
    /// A region to initialize is too short for a descriptor and one frame, or
    /// longer than a `u16` capacity can describe.
    RegionLength {
        /// The region's length.
        len: usize,
    },
    /// The frame can never fit: it is longer than one packet carries, or than the
    /// ring's data area. It was dropped and [`Flags::Overflow`] latched.
    FrameTooLarge {
        /// The frame's length.
        len: usize,
        /// The longest frame this ring accepts.
        max: usize,
    },
    /// The next frame is longer than the receive buffer; it was left unread.
    BufferTooSmall {
        /// The buffer's length.
        len: usize,
        /// The frame's length.
        needed: usize,
    },
    /// The writer latched [`Flags::Error`]; the ring accepts no more frames.
    Faulted,
}

impl core::fmt::Display for RingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MagicMismatch => f.write_str("CDBG magic mismatch"),
            Self::VersionMismatch => f.write_str("CDBG version mismatch"),
            Self::CapacityMismatch { descriptor, mapped } => write!(
                f,
                "descriptor capacity {descriptor} exceeds mapped length {mapped}"
            ),
            Self::PointerOutOfBounds { ptr, capacity } => write!(
                f,
                "ring pointer {ptr} is out of bounds (capacity {capacity})"
            ),
            Self::RegionLength { len } => write!(f, "a {len}-byte region cannot hold a ring"),
            Self::FrameTooLarge { len, max } => {
                write!(f, "a {len}-byte frame exceeds the ring's {max}-byte limit")
            }
            Self::BufferTooSmall { len, needed } => {
                write!(f, "a {needed}-byte frame does not fit a {len}-byte buffer")
            }
            Self::Faulted => f.write_str("the ring writer latched an error"),
        }
    }
}

impl core::error::Error for RingError {}

/// Read and validate the descriptor at the start of `region`.
///
/// Checks the magic, the version, that `capacity` fits the region, and that both
/// cursors lie in the data area — so every later cursor dereference stays inside
/// the region.
pub(crate) fn validate<R: Region>(region: &R) -> Result<Descriptor, RingError> {
    let len = region.len();
    if len < DESCRIPTOR_SIZE {
        return Err(RingError::CapacityMismatch {
            descriptor: 0,
            mapped: len,
        });
    }
    region.invalidate();
    // SAFETY: the region is valid for `len >= DESCRIPTOR_SIZE` bytes.
    let desc = unsafe { Descriptor::from_raw_ptr(base(region)) };
    desc.header.check(MAGIC, VERSION).map_err(|e| match e {
        DescriptorHeaderError::BadVersion(..) => RingError::VersionMismatch,
        _ => RingError::MagicMismatch,
    })?;
    if desc.capacity as usize > len {
        return Err(RingError::CapacityMismatch {
            descriptor: desc.capacity,
            mapped: len,
        });
    }
    let data_start = DESCRIPTOR_SIZE as u16;
    for ptr in [desc.read_ptr, desc.write_ptr] {
        if ptr < data_start || ptr >= desc.capacity {
            return Err(RingError::PointerOutOfBounds {
                ptr,
                capacity: desc.capacity,
            });
        }
    }
    Ok(desc)
}

fn base<R: Region>(region: &R) -> *mut u8 {
    // SAFETY: `Region::as_ptr` only obliges the caller to stay within `len` bytes;
    // every access below is bounded by a descriptor `capacity` checked against it.
    unsafe { region.as_ptr() }
}

/// The longest frame a ring of `capacity` bytes accepts.
fn max_frame(capacity: u16) -> usize {
    (capacity as usize)
        .saturating_sub(DESCRIPTOR_SIZE + PACKET_HEADER_SIZE)
        .min(MAX_FRAME_LEN)
}

/// The firmware end of a `CDBG` ring: appends frames, never waits.
pub struct RingWriter<R> {
    region: R,
    capacity: u16,
}

impl<R: Region> RingWriter<R> {
    /// Reset `region` to a fresh, empty ring and write into it.
    ///
    /// Debug SRAM often survives a warm reset, so the data area is zeroed before the
    /// cursors are parked: a stale packet header that still parses as valid would
    /// otherwise read as unread data and refuse the first real frames.
    ///
    /// # Safety
    ///
    /// This must be the ring's only writer for as long as it lives: two writers
    /// would race on `write_ptr` and the packet data.
    pub unsafe fn init(region: R) -> Result<Self, RingError> {
        let len = region.len();
        let Ok(capacity) = u16::try_from(len) else {
            return Err(RingError::RegionLength { len });
        };
        if max_frame(capacity) == 0 {
            return Err(RingError::RegionLength { len });
        }
        let base = base(&region);
        for offset in DESCRIPTOR_SIZE..len {
            // SAFETY: `offset < len`, inside the region; a byte volatile store has no
            // alignment requirement and is valid for device or normal memory.
            #[allow(clippy::multiple_unsafe_ops_per_block)]
            unsafe {
                base.add(offset).write_volatile(0)
            };
        }
        // SAFETY: a fresh descriptor parks both cursors at the data start and clears
        // every flag; `len >= DESCRIPTOR_SIZE` was checked through `max_frame`.
        unsafe { Descriptor::new(capacity).write_to_raw_ptr(base) };
        region.flush();
        Ok(Self { region, capacity })
    }

    /// Continue writing into the ring already in `region`, after validating it.
    ///
    /// # Safety
    ///
    /// As for [`init`](Self::init): this must be the ring's only writer.
    pub unsafe fn attach(region: R) -> Result<Self, RingError> {
        let capacity = validate(&region)?.capacity;
        Ok(Self { region, capacity })
    }

    /// Continue writing into a ring of `capacity` bytes the caller has just
    /// validated or initialized, skipping [`attach`](Self::attach)'s checks.
    ///
    /// For the logger's per-frame path, where `acquire()` has already done that
    /// work for this very frame.
    ///
    /// # Safety
    ///
    /// As for [`init`](Self::init), and the descriptor at the start of `region`
    /// must be a valid `CDBG` descriptor of exactly `capacity` bytes, with
    /// `capacity <= region.len()`.
    #[cfg(feature = "logger")]
    pub(crate) unsafe fn attach_unchecked(region: R, capacity: u16) -> Self {
        Self { region, capacity }
    }

    /// The region back.
    pub fn into_region(self) -> R {
        self.region
    }

    /// A point-in-time copy of the descriptor.
    pub fn snapshot(&self) -> Descriptor {
        self.region.invalidate();
        // SAFETY: the descriptor was validated or written at construction.
        unsafe { Descriptor::from_raw_ptr(base(&self.region)) }
    }

    /// Latch [`Flags::Panic`], keeping every other flag, so the reader can tell a
    /// crash from a clean end of stream.
    pub fn mark_panic(&mut self) {
        let mut desc = self.snapshot();
        desc.flags.insert(Flags::Panic);
        // SAFETY: the flags word is writer-owned and inside the descriptor.
        unsafe { wv_u16(base(&self.region), FLAGS_OFFSET, desc.flags.bits()) };
        self.region.flush();
    }
}

impl<R: Region> TransportError for RingWriter<R> {
    type Error = RingError;
}

/// Appends one frame as one packet.
///
/// - `Ok`: the frame is in the ring and [`Flags::Overflow`] is cleared.
/// - [`TryError::WouldBlock`]: the ring is full of unread frames; the frame was
///   dropped and `Overflow` latched. A later attempt succeeds once the reader
///   drains.
/// - [`RingError::FrameTooLarge`]: the frame can never fit; dropped, `Overflow`
///   latched.
/// - [`RingError::Faulted`]: the writer latched [`Flags::Error`]; nothing written.
///
/// Logs nothing: it runs inside the `defmt` logger, which must not re-enter
/// `defmt`.
impl<R: Region> TrySendTransport for RingWriter<R> {
    fn try_send(&mut self, frame: &[u8]) -> Result<(), TryError<RingError>> {
        let base = base(&self.region);
        // Observe the reader's consumed bits and `read_ptr`, not a stale line.
        self.region.invalidate();
        // SAFETY: the descriptor was validated or written at construction.
        let mut desc = unsafe { Descriptor::from_raw_ptr(base) };
        if desc.flags.contains(Flags::Error) {
            return Err(TryError::Fail(RingError::Faulted));
        }
        // `capacity` is this writer's own field, so a different value means the ring
        // was re-initialized underneath it; never trust it past the region.
        if desc.capacity != self.capacity {
            return Err(TryError::Fail(RingError::CapacityMismatch {
                descriptor: desc.capacity,
                mapped: self.region.len(),
            }));
        }
        // SAFETY: `base` owns this descriptor's ring, `capacity` bytes long and
        // inside the region (checked at construction).
        let wrote = unsafe { desc.write_one_frame(base, frame) };
        self.region.flush();
        if wrote {
            return Ok(());
        }
        let max = self.max_send_size();
        if frame.len() > max {
            Err(TryError::Fail(RingError::FrameTooLarge {
                len: frame.len(),
                max,
            }))
        } else {
            Err(TryError::WouldBlock)
        }
    }

    fn max_send_size(&self) -> usize {
        max_frame(self.capacity)
    }
}

/// The reading end of a `CDBG` ring: takes frames, never waits, never allocates.
pub struct RingReader<R> {
    region: R,
}

impl<R: Region> RingReader<R> {
    /// Read the ring in `region`, after validating its descriptor.
    ///
    /// # Safety
    ///
    /// This must be the ring's only reader for as long as it lives: two readers
    /// would race on `read_ptr`.
    pub unsafe fn new(region: R) -> Result<Self, RingError> {
        validate(&region)?;
        Ok(Self { region })
    }

    /// The region back.
    pub fn into_region(self) -> R {
        self.region
    }

    /// The region this reader reads.
    pub fn region(&self) -> &R {
        &self.region
    }

    /// A point-in-time copy of the descriptor.
    pub fn snapshot(&self) -> Descriptor {
        self.region.invalidate();
        // SAFETY: the descriptor was validated at construction.
        unsafe { Descriptor::from_raw_ptr(base(&self.region)) }
    }
}

impl<R: Region> TransportError for RingReader<R> {
    type Error = RingError;
}

/// Copies the next frame into `buf` and consumes it.
///
/// [`TryError::WouldBlock`] means the ring is empty. A frame longer than `buf` is
/// [`RingError::BufferTooSmall`] and stays unread, so the caller can retry with a
/// buffer of [`max_recv_size`](TryRecvTransport::max_recv_size) bytes, which
/// always fits.
impl<R: Region> TryRecvTransport for RingReader<R> {
    fn try_recv(&mut self, buf: &mut [u8]) -> Result<usize, TryError<RingError>> {
        let mut desc = self.snapshot();
        // The writer owns `capacity` and the reader owns `read_ptr`, but both live in
        // memory the other core can write: re-check them on every attempt so a
        // misbehaving writer cannot steer a read outside the region.
        if desc.capacity as usize > self.region.len() {
            return Err(TryError::Fail(RingError::CapacityMismatch {
                descriptor: desc.capacity,
                mapped: self.region.len(),
            }));
        }
        if (desc.read_ptr as usize) < DESCRIPTOR_SIZE || desc.read_ptr >= desc.capacity {
            return Err(TryError::Fail(RingError::PointerOutOfBounds {
                ptr: desc.read_ptr,
                capacity: desc.capacity,
            }));
        }
        // SAFETY: `capacity` fits the region and `read_ptr` lies in the data area
        // (both checked just above); `try_read_frame` refuses any packet that would
        // run past `capacity`.
        let outcome = unsafe { desc.try_read_frame(base(&self.region), buf) };
        match outcome {
            ReadOutcome::Empty => Err(TryError::WouldBlock),
            ReadOutcome::Frame(len) => {
                self.region.flush();
                Ok(len)
            }
            ReadOutcome::TooSmall { needed } => Err(TryError::Fail(RingError::BufferTooSmall {
                len: buf.len(),
                needed,
            })),
        }
    }

    fn max_recv_size(&self) -> usize {
        MAX_FRAME_LEN
    }
}
