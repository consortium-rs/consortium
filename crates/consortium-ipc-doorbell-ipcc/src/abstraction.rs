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

//! HAL layer over an IPCC instance.
//!
//! [`Ipcc`] is a zero-cost handle to one IPCC instance's register window for a
//! given processor [`Side`]: [`Primary`] drives the processor-1 bank (`C1*`,
//! waits on `C2TOC1SR`), [`Secondary`] drives the processor-2 bank (`C2*`, waits
//! on `C1TOC2SR`). IPCC connects two processors, so [`Tertiary`] is rejected at
//! compile time.
//!
//! `N` selects the instance (`Ipcc<_, 1>` is IPCC1, `Ipcc<_, 2>` is the SmartRun
//! IPCC2 on STM32MP23/25). Construction follows the dual-path convention:
//! [`Ipcc::steal`] (bare-metal, compile-time base) and [`Ipcc::new`] (Linux,
//! `mmap`'d base).
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Tertiary`]: consortium_ipc::Tertiary

use core::marker::PhantomData;

use consortium_ipc::Side;

use crate::registers::{
    base,
    cr::{ChannelRegister, ControlRegister, StatusSetClearRegister},
    offsets,
};

/// A HAL handle to IPCC instance `N` (1-based) viewed from processor side `S`.
pub struct Ipcc<S: Side, const N: usize> {
    base: usize,
    _side: PhantomData<S>,
}

impl<S: Side, const N: usize> Ipcc<S, N> {
    /// Zero-based index of instance `N` into [`base::BASE`]. Evaluating this
    /// const enforces, at compile time, that `N` names a real IPCC instance and
    /// that `S` is a two-processor side.
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
    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    #[inline]
    const fn cr_offset() -> usize {
        if Self::is_proc1() {
            offsets::C1CR
        } else {
            offsets::C2CR
        }
    }

    #[inline]
    const fn mr_offset() -> usize {
        if Self::is_proc1() {
            offsets::C1MR
        } else {
            offsets::C2MR
        }
    }

    #[inline]
    const fn scr_offset() -> usize {
        if Self::is_proc1() {
            offsets::C1SCR
        } else {
            offsets::C2SCR
        }
    }

    /// Offset of the status register this side writes to (its outgoing channels).
    #[inline]
    const fn outgoing_status_offset() -> usize {
        if Self::is_proc1() {
            offsets::C1TOC2SR
        } else {
            offsets::C2TOC1SR
        }
    }

    /// Offset of the status register this side reads (the remote's outgoing
    /// channels).
    #[inline]
    const fn incoming_status_offset() -> usize {
        if Self::is_proc1() {
            offsets::C2TOC1SR
        } else {
            offsets::C1TOC2SR
        }
    }

    #[inline]
    fn read(&self, offset: usize) -> u32 {
        // SAFETY: `self.base` + `offset` addresses a valid IPCC register
        // (constructor obligation); volatile read.
        unsafe { self.reg(offset).read_volatile() }
    }

    #[inline]
    unsafe fn write(&self, offset: usize, value: u32) {
        // SAFETY: see `read`; caller coordinates write access.
        unsafe { self.reg(offset).write_volatile(value) };
    }

    #[inline]
    unsafe fn update(&self, offset: usize, f: impl FnOnce(u32) -> u32) {
        let reg = self.reg(offset);
        // SAFETY: caller guarantees exclusive access for this read-modify-write.
        let value = unsafe { reg.read_volatile() };
        // SAFETY: see above.
        unsafe { reg.write_volatile(f(value)) };
    }

    /// Returns `true` if this side's outgoing channel `ch` is still occupied.
    #[inline]
    pub fn outgoing_occupied(&self, ch: u8) -> bool {
        ChannelRegister::from_bits(self.read(Self::outgoing_status_offset())).channels() & (1 << ch)
            != 0
    }

    /// Returns `true` if the remote has set incoming channel `ch` occupied.
    #[inline]
    pub fn incoming_occupied(&self, ch: u8) -> bool {
        ChannelRegister::from_bits(self.read(Self::incoming_status_offset())).channels() & (1 << ch)
            != 0
    }

    /// Set this side's outgoing channel `ch` occupied (`CxSCR.CHnS`).
    ///
    /// # Safety
    /// `ch` must be a valid channel id (`< 16`).
    #[inline]
    pub unsafe fn set_outgoing_occupied(&self, ch: u8) {
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_set(1 << ch);
        // SAFETY: SCR is a write-one action register; only `CHnS` is written.
        unsafe { self.write(Self::scr_offset(), reg.bits()) };
    }

    /// Clear the incoming channel `ch` (`CxSCR.CHnC`) to acknowledge it.
    ///
    /// # Safety
    /// `ch` must be a valid channel id (`< 16`).
    #[inline]
    pub unsafe fn clear_incoming_occupied(&self, ch: u8) {
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_clear(1 << ch);
        // SAFETY: SCR is a write-one action register; only `CHnC` is written.
        unsafe { self.write(Self::scr_offset(), reg.bits()) };
    }

    /// Enable RX-occupied interrupts and unmask channel `ch` (`CxCR.RXOIE`,
    /// `CxMR.CHnOM`).
    ///
    /// # Safety
    /// `ch` must be a valid channel id (`< 16`); the caller coordinates access
    /// to the CR/MR interrupt configuration.
    #[inline]
    pub unsafe fn enable_rx_occupied_interrupt(&self, ch: u8) {
        // SAFETY: see method contract.
        unsafe {
            self.update(Self::cr_offset(), |value| {
                let mut reg = ControlRegister::from_bits(value);
                reg.set_rxoie(true);
                reg.bits()
            });
        }
        // CHnOM is an active-high mask: clear the bit to unmask channel `ch`.
        // SAFETY: see method contract.
        unsafe {
            self.update(Self::mr_offset(), |value| {
                let mut reg = ChannelRegister::from_bits(value);
                reg.set_channels(reg.channels() & !(1 << ch));
                reg.bits()
            });
        }
    }

    /// Mask the RX-occupied interrupt for channel `ch` (`CxMR.CHnOM`).
    ///
    /// # Safety
    /// `ch` must be a valid channel id (`< 16`); the caller coordinates access
    /// to the MR interrupt configuration.
    #[inline]
    pub unsafe fn mask_rx_occupied_interrupt(&self, ch: u8) {
        // SAFETY: see method contract.
        unsafe {
            self.update(Self::mr_offset(), |value| {
                let mut reg = ChannelRegister::from_bits(value);
                reg.set_channels(reg.channels() | (1 << ch));
                reg.bits()
            });
        }
    }
}

impl<S: Side, const N: usize> Ipcc<S, N> {
    /// Create a HAL handle from an `mmap`'d virtual base address of IPCC
    /// instance `N`.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of the IPCC window for instance
    /// `N`. Passing a raw physical address is undefined behaviour on Linux.
    pub unsafe fn new(base: usize) -> Self {
        // Force the compile-time checks on `N` and `S`.
        let _ = Self::INDEX;
        Self {
            base,
            _side: PhantomData,
        }
    }
}
