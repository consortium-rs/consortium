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

//! HSEM channel space and validation.
//!
//! A channel is a hardware semaphore id, numbered `0..`[`HSEM_CHAN_COUNT`].
//! Semaphores in [`HSEM_CHAN_OCCUPIED`] are reserved for system/firmware use.

use core::ops::Range;

use consortium_ipc::ChanValidate;

use crate::error::HsemError;

/// Number of hardware semaphores in the HSEM peripheral.
pub const HSEM_CHAN_COUNT: u8 = 16;

/// Semaphore ids reserved for system/firmware use; not available to user
/// channels.
pub const HSEM_CHAN_OCCUPIED: Range<u8> = 0..2;

/// Validates HSEM semaphore ids.
pub struct HsemChanValidate;

impl ChanValidate for HsemChanValidate {
    type Error = HsemError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        if HSEM_CHAN_OCCUPIED.contains(&id) {
            Err(HsemError::ChanReserved)
        } else if id >= HSEM_CHAN_COUNT {
            Err(HsemError::ChanOutOfRange)
        } else {
            // SAFETY: `id` is outside the reserved range and below the count,
            // so it is a valid HSEM semaphore id.
            Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_ipc::Chan;

    #[test]
    fn accepts_unreserved_semaphores() {
        for id in HSEM_CHAN_OCCUPIED.end..HSEM_CHAN_COUNT {
            assert_eq!(Chan::new::<HsemChanValidate>(id).unwrap().id(), id);
        }
    }

    #[test]
    fn rejects_reserved_and_out_of_range() {
        assert_eq!(
            Chan::new::<HsemChanValidate>(0),
            Err(HsemError::ChanReserved)
        );
        assert_eq!(
            Chan::new::<HsemChanValidate>(HSEM_CHAN_COUNT),
            Err(HsemError::ChanOutOfRange)
        );
    }
}
