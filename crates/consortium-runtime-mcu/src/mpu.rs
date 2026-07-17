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

//! ARMv7-M Memory Protection Unit helpers.
//!
//! Shared-memory IPC and debug rings on a cached core (Cortex-M7) must be
//! accessed non-cacheably so the writing and reading sides observe the same
//! bytes without manual clean/invalidate around every descriptor poll. This
//! module programs a single MPU region as Normal, shareable, non-cacheable
//! memory and enables the MPU with the default map kept as a privileged
//! background region.
//!
//! [`disable_cache`] has the same signature on both supported families and is
//! selected by the active target's MPU register block (see
//! [`cortex_m::peripheral::mpu::RegisterBlock`]):
//!
//! - ARMv7-M (`thumbv7em`/`thumbv7m`, e.g. Cortex-M7) uses the `RBAR`/`RASR`
//!   block: power-of-two regions with size encoded in `RASR.SIZE` and inline
//!   `TEX/C/B/S` attributes; the carveout is marked Normal non-cacheable.
//! - ARMv8-M (`thumbv8m`, e.g. Cortex-M33) uses the `RBAR`/`RLAR` base/limit
//!   block with attributes indirected through `MAIR0`.
//!
//! ARMv6-M (`thumbv6m`, e.g. Cortex-M0+) has no data cache, so its SRAM is
//! never cached and there is nothing to disable; no helper is provided there.
//!
//! On targets without a cache to manage (ARMv6-M, host builds, RISC-V) the
//! module is empty.

#[cfg(armv7m)]
pub use armv7m::*;
#[cfg(armv8m)]
pub use armv8m::*;

#[cfg(armv7m)]
mod armv7m {
    use cortex_m::peripheral::MPU;

    /// Smallest region the ARMv7-M MPU can describe (`SIZE` field value 4).
    pub const MIN_REGION_SIZE: u32 = 32;

    // `RASR` attribute fields, see Arm DUI 0646C Table 4-53/4-55/4-57.
    const RASR_ENABLE: u32 = 1 << 0;
    const RASR_SIZE_SHIFT: u32 = 1;
    const RASR_B_SHIFT: u32 = 16;
    const RASR_C_SHIFT: u32 = 17;
    const RASR_S_SHIFT: u32 = 18;
    const RASR_TEX_SHIFT: u32 = 19;
    const RASR_AP_SHIFT: u32 = 24;
    const RASR_XN_SHIFT: u32 = 28;

    // Normal, non-cacheable memory: TEX=0b001, C=0, B=0 (Table 4-55).
    const TEX_NONCACHEABLE: u32 = 0b001;
    const C_NONCACHEABLE: u32 = 0;
    const B_NONCACHEABLE: u32 = 0;
    // Shareable, so both cores see a coherent view of the carveout.
    const S_SHAREABLE: u32 = 1;
    // Full read/write for privileged and unprivileged software (Table 4-57).
    const AP_RW: u32 = 0b011;
    // Execute-never: these regions hold data buffers, not code.
    const XN: u32 = 1;

    // `MPU_CTRL` bits, see Arm DUI 0646C Table 4-50.
    const CTRL_ENABLE: u32 = 1 << 0;
    const CTRL_PRIVDEFENA: u32 = 1 << 2;

    /// Encodes a region size into the `RASR.SIZE` field.
    ///
    /// The region size is `2^(SIZE + 1)`, so `SIZE = log2(size) - 1`. `size`
    /// must be a power of two no smaller than [`MIN_REGION_SIZE`]; callers are
    /// responsible for guaranteeing that (see [`disable_cache`]).
    const fn size_field(size: u32) -> u32 {
        // `31 - leading_zeros` is `log2(size)` for a power of two.
        (31 - size.leading_zeros()) - 1
    }

    /// Configures MPU `region` to cover `base..base + size` as Normal,
    /// shareable, non-cacheable memory and enables the MPU, keeping the default
    /// memory map as a privileged background region.
    ///
    /// The MPU is left enabled with `PRIVDEFENA` set, so addresses outside any
    /// configured region keep their default attributes for privileged code.
    ///
    /// # Safety
    ///
    /// This writes the raw MPU registers and changes the memory attributes of a
    /// live region, so the caller must ensure all of the following:
    ///
    /// 1. `region` is a valid region number for this implementation and the
    ///    field values passed here are the ones intended for it; this
    ///    reprograms whichever region is selected without preserving its prior
    ///    contents.
    /// 2. The usual `cortex_m` MPU preconditions hold: an MPU is present, the
    ///    code runs in privileged mode with exclusive access to the MPU, and no
    ///    other context concurrently reprograms it.
    /// 3. `size` is a power of two and at least [`MIN_REGION_SIZE`] (32) bytes,
    ///    and `base` is aligned to `size` (the ARMv7-M `RBAR`/`RASR` region
    ///    rules). Violating this corrupts the region the MPU actually enforces.
    pub unsafe fn disable_cache(mpu: &mut MPU, region: u8, base: u32, size: u32) {
        debug_assert!(size >= MIN_REGION_SIZE, "MPU region must be >= 32 bytes");
        debug_assert!(
            size.is_power_of_two(),
            "MPU region size must be a power of two"
        );
        debug_assert!(base % size == 0, "MPU base must be aligned to its size");

        let rasr = RASR_ENABLE
            | (size_field(size) << RASR_SIZE_SHIFT)
            | (TEX_NONCACHEABLE << RASR_TEX_SHIFT)
            | (C_NONCACHEABLE << RASR_C_SHIFT)
            | (B_NONCACHEABLE << RASR_B_SHIFT)
            | (S_SHAREABLE << RASR_S_SHIFT)
            | (AP_RW << RASR_AP_SHIFT)
            | (XN << RASR_XN_SHIFT);

        // Ordering: disable the MPU, program the region, re-enable, then
        // barrier so the new attributes are in effect before later accesses.
        // SAFETY: per the contract above the caller holds exclusive privileged
        // access to the MPU. We disable it before reprogramming so no access is
        // governed by a half-written region.
        unsafe {
            mpu.ctrl.write(0);
            // Select the region, then write its base (already size-aligned, so
            // the reserved low bits are zero) and attributes.
            mpu.rnr.write(region as u32);
            mpu.rbar.write(base);
            mpu.rasr.write(rasr);
            // Re-enable the MPU; PRIVDEFENA keeps the default map as a
            // privileged background region for everything else.
            mpu.ctrl.write(CTRL_ENABLE | CTRL_PRIVDEFENA);
        }

        // Ensure the register writes complete and are visible before any
        // subsequent memory access relies on the new attributes.
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn size_field_encodes_log2_minus_one() {
            assert_eq!(size_field(32), 4); // minimum region
            assert_eq!(size_field(1024), 9); // 1 KiB
            assert_eq!(size_field(1 << 20), 19); // 1 MiB
        }
    }
}

#[cfg(armv8m)]
mod armv8m {
    use cortex_m::peripheral::MPU;

    /// Region granule: ARMv8-M MPU base and limit are aligned to 32 bytes.
    pub const REGION_GRANULE: u32 = 32;

    // `RBAR` fields, see Arm 100235 Table 4-57.
    const RBAR_SH_SHIFT: u32 = 3;
    const RBAR_AP_SHIFT: u32 = 1;
    // Outer shareable, so the carveout is coherent across cores.
    const SH_OUTER_SHAREABLE: u32 = 0b10;
    // Read/write by any privilege level.
    const AP_RW: u32 = 0b01;
    // Execute-never: these regions hold data buffers, not code.
    const RBAR_XN: u32 = 1;

    // `RLAR` fields, see Arm 100235 Table 4-59.
    const RLAR_EN: u32 = 1 << 0;
    const RLAR_ATTRINDX_SHIFT: u32 = 1;
    // Attribute index into `MAIR0` we program for the non-cacheable encoding.
    const ATTR_INDEX: u32 = 0;
    // `MAIR` byte for Normal, outer + inner non-cacheable memory (Arm 100235
    // 4.5.17): outer nibble 0b0100, inner nibble 0b0100.
    const MAIR_NORMAL_NONCACHEABLE: u32 = 0x44;

    // `MPU_CTRL` bits, see Arm 100235 Table 4-56.
    const CTRL_ENABLE: u32 = 1 << 0;
    const CTRL_PRIVDEFENA: u32 = 1 << 2;

    /// Configures MPU `region` to cover `base..base + size` as Normal,
    /// outer-shareable, non-cacheable memory and enables the MPU, keeping the
    /// default memory map as a privileged background region.
    ///
    /// The non-cacheable encoding is installed in `MAIR0` attribute slot
    /// [`ATTR_INDEX`] and referenced by the region; the MPU is left enabled with
    /// `PRIVDEFENA` set.
    ///
    /// # Safety
    ///
    /// This writes the raw MPU registers and changes the memory attributes of a
    /// live region, so the caller must ensure all of the following:
    ///
    /// 1. `region` is a valid region number for this implementation and the
    ///    field values passed here are the ones intended for it; this
    ///    reprograms whichever region is selected, and overwrites `MAIR0`
    ///    attribute slot [`ATTR_INDEX`], without preserving prior contents.
    /// 2. The usual `cortex_m` MPU preconditions hold: an MPU is present, the
    ///    code runs in privileged mode with exclusive access to the MPU, and no
    ///    other context concurrently reprograms it.
    /// 3. `base` and `size` are multiples of [`REGION_GRANULE`] (32) with
    ///    `size >= REGION_GRANULE` (the ARMv8-M 32-byte base/limit granule).
    ///    Unlike ARMv7-M the size need not be a power of two. Violating this
    ///    corrupts the region the MPU actually enforces.
    pub unsafe fn disable_cache(mpu: &mut MPU, region: u8, base: u32, size: u32) {
        debug_assert!(size >= REGION_GRANULE, "MPU region must be >= 32 bytes");
        debug_assert!(
            base.is_multiple_of(REGION_GRANULE),
            "MPU base must be 32-byte aligned"
        );
        debug_assert!(
            size.is_multiple_of(REGION_GRANULE),
            "MPU size must be a 32-byte multiple"
        );

        // `RBAR.BASE` is bits[31:5]; `base` is 32-aligned so the low bits are
        // zero and the field-bearing bits carry the shareability/AP/XN flags.
        let rbar = (base & 0xFFFF_FFE0)
            | (SH_OUTER_SHAREABLE << RBAR_SH_SHIFT)
            | (AP_RW << RBAR_AP_SHIFT)
            | RBAR_XN;
        // `RLAR.LIMIT` is bits[31:5] of the last byte in the region; hardware
        // postfixes 0x1F, so mask the inclusive limit to the granule.
        let limit = base + size - 1;
        let rlar = (limit & 0xFFFF_FFE0) | (ATTR_INDEX << RLAR_ATTRINDX_SHIFT) | RLAR_EN;

        // SAFETY: per the contract above the caller holds exclusive privileged
        // access to the MPU. We disable it before reprogramming so no access is
        // governed by a half-written region.
        unsafe {
            mpu.ctrl.write(0);
            // Install the non-cacheable encoding in attribute slot ATTR_INDEX
            // (low byte of MAIR0), leaving the other slots untouched.
            let mair0 = mpu.mair[0].read();
            let cleared = mair0 & !(0xFF << (8 * ATTR_INDEX));
            mpu.mair[0].write(cleared | (MAIR_NORMAL_NONCACHEABLE << (8 * ATTR_INDEX)));

            mpu.rnr.write(region as u32);
            mpu.rbar.write(rbar);
            mpu.rlar.write(rlar);
            // Re-enable the MPU; PRIVDEFENA keeps the default map as a
            // privileged background region for everything else.
            mpu.ctrl.write(CTRL_ENABLE | CTRL_PRIVDEFENA);
        }

        // Ensure the register writes complete and are visible before any
        // subsequent memory access relies on the new attributes.
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }
}
