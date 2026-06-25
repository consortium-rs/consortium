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

//! HAL layer over the HSEM peripheral.
//!
//! [`Hsem`] is a zero-cost handle to the HSEM register window for a given
//! processor [`Side`]. The side selects this core's interrupt line and its
//! `LOCKID` (bus master id): [`Primary`] is C1 / `LOCKID` 1, [`Secondary`] is
//! C2 / `LOCKID` 2, [`Tertiary`] is C3 / `LOCKID` 3.
//!
//! HSEM is a single-instance peripheral, so `N` is always `1`. Construction
//! follows the dual-path convention: [`Hsem::steal`] (bare-metal, compile-time
//! base) and [`Hsem::new`] (Linux, `mmap`'d base).

use core::marker::PhantomData;

use consortium_ipc::Side;

use crate::registers::{
    base,
    cr::{InterruptRegister, SemaphoreRegister},
    offsets,
};

/// A HAL handle to the HSEM peripheral (instance `N`, always `1`) viewed from
/// processor side `S`.
pub struct Hsem<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

impl<S: Side, const N: usize> Hsem<S, N> {
    /// Zero-based index of instance `N` into [`base::BASE`]. Evaluating this
    /// const enforces, at compile time, that `N` names a real HSEM instance.
    const INDEX: usize = {
        assert!(
            N >= 1 && N <= base::BASE.len(),
            "HSEM instance N is out of range (HSEM is single-instance, use 1)"
        );
        N - 1
    };

    /// This core's `LOCKID` (bus master id) written when locking a semaphore:
    /// `Primary` = 1, `Secondary` = 2, `Tertiary` = 3.
    pub const LOCKID: u8 = S::INDEX + 1;

    /// Base address of the HSEM register window.
    #[inline]
    pub const fn base(&self) -> usize {
        self.base
    }

    #[inline]
    fn rx_addr(&self, sem: u8) -> *mut u32 {
        (self.base + offsets::R as usize + sem as usize * 4) as *mut u32
    }

    #[inline]
    fn rlr_addr(&self, sem: u8) -> *const u32 {
        (self.base + offsets::RLR as usize + sem as usize * 4) as *const u32
    }

    /// Offset of this core's interrupt register block (`CnIER` base).
    #[inline]
    const fn cn_block(&self) -> usize {
        // C1IER/C2IER/C3IER are 0x10 apart; ICR/ISR/MISR follow within a block.
        offsets::C1IER as usize + 0x10 * S::INDEX as usize
    }

    #[inline]
    fn ier_addr(&self) -> *mut u32 {
        (self.base + self.cn_block()) as *mut u32
    }

    #[inline]
    fn icr_addr(&self) -> *mut u32 {
        (self.base + self.cn_block() + 0x04) as *mut u32
    }

    #[inline]
    fn misr_addr(&self) -> *const u32 {
        (self.base + self.cn_block() + 0x0C) as *const u32
    }

    /// Perform a 1-step read-lock on semaphore `sem` via `HSEM_RLRx`, returning
    /// the locked register state.
    ///
    /// # Safety
    /// `sem` must be a valid semaphore id (`< 16`).
    #[inline]
    pub unsafe fn read_lock(&self, sem: u8) -> SemaphoreRegister {
        // SAFETY: `self.base` is a valid HSEM window; `sem` is in range.
        SemaphoreRegister::from_bits(unsafe { self.rlr_addr(sem).read_volatile() })
    }

    /// Write `value` to `HSEM_Rx` for semaphore `sem` (used to unlock and raise
    /// the remote interrupt).
    ///
    /// # Safety
    /// `sem` must be a valid semaphore id (`< 16`).
    #[inline]
    pub unsafe fn write_rx(&self, sem: u8, value: SemaphoreRegister) {
        // SAFETY: `self.base` is a valid HSEM window; `sem` is in range.
        unsafe { self.rx_addr(sem).write_volatile(value.bits()) };
    }

    /// Returns `true` if semaphore `sem` is currently locked (`LOCK` bit set).
    #[inline]
    pub fn is_locked(&self, sem: u8) -> bool {
        // SAFETY: `self.base` is a valid HSEM window; volatile read.
        SemaphoreRegister::from_bits(unsafe { self.rx_addr(sem).read_volatile() }).lock()
    }

    /// Enable this core's interrupt for semaphore `sem` (`CnIER[ISEm]`).
    ///
    /// # Safety
    /// `sem` must be a valid semaphore id (`< 16`); the read-modify-write must
    /// not race other `CnIER` writers.
    #[inline]
    pub unsafe fn enable_interrupt(&self, sem: u8) {
        let ier = self.ier_addr();
        // SAFETY: see method contract.
        let value =
            InterruptRegister::from_bits(unsafe { ier.read_volatile() }).sems() | (1 << sem);
        let mut reg = InterruptRegister::from_bits(0);
        reg.set_sems(value);
        // SAFETY: see method contract.
        unsafe { ier.write_volatile(reg.bits()) };
    }

    /// Disable this core's interrupt for semaphore `sem` (`CnIER[ISEm]`).
    ///
    /// # Safety
    /// `sem` must be a valid semaphore id (`< 16`); the read-modify-write must
    /// not race other `CnIER` writers.
    #[inline]
    pub unsafe fn disable_interrupt(&self, sem: u8) {
        let ier = self.ier_addr();
        // SAFETY: see method contract.
        let value =
            InterruptRegister::from_bits(unsafe { ier.read_volatile() }).sems() & !(1 << sem);
        let mut reg = InterruptRegister::from_bits(0);
        reg.set_sems(value);
        // SAFETY: see method contract.
        unsafe { ier.write_volatile(reg.bits()) };
    }

    /// Clear this core's interrupt status for semaphore `sem` (`CnICR`, W1).
    ///
    /// # Safety
    /// `sem` must be a valid semaphore id (`< 16`).
    #[inline]
    pub unsafe fn clear_interrupt(&self, sem: u8) {
        // SAFETY: see method contract.
        unsafe { self.icr_addr().write_volatile(1 << sem) };
    }

    /// Returns `true` if semaphore `sem` has a masked interrupt pending in this
    /// core's `CnMISR`.
    #[inline]
    pub fn interrupt_pending(&self, sem: u8) -> bool {
        // SAFETY: `self.base` is a valid HSEM window; volatile read.
        InterruptRegister::from_bits(unsafe { self.misr_addr().read_volatile() }).sems()
            & (1 << sem)
            != 0
    }
}

#[cfg(feature = "std")]
impl<S: Side, const N: usize> Hsem<S, N> {
    /// Create a HAL handle from an `mmap`'d virtual base address of the HSEM
    /// peripheral.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of the HSEM window. Passing a raw
    /// physical address is undefined behaviour on Linux.
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
impl<S: Side, const N: usize> Hsem<S, N> {
    /// Create a HAL handle to the HSEM peripheral using the compile-time base
    /// from [`crate::registers::base`].
    ///
    /// # Safety
    /// Grants raw access to the HSEM registers; the caller must ensure no
    /// conflicting handle for the same core exists.
    pub const unsafe fn steal() -> Self {
        Self {
            base: base::BASE[Self::INDEX],
            _side: PhantomData,
        }
    }
}
