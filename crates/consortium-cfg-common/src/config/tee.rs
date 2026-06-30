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

use uuid::Uuid;

/// Compile-time environment variable carrying the OP-TEE TA UUID.
///
/// The builder's OP-TEE compiler sets this before `cargo build`, and each TA's
/// `build.rs` reads it back, so both sides must name the same variable.
pub const TEE_UUID_ENV: &str = "CONSORTIUM_TEE_UUID";

/// `[tee]` section: trusted execution configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tee {
    /// UUID assigned to the trusted application.
    pub uuid: Uuid,
}
