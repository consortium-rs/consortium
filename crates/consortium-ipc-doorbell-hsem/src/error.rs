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

/// Errors raised by the HSEM doorbell.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HsemError {
    /// Semaphore already locked by another master when `ring()` was called.
    #[error("HSEM semaphore already locked by another master")]
    AlreadyLocked,
    /// Chan id is not a valid semaphore id (`>= HSEM_CHAN_COUNT`).
    #[error("HSEM semaphore channel id out of range")]
    ChanOutOfRange,
    /// Chan id falls in the reserved semaphore range and cannot be used for
    /// user channels.
    #[error("HSEM semaphore channel id is reserved")]
    ChanReserved,
}

#[cfg(feature = "defmt")]
impl defmt::Format for HsemError {
    fn format(&self, f: defmt::Formatter<'_>) {
        match self {
            HsemError::AlreadyLocked => {
                defmt::write!(f, "HSEM semaphore already locked by another master")
            }
            HsemError::ChanOutOfRange => defmt::write!(f, "HSEM semaphore channel id out of range"),
            HsemError::ChanReserved => defmt::write!(f, "HSEM semaphore channel id is reserved"),
        }
    }
}
