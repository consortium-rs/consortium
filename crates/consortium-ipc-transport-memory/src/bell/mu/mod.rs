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

//! NXP i.MX Messaging Unit (MU) register model.
//!
//! An MU instance offers two independent signaling mechanisms, packed into the one
//! flat channel space [`Doorbell`](consortium_ipc::Doorbell) expects:
//!
//! | Channel | Line                               | Registers                         |
//! | ------- | ---------------------------------- | --------------------------------- |
//! | `0..4`  | general-purpose interrupt, [`MuLine::Gi`] | `GCR` / `GSR` / `GIER`     |
//! | `4..8`  | receive-full, [`MuLine::Rf`]       | `TRn`/`RRn` + `TSR`/`RSR`/`RCR`   |
//!
//! GI lines are pure interrupts. RF lines work by writing a token word to a transmit
//! register, which sets the peer's receive-full flag — a doorbell that happens to
//! carry four bytes this model does not use. Neither kind can report
//! [`BellError::Busy`]: setting a pending GI is idempotent, and an RF ring whose
//! transmit register is still full is skipped, because the peer's flag is already
//! raised and will wake it anyway.
//!
//! # Windows
//!
//! Each instance's side-B (MUB) window sits [`registers::base::LENGTH`] above its
//! side-A (MUA) base, and [`Side`] selects between them.
//!
//! **`Side` here is the register window, not the descriptor role.** It says which
//! half of the block this core drives — [`Primary`] is MUA, [`Secondary`] is MUB —
//! and which half a core gets is a silicon fact. It is unrelated to
//! `SharedMemoryTransport`'s primary/secondary role, which the manifest chooses. On
//! both i.MX 93 and i.MX 95 the A-core is on MUA and the M-core on MUB, so the two
//! coincide for the documented `primary = "linux"`, but a manifest that swaps the
//! roles keeps the windows — and code that conflates them fails the
//! `Instance<Side = S>` bound of [`MuDoorbell::new`].
//!
//! # Construction
//!
//! - **Firmware** — [`MuDoorbell::new`] consumes the chip HAL's `MUn_MUx` singleton,
//!   whose side must match `S`; a const assertion checks the instance number. On
//!   i.MX 93 it also opens the window's LPCG clock gate, which Linux leaves closed
//!   unless a kernel driver claims the MU.
//! - **Linux/UIO** — [`BellA::new_mapped`] takes an `mmap`'d virtual base.
//!
//! The vectors a core drives — i.MX93 CM33 `MU1_B`/`MU2_B`, i.MX95 CM7
//! `MU5_A`/`MU7_B`/`MU8_B` — forward into
//! [`service_interrupt::<Mu<S, N>>()`](super::service_interrupt), which
//! `consortium-cfg` generates.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary
//! [`BellA::new_mapped`]: super::BellA::new_mapped

pub mod registers;

use core::{marker::PhantomData, ops::Range};

use consortium_hal_imx9::{Peri, PeripheralType};
use consortium_ipc::Side;

use self::registers::{
    base,
    cr::{FlagRegister, GeneralInterruptRegister},
    offsets,
};
use super::{BellA, BellError, BellHw, LineSet};

// The chip features select different instance counts, base tables and singleton
// bindings, so they cannot be unified. Cargo unifies features across a build graph,
// so this fires when *any* dependent selects the other chip too; diagnosing it here
// rather than through the HAL's duplicate `mod chip` or the PAC's duplicate
// `Interrupt` is what makes the error name the actual mistake.
#[cfg(all(feature = "mimx93", feature = "mimx95"))]
compile_error!(
    "chip features are mutually exclusive; enable exactly one of: `mimx93`, `mimx95`. \
     A dependency selecting the non-default chip must set `default-features = false`."
);

/// Number of MU instances on the selected chip: eight on i.MX 95, two on i.MX 93.
#[cfg(feature = "mimx95")]
pub const MU_INSTANCE_COUNT: usize = 8;

/// Number of MU instances on the selected chip: eight on i.MX 95, two on i.MX 93.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
pub const MU_INSTANCE_COUNT: usize = 2;

/// General-purpose interrupt (GI) lines per instance, channels `0..MU_GI_CHAN_COUNT`.
pub const MU_GI_CHAN_COUNT: u8 = 4;

/// Receive-full (RF) lines per instance, following the GI channels.
pub const MU_RF_CHAN_COUNT: u8 = 4;

/// Total doorbell channels per instance: the GI lines followed by the RF lines.
pub const MU_CHAN_COUNT: u8 = MU_GI_CHAN_COUNT + MU_RF_CHAN_COUNT;

/// Channel ids reserved from user channels. Every MU line is available.
pub const MU_CHAN_RESERVED: Range<u8> = 0..0;

/// Token written to `TRn` to raise the peer's receive-full flag.
const RF_TOKEN: u32 = 0x1;

/// The MU doorbell for instance `N` (1-based), register window `S`.
pub type MuDoorbell<S, const N: usize> = BellA<Mu<S, N>>;

/// One MU signaling line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuLine {
    /// General-purpose interrupt line `0..MU_GI_CHAN_COUNT`.
    Gi(u8),
    /// Receive-full line `0..MU_RF_CHAN_COUNT`.
    Rf(u8),
}

/// A chip-HAL peripheral singleton naming one MU register window.
pub trait Instance: PeripheralType {
    /// The window this singleton names: [`Primary`](consortium_ipc::Primary) for
    /// MUA, [`Secondary`](consortium_ipc::Secondary) for MUB.
    type Side: Side;
    /// 1-based MU instance number.
    const INSTANCE: usize;
    /// LPCG clock-gate offset to open before first access, where the chip has one.
    const LPCG: Option<u32>;
}

/// Bind a chip-HAL peripheral singleton to its window and instance number.
macro_rules! impl_mu {
    ($instance:ident, $side:ty, $number:expr) => {
        impl Instance for ::consortium_hal_imx9::peripherals::$instance {
            type Side = $side;
            const INSTANCE: usize = $number;
            const LPCG: Option<u32> = ::consortium_hal_imx9::clock::lpcg(stringify!($instance));
        }
    };
}

// The i.MX95 CM7 reaches MU5 side A and MU7/MU8 side B; the i.MX93 CM33 reaches both
// windows of MU1/MU2 and drives side B. Both lists come from the per-core dumps that
// generate the HAL's singletons, so a window this core cannot see has none to pass.
#[cfg(feature = "mimx95")]
mod bindings {
    use super::Instance;
    use consortium_ipc::{Primary, Secondary};

    impl_mu!(MU5_MUA, Primary, 5);
    impl_mu!(MU7_MUB, Secondary, 7);
    impl_mu!(MU8_MUB, Secondary, 8);
}

#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
mod bindings {
    use super::Instance;
    use consortium_ipc::{Primary, Secondary};

    impl_mu!(MU1_MUA, Primary, 1);
    impl_mu!(MU1_MUB, Secondary, 1);
    impl_mu!(MU2_MUA, Primary, 2);
    impl_mu!(MU2_MUB, Secondary, 2);
}

impl<S: Side, const N: usize> MuDoorbell<S, N> {
    /// Create a doorbell over MU instance `N`, window `S`, from the chip HAL's
    /// peripheral singleton.
    ///
    /// The `Instance<Side = S>` bound rejects the other window's singleton, and the
    /// window address comes from [`base::BASE`]. On Linux the window is an `mmap`
    /// with no singleton — use [`new_mapped`](BellA::new_mapped) there.
    pub fn new<T: Instance<Side = S>>(peri: Peri<'static, T>) -> Self {
        const {
            assert!(
                T::INSTANCE == N,
                "MuDoorbell instance number does not match the peripheral singleton",
            );
        }
        // The singleton is proof of exclusivity, not state: `init()` issues it once
        // and the `'static` bound means nothing hands it back. It is deliberately
        // not stored: `split()` clones the doorbell, and other doorbells may drive
        // disjoint lines of the same window.
        let _ = peri;
        // On i.MX 93 Linux has closed this window's gate unless a kernel driver
        // claims it; every later register access would BusFault without this.
        ::consortium_hal_imx9::clock::ungate(T::LPCG);
        // SAFETY: `PHYS_BASE` is instance `N`'s side-`S` window from this crate's
        // vetted address table. The consumed singleton keeps every other driver off
        // it; another doorbell on it can only come from a `steal()` whose caller
        // guarantees disjoint lines, and the shared enable-register writes run in a
        // critical section.
        unsafe { Self::new_mapped(Mu::<S, N>::PHYS_BASE) }
    }
}

/// A handle to MU instance `N`'s register window `S`.
///
/// A thin, checked wrapper over MMIO. Line validation and the protocol live in
/// [`BellA`]; this only maps a [`MuLine`] onto its registers.
pub struct Mu<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

// Manual `Clone` so the bound is independent of `S` (the side markers are
// zero-sized and not themselves `Clone`).
impl<S: Side, const N: usize> Clone for Mu<S, N> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            _side: PhantomData,
        }
    }
}

impl<S: Side, const N: usize> Mu<S, N> {
    /// Zero-based index of instance `N` into [`base::BASE`]. Evaluating this const
    /// enforces, at compile time, that `N` names a real MU instance and that `S`
    /// names one of its two windows.
    const INDEX: usize = {
        assert!(
            S::INDEX < 2,
            "an MU has two windows; use Primary or Secondary"
        );
        assert!(
            N >= 1 && N <= base::BASE.len(),
            "MU instance N is out of range for the selected chip"
        );
        N - 1
    };

    /// Base address of this window.
    #[inline]
    pub const fn base(&self) -> usize {
        self.base
    }

    #[inline]
    fn reg(&self, offset: u32) -> *mut u32 {
        (self.base + offset as usize) as *mut u32
    }

    #[inline]
    fn read(&self, offset: u32) -> u32 {
        // SAFETY: `self.base + offset` addresses a valid MU register (constructor
        // obligation); volatile read.
        unsafe { self.reg(offset).read_volatile() }
    }

    #[inline]
    unsafe fn write(&self, offset: u32, value: u32) {
        // SAFETY: see `read`; the caller coordinates write access.
        unsafe { self.reg(offset).write_volatile(value) };
    }

    #[inline]
    fn gi(&self, offset: u32) -> u32 {
        GeneralInterruptRegister::from_bits(self.read(offset)).gi()
    }

    #[inline]
    fn flags(&self, offset: u32) -> u32 {
        FlagRegister::from_bits(self.read(offset)).flags()
    }

    /// Set (`on = true`) or clear the `GIER` bits in `gi`.
    #[inline]
    unsafe fn set_gier(&self, gi: u32, on: bool) {
        let mut reg = GeneralInterruptRegister::from_bits(self.read(offsets::GIER));
        reg.set_gi(if on { reg.gi() | gi } else { reg.gi() & !gi });
        // SAFETY: the caller serializes the GIER read-modify-write.
        unsafe { self.write(offsets::GIER, reg.bits()) };
    }

    /// Set (`on = true`) or clear the `RCR` bits in `rf`.
    #[inline]
    unsafe fn set_rcr(&self, rf: u32, on: bool) {
        let mut reg = FlagRegister::from_bits(self.read(offsets::RCR));
        reg.set_flags(if on {
            reg.flags() | rf
        } else {
            reg.flags() & !rf
        });
        // SAFETY: the caller serializes the RCR read-modify-write.
        unsafe { self.write(offsets::RCR, reg.bits()) };
    }
}

/// Bits of a [`LineSet`] that hold GI lines; RF lines sit above them.
const GI_MASK: LineSet = (1 << MU_GI_CHAN_COUNT) - 1;

// SAFETY: `PHYS_BASE` is the vetted `base::BASE` entry plus the window offset;
// `raw_pending` and `asserted` read the raw `GSR`/`RSR`, independent of
// `GIER`/`RCR`; every write touches only the line bits it is given; channel ids
// are `0..8`, GI lines first, and `asserted` packs the same way.
unsafe impl<S: Side, const N: usize> BellHw for Mu<S, N> {
    type Line = MuLine;

    const PHYS_BASE: usize = base::BASE[Self::INDEX] + (S::INDEX as usize) * base::LENGTH;

    unsafe fn at(base: usize) -> Self {
        // Force the compile-time checks on `N` and `S`.
        let _ = Self::INDEX;
        Self {
            base,
            _side: PhantomData,
        }
    }

    const CHANNELS: u8 = MU_CHAN_COUNT;

    const RESERVED: Range<u8> = MU_CHAN_RESERVED;

    #[inline]
    fn line(id: u8) -> MuLine {
        if id < MU_GI_CHAN_COUNT {
            MuLine::Gi(id)
        } else {
            MuLine::Rf(id - MU_GI_CHAN_COUNT)
        }
    }

    fn ring(&self, line: MuLine) -> Result<(), BellError> {
        match line {
            MuLine::Gi(gi) => {
                let mut reg = GeneralInterruptRegister::from_bits(self.read(offsets::GCR));
                reg.set_gi(reg.gi() | (1 << gi));
                // SAFETY: `gi < 4`; `ring` is the sole writer of `GCR`, and `BellA`
                // serializes rings, so this read-modify-write cannot drop a bit.
                unsafe { self.write(offsets::GCR, reg.bits()) };
            }
            MuLine::Rf(rf) => {
                // `TSR[TEn]` clear means the previous token is unread, so the peer's
                // receive-full flag is still raised; writing again would be lost.
                if self.flags(offsets::TSR) & (1 << rf) != 0 {
                    // SAFETY: `rf < 4`, so `TRn` is in the window; `ring` is its
                    // sole writer.
                    unsafe { self.write(offsets::TR0 + u32::from(rf) * 4, RF_TOKEN) };
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn raw_pending(&self, line: MuLine) -> bool {
        match line {
            MuLine::Gi(gi) => self.gi(offsets::GSR) & (1 << gi) != 0,
            MuLine::Rf(rf) => self.flags(offsets::RSR) & (1 << rf) != 0,
        }
    }

    unsafe fn ack(&self, line: MuLine) {
        match line {
            MuLine::Gi(gi) => {
                // SAFETY: `GSR` is write-one-to-clear; only `GIPn` is written.
                unsafe {
                    self.write(
                        offsets::GSR,
                        GeneralInterruptRegister::from_bits(1 << gi).bits(),
                    );
                }
            }
            MuLine::Rf(rf) => {
                // Reading `RRn` clears `RSR[RFn]`; the token itself is unused.
                let _ = self.read(offsets::RR0 + u32::from(rf) * 4);
            }
        }
    }

    unsafe fn arm(&self, line: MuLine) {
        match line {
            // SAFETY: the caller serializes the enable-register read-modify-write.
            MuLine::Gi(gi) => unsafe { self.set_gier(1 << gi, true) },
            // SAFETY: as above.
            MuLine::Rf(rf) => unsafe { self.set_rcr(1 << rf, true) },
        }
    }

    unsafe fn disarm(&self, line: MuLine) {
        match line {
            // SAFETY: the caller serializes the enable-register read-modify-write.
            MuLine::Gi(gi) => unsafe { self.set_gier(1 << gi, false) },
            // SAFETY: as above.
            MuLine::Rf(rf) => unsafe { self.set_rcr(1 << rf, false) },
        }
    }

    #[inline]
    fn asserted(&self) -> LineSet {
        self.gi(offsets::GSR) | (self.flags(offsets::RSR) << MU_GI_CHAN_COUNT)
    }

    unsafe fn disarm_all(&self, lines: LineSet) {
        let gi = lines & GI_MASK;
        let rf = (lines >> MU_GI_CHAN_COUNT) & GI_MASK;
        if gi != 0 {
            // SAFETY: the caller serializes the GIER read-modify-write.
            unsafe { self.set_gier(gi, false) };
        }
        if rf != 0 {
            // SAFETY: the caller serializes the RCR read-modify-write.
            unsafe { self.set_rcr(rf, false) };
        }
    }

    #[inline]
    fn waker(&self, id: u8) -> &atomic_waker::AtomicWaker {
        &MU_WAKERS[Self::INDEX][id as usize]
    }
}

/// Wakers separated by MU instance and line. Both windows of
/// an instance share a row: a core drives one window of it.
static MU_WAKERS: [[atomic_waker::AtomicWaker; MU_CHAN_COUNT as usize]; MU_INSTANCE_COUNT] =
    [const { [const { atomic_waker::AtomicWaker::new() }; MU_CHAN_COUNT as usize] };
        MU_INSTANCE_COUNT];

#[cfg(test)]
mod tests;
