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

//! Shared-memory regions -> UIO device.
//!
//! Pattern-specific shared-memory regions can share a single UIO device.

use consortium_cfg_common::config::MemoryRegion;

use crate::components::uio::{ToUio, UioBacking, UioDevice};
use crate::error::CfgError;

const UIO_MAX_MMAPS: usize = 5;

/// One channel window and its controller-side cache policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedWindow {
    /// Physical base and length from `config.memory`.
    pub memory: MemoryRegion,
    /// Whether the controller maps the window cacheably.
    pub cache: bool,
}

impl ToUio for [SharedWindow] {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        // Nothing to expose if neither region is configured.
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() > UIO_MAX_MMAPS {
            return Err(CfgError::UioMmapLimit {
                device: "shared-memory".to_string(),
                found: self.len(),
                max: UIO_MAX_MMAPS,
            });
        }

        let mmap = self
            .iter()
            .map(|window| (window.memory.base, window.memory.length))
            .collect();

        // Shared memory carries no interrupt of its own; doorbells signal it.
        // It is a RAM carveout, so it is exposed through `reserved-memory`.
        Ok(Some(UioDevice {
            mmap,
            irq: None,
            backing: UioBacking::ReservedMemory,
            dma_coherent: self.iter().map(|window| window.cache).collect(),
            interrupt_parent: None,
            clock: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(base: u32, length: u32) -> SharedWindow {
        SharedWindow {
            memory: MemoryRegion { base, length },
            cache: false,
        }
    }

    #[test]
    fn maps_regions_in_slice_order() {
        let memory = [region(0x0040_0000, 0x8000), region(0x0040_8000, 0x2000)];

        let uio = memory.as_slice().to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000), (0x0040_8000, 0x2000)]);
        assert!(uio.irq.is_none());
    }

    #[test]
    fn maps_single_region() {
        let memory = [region(0x0040_0000, 0x8000)];

        let uio = memory.as_slice().to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000)]);
    }

    #[test]
    fn none_when_no_regions() {
        let memory: [SharedWindow; 0] = [];
        assert!(memory.as_slice().to_uio().unwrap().is_none());
    }

    #[test]
    fn errors_when_more_than_five_regions() {
        let memory = [
            region(0x0040_0000, 0x1000),
            region(0x0040_1000, 0x1000),
            region(0x0040_2000, 0x1000),
            region(0x0040_3000, 0x1000),
            region(0x0040_4000, 0x1000),
            region(0x0040_5000, 0x1000),
        ];

        assert_eq!(
            memory.as_slice().to_uio().err(),
            Some(CfgError::UioMmapLimit {
                device: "shared-memory".to_string(),
                found: 6,
                max: 5,
            })
        );
    }

    #[test]
    fn dma_coherent_follows_each_window_cache_policy() {
        let mut memory = [region(0x0040_0000, 0x8000), region(0x0040_8000, 0x2000)];
        assert_eq!(
            memory.as_slice().to_uio().unwrap().unwrap().dma_coherent,
            vec![false, false]
        );

        memory[1].cache = true;
        assert_eq!(
            memory.as_slice().to_uio().unwrap().unwrap().dma_coherent,
            vec![false, true]
        );
    }
}
