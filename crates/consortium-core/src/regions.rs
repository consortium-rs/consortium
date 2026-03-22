//! Concrete [`Region`] implementations.
//!
//! | Type           | Backing memory | Cache ops            | `is_coherent` |
//! |----------------|---------------|----------------------|---------------|
//! | [`SramRegion`] | SRAM slice    | noops                | `true`        |
//! | [`DdrRegion`]  | DDR slice     | real dcache insns    | `false`       |
//! | [`SerialRegion`]| none         | noops                | `true`        |

use crate::region::Region;

/// AArch64 data-cache line size (bytes).  Cortex-A35/A55 all use 64 B lines.
#[cfg(target_arch = "aarch64")]
const CACHE_LINE_SIZE: usize = 64;

// ---------------------------------------------------------------------------
// SramRegion
// ---------------------------------------------------------------------------

/// A region backed by on-chip SRAM.
///
/// SRAM is always cache-coherent on STM32MP2 (it lives inside the coherent
/// interconnect), so [`flush`] and [`invalidate`] are both noops and
/// [`is_coherent`] returns `true`, letting the compiler eliminate dead cache
/// management code through monomorphization.
///
/// [`flush`]: Region::flush
/// [`invalidate`]: Region::invalidate
/// [`is_coherent`]: Region::is_coherent
pub struct SramRegion {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: SRAM is accessible from multiple cores simultaneously;
// the caller (via `new`) takes responsibility for aliasing.
unsafe impl Send for SramRegion {}
unsafe impl Sync for SramRegion {}

impl SramRegion {
    /// Wrap a raw SRAM slice as a region.
    ///
    /// # Safety
    /// `ptr` must be valid for reads and writes for `len` bytes for the
    /// lifetime of this `SramRegion`.  The caller must ensure no conflicting
    /// aliases exist while any `Transport` is using this region.
    pub unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

impl Region for SramRegion {
    unsafe fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    /// Noop — SRAM is cache-coherent.
    fn flush(&self) {}

    /// Noop — SRAM is cache-coherent.
    fn invalidate(&self) {}

    fn is_coherent(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// DdrRegion
// ---------------------------------------------------------------------------

/// A region backed by DDR DRAM.
///
/// DDR is *not* coherent between the Cortex-A and Cortex-M cores on
/// STM32MP2, so [`flush`] performs a `dc cvac` (clean to PoC) sweep and
/// [`invalidate`] performs a `dc ivac` sweep over every cache line in the
/// region, each followed by a `dsb sy` barrier.
///
/// On non-AArch64 targets the cache operations compile away to noops so
/// that host-side tests still build without special configuration.
///
/// [`flush`]: Region::flush
/// [`invalidate`]: Region::invalidate
pub struct DdrRegion {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: same as SramRegion — caller manages aliasing.
unsafe impl Send for DdrRegion {}
unsafe impl Sync for DdrRegion {}

impl DdrRegion {
    /// Wrap a raw DDR slice as a region.
    ///
    /// # Safety
    /// `ptr` must be valid for reads and writes for `len` bytes for the
    /// lifetime of this `DdrRegion`.  The caller must ensure no conflicting
    /// aliases exist while any `Transport` is using this region.
    pub unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

impl Region for DdrRegion {
    unsafe fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    /// Clean (write-back) every dirty cache line in the region to DDR.
    ///
    /// On AArch64 this issues `dc cvac` for each 64-byte line followed by
    /// `dsb sy`.  On other targets this is a noop.
    fn flush(&self) {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: we only read the pointer value as an integer for the cache
        // maintenance instruction; we do not create a Rust reference.
        unsafe {
            let start = self.ptr as usize;
            let end = start + self.len;
            let mut addr = start & !(CACHE_LINE_SIZE - 1);
            while addr < end {
                core::arch::asm!("dc cvac, {}", in(reg) addr, options(nostack));
                addr += CACHE_LINE_SIZE;
            }
            core::arch::asm!("dsb sy", options(nostack));
        }
    }

    /// Invalidate every cache line in the region so the next read fetches
    /// fresh data from DDR.
    ///
    /// On AArch64 this issues `dc ivac` for each 64-byte line followed by
    /// `dsb sy`.  On other targets this is a noop.
    fn invalidate(&self) {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: same as flush.
        unsafe {
            let start = self.ptr as usize;
            let end = start + self.len;
            let mut addr = start & !(CACHE_LINE_SIZE - 1);
            while addr < end {
                core::arch::asm!("dc ivac, {}", in(reg) addr, options(nostack));
                addr += CACHE_LINE_SIZE;
            }
            core::arch::asm!("dsb sy", options(nostack));
        }
    }

    fn is_coherent(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// SerialRegion
// ---------------------------------------------------------------------------

/// A virtual region for serial-line transports (UART, SWD trace, …).
///
/// Serial channels have no shared backing memory; the transport reads and
/// writes the peripheral registers directly.  `SerialRegion` satisfies the
/// [`Region`] contract by:
///
/// * returning a null pointer from [`as_ptr`] — callers **must not**
///   dereference it;
/// * reporting `max_frame` as [`len`], which the `Transport` implementation
///   uses to size its internal frame buffer;
/// * treating all cache operations as noops and [`is_coherent`] as `true`,
///   because there are no cache lines to manage.
///
/// [`as_ptr`]: Region::as_ptr
/// [`len`]: Region::len
/// [`is_coherent`]: Region::is_coherent
pub struct SerialRegion {
    max_frame: usize,
}

impl SerialRegion {
    /// Create a `SerialRegion` whose reported length is `max_frame` bytes.
    pub fn new(max_frame: usize) -> Self {
        Self { max_frame }
    }
}

impl Region for SerialRegion {
    /// Returns a null pointer.
    ///
    /// # Safety
    /// The returned pointer **must not** be dereferenced.  It is provided
    /// only so that generic code that queries region metadata via the trait
    /// can call this method without branching on the concrete type.
    unsafe fn as_ptr(&self) -> *mut u8 {
        core::ptr::null_mut()
    }

    /// Maximum frame payload size in bytes.
    fn len(&self) -> usize {
        self.max_frame
    }

    /// Noop — no backing memory.
    fn flush(&self) {}

    /// Noop — no backing memory.
    fn invalidate(&self) {}

    fn is_coherent(&self) -> bool {
        true
    }
}
