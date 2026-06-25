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

//! Hardware Semaphore (HSEM) hardware description.
//!
//! Reference: RM0457 Rev 5, Section 22 (STM32MP23/25); RM0399 (STM32H7x5/7).
//! All register offsets are relative to the single HSEM peripheral base. The
//! per-processor interrupt registers (`CnIER`/`CnICR`/`CnISR`/`CnMISR`) are laid
//! out one 16-byte block per core, `n = 1..=3`.

/// HSEM base address and register-window length.
pub mod base {
    /// HSEM physical base address of each instance. HSEM is a single-instance
    /// peripheral, so this has one entry.
    #[cfg(any(feature = "stm32mp23", feature = "stm32mp25"))]
    pub const BASE: [usize; 1] = [0x5624_0000]; // RM0457, STM32MP23x/25x.

    /// HSEM physical base address of each instance.
    #[cfg(all(
        not(any(feature = "stm32mp23", feature = "stm32mp25")),
        any(feature = "stm32h7x5", feature = "stm32h7x7")
    ))]
    pub const BASE: [usize; 1] = [0x5802_6400]; // RM0399, STM32H7x5/7.

    /// HSEM register-window length, in bytes.
    pub const LENGTH: usize = 0x400;
}

/// Register offsets relative to the HSEM base address.
pub mod offsets {
    /// Semaphore register bank base; `HSEM_Rx = R + 4 * x` for `x = 0..16`,
    /// bitfield [`super::cr::SemaphoreRegister`]. Reset `0x0`.
    pub const R: u32 = 0x000;
    /// Read-lock register bank base; `HSEM_RLRx = RLR + 4 * x` for `x = 0..16`,
    /// bitfield [`super::cr::SemaphoreRegister`]. Reset `0x0`.
    pub const RLR: u32 = 0x080;

    /// Core 1 interrupt enable register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C1IER: u32 = 0x100;
    /// Core 1 interrupt clear register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C1ICR: u32 = 0x104;
    /// Core 1 interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C1ISR: u32 = 0x108;
    /// Core 1 masked interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C1MISR: u32 = 0x10C;

    /// Core 2 interrupt enable register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C2IER: u32 = 0x110;
    /// Core 2 interrupt clear register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C2ICR: u32 = 0x114;
    /// Core 2 interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C2ISR: u32 = 0x118;
    /// Core 2 masked interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C2MISR: u32 = 0x11C;

    /// Core 3 interrupt enable register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C3IER: u32 = 0x120;
    /// Core 3 interrupt clear register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C3ICR: u32 = 0x124;
    /// Core 3 interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C3ISR: u32 = 0x128;
    /// Core 3 masked interrupt status register, bitfield
    /// [`super::cr::InterruptRegister`]. Reset `0x0`.
    pub const C3MISR: u32 = 0x12C;

    /// Security configuration register. Reset `0x0`.
    pub const SECCFGR: u32 = 0x200;
    /// Privilege configuration register. Reset `0x0`.
    pub const PRIVCFGR: u32 = 0x210;
    /// Clear register. Reset `0x0`.
    pub const CR: u32 = 0x230;
    /// Clear-key register. Reset `0x0`.
    pub const KEYR: u32 = 0x234;

    /// Hardware configuration register 2. Reset `0x0000_0321`
    /// [`super::reset::HWCFGR2`].
    pub const HWCFGR2: u32 = 0x3EC;
    /// Hardware configuration register 1. Reset `0x0007_3310`
    /// [`super::reset::HWCFGR1`].
    pub const HWCFGR1: u32 = 0x3F0;
    /// Version register. Reset `0x0000_0020` [`super::reset::VERR`].
    pub const VERR: u32 = 0x3F4;
    /// IP identification register. Reset `0x0010_0072` [`super::reset::IPIDR`].
    pub const IPIDR: u32 = 0x3F8;
    /// Size identification register. Reset `0xA3C5_DD01` [`super::reset::SIDR`].
    pub const SIDR: u32 = 0x3FC;
}

/// Register bitfields.
pub mod cr {
    bitfield::bitfield! {
        /// `HSEM_Rx` / `HSEM_RLRx` fields, registers [`super::offsets::R`] and
        /// [`super::offsets::RLR`].
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct SemaphoreRegister(u32);
        impl Debug;
        u32;
        pub lock, set_lock: 31;
        pub cfen, set_cfen: 30;
        pub priv_attr, set_priv_attr: 13;
        pub sec, set_sec: 12;
        pub lockid, set_lockid: 11, 8;
        pub procid, set_procid: 7, 0;
    }

    impl SemaphoreRegister {
        /// Wraps a raw register word read from MMIO.
        #[inline(always)]
        pub const fn from_bits(bits: u32) -> Self {
            Self(bits)
        }

        /// Returns the raw register word for writing back to MMIO.
        #[inline(always)]
        pub const fn bits(self) -> u32 {
            self.0
        }
    }

    bitfield::bitfield! {
        /// Per-semaphore interrupt bits shared by `CnIER`, `CnICR`, `CnISR` and
        /// `CnMISR`. One bit per semaphore (`0..16`).
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct InterruptRegister(u32);
        impl Debug;
        u32;
        pub sems, set_sems: 15, 0;
    }

    impl InterruptRegister {
        /// Wraps a raw register word read from MMIO.
        #[inline(always)]
        pub const fn from_bits(bits: u32) -> Self {
            Self(bits)
        }

        /// Returns the raw register word for writing back to MMIO.
        #[inline(always)]
        pub const fn bits(self) -> u32 {
            self.0
        }
    }
}

/// Reset values for registers whose reset state is non-zero.
pub mod reset {
    /// Reset value of [`super::offsets::HWCFGR2`].
    pub const HWCFGR2: u32 = 0x0000_0321;
    /// Reset value of [`super::offsets::HWCFGR1`].
    pub const HWCFGR1: u32 = 0x0007_3310;
    /// Reset value of [`super::offsets::VERR`].
    pub const VERR: u32 = 0x0000_0020;
    /// Reset value of [`super::offsets::IPIDR`].
    pub const IPIDR: u32 = 0x0010_0072;
    /// Reset value of [`super::offsets::SIDR`].
    pub const SIDR: u32 = 0xA3C5_DD01;
}

/// Interrupt line numbers for the HSEM per-core interrupts.
pub mod interrupts {
    /// GIC SPI numbers of the HSEM core interrupts, index 0 = core 1.
    ///
    /// Not yet populated; routing is board- and SoC-specific. Fill in from the
    /// SoC interrupt map when wiring the A-core (GIC) side.
    pub const GIC: Option<&[u32]> = None;

    /// NVIC IRQ numbers of the HSEM core interrupts, index 0 = core 1.
    ///
    /// Not yet populated; routing is board- and SoC-specific. Fill in from the
    /// SoC interrupt map when wiring the M-core (NVIC) side.
    pub const NVIC: Option<&[u32]> = None;
}
