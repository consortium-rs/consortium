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
/// The i.MX 93 offsets are Linux's `clk-imx93.c` gate table. Both MU windows of
/// one side share a gate (`mu1_b`/`mu2_b` are one LPCG under a share count), so
/// MU1 and MU2 name the same offset. Unlisted peripherals return `None`: add an
/// entry here when a driver starts consuming a new one on i.MX 93.
pub const fn lpcg(name: &str) -> Option<u32> {
    #[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
    {
        const TABLE: &[(&str, u32)] = &[
            ("LPIT1", 0x8a00),
            ("LPIT2", 0x8a40),
            ("MU1_MUA", 0x84c0),
            ("MU2_MUA", 0x84c0),
            ("MU1_MUB", 0x8500),
            ("MU2_MUB", 0x8500),
        ];
        let mut i = 0;
        while i < TABLE.len() {
            if str_eq(TABLE[i].0, name) {
                return Some(TABLE[i].1);
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
        // SAFETY: `gate` is an LPCG register block inside `CCM_CTRL`, an
        // always-clocked block the CM33 can reach; the offsets come from the
        // vendor gate table above. Opening an already-open gate is idempotent,
        // and only the consumer of the gated peripheral's singleton calls this.
        unsafe {
            let authen = ((gate + AUTHEN_OFFSET) as *const u32).read_volatile();
            if authen & AUTHEN_CPULPM_EN != 0 {
                ((gate + LPM_CUR_OFFSET) as *mut u32).write_volatile(LPM_SETTING_ON);
            } else {
                let direct = (gate + DIRECT_OFFSET) as *mut u32;
                direct.write_volatile(direct.read_volatile() | DIRECT_ON);
            }
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
    fn unlisted_peripherals_have_no_gate() {
        assert_eq!(lpcg("LPIT"), None);
        assert_eq!(lpcg("LPIT10"), None);
        assert_eq!(lpcg("GPIO2"), None);
    }
}
