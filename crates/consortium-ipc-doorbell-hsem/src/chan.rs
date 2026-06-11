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

use crate::error::HsemError;

pub struct HsemChanValidate;

pub const HSEM_SEM_COUNT: u8 = 16;
pub const HSEM_SEM_RESERVED: u8 = 2; // sem 0 and 1 reserved for system use

impl ChanValidate for HsemChanValidate {
    type Error = HsemError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        match id {
            i if i < HSEM_SEM_RESERVED => Err(HsemError::ChanReserved),
            i if i >= HSEM_SEM_COUNT => Err(HsemError::ChanOutOfRange),
            // SAFETY: caller-validated by the match arms above; id is guaranteed to be in the valid range for HSEM semaphores.
            _ => Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) }),
        }
    }
}
