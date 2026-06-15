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

use consortium_ipc::ChanValidate;

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
pub const MU_INSTANCE_COUNT: usize = if cfg!(feature = "imx93") {
    2
} else if cfg!(feature = "imx95") {
    8
} else if cfg!(feature = "imx8mp") {
    1
} else {
    0
};

// ── Channel counts ────────────────────────────────────────────────────────────

/// Total number of virtual channels across all MU instances (4 GI channels × MU count).
pub const MU_CHAN_COUNT: u8 = (MU_INSTANCE_COUNT * 4) as u8;

/// Channels 0 and 1 are reserved for system/firmware use across all chips.
pub const MU_CHAN_RESERVED: u8 = 2;

/// Default i.MX95 CA55 <-> CM7 transmit-side channel.
///
/// i.MX95 MU7 and MU8 connect CA55 <-> CM7. Channel IDs are zero-based and
/// laid out four per MU instance, so MU7 starts at channel 24.
#[cfg(feature = "imx95")]
pub const IMX95_CA55_CM7_TX_CHAN: u8 = 24;

/// Default i.MX95 CA55 <-> CM7 receive-side channel.
///
/// Uses MU7 GI1, paired with [`IMX95_CA55_CM7_TX_CHAN`] on MU7 GI0.
#[cfg(feature = "imx95")]
pub const IMX95_CA55_CM7_RX_CHAN: u8 = 25;

// ── Channel validation ────────────────────────────────────────────────────────

pub struct MuChanValidate;

impl ChanValidate for MuChanValidate {
    type Error = MuError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        match id {
            i if i < MU_CHAN_RESERVED => Err(MuError::ChanReserved),
            i if i >= MU_CHAN_COUNT => Err(MuError::ChanOutOfRange),
            // SAFETY: caller-validated by the match arms above; id is guaranteed to be in the valid range for MU channels.
            _ => Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) }),
        }
    }
}

// ── Channel-to-MU-instance mapping ───────────────────────────────────────────

/// Extension methods for mapping a [`Chan`] to a physical MU instance and
/// its general-purpose interrupt index.
///
/// Channels are laid out as 4 consecutive IDs per MU instance:
/// - channels `[0, 4)` -> MU1 (index 0), GI channels 0-3
/// - channels `[4, 8)` -> MU2 (index 1), GI channels 0-3
/// - ...
/// - channels `[4*(N-1), 4*N)` -> MU_N (index N-1), GI channels 0-3
///
/// [`Chan`]: consortium_ipc::Chan
pub trait MuChanInfo {
    /// Zero-based MU instance index (0 = MU1, 1 = MU2, …).
    fn mu_id(&self) -> u8;
    /// General-purpose interrupt index within the MU instance (0-3).
    fn gi_id(&self) -> u8;
}

impl MuChanInfo for consortium_ipc::Chan {
    #[inline]
    fn mu_id(&self) -> u8 {
        self.id() / 4
    }

    #[inline]
    fn gi_id(&self) -> u8 {
        self.id() % 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_ipc::Chan;

    #[test]
    #[cfg(feature = "imx95")]
    fn imx95_default_ca55_cm7_channels_use_mu7() {
        let tx = Chan::new::<MuChanValidate>(IMX95_CA55_CM7_TX_CHAN).unwrap();
        let rx = Chan::new::<MuChanValidate>(IMX95_CA55_CM7_RX_CHAN).unwrap();

        assert_eq!(tx.mu_id(), 6);
        assert_eq!(tx.gi_id(), 0);
        assert_eq!(rx.mu_id(), 6);
        assert_eq!(rx.gi_id(), 1);
    }
}
