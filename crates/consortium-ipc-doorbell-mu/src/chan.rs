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

//! MU channel space and validation.
//!
//! A [`crate::doorbell::MuDoorbell`] drives one MU instance, so a channel here
//! is a general-purpose interrupt (GI) line within that instance, numbered
//! `0..`[`MU_CHAN_COUNT`]. The MU instance itself is selected at the type level
//! by the const `N` of [`crate::abstraction::Mu`].

use core::ops::Range;

use consortium_ipc::ChanValidate;

use crate::error::MuError;

// ── Chip-feature guard ────────────────────────────────────────────────────────

#[cfg(not(any(feature = "imx93", feature = "imx95", feature = "imx8mp")))]
compile_error!("no chip feature selected; enable one of: `imx93`, `imx95`, `imx8mp`");

/// Number of MU peripheral instances on the selected chip.
///
/// | Feature  | Count | Chips         |
/// |----------|-------|---------------|
/// | `imx95`  |   8   | i.MX 95       |
/// | `imx93`  |   2   | i.MX 93       |
/// | `imx8mp` |   1   | i.MX 8M Plus  |
///
/// Defined with `cfg!` so enabling several chip features triggers the
/// `compile_error!` above rather than a duplicate-definition error.
pub const MU_INSTANCE_COUNT: usize = if cfg!(feature = "imx95") {
    8
} else if cfg!(feature = "imx93") {
    2
} else if cfg!(feature = "imx8mp") {
    1
} else {
    0
};

/// Number of general-purpose interrupt (GI) channels per MU instance.
pub const MU_CHAN_COUNT: u8 = 4;

/// Channel ids reserved for system/firmware use. The MU GI lines are all
/// available to user channels, so this range is empty.
pub const MU_CHAN_OCCUPIED: Range<u8> = 0..0;

/// Validates GI channel ids for a single MU instance.
pub struct MuChanValidate;

impl ChanValidate for MuChanValidate {
    type Error = MuError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        if MU_CHAN_OCCUPIED.contains(&id) {
            Err(MuError::ChanReserved)
        } else if id >= MU_CHAN_COUNT {
            Err(MuError::ChanOutOfRange)
        } else {
            // SAFETY: `id` is outside the reserved range and below the channel
            // count, so it is a valid GI channel for this MU instance.
            Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_ipc::Chan;

    #[test]
    fn accepts_all_gi_channels() {
        for id in 0..MU_CHAN_COUNT {
            assert_eq!(Chan::new::<MuChanValidate>(id).unwrap().id(), id);
        }
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(
            Chan::new::<MuChanValidate>(MU_CHAN_COUNT),
            Err(MuError::ChanOutOfRange)
        );
    }
}
