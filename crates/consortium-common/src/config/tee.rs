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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Compile-time environment variable carrying the OP-TEE TA UUID.
pub const TEE_UUID_ENV: &str = "CONSORTIUM_TEE_UUID";

/// Trusted Application metadata declared in `Consortium.toml` under `[tee]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct TeeConfig {
    /// UUID assigned to the OP-TEE Trusted Application.
    pub uuid: String,
}

impl TeeConfig {
    /// Environment variable used by TA build scripts.
    pub fn env_pair(&self) -> (&'static str, &str) {
        (TEE_UUID_ENV, self.uuid.as_str())
    }
}
