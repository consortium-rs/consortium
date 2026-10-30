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

//! STM32 hardware semaphore (HSEM) register model.
//!
//! **Frozen: prefer [`ipcc`](super::ipcc) for new STM32MP2 work.** IPCC is present
//! across STM32MP21/23/25 and has the better-supported ST integration path. HSEM is
//! absent on STM32MP21, and field testing has shown surprising access failures where
//! the peripheral clock and RIF assignment looked correct but reads still produced a
//! bus error. This model is kept for experiments and existing STM32MP23/25 users.
//!
//! HSEM is a lock peripheral, not a mailbox, so signaling is a side effect of the
//! locking protocol:
//!
//! | Operation      | HSEM action                                                     |
//! | -------------- | --------------------------------------------------------------- |
//! | `ring`         | 1-step read-lock (`RLRn`), then unlock (`Rn`); the unlock raises the remote interrupt |
//! | `raw_pending`  | this bank's raw status (`CnISR`)                                 |
//! | `ack`          | `CnICR` write-one-to-clear                                       |
//! | `arm`/`disarm` | `CnIER`                                                          |
//!
//! `ring` reports [`BellError::Busy`] when another core holds the semaphore.
//!
//! # Banks
//!
//! HSEM is a single instance with an interrupt bank per core: [`Primary`] is C1,
//! [`Secondary`] C2, and [`Tertiary`] C3, so `N` is always `1`
//! (`HsemDoorbell<Primary, 1>`). The bank also sets the lock id this side writes.
//! Its channels are the 16 semaphores.
//!
//! # Construction
//!
//! - **Firmware** — [`HsemDoorbell::new`] consumes the chip HAL's `HSEM` singleton.
//! - **Linux/UIO** — [`BellA::new_mapped`] takes an `mmap`'d virtual base.
//!
//! The non-secure `HSEM` vector forwards into
//! [`service_interrupt::<Hsem<S, 1>>()`](super::service_interrupt), which
//! `consortium-cfg` generates. Naming the bank lets the vector read `CnISR` and mask
//! exactly the semaphores that fired, like every other Type-A bell.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary
//! [`Tertiary`]: consortium_ipc::Tertiary
//! [`BellA::new_mapped`]: super::BellA::new_mapped

pub mod registers;

use core::{marker::PhantomData, ops::Range};

use consortium_hal_stm32mp2::{Peri, PeripheralType};
use consortium_ipc::Side;

use self::registers::{
    base,
    cr::{InterruptRegister, SemaphoreRegister},
    offsets,
};
use super::{BellA, BellError, BellHw, LineSet};

/// Number of semaphores, and so channels.
pub const HSEM_CHAN_COUNT: u8 = 16;

/// Semaphore ids reserved from user channels.
pub const HSEM_CHAN_RESERVED: Range<u8> = 0..2;

/// The HSEM doorbell for interrupt bank `S`; `N` is always `1`.
pub type HsemDoorbell<S, const N: usize> = BellA<Hsem<S, N>>;

/// A chip-HAL peripheral singleton naming the HSEM block.
pub trait Instance: PeripheralType {
    /// 1-based instance number; HSEM has one.
    const INSTANCE: usize;
}

impl Instance for consortium_hal_stm32mp2::peripherals::HSEM {
    const INSTANCE: usize = 1;
}

impl<S: Side, const N: usize> HsemDoorbell<S, N> {
    /// Create a doorbell over the HSEM block, bank `S`, from the chip HAL's
    /// peripheral singleton.
    ///
    /// The window address comes from [`base::BASE`]. On Linux the window is an
    /// `mmap` with no singleton — use [`new_mapped`](BellA::new_mapped) there.
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        const {
            assert!(
                T::INSTANCE == N,
                "HsemDoorbell instance number does not match the peripheral singleton",
            );
        }
        // The singleton is proof of exclusivity, not state: `init()` issues it once
        // and the `'static` bound means nothing hands it back. It is deliberately
        // not stored: `split()` clones the doorbell, and other doorbells may drive
        // disjoint semaphores of the same block.
        let _ = peri;
        // SAFETY: `PHYS_BASE` is the HSEM window from this crate's vetted address
        // table. The consumed singleton keeps every other driver off it; another
        // doorbell on it can only come from a `steal()` whose caller guarantees
        // disjoint semaphores, and the shared enable-register writes run in a
        // critical section.
        unsafe { Self::new_mapped(Hsem::<S, N>::PHYS_BASE) }
    }
}

/// A handle to the HSEM register window, viewed from interrupt bank `S`.
///
/// A thin, checked wrapper over MMIO: lock, unlock, and this bank's interrupt
/// registers for one semaphore.
pub struct Hsem<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

// Manual `Clone` so the bound is independent of `S` (the side markers are
// zero-sized and not themselves `Clone`).
impl<S: Side, const N: usize> Clone for Hsem<S, N> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            _side: PhantomData,
        }
    }
}

impl<S: Side, const N: usize> Hsem<S, N> {
    /// Zero-based instance index. Evaluating this const enforces, at compile time,
    /// that `N` is `1` and that `S` names one of the three interrupt banks.
    const INDEX: usize = {
        assert!(S::INDEX < 3, "HSEM has three interrupt banks (C1..C3)");
        assert!(
            N >= 1 && N <= base::BASE.len(),
            "HSEM instance N is out of range (HSEM is single-instance, use 1)"
        );
        N - 1
    };

    /// The lock id this bank writes: core `Cn` locks with id `n`.
    pub const LOCKID: u8 = S::INDEX + 1;

    /// Base address of the register window.
    #[inline]
    pub const fn base(&self) -> usize {
        self.base
    }

    #[inline]
    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    #[inline]
    fn read(&self, offset: usize) -> u32 {
        // SAFETY: `self.base + offset` addresses a valid HSEM register (constructor
        // obligation); volatile read.
        unsafe { self.reg(offset).read_volatile() }
    }

    #[inline]
    unsafe fn write(&self, offset: usize, value: u32) {
        // SAFETY: see `read`; the caller coordinates write access.
        unsafe { self.reg(offset).write_volatile(value) };
    }

    /// This bank's `CnIER`; `CnICR`, `CnISR`, and `CnMISR` follow at `+4`, `+8`,
    /// `+0xC`, and the banks are `0x10` apart.
    #[inline]
    const fn bank() -> usize {
        offsets::C1IER as usize + 0x10 * S::INDEX as usize
    }

    #[inline]
    fn sems(&self, offset: usize) -> u32 {
        InterruptRegister::from_bits(self.read(offset)).sems()
    }

    /// Set (`on = true`) or clear the `CnIER` bits in `sems`.
    #[inline]
    unsafe fn set_ier(&self, sems: u32, on: bool) {
        let current = self.sems(Self::bank());
        let mut reg = InterruptRegister::from_bits(0);
        reg.set_sems(if on { current | sems } else { current & !sems });
        // SAFETY: the caller serializes the IER read-modify-write.
        unsafe { self.write(Self::bank(), reg.bits()) };
    }
}

// SAFETY: `PHYS_BASE` comes from the vetted `base::BASE` table; `raw_pending` and
// `asserted` read the raw `CnISR`, not `CnMISR`; every write touches only the
// semaphore bits it is given; status bit `n` is semaphore `n`, `< 16`.
unsafe impl<S: Side, const N: usize> BellHw for Hsem<S, N> {
    type Line = u8;

    const PHYS_BASE: usize = base::BASE[Self::INDEX];

    const CHANNELS: u8 = HSEM_CHAN_COUNT;

    const RESERVED: Range<u8> = HSEM_CHAN_RESERVED;

    unsafe fn at(base: usize) -> Self {
        // Force the compile-time checks on `N` and `S`.
        let _ = Self::INDEX;
        Self {
            base,
            _side: PhantomData,
        }
    }

    #[inline]
    fn line(id: u8) -> u8 {
        id
    }

    fn ring(&self, sem: u8) -> Result<(), BellError> {
        let rx = offsets::R as usize + usize::from(sem) * 4;
        // 1-step read-lock: reading `RLRn` locks the semaphore for this core's id
        // and returns the result.
        let locked =
            SemaphoreRegister::from_bits(self.read(offsets::RLR as usize + usize::from(sem) * 4));
        if locked.lock() && locked.lockid() as u8 != Self::LOCKID {
            return Err(BellError::Busy);
        }
        // Unlocking with the same id and LOCK = 0 raises the remote interrupt.
        let mut unlock = locked;
        unlock.set_lock(false);
        // SAFETY: `sem < 16`, so `Rn` is in the window; this core holds the lock.
        unsafe { self.write(rx, unlock.bits()) };
        Ok(())
    }

    #[inline]
    fn raw_pending(&self, sem: u8) -> bool {
        self.sems(Self::bank() + 0x08) & (1 << sem) != 0
    }

    unsafe fn ack(&self, sem: u8) {
        // SAFETY: `CnICR` is write-one-to-clear; only this semaphore's bit is set.
        unsafe { self.write(Self::bank() + 0x04, 1 << sem) };
    }

    unsafe fn arm(&self, sem: u8) {
        // SAFETY: the caller serializes the IER read-modify-write.
        unsafe { self.set_ier(1 << sem, true) };
    }

    unsafe fn disarm(&self, sem: u8) {
        // SAFETY: the caller serializes the IER read-modify-write.
        unsafe { self.set_ier(1 << sem, false) };
    }

    #[inline]
    fn asserted(&self) -> LineSet {
        self.sems(Self::bank() + 0x08)
    }

    unsafe fn disarm_all(&self, lines: LineSet) {
        // SAFETY: the caller serializes the IER read-modify-write.
        unsafe { self.set_ier(lines, false) };
    }

    #[cfg(not(feature = "std"))]
    #[inline]
    fn waker(id: u8) -> &'static atomic_waker::AtomicWaker {
        &HSEM_WAKERS[id as usize]
    }
}

/// Wakers separated by semaphore on the bare-metal path. A core drives one bank of
/// the single instance, so one row suffices.
#[cfg(not(feature = "std"))]
static HSEM_WAKERS: [atomic_waker::AtomicWaker; HSEM_CHAN_COUNT as usize] =
    [const { atomic_waker::AtomicWaker::new() }; HSEM_CHAN_COUNT as usize];

#[cfg(test)]
mod tests;
