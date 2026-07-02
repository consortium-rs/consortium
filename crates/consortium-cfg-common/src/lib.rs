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

pub mod config;
pub mod database;

/// Re-export of the dependency-free [`consortium_env_value`] crate so existing
/// consumers keep using `consortium_cfg_common::env_value::{parse_u32, parse_u64}`.
/// The helpers live in their own `no_std` crate so bare-metal consumers can use
/// them without pulling in this std-only schema crate.
pub use consortium_env_value as env_value;
