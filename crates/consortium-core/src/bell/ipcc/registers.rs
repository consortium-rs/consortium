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

//! STM32 IPCC hardware description.
//!
//! Offsets are relative to an IPCC instance base address. IPCC1 has the same
//! base on STM32MP21/23/25; IPCC2, the CM0+'s SmartRun mailbox, exists only on
//! STM32MP25 (STM32MP23 has no CM0+).

/// IPCC instance base addresses and register-window length.
pub mod base {
    /// IPCC physical base address of each instance, index 0 = IPCC1.
    ///
    /// STM32MP25 adds the SmartRun IPCC2 to IPCC1.
    #[cfg(feature = "stm32mp25x")]
    pub const BASE: [usize; 2] = [0x4049_0000, 0x4625_0000];

    /// IPCC physical base address of each instance, index 0 = IPCC1.
    ///
    /// STM32MP21 and STM32MP23 have only IPCC1.
    #[cfg(all(
        any(feature = "stm32mp21x", feature = "stm32mp23x"),
        not(feature = "stm32mp25x")
    ))]
    pub const BASE: [usize; 1] = [0x4049_0000];

    /// IPCC register-window length, in bytes.
    pub const LENGTH: usize = 0x400;
}

/// Register offsets relative to an IPCC instance base address.
pub mod offsets {
    /// Processor 1 control register, bitfield [`super::cr::ControlRegister`].
    /// Reset `0x0`.
    pub const C1CR: usize = 0x000;
    /// Processor 1 mask register, bitfield [`super::cr::ChannelRegister`].
    /// Reset `0x0`.
    pub const C1MR: usize = 0x004;
    /// Processor 1 status set/clear register, bitfield
    /// [`super::cr::StatusSetClearRegister`]. Reset `0x0`.
    pub const C1SCR: usize = 0x008;
    /// Processor 1-to-2 channel status register, bitfield
    /// [`super::cr::ChannelRegister`]. Reset `0x0`.
    pub const C1TOC2SR: usize = 0x00C;
    /// Processor 2 control register, bitfield [`super::cr::ControlRegister`].
    /// Reset `0x0`.
    pub const C2CR: usize = 0x010;
    /// Processor 2 mask register, bitfield [`super::cr::ChannelRegister`].
    /// Reset `0x0`.
    pub const C2MR: usize = 0x014;
    /// Processor 2 status set/clear register, bitfield
    /// [`super::cr::StatusSetClearRegister`]. Reset `0x0`.
    pub const C2SCR: usize = 0x018;
    /// Processor 2-to-1 channel status register, bitfield
    /// [`super::cr::ChannelRegister`]. Reset `0x0`.
    pub const C2TOC1SR: usize = 0x01C;

    /// Processor 1 security configuration register. Reset `0x0`.
    pub const C1SECCFGR: usize = 0x080;
    /// Processor 1 privilege configuration register. Reset `0x0`.
    pub const C1PRIVCFGR: usize = 0x084;
    /// Processor 1 CID configuration register. Reset `0x0`.
    pub const C1CIDCFGR: usize = 0x088;
    /// Processor 2 security configuration register. Reset `0x0`.
    pub const C2SECCFGR: usize = 0x090;
    /// Processor 2 privilege configuration register. Reset `0x0`.
    pub const C2PRIVCFGR: usize = 0x094;
    /// Processor 2 CID configuration register. Reset `0x0`.
    pub const C2CIDCFGR: usize = 0x098;

    /// Hardware configuration register (read-only, SoC-specific reset).
    pub const HWCFGR: usize = 0x3F0;
    /// Version register (read-only, SoC-specific reset).
    pub const VERR: usize = 0x3F4;
    /// IP identification register (read-only, SoC-specific reset).
    pub const IPIDR: usize = 0x3F8;
    /// Size identification register (read-only, SoC-specific reset).
    pub const SIDR: usize = 0x3FC;
}

/// Register bitfields.
pub mod cr {
    bitfield::bitfield! {
        /// `IPCC_CxCR` interrupt-enable fields, register [`super::offsets::C1CR`]
        /// / [`super::offsets::C2CR`].
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct ControlRegister(u32);
        impl Debug;
        u32;
        pub rxoie, set_rxoie: 0;
        pub secrxoie, set_secrxoie: 1;
        pub txfie, set_txfie: 16;
        pub sectxfie, set_sectxfie: 17;
    }

    impl ControlRegister {
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
        /// Per-channel bits in the mask (`CxMR`) and channel-status
        /// (`CxTOCySR`) registers. One bit per channel (`0..16`).
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct ChannelRegister(u32);
        impl Debug;
        u32;
        pub channels, set_channels: 15, 0;
    }

    impl ChannelRegister {
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
        /// `IPCC_CxSCR` write-one action fields, register
        /// [`super::offsets::C1SCR`] / [`super::offsets::C2SCR`]. Writing `CHnC`
        /// clears a channel; writing `CHnS` sets it occupied.
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct StatusSetClearRegister(u32);
        impl Debug;
        u32;
        pub channel_clear, set_channel_clear: 15, 0;
        pub channel_set, set_channel_set: 31, 16;
    }

    impl StatusSetClearRegister {
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
///
/// All writable IPCC registers reset to `0x0`. The read-only `HWCFGR`, `VERR`,
/// `IPIDR` and `SIDR` registers carry SoC-specific reset values that are not
/// recorded here.
pub mod reset {}

/// Interrupt line numbers for the IPCC RX-occupied / TX-free interrupts.
pub mod interrupts {
    /// GIC SPI numbers of the IPCC interrupts, index 0 = processor 1.
    ///
    /// Not yet populated; routing differs by family and interrupt-routing
    /// configuration. Fill in from the SoC interrupt map when wiring the
    /// A-core (GIC) side.
    pub const GIC: Option<&[u32]> = None;

    /// NVIC IRQ numbers of the IPCC interrupts, index 0 = processor 1.
    ///
    /// Not yet populated; routing differs by family and interrupt-routing
    /// configuration. Fill in from the SoC interrupt map when wiring the
    /// M-core (NVIC) side.
    pub const NVIC: Option<&[u32]> = None;
}
