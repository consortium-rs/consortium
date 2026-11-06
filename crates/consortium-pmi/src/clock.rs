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

//! Clock gating and rate control.

use crate::MaybeSend;

crate::resource_id! {
    /// A clock, numbered by the backend (an SCMI clock domain on an SCMI backend).
    ClockId
}

/// Gates clocks and sets their rates.
pub trait ClockProvider {
    /// Backend error.
    type Error: core::fmt::Debug;

    /// Ungate `id`.
    fn enable(&mut self, id: ClockId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Gate `id`.
    fn disable(&mut self, id: ClockId)
    -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Request `hz` for `id`. The backend may round; read [`rate`](Self::rate)
    /// back for the rate actually applied.
    fn set_rate(
        &mut self,
        id: ClockId,
        hz: u64,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// The current rate of `id`, in hertz.
    fn rate(&mut self, id: ClockId) -> impl Future<Output = Result<u64, Self::Error>> + MaybeSend;
}
