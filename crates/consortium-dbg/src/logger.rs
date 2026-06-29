#![cfg(feature = "logger")]
use consortium_shared_memory::descriptor::DescriptorHeader;
use defmt::{Logger, global_logger};

use crate::config;
use crate::descriptor::{Descriptor, Flags, MAGIC, VERSION};

#[global_logger]
pub struct ConsortiumLogger;

impl ConsortiumLogger {
    /// Initializes the descriptor at `base` when needed and marks the logger as writing.
    ///
    /// # Safety
    /// `base` must point to a writable debug shared-memory region of at least `size` bytes.
    pub unsafe fn acquire_shared_memory(base: *mut u8, size: u16) {
        // SAFETY: the caller guarantees `base` is readable for the descriptor header.
        let header = unsafe { DescriptorHeader::from_raw_ptr(base) };
        if header.check(MAGIC, VERSION).is_err() {
            let descriptor = Descriptor::new(size);
            // SAFETY: the caller guarantees `base` is writable for `size` bytes.
            unsafe { descriptor.write_to_raw_ptr(base) };
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
        // SAFETY: `config::BASE_PTR`/`config::SIZE` describe the debug shared-memory region
        // declared in `[dbg]` and injected by the builder.
        unsafe {
            Self::acquire_shared_memory(config::BASE_PTR, config::SIZE);
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
        // SAFETY: `config::BASE_PTR` describes the debug shared-memory region.
        unsafe {
            Self::release_shared_memory(config::BASE_PTR);
        }
    }

    unsafe fn write(bytes: &[u8]) {
        if config::BASE == 0 {
            return;
        }
        // SAFETY: `config::BASE_PTR` describes the debug shared-memory region.
        unsafe {
            Self::write_shared_memory(config::BASE_PTR, bytes);
        }
    }
}
