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

//! Transpilers from configuration/database resources into device components.

use crate::components::uio::UioDevice;
use crate::error::CfgError;

pub mod memory_to_uio;

/// Conversion from a configuration or database resource into a [`UioDevice`].
pub trait ToUio {
    /// Builds the UIO device that exposes this resource to the Linux side.
    ///
    /// Returns `Ok(None)` when the resource exposes nothing, and an error when
    /// the resource cannot be represented as a UIO device.
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError>;
}
