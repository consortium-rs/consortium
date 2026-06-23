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

/// HMI build configuration consumed by `consortium-hmi` at compile time.
///
/// Declared in `Consortium.toml` under `[hmi]` and injected as environment
/// variables by `ApplicationCompiler` before invoking `cargo build`.
/// `consortium-hmi` reads them via `option_env!` at compile time; no file
/// I/O occurs on the target device.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct HmiConfig {
    /// Absolute path to the compiled frontend dist directory.
    /// Served via the `csti://` protocol in release builds.
    /// Relative paths in `Consortium.toml` are resolved to absolute by the
    /// builder before injection.
    pub dist: String,
    /// Dev-server URL loaded directly in debug builds.
    /// When absent, debug builds fall back to the `csti://` static handler.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub debug_url: Option<String>,
}
