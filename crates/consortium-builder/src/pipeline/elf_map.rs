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

//! Stage 2 glue: map each debug core to the firmware ELF the AP embeds.
//!
//! The defmt "ELF map" is the set of build-host paths the AP codegen
//! `include_bytes!`es (see
//! [`DebugLogs::with_resolved_elf_paths`](consortium_cfg::components::dbg::DebugLogs::with_resolved_elf_paths)).
//! Each `[dbg.<core>]` core must have been compiled in Stage 1; if not, there is
//! nothing to embed and it is a hard error rather than a silent skip.

use std::collections::HashMap;
use std::path::PathBuf;

use consortium_cfg_common::config::VisibleCore;

use super::PipelineError;

/// Resolves the firmware ELF path for every debug core.
///
/// `firmware_outputs` is Stage 1's compiled-ELF-per-core map; `dbg_cores` is the
/// set of cores that declare a `[dbg.<core>]` ring. Fails with
/// [`PipelineError::DbgCoreNotBuilt`] when a debug core was not built — in
/// practice when `[dbg.<core>]` names a `Manager`-variant coprocessor, which
/// Stage 1 never compiles.
///
/// Each path is made absolute (lexically, without requiring the ELF to exist —
/// Stage 1 compiles it later) so the AP codegen can embed it verbatim and have
/// `include_bytes!` resolve the same image regardless of where the generated
/// file lands.
pub fn resolve_dbg_elf_paths(
    firmware_outputs: &HashMap<VisibleCore, PathBuf>,
    dbg_cores: impl Iterator<Item = VisibleCore>,
) -> Result<HashMap<VisibleCore, PathBuf>, PipelineError> {
    let mut resolved = HashMap::new();
    for core in dbg_cores {
        let path = firmware_outputs
            .get(&core)
            .ok_or_else(|| PipelineError::DbgCoreNotBuilt(core.to_string()))?;
        let absolute = std::path::absolute(path).unwrap_or_else(|_| path.clone());
        resolved.insert(core, absolute);
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cm33() -> VisibleCore {
        VisibleCore::Mcore("cm33".parse().unwrap())
    }

    fn cm0p() -> VisibleCore {
        VisibleCore::Mcore("cm0p".parse().unwrap())
    }

    #[test]
    fn resolves_paths_for_built_cores() {
        let outputs = HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]);
        let resolved =
            resolve_dbg_elf_paths(&outputs, [cm33()].into_iter()).expect("cm33 was built");
        assert_eq!(resolved[&cm33()], PathBuf::from("/build/cm33.elf"));
    }

    #[test]
    fn errors_when_dbg_core_was_not_built() {
        // `[dbg.cm0p]` names a manager core Stage 1 never compiles, so there is
        // no ELF to embed: a deliberate hard error.
        let outputs = HashMap::from([(cm33(), PathBuf::from("/build/cm33.elf"))]);
        let err = resolve_dbg_elf_paths(&outputs, [cm0p()].into_iter())
            .expect_err("cm0p has no firmware output");
        assert!(matches!(err, PipelineError::DbgCoreNotBuilt(core) if core == "cm0p"));
    }
}
