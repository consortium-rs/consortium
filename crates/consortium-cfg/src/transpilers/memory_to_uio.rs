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
//! The IPC and debug shared-memory regions share a single UIO device.

use consortium_cfg_common::config::{Memory, SharedMemory};

use super::ToUio;
use crate::components::uio::UioDevice;
use crate::error::CfgError;

impl ToUio for Memory {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        let regions: Vec<&SharedMemory> = [self.ipc.as_ref(), self.dbg.as_ref()]
            .into_iter()
            .flatten()
            .collect();

        // Nothing to expose if neither region is configured.
        if regions.is_empty() {
            return Ok(None);
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

    fn region(base: u64, length: u64) -> SharedMemory {
        SharedMemory {
            base,
            length,
            mtu: 64,
        }
    }

    #[test]
    fn maps_ipc_and_dbg_in_schema_order() {
        let memory = Memory {
            dbg: Some(region(0x0040_8000, 0x2000)),
            ipc: Some(region(0x0040_0000, 0x8000)),
        };

        let uio = memory.to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000), (0x0040_8000, 0x2000)]);
        assert!(uio.irq.is_none());
    }

    #[test]
    fn skips_absent_regions() {
        let memory = Memory {
            ipc: Some(region(0x0040_0000, 0x8000)),
            dbg: None,
        };

        let uio = memory.to_uio().unwrap().unwrap();
        assert_eq!(uio.mmap, vec![(0x0040_0000, 0x8000)]);
    }

    #[test]
    fn none_when_no_regions() {
        let memory = Memory {
            ipc: None,
            dbg: None,
        };

        assert!(memory.to_uio().unwrap().is_none());
    }
}
