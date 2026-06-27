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

//! `[memory]` -> UIO device.
//!
//! The IPC and debug shared-memory regions share a single UIO device; each
//! becomes one of its `mapN` windows, ordered by the region's `mmap` index.

use consortium_cfg_common::config::{Memory, SharedMemory};

use super::ToUio;
use crate::components::uio::UioDevice;
use crate::error::CfgError;

impl ToUio for Memory {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        // Collect the present regions and order them by their declared `mmap`
        // index so window N lands at the device's `mapN`.
        let mut regions: Vec<&SharedMemory> = [self.ipc.as_ref(), self.dbg.as_ref()]
            .into_iter()
            .flatten()
            .collect();
        regions.sort_by_key(|region| region.mmap);

        // Nothing to expose if neither region is configured.
        if regions.is_empty() {
            return Ok(None);
        }

        // The kernel exposes mappings as `map0..mapN`, so the declared indices
        // must be exactly `0, 1, .., N-1` with no gaps or duplicates.
        if regions
            .iter()
            .enumerate()
            .any(|(expected, region)| region.mmap as usize != expected)
        {
            return Err(CfgError::MmapOrder {
                found: regions.iter().map(|region| region.mmap).collect(),
            });
        }

        // Bases and lengths are validated to lie within SRAM (a `u32` window)
        // during session validation, so the narrowing cast is lossless here.
        let mmap = regions
            .iter()
            .map(|region| (region.base as u32, region.length as u32))
            .collect();

        // Shared memory carries no interrupt of its own; doorbells signal it.
        Ok(Some(UioDevice { mmap, irq: None }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(mmap: u32, base: u64, length: u64) -> SharedMemory {
        SharedMemory {
            mmap,
            base,
            length,
            mtu: 64,
        }
    }

    #[test]
    fn maps_ipc_and_dbg_ordered_by_mmap_index() {
        let memory = Memory {
            uio: 0,
            // Declared out of order to prove the sort by `mmap`.
            dbg: Some(region(1, 0x0040_8000, 0x2000)),
            ipc: Some(region(0, 0x0040_0000, 0x8000)),
        };

        let uio = memory.to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000), (0x0040_8000, 0x2000)]);
        assert!(uio.irq.is_none());
    }

    #[test]
    fn skips_absent_regions() {
        let memory = Memory {
            uio: 0,
            ipc: Some(region(0, 0x0040_0000, 0x8000)),
            dbg: None,
        };

        let uio = memory.to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000)]);
    }

    #[test]
    fn none_when_no_regions() {
        let memory = Memory {
            uio: 0,
            ipc: None,
            dbg: None,
        };

        assert!(memory.to_uio().unwrap().is_none());
    }

    #[test]
    fn errors_on_non_contiguous_mmap_indices() {
        // ipc at map0, dbg at map2 leaves a hole at map1.
        let memory = Memory {
            uio: 0,
            ipc: Some(region(0, 0x0040_0000, 0x8000)),
            dbg: Some(region(2, 0x0040_8000, 0x2000)),
        };

        assert_eq!(
            memory.to_uio().err(),
            Some(CfgError::MmapOrder { found: vec![0, 2] })
        );
    }
}
