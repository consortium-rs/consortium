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

//! Messaging Unit (MU) hardware description.
//!
//! Reference: i.MX 95 Reference Manual, MU chapter (offsets match the i.MX 93
//! RM, Rev 7, Chapter 20). All register offsets are relative to a single MU
//! instance base address. The side-B (MUB) window of each instance sits one
//! [`base::LENGTH`] above the side-A (MUA) base, so a single [`base::BASE`]
//! array plus the window length describes both processor views.

/// MU instance base addresses and the per-instance register-window length.
pub mod base {
    use super::super::MU_INSTANCE_COUNT;

    /// Side-A (MUA) physical base address of each MU instance, index 0 = MU1.
    ///
    /// The side-B (MUB) base of instance `i` is `BASE[i] + LENGTH`.
    #[cfg(feature = "mimx95")]
    pub const BASE: [usize; MU_INSTANCE_COUNT] = [
        0x4422_0000, // MU1
        0x445B_0000, // MU2
        0x445D_0000, // MU3
        0x445F_0000, // MU4
        0x4461_0000, // MU5
        0x4463_0000, // MU6
        0x4243_0000, // MU7
        0x4273_0000, // MU8
    ];

    /// Side-A (MUA) physical base address of each MU instance, index 0 = MU1.
    ///
    /// `not(mimx95)` suppresses a duplicate-definition cascade when both chip
    /// features are on; that combination is already fatal via the
    /// `compile_error!` in `bell::mu`.
    #[cfg(all(not(feature = "mimx95"), feature = "mimx93"))]
    pub const BASE: [usize; MU_INSTANCE_COUNT] = [
        0x4422_0000, // MU1
        0x4243_0000, // MU2
    ];

    /// Register-window length of one MU instance, in bytes. Also the offset
    /// from a side-A (MUA) base to the matching side-B (MUB) base.
    pub const LENGTH: usize = 0x1_0000;
}

/// Register offsets relative to a MU instance base address.
pub mod offsets {
    /// Version ID register. Reset `0x0309_000F` [`super::reset::VER`].
    pub const VER: u32 = 0x000;
    /// Parameter register. Reset `0x0304_0404` [`super::reset::PAR`].
    pub const PAR: u32 = 0x004;
    /// Control register. Reset `0x0`.
    pub const CR: u32 = 0x008;
    /// Status register. Reset `0x0`.
    pub const SR: u32 = 0x00C;
    /// Core control 0 register. Reset `0x0000_0014` [`super::reset::CCR0`].
    pub const CCR0: u32 = 0x010;
    /// Core interrupt enable 0 register. Reset `0x0`.
    pub const CIER0: u32 = 0x014;
    /// Core sticky status 0 register. Reset `0x0`.
    pub const CSSR0: u32 = 0x018;
    /// Core status 0 register. Reset `0x0`.
    #[cfg(feature = "mimx95")]
    pub const CSR0: u32 = 0x01C;

    /// Flag control register. Reset `0x0`.
    pub const FCR: u32 = 0x100;
    /// Flag status register. Reset `0x0`.
    pub const FSR: u32 = 0x104;

    /// General-purpose interrupt enable register, bitfield
    /// [`super::cr::GeneralInterruptRegister`]. Reset `0x0`.
    pub const GIER: u32 = 0x110;
    /// General-purpose control register, bitfield
    /// [`super::cr::GeneralInterruptRegister`]. Reset `0x0`.
    pub const GCR: u32 = 0x114;
    /// General-purpose status register, bitfield
    /// [`super::cr::GeneralInterruptRegister`]. Reset `0x0`.
    pub const GSR: u32 = 0x118;

    /// Transmit control register, bitfield [`super::cr::FlagRegister`]
    /// (`TIEn`, one bit per transmit register). Reset `0x0`.
    pub const TCR: u32 = 0x120;
    /// Transmit status register, bitfield [`super::cr::FlagRegister`]
    /// (`TEn`, transmit-empty, one bit per transmit register).
    /// Reset `0x0000_000F` [`super::reset::TSR`].
    pub const TSR: u32 = 0x124;
    /// Receive control register, bitfield [`super::cr::FlagRegister`]
    /// (`RIEn`, one bit per receive register). Reset `0x0`.
    pub const RCR: u32 = 0x128;
    /// Receive status register, bitfield [`super::cr::FlagRegister`]
    /// (`RFn`, receive-full, one bit per receive register). Reset `0x0`.
    pub const RSR: u32 = 0x12C;

    /// Transmit register 0. Reset `0x0`.
    pub const TR0: u32 = 0x200;
    /// Transmit register 1. Reset `0x0`.
    pub const TR1: u32 = 0x204;
    /// Transmit register 2. Reset `0x0`.
    pub const TR2: u32 = 0x208;
    /// Transmit register 3. Reset `0x0`.
    pub const TR3: u32 = 0x20C;

    /// Receive register 0. Reset `0x0`.
    pub const RR0: u32 = 0x280;
    /// Receive register 1. Reset `0x0`.
    pub const RR1: u32 = 0x284;
    /// Receive register 2. Reset `0x0`.
    pub const RR2: u32 = 0x288;
    /// Receive register 3. Reset `0x0`.
    pub const RR3: u32 = 0x28C;
}

/// Register bitfields.
pub mod cr {
    bitfield::bitfield! {
        /// General-purpose interrupt bits shared by `GIER`, `GCR` and `GSR`,
        /// registers [`super::offsets::GIER`], [`super::offsets::GCR`] and
        /// [`super::offsets::GSR`]. One bit per GI channel (`0..4`).
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct GeneralInterruptRegister(u32);
        impl Debug;
        u32;
        pub gi, set_gi: 3, 0;
    }

    impl GeneralInterruptRegister {
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
        /// Per-register flag bits shared by the transmit/receive control and
        /// status registers `TCR`, `TSR`, `RCR` and `RSR`, registers
        /// [`super::offsets::TCR`], [`super::offsets::TSR`],
        /// [`super::offsets::RCR`] and [`super::offsets::RSR`]. One bit per
        /// transmit/receive register (`0..4`):
        ///
        /// - `TCR`: transmit-interrupt-enable (`TIEn`).
        /// - `TSR`: transmit-empty (`TEn`).
        /// - `RCR`: receive-interrupt-enable (`RIEn`).
        /// - `RSR`: receive-full (`RFn`).
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct FlagRegister(u32);
        impl Debug;
        u32;
        pub flags, set_flags: 3, 0;
    }

    impl FlagRegister {
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
    /// Reset value of [`super::offsets::VER`].
    pub const VER: u32 = 0x0309_000F;
    /// Reset value of [`super::offsets::PAR`].
    pub const PAR: u32 = 0x0304_0404;
    /// Reset value of [`super::offsets::CCR0`].
    pub const CCR0: u32 = 0x0000_0014;
    /// Reset value of [`super::offsets::TSR`].
    pub const TSR: u32 = 0x0000_000F;
}

/// Interrupt line numbers for each MU instance's general-purpose interrupt.
pub mod interrupts {
    /// GIC SPI numbers of the MU general-purpose interrupts, index 0 = MU1.
    ///
    /// Not yet populated; routing is board- and SoC-specific. Fill in from the
    /// SoC interrupt map when wiring the A-core (GIC) side.
    pub const GIC: Option<&[u32]> = None;

    /// NVIC IRQ numbers of the MU general-purpose interrupts, index 0 = MU1.
    ///
    /// Not yet populated; routing is board- and SoC-specific. Fill in from the
    /// SoC interrupt map when wiring the M-core (NVIC) side.
    pub const NVIC: Option<&[u32]> = None;
}
