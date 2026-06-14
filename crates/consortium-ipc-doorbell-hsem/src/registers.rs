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

//! Hardware Semaphore (HSEM) Register Definitions
//! Reference: RM0457 Rev 5, Section 22
//!
//! Base address must be added to each offset at runtime or via a base const.
//! All offsets are relative to the HSEM peripheral base address.

pub const HSEM_BASE: u32 = 0x5624_0000; // Verified in RM0457 for STM32MP25x; adjust as needed for other targets

// ============================================================================
// Module: Semaphore R registers (2-step write lock / unlock / read)
// HSEM_Rx  - Address offset: 0x000 + 0x4 * x  (x = 0..=15)
// Reset value: 0x0000_0000
// ============================================================================
pub mod semaphore_r {
    #[allow(clippy::identity_op)]
    pub const HSEM_R0: u32 = 0x000;
    pub const HSEM_R1: u32 = 0x004;
    pub const HSEM_R2: u32 = 0x008;
    pub const HSEM_R3: u32 = 0x00C;
    pub const HSEM_R4: u32 = 0x010;
    pub const HSEM_R5: u32 = 0x014;
    pub const HSEM_R6: u32 = 0x018;
    pub const HSEM_R7: u32 = 0x01C;
    pub const HSEM_R8: u32 = 0x020;
    pub const HSEM_R9: u32 = 0x024;
    pub const HSEM_R10: u32 = 0x028;
    pub const HSEM_R11: u32 = 0x02C;
    pub const HSEM_R12: u32 = 0x030;
    pub const HSEM_R13: u32 = 0x034;
    pub const HSEM_R14: u32 = 0x038;
    pub const HSEM_R15: u32 = 0x03C;

    /// Bit fields within HSEM_Rx
    pub mod fields {
        bitfield::bitfield! {
            /// HSEM_Rx/HSEM_RLRx fields.
            #[derive(Clone, Copy)]
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
            #[inline(always)]
            pub const fn from_bits(bits: u32) -> Self {
                Self(bits)
            }

            #[inline(always)]
            pub const fn bits(self) -> u32 {
                self.0
            }
        }

        /// Bit 31 - LOCK: semaphore lock indication (rw)
        pub const LOCK_BIT: u32 = 1 << 31;
        /// Bit 30 - CFEN: CID filter indication (r)
        pub const CFEN_BIT: u32 = 1 << 30;
        /// Bit 13 - PRIV: privileged domain lock (rw)
        pub const PRIV_BIT: u32 = 1 << 13;
        /// Bit 12 - SEC: secure domain lock (rw)
        pub const SEC_BIT: u32 = 1 << 12;
        /// Bits 11:8 - LOCKID[3:0] (rw)
        pub const LOCKID_SHIFT: u32 = 8;
        pub const LOCKID_MASK: u32 = 0xF << LOCKID_SHIFT;
        /// Bits 7:0 - PROCID[7:0] (rw)
        pub const PROCID_SHIFT: u32 = 0;
        pub const PROCID_MASK: u32 = 0xFF;

        #[inline(always)]
        pub const fn lockid(value: u32) -> u8 {
            ((value & LOCKID_MASK) >> LOCKID_SHIFT) as u8
        }
    }
}

// ============================================================================
// Module: Semaphore RLR registers (1-step read lock)
// HSEM_RLRx - Address offset: 0x080 + 0x4 * x  (x = 0..=15)
// Reset value: 0x0000_0000
// Accesses same physical bits as HSEM_Rx; all fields are read-only here.
// ============================================================================
pub mod semaphore_rlr {
    pub const HSEM_RLR0: u32 = 0x080;
    pub const HSEM_RLR1: u32 = 0x084;
    pub const HSEM_RLR2: u32 = 0x088;
    pub const HSEM_RLR3: u32 = 0x08C;
    pub const HSEM_RLR4: u32 = 0x090;
    pub const HSEM_RLR5: u32 = 0x094;
    pub const HSEM_RLR6: u32 = 0x098;
    pub const HSEM_RLR7: u32 = 0x09C;
    pub const HSEM_RLR8: u32 = 0x0A0;
    pub const HSEM_RLR9: u32 = 0x0A4;
    pub const HSEM_RLR10: u32 = 0x0A8;
    pub const HSEM_RLR11: u32 = 0x0AC;
    pub const HSEM_RLR12: u32 = 0x0B0;
    pub const HSEM_RLR13: u32 = 0x0B4;
    pub const HSEM_RLR14: u32 = 0x0B8;
    pub const HSEM_RLR15: u32 = 0x0BC;

    /// Bit fields within HSEM_RLRx (all read-only at this address)
    pub mod fields {
        /// Bit 31 - LOCK (r): reading a free semaphore locks it and returns 1
        pub const LOCK_BIT: u32 = 1 << 31;
        /// Bit 30 - CFEN (r)
        pub const CFEN_BIT: u32 = 1 << 30;
        /// Bit 13 - PRIV (r)
        pub const PRIV_BIT: u32 = 1 << 13;
        /// Bit 12 - SEC (r)
        pub const SEC_BIT: u32 = 1 << 12;
        /// Bits 11:8 - LOCKID[3:0] (r)
        pub const LOCKID_SHIFT: u32 = 8;
        pub const LOCKID_MASK: u32 = 0xF << LOCKID_SHIFT;
        /// Bits 7:0 - PROCID[7:0] (r); always 0 after a read-lock
        pub const PROCID_SHIFT: u32 = 0;
        pub const PROCID_MASK: u32 = 0xFF;
    }
}

// ============================================================================
// Module: Non-secure interrupt registers (per processor n = 1..=3)
// ============================================================================
pub mod interrupt_nonsecure {
    // --- Interrupt Enable Registers (HSEM_CnIER) ---
    // Offset: 0x100 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_C1IER: u32 = 0x100;
    pub const HSEM_C2IER: u32 = 0x110;
    pub const HSEM_C3IER: u32 = 0x120;

    // --- Interrupt Clear Registers (HSEM_CnICR) ---
    // Offset: 0x104 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_C1ICR: u32 = 0x104;
    pub const HSEM_C2ICR: u32 = 0x114;
    pub const HSEM_C3ICR: u32 = 0x124;

    // --- Interrupt Status Registers (HSEM_CnISR) ---
    // Offset: 0x108 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_C1ISR: u32 = 0x108;
    pub const HSEM_C2ISR: u32 = 0x118;
    pub const HSEM_C3ISR: u32 = 0x128;

    // --- Masked Interrupt Status Registers (HSEM_CnMISR) ---
    // Offset: 0x10C + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_C1MISR: u32 = 0x10C;
    pub const HSEM_C2MISR: u32 = 0x11C;
    pub const HSEM_C3MISR: u32 = 0x12C;

    /// Bit fields shared across CnIER / CnICR / CnISR / CnMISR
    pub mod fields {
        bitfield::bitfield! {
            /// Per-semaphore interrupt fields shared by IER/ICR/ISR/MISR.
            #[derive(Clone, Copy)]
            pub struct SemInterruptRegister(u32);
            impl Debug;
            u32;
            pub sems, set_sems: 15, 0;
        }

        impl SemInterruptRegister {
            #[inline(always)]
            pub const fn from_bits(bits: u32) -> Self {
                Self(bits)
            }

            #[inline(always)]
            pub const fn bits(self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn contains_sem(self, sem: u8) -> bool {
                self.bits() & sem_bit(sem) != 0
            }

            #[inline(always)]
            pub fn with_sem_enabled(mut self, sem: u8) -> Self {
                self.set_sems(self.sems() | sem_bit(sem));
                self
            }

            #[inline(always)]
            pub fn with_sem_disabled(mut self, sem: u8) -> Self {
                self.set_sems(self.sems() & !sem_bit(sem));
                self
            }
        }

        /// Bits 15:0 - ISE[15:0] in IER: enable bit per semaphore (rw)
        /// Bits 15:0 - ISC[15:0] in ICR: clear bit per semaphore (rc_w1)
        /// Bits 15:0 - ISF[15:0] in ISR: status before masking (r)
        /// Bits 15:0 - MISF[15:0] in MISR: masked status (r)
        pub const SEM_MASK: u32 = 0xFFFF; // one bit per semaphore 0..=15

        /// Helper: bit mask for semaphore index i (0..=15)
        #[inline(always)]
        pub const fn sem_bit(i: u8) -> u32 {
            1 << i
        }

        #[inline(always)]
        pub const fn sem_clear(i: u8) -> SemInterruptRegister {
            SemInterruptRegister::from_bits(sem_bit(i))
        }
    }
}

// ============================================================================
// Module: Secure interrupt registers (per processor n = 1..=3)
// ============================================================================
pub mod interrupt_secure {
    // --- Secure Interrupt Enable Registers (HSEM_SCnIER) ---
    // Offset: 0x180 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_SC1IER: u32 = 0x180;
    pub const HSEM_SC2IER: u32 = 0x190;
    pub const HSEM_SC3IER: u32 = 0x1A0;

    // --- Secure Interrupt Clear Registers (HSEM_SCnICR) ---
    // Offset: 0x184 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_SC1ICR: u32 = 0x184;
    pub const HSEM_SC2ICR: u32 = 0x194;
    pub const HSEM_SC3ICR: u32 = 0x1A4;

    // --- Secure Interrupt Status Registers (HSEM_SCnISR) ---
    // Offset: 0x188 + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_SC1ISR: u32 = 0x188;
    pub const HSEM_SC2ISR: u32 = 0x198;
    pub const HSEM_SC3ISR: u32 = 0x1A8;

    // --- Secure Masked Interrupt Status Registers (HSEM_SCnMISR) ---
    // Offset: 0x18C + 0x10 * (n - 1)
    // Reset: 0x0000_0000
    pub const HSEM_SC1MISR: u32 = 0x18C;
    pub const HSEM_SC2MISR: u32 = 0x19C;
    pub const HSEM_SC3MISR: u32 = 0x1AC;

    /// Bit fields shared across SCnIER / SCnICR / SCnISR / SCnMISR
    pub mod fields {
        /// Bits 15:0 - SISE[15:0] in SCnIER (rw)
        /// Bits 15:0 - SISC[15:0] in SCnICR (rc_w1)
        /// Bits 15:0 - SISF[15:0] in SCnISR (r)
        /// Bits 15:0 - SMISF[15:0] in SCnMISR (r)
        pub const SEM_MASK: u32 = 0xFFFF;

        #[inline(always)]
        pub const fn sem_bit(i: u8) -> u32 {
            1 << i
        }
    }
}

// ============================================================================
// Module: Attribute configuration registers
// ============================================================================
pub mod attributes {
    // --- Security Configuration Register (HSEM_SECCFGR) ---
    // Offset: 0x200  Reset: 0x0000_0000
    // Write: secure + privileged only. Read: unrestricted.
    pub const HSEM_SECCFGR: u32 = 0x200;

    // --- Privilege Configuration Register (HSEM_PRIVCFGR) ---
    // Offset: 0x210  Reset: 0x0000_0000
    // Write: privileged only. Read: unrestricted.
    pub const HSEM_PRIVCFGR: u32 = 0x210;

    pub mod fields {
        /// Bits 15:0 - SEC[15:0] in SECCFGR: security attribute per semaphore (rw)
        /// Bits 15:0 - PRIV[15:0] in PRIVCFGR: privilege attribute per semaphore (rw)
        pub const SEM_MASK: u32 = 0xFFFF;

        #[inline(always)]
        pub const fn sem_bit(i: u8) -> u32 {
            1 << i
        }
    }
}

// ============================================================================
// Module: CID (Compartment ID) configuration registers
// ============================================================================
pub mod cid_config {
    // --- Processor CID Configuration Registers (HSEM_CnCIDCFGR) ---
    // Offset: 0x220 + 0x4 * (n - 1)  (n = 1..=3)
    // Reset: 0x0000_0000
    // Write: secure + privileged + trusted domain CID only. Read: unrestricted.
    pub const HSEM_C1CIDCFGR: u32 = 0x220;
    pub const HSEM_C2CIDCFGR: u32 = 0x224;
    pub const HSEM_C3CIDCFGR: u32 = 0x228;

    // --- Group CID Configuration Registers (HSEM_GpCIDCFGR) ---
    // Offset: 0x240 + 0x4 * p  (p = 0..=3)
    //   p=0 → Group HSEM[3:0]
    //   p=1 → Group HSEM[7:4]
    //   p=2 → Group HSEM[11:8]
    //   p=3 → Group HSEM[15:12]
    // Reset: 0x0000_0000
    // Write: secure + privileged + trusted domain CID only. Read: unrestricted.
    pub const HSEM_G0CIDCFGR: u32 = 0x240; // HSEM[3:0]
    pub const HSEM_G1CIDCFGR: u32 = 0x244; // HSEM[7:4]
    pub const HSEM_G2CIDCFGR: u32 = 0x248; // HSEM[11:8]
    pub const HSEM_G3CIDCFGR: u32 = 0x24C; // HSEM[15:12]

    /// Bit fields for HSEM_CnCIDCFGR
    pub mod cn_fields {
        /// Bits 6:4 - CID[2:0]: processor CID domain ID (rw)
        pub const CID_SHIFT: u32 = 4;
        pub const CID_MASK: u32 = 0x7 << CID_SHIFT;
        /// Bit 0 - CFEN: CID filtering enable for processor n (rw)
        pub const CFEN_BIT: u32 = 1 << 0;
    }

    /// Bit fields for HSEM_GpCIDCFGR
    pub mod gp_fields {
        /// Bits 18:16 - SEM_WLIST_C[3:1]: white list enable per CID (rw)
        pub const SEM_WLIST_SHIFT: u32 = 16;
        pub const SEM_WLIST_MASK: u32 = 0x7 << SEM_WLIST_SHIFT;
        /// SEM_WLIST_C1 (processor 1 CID white listed)
        pub const SEM_WLIST_C1: u32 = 1 << 16;
        /// SEM_WLIST_C2 (processor 2 CID white listed)
        pub const SEM_WLIST_C2: u32 = 1 << 17;
        /// SEM_WLIST_C3 (processor 3 CID white listed)
        pub const SEM_WLIST_C3: u32 = 1 << 18;
        /// Bit 0 - CFEN: group-level CID filtering enable (rw)
        pub const CFEN_BIT: u32 = 1 << 0;
    }
}

// ============================================================================
// Module: Clear / Key registers
// ============================================================================
pub mod clear {
    // --- Clear Register (HSEM_CR) ---
    // Offset: 0x230  Reset: 0x0000_0000
    // Write: secure + privileged + trusted domain CID only. Read: always 0.
    pub const HSEM_CR: u32 = 0x230;

    // --- Clear Key Register (HSEM_KEYR) ---
    // Offset: 0x234  Reset: 0x0000_0000
    // Write: secure + privileged + trusted domain CID only.
    pub const HSEM_KEYR: u32 = 0x234;

    /// Bit fields for HSEM_CR
    pub mod cr_fields {
        /// Bits 31:16 - KEY[15:0]: must match HSEM_KEYR.KEY to clear (w)
        pub const KEY_SHIFT: u32 = 16;
        pub const KEY_MASK: u32 = 0xFFFF << KEY_SHIFT;
        /// Bit 13 - PRIV: PRIV of semaphores to clear (w)
        pub const PRIV_BIT: u32 = 1 << 13;
        /// Bit 12 - SEC: SEC of semaphores to clear (w)
        pub const SEC_BIT: u32 = 1 << 12;
        /// Bits 11:8 - LOCKID[3:0]: LOCKID of semaphores to clear (w)
        pub const LOCKID_SHIFT: u32 = 8;
        pub const LOCKID_MASK: u32 = 0xF << LOCKID_SHIFT;
        /// Bit 0 - CFEN: CID-filtered semaphores to clear (w)
        pub const CFEN_BIT: u32 = 1 << 0;
    }

    /// Bit fields for HSEM_KEYR
    pub mod keyr_fields {
        /// Bits 31:16 - KEY[15:0]: semaphore clear key (rw)
        pub const KEY_SHIFT: u32 = 16;
        pub const KEY_MASK: u32 = 0xFFFF << KEY_SHIFT;
    }
}

// ============================================================================
// Module: Hardware configuration and identification registers (read-only)
// ============================================================================
pub mod info {
    // --- Hardware Configuration Register 2 (HSEM_HWCFGR2) ---
    // Offset: 0x3EC  Reset: 0x0000_0321
    pub const HSEM_HWCFGR2: u32 = 0x3EC;

    // --- Hardware Configuration Register 1 (HSEM_HWCFGR1) ---
    // Offset: 0x3F0  Reset: 0x0007_3310
    pub const HSEM_HWCFGR1: u32 = 0x3F0;

    // --- Version Register (HSEM_VERR) ---
    // Offset: 0x3F4  Reset: 0x0000_0020
    pub const HSEM_VERR: u32 = 0x3F4;

    // --- IP Identification Register (HSEM_IPIDR) ---
    // Offset: 0x3F8  Reset: 0x0010_0072
    pub const HSEM_IPIDR: u32 = 0x3F8;

    // --- Size Identification Register (HSEM_SIDR) ---
    // Offset: 0x3FC  Reset: 0xA3C5_DD01
    pub const HSEM_SIDR: u32 = 0x3FC;

    /// Bit fields for HSEM_HWCFGR2
    pub mod hwcfgr2_fields {
        /// Bits 15:12 - MASTERID4[3:0] (r)
        pub const MASTERID4_SHIFT: u32 = 12;
        pub const MASTERID4_MASK: u32 = 0xF << MASTERID4_SHIFT;
        /// Bits 11:8  - MASTERID3[3:0] (r)
        pub const MASTERID3_SHIFT: u32 = 8;
        pub const MASTERID3_MASK: u32 = 0xF << MASTERID3_SHIFT;
        /// Bits 7:4   - MASTERID2[3:0] (r)
        pub const MASTERID2_SHIFT: u32 = 4;
        pub const MASTERID2_MASK: u32 = 0xF << MASTERID2_SHIFT;
        /// Bits 3:0   - MASTERID1[3:0] (r)
        pub const MASTERID1_SHIFT: u32 = 0;
        pub const MASTERID1_MASK: u32 = 0xF;
    }

    /// Bit fields for HSEM_HWCFGR1
    pub mod hwcfgr1_fields {
        /// Bit 18 - CFEN: compartment filtering supported (r)
        pub const CFEN_BIT: u32 = 1 << 18;
        /// Bit 17 - PRIVEN: privilege protection supported (r)
        pub const PRIVEN_BIT: u32 = 1 << 17;
        /// Bit 16 - SECEN: security protection supported (r)
        pub const SECEN_BIT: u32 = 1 << 16;
        /// Bits 15:12 - CID_WIDTH[3:0]: CID bus width (r)
        pub const CID_WIDTH_SHIFT: u32 = 12;
        pub const CID_WIDTH_MASK: u32 = 0xF << CID_WIDTH_SHIFT;
        /// Bits 11:8  - NBINT[3:0]: number of interrupt lines / supported master IDs (r)
        pub const NBINT_SHIFT: u32 = 8;
        pub const NBINT_MASK: u32 = 0xF << NBINT_SHIFT;
        /// Bits 7:0   - NBSEM[7:0]: number of semaphores (r)
        pub const NBSEM_SHIFT: u32 = 0;
        pub const NBSEM_MASK: u32 = 0xFF;
    }

    /// Bit fields for HSEM_VERR
    pub mod verr_fields {
        /// Bits 7:4 - MAJREV[3:0]: major revision (r)
        pub const MAJREV_SHIFT: u32 = 4;
        pub const MAJREV_MASK: u32 = 0xF << MAJREV_SHIFT;
        /// Bits 3:0 - MINREV[3:0]: minor revision (r)
        pub const MINREV_SHIFT: u32 = 0;
        pub const MINREV_MASK: u32 = 0xF;
    }
}

// ============================================================================
// Reset values (for verification / init checks)
// ============================================================================
pub mod reset_values {
    pub const HSEM_RX_RESET: u32 = 0x0000_0000;
    pub const HSEM_RLRX_RESET: u32 = 0x0000_0000;
    pub const HSEM_CNIER_RESET: u32 = 0x0000_0000;
    pub const HSEM_CNICR_RESET: u32 = 0x0000_0000;
    pub const HSEM_CNISR_RESET: u32 = 0x0000_0000;
    pub const HSEM_CNMISR_RESET: u32 = 0x0000_0000;
    pub const HSEM_SCNIER_RESET: u32 = 0x0000_0000;
    pub const HSEM_SCNICR_RESET: u32 = 0x0000_0000;
    pub const HSEM_SCNISR_RESET: u32 = 0x0000_0000;
    pub const HSEM_SCNMISR_RESET: u32 = 0x0000_0000;
    pub const HSEM_SECCFGR_RESET: u32 = 0x0000_0000;
    pub const HSEM_PRIVCFGR_RESET: u32 = 0x0000_0000;
    pub const HSEM_CNCIDCFGR_RESET: u32 = 0x0000_0000;
    pub const HSEM_CR_RESET: u32 = 0x0000_0000;
    pub const HSEM_KEYR_RESET: u32 = 0x0000_0000;
    pub const HSEM_GPCIDCFGR_RESET: u32 = 0x0000_0000;
    pub const HSEM_HWCFGR2_RESET: u32 = 0x0000_0321;
    pub const HSEM_HWCFGR1_RESET: u32 = 0x0007_3310;
    pub const HSEM_VERR_RESET: u32 = 0x0000_0020;
    pub const HSEM_IPIDR_RESET: u32 = 0x0010_0072;
    pub const HSEM_SIDR_RESET: u32 = 0xA3C5_DD01;
}

pub const HSEM_SEM_COUNT: u8 = 16;

/// LOCKID this core writes when locking. Must match the AHB bus master ID.
/// Processor 2 (M33-core) = MASTERID 2 per Table 188, STM32MP2.
#[cfg(not(feature = "std"))]
pub(crate) const M_CORE_LOCKID: u8 = 2;

/// LOCKID for the A-core (Linux side). MASTERID 1 per Table 188, STM32MP2.
#[cfg(feature = "std")]
pub(crate) const A_CORE_LOCKID: u8 = 1;
