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

//! HAL layer over a single MU instance.
//!
//! [`Mu`] is a zero-cost handle to one MU instance's register window for a
//! given processor [`Side`]. It does not own a peripheral singleton; it is
//! constructed `unsafe` from a base address and exposes the general-purpose
//! interrupt (GI) operations the doorbell layer builds on.
//!
//! Construction follows the dual-path convention:
//!
//! - bare-metal (`no_std`): [`Mu::steal`] derives the base from the compile-time
//!   [`registers::base`] tables for instance `N` and side `S`.
//! - Linux (`std`): [`Mu::new`] takes the `mmap`'d virtual base address.

use core::marker::PhantomData;

use consortium_ipc::Side;
#[cfg(not(feature = "std"))]
use consortium_ipc::{Primary, Secondary};

use crate::registers::{base, cr::GeneralInterruptRegister, offsets};

/// A HAL handle to MU instance `N` (1-based) viewed from processor side `S`.
///
/// `N` selects the hardware instance (`Mu<_, 7>` is MU7). `S` selects the
/// register window: [`Primary`] is MUA, [`Secondary`] is MUB. The i.MX95
/// CA55 <-> CM7 default route is MU7, i.e. `Mu<Primary, 7>` on the CA55 side
/// and `Mu<Secondary, 7>` on the CM7 side.
pub struct Mu<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

impl<S: Side, const N: usize> Mu<S, N> {
    /// Zero-based index of instance `N` into the [`base::BASE`] table.
    ///
    /// Evaluating this const enforces, at compile time, that `N` names a real
    /// MU instance for the selected chip.
    const INDEX: usize = {
        assert!(
            N >= 1 && N <= base::BASE.len(),
            "MU instance N is out of range for the selected chip"
        );
        N - 1
    };

    /// Base address of this instance's register window.
    #[inline]
    pub const fn base(&self) -> usize {
        self.base
    }

    #[inline]
    fn reg(&self, offset: u32) -> *mut u32 {
        (self.base + offset as usize) as *mut u32
    }

    #[inline]
    fn read(&self, offset: u32) -> GeneralInterruptRegister {
        // SAFETY: `self.base` is a valid mapped MU window (constructor
        // obligation); `offset` is a register offset within that window.
        GeneralInterruptRegister::from_bits(unsafe { self.reg(offset).read_volatile() })
    }

    #[inline]
    unsafe fn write(&self, offset: u32, value: GeneralInterruptRegister) {
        // SAFETY: caller guarantees exclusive access; `self.base` + `offset`
        // addresses a valid MU register.
        unsafe { self.reg(offset).write_volatile(value.bits()) };
    }

    /// Set `GCR[GIRn]` to signal the remote side on GI channel `gi`.
    ///
    /// Idempotent: setting an already-pending request leaves a single pending
    /// signal on the remote.
    ///
    /// # Safety
    /// `gi` must be a valid GI index (`< 4`) and the caller must coordinate
    /// concurrent access to `GCR`.
    #[inline]
    pub unsafe fn request_gi(&self, gi: u8) {
        let mut reg = self.read(offsets::GCR);
        reg.set_gi(reg.gi() | (1 << gi));
        // SAFETY: see method contract.
        unsafe { self.write(offsets::GCR, reg) };
    }

    /// Returns `true` if `GSR[GIPn]` is set, i.e. the remote side has signaled
    /// GI channel `gi`. Non-destructive.
    #[inline]
    pub fn gi_pending(&self, gi: u8) -> bool {
        self.read(offsets::GSR).gi() & (1 << gi) != 0
    }

    /// Clear `GSR[GIPn]` (write-1-to-clear) for GI channel `gi`, acknowledging
    /// the signal. Only the target bit is written.
    ///
    /// # Safety
    /// `gi` must be a valid GI index (`< 4`) and `GSR[GIPn]` should be pending.
    #[inline]
    pub unsafe fn clear_gi(&self, gi: u8) {
        // W1C: write only the target bit; do not read-modify-write.
        // SAFETY: see method contract.
        unsafe { self.write(offsets::GSR, GeneralInterruptRegister::from_bits(1 << gi)) };
    }

    /// Enable `GIER[GIEn]` to arm the GI interrupt for channel `gi`.
    ///
    /// # Safety
    /// `gi` must be a valid GI index (`< 4`) and the caller must coordinate
    /// concurrent access to `GIER`.
    #[inline]
    pub unsafe fn enable_gi(&self, gi: u8) {
        let mut reg = self.read(offsets::GIER);
        reg.set_gi(reg.gi() | (1 << gi));
        // SAFETY: see method contract.
        unsafe { self.write(offsets::GIER, reg) };
    }

    /// Disable `GIER[GIEn]` for channel `gi`.
    ///
    /// # Safety
    /// `gi` must be a valid GI index (`< 4`) and the caller must coordinate
    /// concurrent access to `GIER`.
    #[inline]
    pub unsafe fn disable_gi(&self, gi: u8) {
        let mut reg = self.read(offsets::GIER);
        reg.set_gi(reg.gi() & !(1 << gi));
        // SAFETY: see method contract.
        unsafe { self.write(offsets::GIER, reg) };
    }
}

#[cfg(feature = "std")]
impl<S: Side, const N: usize> Mu<S, N> {
    /// Create a HAL handle from an `mmap`'d virtual base address of MU instance
    /// `N`, side `S`.
    ///
    /// # Safety
    /// `base` must be the virtual address of the mapped MU window for instance
    /// `N` and the side matching `S`. Passing a raw physical address is
    /// undefined behaviour on Linux.
    pub unsafe fn new(base: usize) -> Self {
        // Force the compile-time range check on `N`.
        let _ = Self::INDEX;
        Self {
            base,
            _side: PhantomData,
        }
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> Mu<Primary, N> {
    /// Create a HAL handle to the side-A (MUA) window of MU instance `N` using
    /// the compile-time base from [`registers::base`].
    ///
    /// # Safety
    /// Grants raw access to the MU registers; the caller must ensure no
    /// conflicting handle to the same instance/side exists.
    ///
    /// [`registers::base`]: crate::registers::base
    pub const unsafe fn steal() -> Self {
        Self {
            base: base::BASE[Self::INDEX],
            _side: PhantomData,
        }
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> Mu<Secondary, N> {
    /// Create a HAL handle to the side-B (MUB) window of MU instance `N` using
    /// the compile-time base from [`registers::base`].
    ///
    /// # Safety
    /// Grants raw access to the MU registers; the caller must ensure no
    /// conflicting handle to the same instance/side exists.
    ///
    /// [`registers::base`]: crate::registers::base
    pub const unsafe fn steal() -> Self {
        Self {
            base: base::BASE[Self::INDEX] + base::LENGTH,
            _side: PhantomData,
        }
    }
}
