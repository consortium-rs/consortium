// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! Optional per-core Linux remoteproc memory-layout overrides.

use std::collections::{BTreeMap, HashMap};

/// `[remoteproc.<core>]` tables, keyed by the remote-core name (`cm7`, `cm33`).
pub type Remoteproc = HashMap<String, RemoteprocCore>;

/// Optional overrides for one remote core.
///
/// Every omitted field inherits the corresponding chip-database default.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteprocCore {
    #[serde(default)]
    pub vector: RemoteprocWindowOverride,
    #[serde(default)]
    pub flash: RemoteprocWindowOverride,
    #[serde(default)]
    pub shmem: RemoteprocWindowOverride,
    #[serde(default)]
    pub ram: RemoteprocWindowOverride,
    /// Optional user-defined linker memory regions, keyed by linker name.
    #[serde(default)]
    pub regions: BTreeMap<String, RemoteprocRegion>,
}

/// One optional linker region and the input sections explicitly placed in it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteprocRegion {
    /// Region origin used by the firmware linker, ELF loader, and Linux DTS.
    pub origin: u64,
    pub length: u64,
    #[serde(default = "default_true")]
    pub reserve: bool,
    #[serde(default = "default_true")]
    pub read: bool,
    #[serde(default)]
    pub write: bool,
    #[serde(default)]
    pub execute: bool,
    #[serde(default)]
    pub sections: Vec<RemoteprocSection>,
}

/// An output section injected alongside `cortex-m-rt`'s `link.x` and bound to
/// the containing optional memory region.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteprocSection {
    pub output: String,
    pub inputs: Vec<String>,
    #[serde(default)]
    pub keep: bool,
    /// Existing output section after which this section is inserted.
    pub insert_after: String,
}

const fn default_true() -> bool {
    true
}

/// A partial override of one firmware load window.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteprocWindowOverride {
    /// Region origin used by the firmware linker, ELF loader, and Linux DTS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<u64>,
    /// Window length in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    /// Whether Linux DTS must reserve this window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve: Option<bool>,
}
