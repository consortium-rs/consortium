use consortium_ipc::chan::ChanValidate;

use crate::error::MuError;

// ── Chip-feature guard ────────────────────────────────────────────────────────

#[cfg(all(feature = "imx93", feature = "imx95"))]
compile_error!("features `imx93` and `imx95` are mutually exclusive; enable exactly one");

#[cfg(all(feature = "imx93", feature = "imx8mp"))]
compile_error!("features `imx93` and `imx8mp` are mutually exclusive; enable exactly one");

#[cfg(all(feature = "imx95", feature = "imx8mp"))]
compile_error!("features `imx95` and `imx8mp` are mutually exclusive; enable exactly one");

#[cfg(not(any(feature = "imx93", feature = "imx95", feature = "imx8mp")))]
compile_error!("no chip feature selected; enable exactly one of: `imx93`, `imx95`, `imx8mp`");

// ── MU instance count (chip-specific) ────────────────────────────────────────

/// Number of MU peripheral instances on the selected chip.
///
/// | Feature | Count | Chips                      |
/// |---------|-------|----------------------------|
/// | `imx93` |   2   | i.MX 93  |
/// | `imx95` |   8   | i.MX 95 |
/// | `imx8mp` |   1   | i.MX 8M Plus |
///
/// Defined as a single const using `cfg!` so enabling both features produces
/// the `compile_error!` message above rather than a duplicate-definition error.
pub const MU_INSTANCE_COUNT: usize = if cfg!(feature = "imx93") { 2 } else if cfg!(feature = "imx95") { 8 } else if cfg!(feature = "imx8mp") { 1 } else { 0 };

// ── Channel counts ────────────────────────────────────────────────────────────

/// Total number of virtual channels across all MU instances (4 tx/rx pairs × MU count).
pub const MU_CHAN_COUNT: u8 = (MU_INSTANCE_COUNT * 4) as u8;

/// Channels 0 and 1 are reserved for system/firmware use across all chips.
pub const MU_CHAN_RESERVED: u8 = 2;

// ── Channel validation ────────────────────────────────────────────────────────

pub struct MuChanValidate;

impl ChanValidate for MuChanValidate {
    type Error = MuError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        match id {
            i if i < MU_CHAN_RESERVED => Err(MuError::ChanReserved),
            i if i >= MU_CHAN_COUNT => Err(MuError::ChanOutOfRange),
            _ => Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) }),
        }
    }
}

// ── Channel-to-MU-instance mapping ───────────────────────────────────────────

/// Extension methods for mapping a [`Chan`] to a physical MU instance and
/// its tx/rx register pair index.
///
/// Channels are laid out as 4 consecutive IDs per MU instance:
/// - channels `[0, 4)` → MU1 (index 0), register pairs 0–3
/// - channels `[4, 8)` → MU2 (index 1), register pairs 0–3
/// - ...
/// - channels `[4*(N-1), 4*N)` → MU_N (index N-1), register pairs 0–3
///
/// [`Chan`]: consortium_ipc::Chan
pub trait MuChanInfo {
    /// Zero-based MU instance index (0 = MU1, 1 = MU2, …).
    fn mu_id(&self) -> u8;
    /// TX/RX register pair index within the MU instance (0–3).
    fn tx_rx_id(&self) -> u8;
}

impl MuChanInfo for consortium_ipc::Chan {
    #[inline]
    fn mu_id(&self) -> u8 {
        self.id() / 4
    }

    #[inline]
    fn tx_rx_id(&self) -> u8 {
        self.id() % 4
    }
}
