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
    pub fn has_rt_core(&self, core: &CortexMcuCore, idx: usize) -> bool {
        self.remoteproc_name(core, idx).is_some()
    }

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

    pub fn app_compile_target(&self) -> &'static str {
        let cpus = self
            .app_cpu()
            .iter()
            .map(|(core, _)| core.to_rust_target())
            .collect::<Vec<_>>();
        // If all CPUs have the same target, return that target. Otherwise, return a panic message since we don't support heterogeneous CPU targets yet.
        if cpus.iter().all(|&t| t == cpus[0]) {
            cpus[0]
        } else {
            panic!("Heterogeneous CPU targets are not supported yet")
        }
    }

    // Returns (remoteproc name, programmable by remoteproc) for the given core and index
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
}
