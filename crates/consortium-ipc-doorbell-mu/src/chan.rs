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
//! is one signaling line within that instance, numbered `0..`[`MU_CHAN_COUNT`].
//! The instance exposes two kinds of line, packed into a single channel space:
//!
//! - channels `0..`[`MU_GI_CHAN_COUNT`] are general-purpose interrupt (GI)
//!   lines, driven through `GCR`/`GSR`/`GIER`.
//! - channels [`MU_GI_CHAN_COUNT`]`..`[`MU_CHAN_COUNT`] are receive-full (RF)
//!   lines, driven through the transmit/receive registers and `RSR`/`RCR`.
//!
//! The MU instance itself is selected at the type level by the const `N` of
//! [`crate::abstraction::Mu`].

use core::ops::Range;

use consortium_ipc::ChanValidate;

use crate::error::MuError;

// ── Chip-feature guard ────────────────────────────────────────────────────────

#[cfg(not(any(feature = "mimx93", feature = "mimx95")))]
compile_error!("no chip feature selected; enable one of: `mimx93`, `mimx95`");

// The chip features select different instance counts, base tables and singleton
// bindings, so they cannot be unified. Cargo unifies features across a build
// graph, so this fires when *any* dependent forgets `default-features = false`
// while selecting the non-default chip. Diagnosing it here — rather than through
// the duplicate `mod chip` in the HAL or the duplicate `Interrupt` enum in the
// PAC — is what makes the error name the actual mistake.
#[cfg(all(feature = "mimx93", feature = "mimx95"))]
compile_error!(
    "chip features are mutually exclusive; enable exactly one of: `mimx93`, `mimx95`. \
     A dependency selecting the non-default chip must set `default-features = false`."
);

/// Number of MU peripheral instances on the selected chip.
///
/// | Feature  | Count | Chips         |
/// |----------|-------|---------------|
/// | `mimx95` |   8   | i.MX 95       |
/// | `mimx93` |   2   | i.MX 93       |
#[cfg(feature = "mimx95")]
pub const MU_INSTANCE_COUNT: usize = 8;

/// Number of MU peripheral instances on the selected chip. See the `mimx95`
/// definition for the full table.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
pub const MU_INSTANCE_COUNT: usize = 2;

/// Number of MU peripheral instances on the selected chip. Placeholder for the
/// no-chip-feature build, which the guard above already rejects; it exists so
/// that build fails on the guard rather than on a cascade of missing items.
#[cfg(not(any(feature = "mimx93", feature = "mimx95")))]
pub const MU_INSTANCE_COUNT: usize = 0;

/// Number of general-purpose interrupt (GI) channels per MU instance. These
/// occupy the low end of the channel space, ids `0..MU_GI_CHAN_COUNT`.
pub const MU_GI_CHAN_COUNT: u8 = 4;

/// Number of receive-full (RF) channels per MU instance. These follow the GI
/// channels, ids `MU_GI_CHAN_COUNT..MU_CHAN_COUNT`.
pub const MU_RF_CHAN_COUNT: u8 = 4;

/// Total number of doorbell channels per MU instance: the GI lines followed by
/// the RF lines.
pub const MU_CHAN_COUNT: u8 = MU_GI_CHAN_COUNT + MU_RF_CHAN_COUNT;

/// Channel ids reserved for system/firmware use. All MU GI and RF lines are
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
    fn accepts_all_gi_and_rf_channels() {
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
