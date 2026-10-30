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

//! STM32 inter-processor communication controller (IPCC) register model.
//!
//! The IPCC carries no payload. It exposes per-channel status flags used to transfer
//! ownership of shared memory between processor 1 and processor 2, and this model
//! uses their simplex half:
//!
//! | Operation      | IPCC action                                                   |
//! | -------------- | ------------------------------------------------------------- |
//! | `ring`         | set the local transmit channel *occupied* (`CxSCR.CHnS`)       |
//! | `raw_pending`  | read the remote transmit channel's status (`CyTOCxSR.CHnF`)    |
//! | `ack`          | clear the incoming channel (`CxSCR.CHnC`)                      |
//! | `arm`/`disarm` | `CxCR.RXOIE` plus the `CxMR.CHnOM` mask bit                    |
//!
//! Because occupancy is a flag rather than a queue, `ring` reports
//! [`BellError::Busy`] while the previous signal is unconsumed. That is a real
//! condition, not an internal retry: the peer has not caught up.
//!
//! IPCC is the **recommended STM32MP2 doorbell**. It is present across STM32MP21/23/25
//! and has the better-supported ST integration path; HSEM is absent on STM32MP21 and
//! has shown surprising bus-error access failures in the field.
//!
//! # Banks
//!
//! An IPCC instance is one register block with a bank per processor, so the
//! peripheral singleton names the block and the bank is a type parameter:
//! [`Primary`] drives processor 1 (`C1*`, waits on `C2TOC1SR`), [`Secondary`] drives
//! processor 2 (`C2*`, waits on `C1TOC2SR`). `N` is the 1-based instance:
//! `IpccDoorbell<Primary, 1>` is processor 1 on IPCC1.
//!
//! # Construction
//!
//! - **Firmware** — [`IpccDoorbell::new`] consumes the chip HAL's `IPCCn` singleton; a
//!   const assertion checks it against `N`. The window address still comes from
//!   [`registers::base`] so the security alias does not change underfoot.
//! - **Linux/UIO** — [`BellA::new_mapped`] takes an `mmap`'d virtual base.
//!
//! The non-secure `IPCC1_RX` / `IPCC2_RX` vectors forward into
//! [`service_interrupt::<Ipcc<S, N>>()`](super::service_interrupt), which
//! `consortium-cfg` generates.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary
//! [`BellA::new_mapped`]: super::BellA::new_mapped

pub mod registers;

use core::{marker::PhantomData, ops::Range};

use consortium_hal_stm32mp2::{Peri, PeripheralType};
use consortium_ipc::Side;

use self::registers::{
    base,
    cr::{ChannelRegister, ControlRegister, StatusSetClearRegister},
    offsets,
};
use super::{BellA, BellError, BellHw, LineSet};

/// Number of channels per IPCC instance.
pub const IPCC_CHAN_COUNT: u8 = 16;

/// Channel ids reserved from user channels.
pub const IPCC_CHAN_RESERVED: Range<u8> = 0..1;

/// The IPCC doorbell for instance `N` (1-based), processor bank `S`.
pub type IpccDoorbell<S, const N: usize> = BellA<Ipcc<S, N>>;

/// A chip-HAL peripheral singleton naming one IPCC instance.
pub trait Instance: PeripheralType {
    /// 1-based IPCC instance number.
    const INSTANCE: usize;
}

/// Bind a chip-HAL peripheral singleton to its instance number.
macro_rules! impl_ipcc {
    ($instance:ident, $number:expr) => {
        impl Instance for ::consortium_hal_stm32mp2::peripherals::$instance {
            const INSTANCE: usize = $number;
        }
    };
}

// STM32MP25 adds the CM0+'s SmartRun IPCC2; STM32MP21/23 have only IPCC1.
#[cfg(feature = "stm32mp25x")]
impl_ipcc!(IPCC1, 1);
#[cfg(feature = "stm32mp25x")]
impl_ipcc!(IPCC2, 2);
#[cfg(all(
    any(feature = "stm32mp21x", feature = "stm32mp23x"),
    not(feature = "stm32mp25x")
))]
impl_ipcc!(IPCC1, 1);

impl<S: Side, const N: usize> IpccDoorbell<S, N> {
    /// Create a doorbell over IPCC instance `N`, bank `S`, from the chip HAL's
    /// peripheral singleton.
    ///
    /// This is the firmware constructor: the singleton proves nothing else drives
    /// the instance, and the window address comes from [`base::BASE`]. On Linux the
    /// window is an `mmap` with no singleton — use
    /// [`new_mapped`](BellA::new_mapped) there.
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        const {
            assert!(
                T::INSTANCE == N,
                "IpccDoorbell instance number does not match the peripheral singleton",
            );
        }
        // The singleton is proof of exclusivity, not state: `init()` issues it once
        // and the `'static` bound means nothing hands it back. It is deliberately
        // not stored: `split()` clones the doorbell, and other doorbells may drive
        // disjoint channels of the same block.
        let _ = peri;
        // SAFETY: `PHYS_BASE` is instance `N`'s window from this crate's vetted
        // address table. The consumed singleton keeps every other driver off it;
        // another doorbell on it can only come from a `steal()` whose caller
        // guarantees disjoint channels, and the shared enable-register writes run
        // in a critical section.
        unsafe { Self::new_mapped(Ipcc::<S, N>::PHYS_BASE) }
    }
}

/// A handle to IPCC instance `N`'s register window, viewed from processor bank `S`.
///
/// A thin, checked wrapper over MMIO: set, clear, poll, mask, and unmask one channel
/// in one bank. IPCC connects two processors, so [`Tertiary`] is rejected at compile
/// time.
///
/// [`Tertiary`]: consortium_ipc::Tertiary
pub struct Ipcc<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

// Manual `Clone` so the bound is independent of `S` (the side markers are
// zero-sized and not themselves `Clone`).
impl<S: Side, const N: usize> Clone for Ipcc<S, N> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            _side: PhantomData,
        }
    }
}

impl<S: Side, const N: usize> Ipcc<S, N> {
    /// Zero-based index of instance `N` into [`base::BASE`]. Evaluating this const
    /// enforces, at compile time, that `N` names a real IPCC instance and that `S` is
    /// a two-processor side.
    const INDEX: usize = {
        assert!(
            S::INDEX < 2,
            "IPCC connects two processors; use Primary or Secondary"
        );
        assert!(
            N >= 1 && N <= base::BASE.len(),
            "IPCC instance N is out of range for the selected chip"
        );
        N - 1
    };

    /// `true` if this side drives the processor-1 register bank.
    #[inline]
    const fn is_proc1() -> bool {
        S::INDEX == 0
    }

    /// Base address of this instance's register window.
    #[inline]
    pub const fn base(&self) -> usize {
        self.base
    }

    #[inline]
    const fn bank(proc1: usize, proc2: usize) -> usize {
        if Self::is_proc1() { proc1 } else { proc2 }
    }

    /// This bank's outgoing status register.
    #[inline]
    const fn outgoing_status() -> usize {
        Self::bank(offsets::C1TOC2SR, offsets::C2TOC1SR)
    }

    /// The remote bank's outgoing status register, which this bank reads.
    #[inline]
    const fn incoming_status() -> usize {
        Self::bank(offsets::C2TOC1SR, offsets::C1TOC2SR)
    }

    #[inline]
    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    #[inline]
    fn read(&self, offset: usize) -> u32 {
        // SAFETY: `self.base + offset` addresses a valid IPCC register (constructor
        // obligation); volatile read.
        unsafe { self.reg(offset).read_volatile() }
    }

    #[inline]
    unsafe fn write(&self, offset: usize, value: u32) {
        // SAFETY: see `read`; the caller coordinates write access.
        unsafe { self.reg(offset).write_volatile(value) };
    }

    #[inline]
    unsafe fn update(&self, offset: usize, f: impl FnOnce(u32) -> u32) {
        let reg = self.reg(offset);
        // SAFETY: the caller guarantees exclusive access for this read-modify-write.
        let value = unsafe { reg.read_volatile() };
        // SAFETY: see above.
        unsafe { reg.write_volatile(f(value)) };
    }

    #[inline]
    fn channels(&self, offset: usize) -> u32 {
        ChannelRegister::from_bits(self.read(offset)).channels()
    }

    /// Set (`mask = true`) or clear the `CxMR.CHnOM` bits in `channels`. `CHnOM` is
    /// an active-high mask.
    #[inline]
    unsafe fn set_masked(&self, channels: u32, mask: bool) {
        // SAFETY: the caller serializes the MR read-modify-write.
        unsafe {
            self.update(Self::bank(offsets::C1MR, offsets::C2MR), |value| {
                let mut reg = ChannelRegister::from_bits(value);
                let bits = if mask {
                    reg.channels() | channels
                } else {
                    reg.channels() & !channels
                };
                reg.set_channels(bits);
                reg.bits()
            });
        }
    }

    #[inline]
    unsafe fn write_scr(&self, reg: StatusSetClearRegister) {
        // SAFETY: SCR is a write-one action register; only the given bits act.
        unsafe { self.write(Self::bank(offsets::C1SCR, offsets::C2SCR), reg.bits()) };
    }
}

// SAFETY: `PHYS_BASE` comes from the vetted `base::BASE` table; `raw_pending` and
// `asserted` read the raw `CyTOCxSR`, not the `CxMR`-filtered view; every write
// touches only the channel bits it is given; status bit `n` is channel `n`, `< 16`.
unsafe impl<S: Side, const N: usize> BellHw for Ipcc<S, N> {
    type Line = u8;

    const PHYS_BASE: usize = base::BASE[Self::INDEX];

    unsafe fn at(base: usize) -> Self {
        // Force the compile-time checks on `N` and `S`.
        let _ = Self::INDEX;
        Self {
            base,
            _side: PhantomData,
        }
    }

    const CHANNELS: u8 = IPCC_CHAN_COUNT;

    const RESERVED: Range<u8> = IPCC_CHAN_RESERVED;

    #[inline]
    fn line(id: u8) -> u8 {
        id
    }

    fn ring(&self, line: u8) -> Result<(), BellError> {
        if self.channels(Self::outgoing_status()) & (1 << line) != 0 {
            return Err(BellError::Busy);
        }
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_set(1 << line);
        // SAFETY: `line < 16`; sets only this side's outgoing `CHnS`.
        unsafe { self.write_scr(reg) };
        Ok(())
    }

    #[inline]
    fn raw_pending(&self, line: u8) -> bool {
        self.channels(Self::incoming_status()) & (1 << line) != 0
    }

    unsafe fn ack(&self, line: u8) {
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_clear(1 << line);
        // SAFETY: `line < 16`; clears only this incoming `CHnC`.
        unsafe { self.write_scr(reg) };
    }

    unsafe fn arm(&self, line: u8) {
        // SAFETY: the caller serializes the CR read-modify-write.
        unsafe {
            self.update(Self::bank(offsets::C1CR, offsets::C2CR), |value| {
                let mut reg = ControlRegister::from_bits(value);
                reg.set_rxoie(true);
                reg.bits()
            });
        }
        // SAFETY: the caller serializes the MR read-modify-write.
        unsafe { self.set_masked(1 << line, false) };
    }

    unsafe fn disarm(&self, line: u8) {
        // SAFETY: the caller serializes the MR read-modify-write.
        unsafe { self.set_masked(1 << line, true) };
    }

    #[inline]
    fn asserted(&self) -> LineSet {
        // Like embassy-stm32, use the remote status directly. Filtering it through
        // CxMR would skip a channel that is already masked but still occupied.
        self.channels(Self::incoming_status())
    }

    unsafe fn disarm_all(&self, lines: LineSet) {
        // SAFETY: the caller serializes the MR read-modify-write.
        unsafe { self.set_masked(lines, true) };
    }

    #[cfg(not(feature = "std"))]
    #[inline]
    fn waker(id: u8) -> &'static atomic_waker::AtomicWaker {
        &IPCC_WAKERS[Self::INDEX][id as usize]
    }
}

/// Wakers separated by IPCC instance and channel on the bare-metal path.
#[cfg(not(feature = "std"))]
static IPCC_WAKERS: [[atomic_waker::AtomicWaker; IPCC_CHAN_COUNT as usize]; base::BASE.len()] =
    [const { [const { atomic_waker::AtomicWaker::new() }; IPCC_CHAN_COUNT as usize] };
        base::BASE.len()];

#[cfg(all(test, feature = "std"))]
mod tests;
