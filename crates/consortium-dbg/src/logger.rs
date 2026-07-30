//! Firmware-side `defmt` sink: the `#[global_logger]` that appends encoded frames
//! to the `CDBG` ring.
//!
//! [`ConsortiumLogger`] is what makes a plain `defmt::info!` in firmware land in
//! shared memory. `defmt`'s `Logger` trait hands over raw frame bytes across an
//! `acquire()` … `write()` … `release()` span; this module encodes them, wraps the
//! result in a [`Packet`](crate::packet::Packet), and appends it to the ring.
//!
//! # Why frames are staged rather than streamed
//!
//! Two constraints meet here. `defmt` frames must be rzcobs-encoded before they
//! reach the wire, because that is what the host decoder expects — so the bytes
//! `write()` hands over are not yet the bytes to store. And a packet needs its
//! `len` in a header that precedes the payload, so the length must be known before
//! the first payload byte is written.
//!
//! So one frame is staged in a fixed buffer between `acquire()` and `release()`,
//! and only appended once complete. The buffer is sized to the packet payload limit
//! (`len` is a `u8`, so 255 bytes); a frame that overruns it is **dropped with an
//! overflow flag latched**, never truncated — a partial frame would desynchronize
//! the decoder for everything after it.
//!
//! # Why `static mut` is sound here
//!
//! The staging statics are plain `static mut` rather than atomics or a critical
//! section. That rests on a guarantee `defmt` itself provides: the
//! `acquire`/`write`/`release` span runs in a single, non-reentrant execution
//! context. Preserve that assumption when editing — every `unsafe` block below
//! cites it.
//!
//! # Warm resets
//!
//! Debug SRAM often survives a reset, so firmware that just starts writing would
//! resume into a stale ring with cursors pointing at a previous run's frames.
//! [`ConsortiumLogger::init`] resets the descriptor to an empty ring at startup;
//! generated `init()` calls it automatically when `[dbg.<core>]` declares a logger
//! for the core.

#![cfg(feature = "logger")]
use core::ptr::addr_of_mut;

use defmt::{Encoder, Logger, global_logger};

use crate::config;
use crate::descriptor::{DESCRIPTOR_SIZE, Descriptor, FLAGS_OFFSET, Flags};
use crate::utils::wv_u16;

#[global_logger]
pub struct ConsortiumLogger;

// defmt's `Logger` contract states the bytes passed to `write()` are *unencoded*:
// the logger must run them through an `Encoder` before they reach the wire, and the
// host decoder expects that rzcobs stream. These statics stage one encoded frame
// between `acquire()` and `release()`. defmt guarantees that span runs in a single,
// non-reentrant execution context, so a plain `static mut` is sufficient here.
static mut ENCODER: Encoder = Encoder::new();
/// Staging buffer for one encoded frame. Sized to the packet payload limit (the
/// packet `len` field is a `u8`); a frame whose encoding does not fit is dropped.
static mut FRAME: [u8; FRAME_CAP] = [0; FRAME_CAP];
/// Number of encoded bytes currently staged in [`FRAME`].
static mut FRAME_LEN: usize = 0;
/// Set when the encoded frame exceeded [`FRAME_CAP`] and had to be dropped.
static mut FRAME_OVERFLOW: bool = false;
const FRAME_CAP: usize = u8::MAX as usize;

/// Appends `Encoder` output to the staging buffer, latching overflow past [`FRAME_CAP`].
///
/// Used as the `write` closure for `Encoder::{start_frame, write, end_frame}`.
fn stage(bytes: &[u8]) {
    // SAFETY: only reached from `acquire()`/`write()`/`release()`, which defmt runs
    // in a single, non-reentrant execution context, so there is no aliasing access
    // to the staging statics.
    #[allow(clippy::multiple_unsafe_ops_per_block)]
    unsafe {
        let buf = &mut *addr_of_mut!(FRAME);
        for &byte in bytes {
            if FRAME_LEN < FRAME_CAP {
                buf[FRAME_LEN] = byte;
                FRAME_LEN += 1;
            } else {
                FRAME_OVERFLOW = true;
            }
        }
    }
}

impl ConsortiumLogger {
    /// Resets the descriptor at `base` to a fresh, empty ring of `size` bytes,
    /// discarding any retained contents (cursors, flags, and stale frames).
    ///
    /// Firmware calls this once at startup (see [`ConsortiumLogger::init`]) so a
    /// warm reset that retains SRAM does not resume into a stale ring. Unlike
    /// [`acquire_shared_memory`](ConsortiumLogger::acquire_shared_memory), it does
    /// not consult the existing header — it always overwrites.
    ///
    /// # Safety
    /// `base` must point to a writable debug shared-memory region of at least
    /// `size` bytes.
    pub unsafe fn force_init_shared_memory(base: *mut u8, size: u16) {
        // Clear the ring payload area before parking the cursors. A warm reset
        // retains SRAM, so stale packet headers (valid magic + `FLAG_VALID`) from
        // a previous boot survive here. `allocate_write_ptr` treats any slot that
        // still parses as a valid packet as unread and refuses to overwrite it, so
        // leaving stale headers behind spuriously latches `Overflow` and drops the
        // first real frames. Zeroing invalidates every stale header.
        for offset in DESCRIPTOR_SIZE..size as usize {
            // SAFETY: the caller guarantees `base` is writable for `size` bytes, so
            // every `offset < size` is in bounds; a byte volatile store has no
            // alignment requirement and is valid for the debug SRAM region.
            #[allow(clippy::multiple_unsafe_ops_per_block)]
            unsafe {
                base.add(offset).write_volatile(0)
            };
        }
        // SAFETY: a fresh descriptor parks both cursors at DATA_START and clears
        // all flags; the caller guarantees `base` is writable for `size` bytes.
        unsafe { Descriptor::new(size).write_to_raw_ptr(base) };
    }

    /// Initializes the descriptor at `base` when needed and marks the logger as writing.
    ///
    /// The ring is (re)initialized when the header is invalid (cold or garbage
    /// SRAM) or when a retained descriptor describes a different `capacity` than
    /// this build configured (a ring left by a differently-sized build). Writing a
    /// fresh descriptor also clears any latched `Error`/`Overflow` from a previous
    /// boot. A valid, same-capacity descriptor is left untouched so the steady-state
    /// error latch survives between log calls.
    ///
    /// # Safety
    /// `base` must point to a writable debug shared-memory region of at least `size` bytes.
    pub unsafe fn acquire_shared_memory(base: *mut u8, size: u16) {
        // SAFETY: the caller guarantees `base` is readable for the descriptor.
        let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
        if !descriptor.check() || descriptor.capacity != size {
            // SAFETY: the caller guarantees `base` is writable for `size` bytes.
            unsafe { Self::force_init_shared_memory(base, size) };
        }
        // SAFETY: the descriptor was either already present or initialized above at `base`.
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        descriptor.flags.insert(Flags::Writing);
    }

    /// Blocks until the host has consumed pending frames unless the descriptor is in error.
    ///
    /// # Safety
    /// `base` must point to a valid debug shared-memory descriptor region.
    pub unsafe fn flush_shared_memory(base: *mut u8) {
        // SAFETY: the caller guarantees `base` points at a valid descriptor.
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        if descriptor.flags.contains(Flags::Error) {
            return;
        }
        // SAFETY: the caller guarantees the descriptor owns a valid shared-memory region.
        unsafe { descriptor.flush(base) };
    }

    /// Clears the writing flag in the descriptor at `base`.
    ///
    /// # Safety
    /// `base` must point to a valid debug shared-memory descriptor region.
    pub unsafe fn release_shared_memory(base: *mut u8) {
        // SAFETY: the caller guarantees `base` points at a valid descriptor.
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        descriptor.flags.remove(Flags::Writing);
    }

    /// Latches [`Flags::Panic`] in the descriptor at `base`, preserving any
    /// error/overflow bits the writer already set.
    ///
    /// Called from the panic handler after the panic message has been written so
    /// the host reader can distinguish a clean end-of-stream from a crash. The
    /// flags word is written through the volatile helper so the store is not
    /// elided before the core halts.
    ///
    /// # Safety
    /// `base` must point to a valid debug shared-memory descriptor region.
    pub unsafe fn mark_panic(base: *mut u8) {
        // SAFETY: the caller guarantees `base` points at a valid descriptor.
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        descriptor.flags.insert(Flags::Panic);
        // SAFETY: the caller guarantees the descriptor fields are valid and coherent.
        unsafe { wv_u16(base, FLAGS_OFFSET, descriptor.flags.bits()) };
    }

    /// Writes one defmt frame payload into the descriptor-backed ring buffer.
    ///
    /// # Safety
    /// `base` must point to a valid debug shared-memory descriptor region.
    pub unsafe fn write_shared_memory(base: *mut u8, bytes: &[u8]) {
        // SAFETY: the caller guarantees `base` points at a valid descriptor.
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        if descriptor.flags.contains(Flags::Error) {
            return;
        }
        // SAFETY: the caller guarantees the descriptor owns a valid shared-memory region.
        unsafe {
            if !descriptor.write_one_frame(base, bytes) {
                descriptor.flags.insert(Flags::Overflow);
            }
        }
    }

    /// Resets the configured debug ring to a fresh, empty state.
    ///
    /// Firmware should call this once during startup, before the first
    /// `defmt::*` call, so a warm reset that retained SRAM contents does not
    /// resume into a stale ring (stale cursors or latched flags). It is a no-op
    /// when the `[dbg]` layout was not configured at build time
    /// (`config::BASE == 0`), matching the no-op behavior of the `Logger` methods.
    pub fn init() {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: `config::BASE_PTR`/`config::SIZE` describe the debug shared-memory
        // region declared in `[dbg]` and injected by the builder.
        unsafe {
            Self::force_init_shared_memory(config::BASE_PTR, config::SIZE);
        }
    }
}

// SAFETY: the implementation only accesses the debug shared-memory region described by the
// compile-time `[dbg]` layout (`config::BASE_PTR`/`config::SIZE`, injected by consortium-builder)
// and delegates all descriptor mutation to the shared-memory helpers above. When the region was
// not configured (`config::BASE` is `0`) every method is a no-op, so a plain `cargo build` links
// cleanly without touching address `0`.
unsafe impl Logger for ConsortiumLogger {
    fn acquire() {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: `config::BASE_PTR`/`config::SIZE` describe the debug shared-memory
        // region declared in `[dbg]` and injected by the builder. The staging statics
        // are only touched between `acquire()` and `release()`, a single execution
        // context per the `Logger` contract.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            Self::acquire_shared_memory(config::BASE_PTR, config::SIZE);
            FRAME_LEN = 0;
            FRAME_OVERFLOW = false;
            // Begin the rzcobs frame; encoded bytes accumulate in `FRAME` via `stage`.
            (*addr_of_mut!(ENCODER)).start_frame(stage);
        }
    }

    unsafe fn flush() {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: `config::BASE_PTR` describes the debug shared-memory region.
        unsafe {
            Self::flush_shared_memory(config::BASE_PTR);
        }
    }

    unsafe fn release() {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: `config::BASE_PTR` describes the debug shared-memory region; the
        // staging statics are exclusively owned by the current execution context
        // between `acquire()` and `release()`.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            // Finish the rzcobs frame, then commit the fully-encoded frame to the ring
            // as one packet so the host decoder receives a valid rzcobs stream.
            (*addr_of_mut!(ENCODER)).end_frame(stage);
            if FRAME_OVERFLOW {
                // The encoded frame exceeded the packet payload limit. Drop it and
                // latch `Overflow` rather than emit a truncated, undecodable frame.
                let descriptor = &mut *(config::BASE_PTR as *mut Descriptor);
                descriptor.flags.insert(Flags::Overflow);
                wv_u16(config::BASE_PTR, FLAGS_OFFSET, descriptor.flags.bits());
            } else {
                let frame =
                    core::slice::from_raw_parts(addr_of_mut!(FRAME) as *const u8, FRAME_LEN);
                Self::write_shared_memory(config::BASE_PTR, frame);
            }
            Self::release_shared_memory(config::BASE_PTR);
        }
    }

    unsafe fn write(bytes: &[u8]) {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: the staging statics are exclusively owned by the current execution
        // context between `acquire()` and `release()`; `write` runs within that span.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            (*addr_of_mut!(ENCODER)).write(bytes, stage);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{DESCRIPTOR_SIZE, Descriptor};

    const SIZE: u16 = 0x200;

    /// A zeroed region big enough for the descriptor plus some ring data.
    fn region() -> Vec<u8> {
        vec![0u8; SIZE as usize]
    }

    #[test]
    fn force_init_writes_fresh_descriptor() {
        let mut buf = region();
        // SAFETY: `buf` covers DESCRIPTOR_SIZE bytes.
        unsafe { ConsortiumLogger::force_init_shared_memory(buf.as_mut_ptr(), SIZE) };

        // SAFETY: just written above.
        let desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        assert!(desc.check());
        assert_eq!(desc.capacity, SIZE);
        assert!(desc.flags.is_empty());
        assert_eq!(desc.read_ptr, desc.write_ptr);
    }

    #[test]
    fn acquire_initializes_garbage_region() {
        let mut buf = vec![0xABu8; SIZE as usize];
        // SAFETY: `buf` covers the descriptor.
        unsafe { ConsortiumLogger::acquire_shared_memory(buf.as_mut_ptr(), SIZE) };

        // SAFETY: written above.
        let desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        assert!(desc.check());
        assert_eq!(desc.capacity, SIZE);
        assert!(desc.flags.contains(Flags::Writing));
        // A fresh ring parks both cursors at the data start, not the 0xABAB
        // garbage they held before.
        assert_eq!(desc.read_ptr, desc.write_ptr);
    }

    #[test]
    fn acquire_reinits_on_capacity_mismatch() {
        let mut buf = region();
        // A valid descriptor from a differently-sized build, with a stale cursor.
        let mut stale = Descriptor::new(SIZE / 2);
        stale.write_ptr = DESCRIPTOR_SIZE as u16 + 8;
        // SAFETY: `buf` covers the descriptor.
        unsafe { stale.write_to_raw_ptr(buf.as_mut_ptr()) };

        // SAFETY: `buf` covers the descriptor.
        unsafe { ConsortiumLogger::acquire_shared_memory(buf.as_mut_ptr(), SIZE) };

        // SAFETY: written above.
        let desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        // Capacity now matches this build and the stale cursor was reset.
        assert_eq!(desc.capacity, SIZE);
        assert_eq!(desc.read_ptr, desc.write_ptr);
    }

    #[test]
    fn force_init_clears_stale_ring_slots() {
        // Simulate a warm reset: the region is full of stale, valid-looking
        // packet headers left by a previous boot.
        let mut buf = vec![0xCDu8; SIZE as usize];
        // SAFETY: `buf` covers the whole region.
        unsafe { ConsortiumLogger::force_init_shared_memory(buf.as_mut_ptr(), SIZE) };

        // The data area must be zeroed so no stale header parses as a valid packet.
        assert!(buf[DESCRIPTOR_SIZE..].iter().all(|&byte| byte == 0));

        // A fresh write must therefore succeed and must not latch Overflow.
        // SAFETY: written above; `buf` covers the descriptor and ring.
        let mut desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        // SAFETY: `buf` owns the ring for `SIZE` bytes.
        let wrote = unsafe { desc.write_one_frame(buf.as_mut_ptr(), &[0xAA, 0xBB]) };
        assert!(wrote);
        assert!(!desc.flags.contains(Flags::Overflow));
    }

    #[test]
    fn mark_panic_latches_flag_and_preserves_others() {
        let mut buf = region();
        // A live ring that has already latched Overflow before the panic.
        let mut desc = Descriptor::new(SIZE);
        desc.flags.insert(Flags::Overflow);
        // SAFETY: `buf` covers the descriptor.
        unsafe { desc.write_to_raw_ptr(buf.as_mut_ptr()) };

        // SAFETY: `buf` covers the descriptor.
        unsafe { ConsortiumLogger::mark_panic(buf.as_mut_ptr()) };

        // SAFETY: written above.
        let desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        assert!(desc.flags.contains(Flags::Panic));
        // The pre-panic Overflow bit is not clobbered.
        assert!(desc.flags.contains(Flags::Overflow));
    }

    #[test]
    fn acquire_preserves_valid_same_capacity_ring() {
        let mut buf = region();
        // A valid, same-capacity descriptor mid-stream (write ahead of read).
        let mut live = Descriptor::new(SIZE);
        live.write_ptr = DESCRIPTOR_SIZE as u16 + 16;
        // SAFETY: `buf` covers the descriptor.
        unsafe { live.write_to_raw_ptr(buf.as_mut_ptr()) };

        // SAFETY: `buf` covers the descriptor.
        unsafe { ConsortiumLogger::acquire_shared_memory(buf.as_mut_ptr(), SIZE) };

        // SAFETY: written above.
        let desc = unsafe { Descriptor::from_raw_ptr(buf.as_ptr()) };
        // The unread data (write_ptr) survived; only the Writing flag was added.
        assert_eq!(desc.write_ptr, DESCRIPTOR_SIZE as u16 + 16);
        assert!(desc.flags.contains(Flags::Writing));
    }
}
