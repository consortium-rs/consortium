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

//! Power-domain control.

use crate::MaybeSend;

crate::resource_id! {
    /// A power domain, numbered by the backend.
    PowerDomainId
}

/// The state of a power domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum PowerState {
    /// Powered and usable.
    On,
    /// Powered off; context is lost.
    Off,
    /// A backend-specific state (retention, a low-power level), carried raw.
    Other(u32),
}

/// Switches power domains.
pub trait PowerProvider {
    /// Backend error.
    type Error: core::fmt::Debug;

    /// Move `id` to `state`.
    fn set_state(
        &mut self,
        id: PowerDomainId,
        state: PowerState,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// The current state of `id`.
    fn state(
        &mut self,
        id: PowerDomainId,
    ) -> impl Future<Output = Result<PowerState, Self::Error>> + MaybeSend;
}
