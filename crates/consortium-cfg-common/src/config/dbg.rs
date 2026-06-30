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
///
/// The optional top-level `mode` field selects how the host (AP) consumes the
/// rings; the per-core tables flatten alongside it, so a manifest reads:
///
/// ```toml
/// [dbg]
/// mode = "auto"
///
/// [dbg.cm33]
/// memory = { base = 0x00408000, length = 0x2000 }
/// mtu = 64
/// ```
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Dbg {
    /// How the host decodes the rings. Defaults to [`DbgMode::Auto`].
    #[serde(default)]
    pub mode: DbgMode,
    /// Per-coprocessor debug loggers, keyed by visible core name (e.g. `cm33`).
    ///
    /// `flatten` captures every `[dbg.<core>]` table; the named `mode` field is
    /// consumed first, so it never appears here.
    #[serde(flatten)]
    pub cores: HashMap<VisibleCore, DbgCore>,
}

/// How the host (AP) consumes each coprocessor's debug ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DbgMode {
    /// Generate a background task per ring that drains it, decodes the defmt
    /// stream, and forwards each frame to `tracing` at the mapped level. Turnkey.
    #[default]
    Auto,
    /// Generate accessors that hand the integrator the host ring readers and
    /// defmt contexts; the integrator drives decoding itself. No background task.
    Interface,
}

/// `[dbg.<core>]` section: a single coprocessor's debug log ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DbgCore {
    /// Shared-memory region backing the debug log ring.
    pub memory: MemoryRegion,
    /// Maximum transfer unit for debug frames.
    pub mtu: u32,
}
