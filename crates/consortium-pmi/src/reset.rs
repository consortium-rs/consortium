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

//! Reset lines.

use crate::MaybeSend;

crate::resource_id! {
    /// A reset domain, numbered by the backend.
    ResetId
}

/// Asserts and releases resets.
pub trait ResetProvider {
    /// Backend error.
    type Error: core::fmt::Debug;

    /// Hold `id` in reset until [`deassert`](Self::deassert).
    fn assert(&mut self, id: ResetId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Release `id` from reset.
    fn deassert(
        &mut self,
        id: ResetId,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Pulse `id`: assert, then release, as one operation.
    fn reset(&mut self, id: ResetId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}
