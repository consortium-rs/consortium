//! Peripheral clock gates (CCM LPCG) the controller core opens for itself.
//!
//! On i.MX 95 the System Manager owns every clock, so this module is inert there
//! ([`lpcg`] is `None` for every peripheral). On i.MX 93 there is no System
//! Manager: Linux's CCM driver owns the gates, and its `clk_disable_unused`
//! closes every one without a kernel consumer at boot. A peripheral the CM33
//! drives is such a gate, and touching it raises a precise BusFault on its first
//! register access — LPIT1's `MCR` write at `0x442F_0008` was the first one seen.
//!
//! Drivers therefore open their gate where they consume the peripheral's
//! singleton: each `Instance` carries its gate as `const LPCG`, looked up here by
//! singleton name, and the constructor calls [`ungate`] before any MMIO.

/// `CCM_CTRL` base in the CM33's non-secure view (`mimx93_cm33.json`).
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const CCM_BASE: usize = 0x4445_0000;

// LPCG register offsets within one gate, from Linux's `clk-gate-93.c`.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const DIRECT_OFFSET: usize = 0x00;
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const LPM_CUR_OFFSET: usize = 0x1c;
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const AUTHEN_OFFSET: usize = 0x30;
/// `AUTHEN.CPULPM_EN`: the gate follows `LPM_CUR` rather than `DIRECT`.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const AUTHEN_CPULPM_EN: u32 = 1 << 2;
/// `LPM_CUR` setting that keeps the clock on in every CPU mode.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const LPM_SETTING_ON: u32 = 0x4;
/// `DIRECT.ON`.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const DIRECT_ON: u32 = 1 << 0;

/// The CCM-relative offset of the LPCG gating peripheral singleton `name`, or
/// `None` when this core need not (or cannot) open one.
///
/// The i.MX 93 offsets are Linux's `clk-imx93.c` gate table (`ccgr_array`),
/// covering every singleton a HAL driver or doorbell consumes. Both MU windows
/// of one side share a gate (`mu1_b`/`mu2_b` are one LPCG under a share count),
/// so MU1 and MU2 name the same offset. eDMA channel singletons
/// (`DMA3_CH<n>`, `DMA4_CH<n>`) resolve to their controller's gate: `DMA3` is
/// Linux's `edma1` at `0x4400_0000`, `DMA4` its `edma2` at `0x4200_0000`.
/// Unlisted peripherals return `None`: add an entry here when a driver starts
/// consuming a new one on i.MX 93.
pub const fn lpcg(name: &str) -> Option<u32> {
    #[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
    {
        if starts_with(name, "DMA3_CH") {
            return Some(0x8540);
        }
        if starts_with(name, "DMA4_CH") {
            return Some(0x8580);
        }
        let mut i = 0;
        while i < IMX93_LPCG.len() {
            if str_eq(IMX93_LPCG[i].0, name) {
                return Some(IMX93_LPCG[i].1);
            }
            i += 1;
        }
        None
    }
    #[cfg(not(all(feature = "mimx93", not(feature = "mimx95"))))]
    {
        let _ = name;
        None
    }
}

/// i.MX 93 LPCG offsets by singleton name, from Linux's `clk-imx93.c`.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const IMX93_LPCG: &[(&str, u32)] = &[
    ("ADC1", 0x82c0),
    ("WDOG1", 0x8300),
    ("WDOG2", 0x8340),
    ("WDOG3", 0x8380),
    ("WDOG4", 0x83c0),
    ("WDOG5", 0x8400),
    ("MU1_MUA", 0x84c0),
    ("MU2_MUA", 0x84c0),
    ("MU1_MUB", 0x8500),
    ("MU2_MUB", 0x8500),
    ("GPIO1", 0x8880),
    ("GPIO2", 0x88c0),
    ("GPIO3", 0x8900),
    ("GPIO4", 0x8940),
    ("LPIT1", 0x8a00),
    ("LPIT2", 0x8a40),
    ("LPTMR1", 0x8a80),
    ("LPTMR2", 0x8ac0),
    ("TPM1", 0x8b00),
    ("TPM2", 0x8b40),
    ("TPM3", 0x8b80),
    ("TPM4", 0x8bc0),
    ("TPM5", 0x8c00),
    ("TPM6", 0x8c40),
    ("CAN1", 0x8c80),
    ("CAN2", 0x8cc0),
    ("LPUART1", 0x8d00),
    ("LPUART2", 0x8d40),
    ("LPUART3", 0x8d80),
    ("LPUART4", 0x8dc0),
    ("LPUART5", 0x8e00),
    ("LPUART6", 0x8e40),
    ("LPUART7", 0x8e80),
    ("LPUART8", 0x8ec0),
    ("LPI2C1", 0x8f00),
    ("LPI2C2", 0x8f40),
    ("LPI2C3", 0x8f80),
    ("LPI2C4", 0x8fc0),
    ("LPI2C5", 0x9000),
    ("LPI2C6", 0x9040),
    ("LPI2C7", 0x9080),
    ("LPI2C8", 0x90c0),
    ("LPSPI1", 0x9100),
    ("LPSPI2", 0x9140),
    ("LPSPI3", 0x9180),
    ("LPSPI4", 0x91c0),
    ("LPSPI5", 0x9200),
    ("LPSPI6", 0x9240),
    ("LPSPI7", 0x9280),
    ("LPSPI8", 0x92c0),
    ("I3C1", 0x9300),
    ("I3C2", 0x9340),
];

/// Opens the LPCG at `offset` (from [`lpcg`]); `None` is a no-op.
///
/// Mirrors Linux's `imx93_clk_gate_do_hardware`: a gate under CPU low-power
/// control (`AUTHEN.CPULPM_EN`) is opened through `LPM_CUR`, any other through
/// `DIRECT.ON`. If this core's domain is not on the gate's `AUTHEN` whitelist
/// the write is ignored, and the peripheral's first access still faults.
pub fn ungate(offset: Option<u32>) {
    #[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
    if let Some(offset) = offset {
        let gate = CCM_BASE + offset as usize;
        // Every access below targets an LPCG register block inside `CCM_CTRL`,
        // an always-clocked block the CM33 can reach; the offsets come from the
        // vendor gate table above. Opening an already-open gate is idempotent,
        // and only the consumer of the gated peripheral's singleton calls this.
        //
        // SAFETY: `AUTHEN` of the gate described above.
        let authen = unsafe { ((gate + AUTHEN_OFFSET) as *const u32).read_volatile() };
        if authen & AUTHEN_CPULPM_EN != 0 {
            // SAFETY: `LPM_CUR` of the gate described above.
            unsafe { ((gate + LPM_CUR_OFFSET) as *mut u32).write_volatile(LPM_SETTING_ON) };
        } else {
            let direct = (gate + DIRECT_OFFSET) as *mut u32;
            // SAFETY: `DIRECT` of the gate described above.
            let value = unsafe { direct.read_volatile() };
            // SAFETY: as above; sets only `DIRECT.ON`, preserving the rest.
            unsafe { direct.write_volatile(value | DIRECT_ON) };
        }
        // The peripheral access that follows must not overtake the gate write:
        // the two target different Device regions, which Armv8-M does not order.
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        // SAFETY: `dsb sy` only waits for outstanding memory accesses; it reads
        // and writes no registers or memory of its own.
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("dsb sy", options(nostack, preserves_flags));
        }
    }
    #[cfg(not(all(feature = "mimx93", not(feature = "mimx95"))))]
    let _ = offset;
}

/// `const` prefix test (`str::starts_with` is not `const`).
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const fn starts_with(s: &str, prefix: &str) -> bool {
    let (s, prefix) = (s.as_bytes(), prefix.as_bytes());
    if s.len() < prefix.len() {
        return false;
    }
    let mut i = 0;
    while i < prefix.len() {
        if s[i] != prefix[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// `const` byte-wise string equality (`str::eq` is not `const`).
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(all(test, feature = "mimx93", not(feature = "mimx95")))]
mod tests {
    use super::lpcg;

    #[test]
    fn bring_up_gates_match_the_vendor_table() {
        assert_eq!(lpcg("LPIT1"), Some(0x8a00));
        assert_eq!(lpcg("MU1_MUB"), Some(0x8500));
        assert_eq!(lpcg("MU2_MUB"), Some(0x8500));
        assert_eq!(lpcg("MU1_MUA"), Some(0x84c0));
    }

    #[test]
    fn driver_gates_match_the_vendor_table() {
        assert_eq!(lpcg("LPUART1"), Some(0x8d00));
        assert_eq!(lpcg("LPI2C8"), Some(0x90c0));
        assert_eq!(lpcg("LPSPI3"), Some(0x9180));
        assert_eq!(lpcg("CAN2"), Some(0x8cc0));
        assert_eq!(lpcg("TPM6"), Some(0x8c40));
        assert_eq!(lpcg("LPTMR2"), Some(0x8ac0));
        assert_eq!(lpcg("GPIO2"), Some(0x88c0));
        assert_eq!(lpcg("ADC1"), Some(0x82c0));
        assert_eq!(lpcg("I3C2"), Some(0x9340));
        assert_eq!(lpcg("WDOG5"), Some(0x8400));
        assert_eq!(lpcg("DMA3_CH0"), Some(0x8540));
        assert_eq!(lpcg("DMA4_CH63"), Some(0x8580));
    }

    /// Every singleton the generated i.MX 93 map binds to a HAL driver must
    /// have a gate, or its constructor's first register access BusFaults.
    #[test]
    fn every_driver_bound_singleton_has_a_gate() {
        let map = include_str!("peripherals/imx93.rs");
        let mut checked = 0;
        for line in map.lines() {
            let Some((_, rest)) = line.trim().split_once("::impl_") else {
                continue;
            };
            let Some((_, args)) = rest.split_once("!(") else {
                continue;
            };
            let name = args.split(',').next().unwrap().trim();
            assert!(lpcg(name).is_some(), "{name} has no LPCG entry");
            checked += 1;
        }
        assert!(checked > 100, "parsed only {checked} bindings");
    }

    #[test]
    fn unlisted_peripherals_have_no_gate() {
        assert_eq!(lpcg("LPIT"), None);
        assert_eq!(lpcg("LPIT10"), None);
        assert_eq!(lpcg("GPIO5"), None);
        assert_eq!(lpcg("DMA3"), None);
    }
}
