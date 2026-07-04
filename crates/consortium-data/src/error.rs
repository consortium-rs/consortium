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

use thiserror::Error;

/// Errors raised while resolving and parsing the bundled chip data.
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataError {
    #[error("no chip-family database matches chip '{0}'")]
    UnknownChip(String),

    #[error("failed to parse chip database for '{0}': {1}")]
    DatabaseParse(String, String),

    #[error("no peripheral/interrupt map is bundled for chip '{chip}' core '{core}'")]
    ChipMapUnavailable { chip: String, core: String },

    #[error("failed to parse the peripheral map for chip '{chip}' core '{core}': {detail}")]
    ChipMapParse {
        chip: String,
        core: String,
        detail: String,
    },
}
