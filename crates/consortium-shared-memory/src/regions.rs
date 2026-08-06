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

//! Concrete [`Region`] implementations.
//!
//! | Type               | Mapping              | Cache ops             | `is_coherent` |
//! |--------------------|----------------------|-----------------------|---------------|
//! | [`UncachedRegion`] | non-cacheable memory | noops                 | `true`        |
//! | [`CachedRegion`]   | cacheable memory     | architecture-specific | `false`       |
//! | [`SerialRegion`]   | no backing memory    | noops                 | `true`        |

use crate::region::Region;

#[cfg(armv7m)]
use core::ptr::NonNull;

/// AArch64 data-cache line size (bytes).  Cortex-A35/A55 all use 64 B lines.
#[cfg(target_arch = "aarch64")]
const CACHE_LINE_SIZE: usize = 64;

// ---------------------------------------------------------------------------
// UncachedRegion
// ---------------------------------------------------------------------------

/// A shared-memory region mapped with non-cacheable attributes.
///
/// CPU caches cannot retain dirty or stale copies of this mapping, so [`flush`]
/// and [`invalidate`] are both noops and
/// [`is_coherent`] returns `true`, letting the compiler eliminate dead cache
/// management code through monomorphization.
///
/// [`flush`]: Region::flush
/// [`invalidate`]: Region::invalidate
/// [`is_coherent`]: Region::is_coherent
pub struct UncachedRegion {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: shared memory is accessible from multiple cores simultaneously;
// the caller (via `new`) takes responsibility for aliasing.
unsafe impl Send for UncachedRegion {}
// SAFETY: shared memory is accessible from multiple cores simultaneously;
// the caller (via `new`) takes responsibility for aliasing.
unsafe impl Sync for UncachedRegion {}

impl UncachedRegion {
    /// Wrap a raw non-cacheable memory slice as a region.
    ///
    /// # Safety
    /// `ptr` must be valid for reads and writes for `len` bytes for the
    /// lifetime of this `UncachedRegion`. The caller must ensure no conflicting
    /// aliases exist while any `Transport` is using this region.
    pub unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

impl Clone for UncachedRegion {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
        }
    }
}

impl Region for UncachedRegion {
    unsafe fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    /// Noop - the mapping is non-cacheable.
    fn flush(&self) {}

    /// Noop - the mapping is non-cacheable.
    fn invalidate(&self) {}

    fn is_coherent(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// CachedRegion
// ---------------------------------------------------------------------------

/// A shared-memory region mapped with cacheable attributes.
///
/// On AArch64, [`flush`] performs a `dc cvac` (clean to PoC) sweep and
/// [`invalidate`] performs a `dc civac` (clean and invalidate to PoC) sweep
/// over every 64-byte cache line, each followed by a `dsb sy` barrier. `dc
/// civac` is deliberate: unlike EL1-only `dc ivac`, it is one of the data-cache
/// operations permitted at EL0 by `SCTLR_EL1.UCI`, and AP-side regions are
/// userspace `/dev/uioN` mappings.
///
/// On ARMv7-M, the Cortex-M `SCB` clean and clean-invalidate-by-address helpers
/// derive the Cortex-M7 line size from `CTR.DminLine`. ARMv8-M is intentionally
/// a no-op even though `cortex-m` exposes the API there: `CTR.DminLine` is
/// architecturally meaningless on cacheless Cortex-M33/M23 parts, so the
/// derived line size and the crate's debug assertions would be garbage. No
/// Cortex-M33 in Consortium's supported chip set has a data cache.
///
/// Host, RISC-V, and ARM Linux builds compile the operations to noops.
///
/// [`flush`]: Region::flush
/// [`invalidate`]: Region::invalidate
pub struct CachedRegion {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: same as UncachedRegion - caller manages aliasing.
unsafe impl Send for CachedRegion {}
// SAFETY: same as UncachedRegion - caller manages aliasing.
unsafe impl Sync for CachedRegion {}

impl CachedRegion {
    /// Wrap a raw cacheable memory slice as a region.
    ///
    /// # Safety
    /// `ptr` must be valid for reads and writes for `len` bytes for the
    /// lifetime of this `CachedRegion`. The caller must ensure no conflicting
    /// aliases exist while any `Transport` is using this region.
    pub unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

impl Clone for CachedRegion {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
        }
    }
}

impl Region for CachedRegion {
    unsafe fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    /// Clean (write-back) every dirty cache line in the region to memory.
    ///
    /// On AArch64 this issues `dc cvac` for each 64-byte line followed by
    /// `dsb sy`. On ARMv7-M this uses `SCB::clean_dcache_by_address`. On other
    /// targets this is a noop.
    fn flush(&self) {
        #[cfg(target_arch = "aarch64")]
        {
            let start = self.ptr as usize;
            let end = start + self.len;
            let mut addr = start & !(CACHE_LINE_SIZE - 1);
            while addr < end {
                // SAFETY: we only read the pointer value as an integer for the cache
                // maintenance instruction; we do not create a Rust reference.
                unsafe {
                    core::arch::asm!("dc cvac, {}", in(reg) addr, options(nostack));
                }
                addr += CACHE_LINE_SIZE;
            }
            // SAFETY: we only read the pointer value as an integer for the cache
            // maintenance instruction; we do not create a Rust reference.
            unsafe {
                core::arch::asm!("dsb sy", options(nostack));
            }
        }

        #[cfg(armv7m)]
        {
            // SAFETY: `SCB` is a zero-sized marker; a `&mut` to a ZST is valid at any non-null,
            // aligned address and never dereferences memory through the reference. The borrow is
            // confined to this statement. The exclusivity that matters is at the hardware level:
            // every CBP register the call writes is write-only and stateless.
            let scb = unsafe { &mut *NonNull::<cortex_m::peripheral::SCB>::dangling().as_ptr() };
            scb.clean_dcache_by_address(self.ptr as usize, self.len);
        }
    }

    /// Invalidate every cache line in the region so the next read fetches
    /// fresh data from memory.
    ///
    /// On AArch64 this issues `dc civac` for each 64-byte line followed by
    /// `dsb sy`. On ARMv7-M this uses
    /// `SCB::clean_invalidate_dcache_by_address`, matching the AArch64
    /// clean-and-invalidate semantics and avoiding the dirty-line data loss of
    /// a plain invalidate. On other targets this is a noop.
    fn invalidate(&self) {
        #[cfg(target_arch = "aarch64")]
        {
            let start = self.ptr as usize;
            let end = start + self.len;
            let mut addr = start & !(CACHE_LINE_SIZE - 1);
            while addr < end {
                // SAFETY: same as flush.
                unsafe { core::arch::asm!("dc civac, {}", in(reg) addr, options(nostack)) };
                addr += CACHE_LINE_SIZE;
            }
            // SAFETY: same as flush.
            unsafe {
                core::arch::asm!("dsb sy", options(nostack));
            }
        }

        #[cfg(armv7m)]
        {
            // SAFETY: `SCB` is a zero-sized marker; a `&mut` to a ZST is valid at any non-null,
            // aligned address and never dereferences memory through the reference. The borrow is
            // confined to this statement. The exclusivity that matters is at the hardware level:
            // every CBP register the call writes is write-only and stateless.
            let scb = unsafe { &mut *NonNull::<cortex_m::peripheral::SCB>::dangling().as_ptr() };
            scb.clean_invalidate_dcache_by_address(self.ptr as usize, self.len);
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
/// * returning a null pointer from [`as_ptr`] - callers **must not**
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

    /// Noop - no backing memory.
    fn flush(&self) {}

    /// Noop - no backing memory.
    fn invalidate(&self) {}

    fn is_coherent(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::Region;

    // -----------------------------------------------------------------------
    // UncachedRegion
    // -----------------------------------------------------------------------

    #[test]
    fn uncached_len_and_ptr() {
        let mut buf = [0u8; 64];
        let ptr = buf.as_mut_ptr();
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { UncachedRegion::new(ptr, 64) };
        assert_eq!(r.len(), 64);
        // SAFETY: the test only compares the pointer and does not dereference it.
        assert_eq!(unsafe { r.as_ptr() }, ptr);
    }

    #[test]
    fn uncached_is_not_empty() {
        let mut buf = [0u8; 8];
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { UncachedRegion::new(buf.as_mut_ptr(), 8) };
        assert!(!r.is_empty());
    }

    #[test]
    fn uncached_zero_len_is_empty() {
        // SAFETY: a null pointer is valid for a zero-length region.
        let r = unsafe { UncachedRegion::new(core::ptr::null_mut(), 0) };
        assert!(r.is_empty());
    }

    #[test]
    fn uncached_is_coherent() {
        let mut buf = [0u8; 8];
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { UncachedRegion::new(buf.as_mut_ptr(), 8) };
        assert!(r.is_coherent());
    }

    #[test]
    fn uncached_flush_and_invalidate_do_not_panic() {
        let mut buf = [0u8; 128];
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { UncachedRegion::new(buf.as_mut_ptr(), 128) };
        r.flush();
        r.invalidate();
    }

    // -----------------------------------------------------------------------
    // CachedRegion
    // -----------------------------------------------------------------------

    #[test]
    fn cached_len_and_ptr() {
        let mut buf = [0u8; 32];
        let ptr = buf.as_mut_ptr();
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { CachedRegion::new(ptr, 32) };
        assert_eq!(r.len(), 32);
        // SAFETY: the test only compares the pointer and does not dereference it.
        assert_eq!(unsafe { r.as_ptr() }, ptr);
    }

    #[test]
    fn cached_is_not_coherent() {
        let mut buf = [0u8; 8];
        // SAFETY: `buf` remains alive and uniquely borrowed for the test.
        let r = unsafe { CachedRegion::new(buf.as_mut_ptr(), 8) };
        assert!(!r.is_coherent());
    }

    #[test]
    fn cached_flush_and_invalidate_do_not_panic() {
        #[repr(align(64))]
        struct Aligned([u8; 128]);

        let mut buf = Aligned([0u8; 128]);
        // SAFETY: `buf` is cache-line aligned, remains alive, and is uniquely
        // borrowed for the test.
        let r = unsafe { CachedRegion::new(buf.0.as_mut_ptr(), 128) };
        r.flush();
        r.invalidate();
    }

    #[test]
    fn cached_maintenance_leaves_neighbouring_bytes_intact() {
        #[repr(align(64))]
        struct Aligned([u8; 192]);

        let mut buf = Aligned([0x5a; 192]);
        buf.0[64..128].fill(0xa5);
        // SAFETY: the selected window is cache-line aligned, remains alive,
        // and is uniquely borrowed for the test.
        let r = unsafe { CachedRegion::new(buf.0[64..128].as_mut_ptr(), 64) };
        r.flush();
        r.invalidate();
        assert_eq!(&buf.0[..64], &[0x5a; 64]);
        assert_eq!(&buf.0[128..], &[0x5a; 64]);
    }

    // -----------------------------------------------------------------------
    // SerialRegion
    // -----------------------------------------------------------------------

    #[test]
    fn serial_len_equals_max_frame() {
        let r = SerialRegion::new(256);
        assert_eq!(r.len(), 256);
        assert!(!r.is_empty());
    }

    #[test]
    fn serial_zero_max_frame_is_empty() {
        let r = SerialRegion::new(0);
        assert!(r.is_empty());
    }

    #[test]
    fn serial_is_coherent() {
        let r = SerialRegion::new(64);
        assert!(r.is_coherent());
    }

    #[test]
    fn serial_ptr_is_null() {
        let r = SerialRegion::new(64);
        // SAFETY: the test only checks the documented null sentinel.
        assert!(unsafe { r.as_ptr() }.is_null());
    }

    #[test]
    fn serial_flush_and_invalidate_do_not_panic() {
        let r = SerialRegion::new(64);
        r.flush();
        r.invalidate();
    }
}
