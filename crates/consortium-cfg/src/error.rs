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

    /// Chip-family database and per-core chip-map resolution errors, raised by
    /// the `consortium-data` loaders.
    #[error(transparent)]
    Data(#[from] consortium_data::DataError),

    /// A vendor device-tree source failed to parse while building the merged
    /// Linux device tree.
    #[error("failed to parse device-tree source '{file}': {detail}")]
    DtsParse {
        /// The offending file name.
        file: String,
        /// The parser's diagnostic.
        detail: String,
    },

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

    #[error(
        "peripheral '{name}' is not accessible from core '{core}' on chip '{chip}'. \
         Only peripherals present in that core's peripheral map may be assigned to it"
    )]
    PeripheralNotOnCore {
        name: String,
        core: String,
        chip: String,
    },

    #[error("IPC channel endpoint '{0}' names core '{1}', which is not visible on chip '{2}'")]
    IpcEndpointNotVisible(String, String, String),

    #[error(
        "doorbell '{doorbell}' does not configure party '{party}' on chip '{chip}'. \
         Configured parties: {available}. Check the doorbell party table in the TRM ({trm}) p. {page}"
    )]
    DoorbellPartyUnavailable {
        doorbell: String,
        party: String,
        chip: String,
        available: String,
        trm: String,
        page: u32,
    },

    #[error(
        "doorbell '{doorbell}' channel {channel} is unavailable on chip '{chip}'. \
         Valid channels are 0..{channels}. Check the channel table in the TRM ({trm}) p. {page}"
    )]
    DoorbellChannelUnavailable {
        doorbell: String,
        channel: u32,
        channels: u32,
        chip: String,
        trm: String,
        page: u32,
    },

    #[error(
        "doorbell '{doorbell}' channel {channel} is assigned more than once: \
         '{first_party}' and '{second_party}' both use it"
    )]
    DoorbellChannelDuplicate {
        doorbell: String,
        channel: u32,
        first_party: String,
        second_party: String,
    },

    #[error(
        "[dbg.ap] is not allowed: the application processor is always the debug host, \
         not a logger. Declare a coprocessor logger such as [dbg.cm33] instead"
    )]
    DbgHostNotLoggable,

    #[error("[dbg.{core}] names core '{core}', which is not a coprocessor on chip '{chip}'")]
    DbgCoreNotVisible { core: String, chip: String },

    #[error("no memory region set")]
    MemoryRegionNotSet,

    #[error("UIO device '{device}' exposes {found} mappings, but Linux UIO supports at most {max}")]
    UioMmapLimit {
        device: String,
        found: usize,
        max: usize,
    },

    #[error(
        "[memory.{region}] spans {base:#x}..{end:#x}, which is not contained in any SRAM region of chip '{chip}'. \
         Check the SRAM memory map in the TRM ({trm}) p. {pages}"
    )]
    RegionOutOfBounds {
        region: String,
        base: u32,
        end: u32,
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
        a_base: u32,
        a_end: u32,
        b: String,
        b_base: u32,
        b_end: u32,
    },

    #[error("[memory.{region}] has zero length")]
    RegionEmpty { region: String },

    #[error(
        "[memory.{region}] ({base:#x}, len {length:#x}) is not a valid MPU region: {requirement}. \
         ARMv7-M/ARMv8-M MPU regions must have a power-of-two size of at least 32 bytes, with the \
         base aligned to the size"
    )]
    RegionNotMpuAligned {
        region: String,
        base: u32,
        length: u32,
        /// The specific constraint that failed.
        requirement: &'static str,
    },
}
