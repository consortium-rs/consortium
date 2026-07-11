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

//! IPCC channel space and validation.
//!
//! A channel is an IPCC hardware channel id, numbered `0..`[`IPCC_CHAN_COUNT`].
//! Channels in [`IPCC_CHAN_OCCUPIED`] are reserved for system/firmware use.

use core::ops::Range;

use consortium_ipc::ChanValidate;

use crate::error::IpccError;

/// Number of IPCC channels per processor bank.
pub const IPCC_CHAN_COUNT: u8 = 16;

/// Channel ids reserved for system/firmware use; not available to user
/// channels. Channel 0 is held for the system/firmware handshake.
pub const IPCC_CHAN_OCCUPIED: Range<u8> = 0..1;

/// Validates IPCC channel ids.
pub struct IpccChanValidate;

impl ChanValidate for IpccChanValidate {
    type Error = IpccError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        if IPCC_CHAN_OCCUPIED.contains(&id) {
            Err(IpccError::ChanReserved)
        } else if id >= IPCC_CHAN_COUNT {
            Err(IpccError::ChanOutOfRange)
        } else {
            // SAFETY: `id` is outside the reserved range and below the count,
            // so it is a valid IPCC channel id.
            Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_ipc::Chan;

    #[test]
    fn rejects_reserved_and_out_of_range() {
        assert_eq!(
            Chan::new::<IpccChanValidate>(0),
            Err(IpccError::ChanReserved)
        );
        assert_eq!(
            Chan::new::<IpccChanValidate>(IPCC_CHAN_COUNT),
            Err(IpccError::ChanOutOfRange)
        );
    }
}
