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

use std::collections::HashMap;

use super::{MemoryRegion, VisibleCore};

/// `[dbg]` section: debug transport configuration.
///
/// Each `[dbg.<core>]` table declares a defmt-over-shared-memory logger running
/// on the named coprocessor (`<core>`). The application processor is always the
/// host that decodes those logs, so it is never named here: there is no
/// `[dbg.ap]`. A coprocessor with no `[dbg.<core>]` table has no debug logging
/// configured.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Dbg(
    /// Per-coprocessor debug loggers, keyed by visible core name (e.g. `cm33`).
    pub HashMap<VisibleCore, DbgCore>,
);

/// `[dbg.<core>]` section: a single coprocessor's debug log ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DbgCore {
    /// Shared-memory region backing the debug log ring.
    pub memory: MemoryRegion,
    /// Maximum transfer unit for debug frames.
    pub mtu: u32,
}
