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

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CfgError {
    #[error("failed to parse Consortium manifest: {0}")]
    Manifest(String),

    #[error("no chip-family database matches chip '{0}'")]
    UnknownChip(String),

    #[error("failed to parse chip database for '{0}': {1}")]
    DatabaseParse(String, String),

    #[error(
        "doorbell '{name}' is not available on chip '{chip}'. Available doorbells: {available}. \
         Refer to the TRM ({trm}). \
         Sorry — customized hardware doorbells are not supported yet."
    )]
    UnknownDoorbell {
        name: String,
        chip: String,
        /// Reference manual URL for the chip.
        trm: String,
        /// Pre-rendered list of supported doorbells with their TRM pages.
        available: String,
    },

    #[error("peripheral '{0}' is owned by '{1}', which is not a core on chip '{2}'")]
    OwnerNotVisible(String, String, String),

    #[error("IPC channel endpoint '{0}' names core '{1}', which is not visible on chip '{2}'")]
    IpcEndpointNotVisible(String, String, String),

    #[error("no memory region set")]
    MemoryRegionNotSet,

    #[error(
        "[memory.{region}] spans {base:#x}..{end:#x}, which is not contained in any SRAM region of chip '{chip}'. \
         Check the SRAM memory map in the TRM ({trm}) p. {pages}"
    )]
    RegionOutOfBounds {
        region: String,
        base: u64,
        end: u64,
        chip: String,
        /// Reference manual URL for the chip.
        trm: String,
        /// Pre-rendered list of TRM pages documenting the memory map.
        pages: String,
    },

    #[error(
        "[memory.{a}] ({a_base:#x}..{a_end:#x}) overlaps [memory.{b}] ({b_base:#x}..{b_end:#x})"
    )]
    RegionOverlap {
        a: String,
        a_base: u64,
        a_end: u64,
        b: String,
        b_base: u64,
        b_end: u64,
    },

    #[error("[memory.{region}] has zero length")]
    RegionEmpty { region: String },
}
