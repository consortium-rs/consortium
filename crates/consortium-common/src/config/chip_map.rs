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

use crate::config::{app_core::CortexAppCore, mcu_core::CortexMcuCore};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub enum Chip {
    Stm32mp21x,
    Stm32mp23x,
    Stm32mp25x,
    Imx93,
    Imx93w,
    Imx937,
    Imx94,
    Imx95,
    Imx952,
}

impl Chip {
    /// Returns `true` if this chip has a remoteproc entry for the given core type and index.
    pub fn has_rt_core(&self, core: &CortexMcuCore, idx: usize) -> bool {
        self.remoteproc_name(core, idx).is_some()
    }

    /// Returns `true` if this chip has a remoteproc entry matching the given abbreviated core name.
    pub fn has_rt_core_name(&self, name: &str) -> bool {
        match name {
            "cm70" => matches!(self, Self::Imx94),
            "cm71" => matches!(self, Self::Imx94),
            _ => {
                let cortex = CortexMcuCore::from_abbr(name);
                cortex.and_then(|c| self.remoteproc_name(&c, 0)).is_some()
            }
        }
    }

    /// Returns the list of application-class CPU cores and their counts for this chip.
    pub fn app_cpu(&self) -> Vec<(CortexAppCore, usize)> {
        match self {
            Self::Stm32mp21x | Self::Stm32mp23x | Self::Stm32mp25x => {
                vec![(CortexAppCore::CortexA35, 2)]
            }
            Self::Imx93 | Self::Imx93w => vec![(CortexAppCore::CortexA55, 2)],
            Self::Imx937 | Self::Imx94 | Self::Imx952 => vec![(CortexAppCore::CortexA55, 4)],
            Self::Imx95 => vec![(CortexAppCore::CortexA55, 6)],
        }
    }

    /// Returns the Rust target triple for cross-compiling application code to this chip.
    ///
    /// Panics if the chip has heterogeneous app-core targets (not currently supported).
    pub fn app_compile_target(&self) -> &'static str {
        let cpus = self
            .app_cpu()
            .iter()
            .map(|(core, _)| core.to_rust_target())
            .collect::<Vec<_>>();
        if cpus.iter().all(|&t| t == cpus[0]) {
            cpus[0]
        } else {
            panic!("Heterogeneous CPU targets are not supported yet")
        }
    }

    /// Returns `(remoteproc_name, programmable)` for the given core type and instance index,
    /// or `None` if this chip has no such core.
    pub fn remoteproc_name(&self, core: &CortexMcuCore, idx: usize) -> Option<(String, bool)> {
        let (id, programmable) = match (self, core, idx) {
            (
                Self::Stm32mp21x | Self::Stm32mp23x | Self::Stm32mp25x,
                CortexMcuCore::CortexM33,
                0,
            ) => ("0".to_string(), true),
            (Self::Stm32mp25x, CortexMcuCore::CortexM0p, 0) => ("1".to_string(), false),
            (
                Self::Imx95 | Self::Imx952 | Self::Imx94 | Self::Imx937,
                CortexMcuCore::CortexM7,
                0,
            ) => ("0".to_string(), true),
            (Self::Imx94, CortexMcuCore::CortexM7, 1) => ("1".to_string(), true),
            (Self::Imx95 | Self::Imx952 | Self::Imx937, CortexMcuCore::CortexM33, 0) => {
                ("1".to_string(), false)
            }
            (Self::Imx94, CortexMcuCore::CortexM33, 0) => ("2".to_string(), false),
            (Self::Imx94, CortexMcuCore::CortexM33, 1) => ("3".to_string(), true),
            (Self::Imx93 | Self::Imx93w, CortexMcuCore::CortexM7, 0) => ("0".to_string(), true),
            _ => return None,
        };
        Some((format!("remoteproc{id}"), programmable))
    }

    /// Returns all programmable real-time cores with their remoteproc names, sorted by name.
    #[cfg(feature = "std")]
    pub fn programmable_rt_cores(&self) -> Vec<(CortexMcuCore, usize, String)> {
        let mut cores = Vec::new();
        for idx in 0..2 {
            for core in &[
                CortexMcuCore::CortexM0p,
                CortexMcuCore::CortexM33,
                CortexMcuCore::CortexM7,
            ] {
                if let Some((name, true)) = self.remoteproc_name(core, idx) {
                    cores.push((core.clone(), idx, name));
                }
            }
        }
        // Sort by remoteproc name for deterministic ordering
        cores.sort_by_key(|(_, _, name)| name.clone());
        cores
    }

    /// Returns the RIFSC family name for STM32MP chips, or `None` for i.MX chips.
    pub fn rifsc_family_name(&self) -> Option<&'static str> {
        match self {
            Self::Stm32mp21x => Some("stm32mp21xxxx"),
            Self::Stm32mp23x => Some("stm32mp23xxxx"),
            Self::Stm32mp25x => Some("stm32mp25xxxx"),
            _ => None,
        }
    }
}
