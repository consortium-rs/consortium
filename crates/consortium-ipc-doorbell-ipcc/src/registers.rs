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

//! STM32 IPCC register offsets.
//!
//! Offsets are relative to the IPCC peripheral base address. IPCC1 has the
//! same base address on STM32MP21/23/25. IPCC2 is the SmartRun IPCC block and
//! exists only on STM32MP23/25.

pub mod base {
    /// IPCC register block size in bytes.
    pub const IPCC_SIZE: usize = 0x400;

    /// IPCC1 physical base address on STM32MP21/23/25.
    pub const IPCC1_BASE: usize = 0x4049_0000;
    /// Last byte in the IPCC1 register window.
    pub const IPCC1_END: usize = 0x4049_03FF;

    /// Backwards-friendly alias for chips with a single IPCC instance.
    pub const IPCC_BASE: usize = IPCC1_BASE;

    /// IPCC2 physical base address on STM32MP23/25 SmartRun.
    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    pub const IPCC2_BASE: usize = 0x4625_0000;
    /// Last byte in the IPCC2 register window.
    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    pub const IPCC2_END: usize = 0x4625_03FF;
}

pub mod offsets {
    pub const C1CR: usize = 0x000;
    pub const C1MR: usize = 0x004;
    pub const C1SCR: usize = 0x008;
    pub const C1TOC2SR: usize = 0x00C;
    pub const C2CR: usize = 0x010;
    pub const C2MR: usize = 0x014;
    pub const C2SCR: usize = 0x018;
    pub const C2TOC1SR: usize = 0x01C;

    pub const C1SECCFGR: usize = 0x080;
    pub const C1PRIVCFGR: usize = 0x084;
    pub const C1CIDCFGR: usize = 0x088;
    pub const C2SECCFGR: usize = 0x090;
    pub const C2PRIVCFGR: usize = 0x094;
    pub const C2CIDCFGR: usize = 0x098;

    pub const HWCFGR: usize = 0x3F0;
    pub const VERR: usize = 0x3F4;
    pub const IPIDR: usize = 0x3F8;
    pub const SIDR: usize = 0x3FC;
}

pub mod cr {
    bitfield::bitfield! {
        /// IPCC_CxCR interrupt enable fields.
        #[derive(Clone, Copy)]
        pub struct ControlRegister(u32);
        impl Debug;
        u32;
        pub rxoie, set_rxoie: 0;
        pub secrxoie, set_secrxoie: 1;
        pub txfie, set_txfie: 16;
        pub sectxfie, set_sectxfie: 17;
    }

    impl ControlRegister {
        #[inline(always)]
        pub const fn from_bits(bits: u32) -> Self {
            Self(bits)
        }

        #[inline(always)]
        pub const fn bits(self) -> u32 {
            self.0
        }
    }

    pub const RXOIE: u32 = 1 << 0;
    pub const SECRXOIE: u32 = 1 << 1;
    pub const TXFIE: u32 = 1 << 16;
    pub const SECTXFIE: u32 = 1 << 17;
}

bitfield::bitfield! {
    /// IPCC channel fields in status and mask registers.
    #[derive(Clone, Copy)]
    pub struct ChannelRegister(u32);
    impl Debug;
    u32;
    pub channels, set_channels: 15, 0;
}

impl ChannelRegister {
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[inline(always)]
    pub const fn bits(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn contains_channel(self, ch: u8) -> bool {
        self.bits() & channel_mask(ch) != 0
    }

    #[inline(always)]
    pub fn with_channel_masked(mut self, ch: u8) -> Self {
        self.set_channels(self.channels() | channel_mask(ch));
        self
    }

    #[inline(always)]
    pub fn with_channel_unmasked(mut self, ch: u8) -> Self {
        self.set_channels(self.channels() & !channel_mask(ch));
        self
    }
}

bitfield::bitfield! {
    /// IPCC_CxSCR write-one action fields.
    #[derive(Clone, Copy)]
    pub struct StatusSetClearRegister(u32);
    impl Debug;
    u32;
    pub channel_clear, set_channel_clear: 15, 0;
    pub channel_set, set_channel_set: 31, 16;
}

impl StatusSetClearRegister {
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[inline(always)]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[inline]
pub const fn channel_mask(ch: u8) -> u32 {
    1 << ch
}

#[inline]
pub const fn channel_set_mask(ch: u8) -> u32 {
    1 << (16 + ch)
}

#[inline]
pub const fn channel_flag(ch: u8) -> ChannelRegister {
    ChannelRegister::from_bits(channel_mask(ch))
}

#[inline]
pub fn channel_clear_flag(ch: u8) -> StatusSetClearRegister {
    let mut register = StatusSetClearRegister::from_bits(0);
    register.set_channel_clear(channel_mask(ch));
    register
}

#[inline]
pub fn channel_set_flag(ch: u8) -> StatusSetClearRegister {
    let mut register = StatusSetClearRegister::from_bits(0);
    register.set_channel_set(channel_mask(ch));
    register
}
