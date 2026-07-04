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

pub mod bridge;
pub mod components;
pub mod error;
pub mod render;
pub mod session;

pub use render::render;

// The chip-family database and per-core chip maps live in `consortium-data`.
// Re-export them under their historical paths so existing `crate::database` /
// `crate::chipmap` references keep resolving.
pub use consortium_data::{chipmap, database};

pub use consortium_cfg_common::config;
pub use error::CfgError;
pub use session::Session;
