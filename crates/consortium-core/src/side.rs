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

//! Processor-side markers for dual- and triple-ported IPC peripherals.
//!
//! A doorbell peripheral exposes a separate register view per processor that
//! it connects. [`Side`] identifies which view a driver instance drives:
//!
//! - [`Primary`] - the first processor (e.g. MU side A, IPCC processor 1,
//!   HSEM core C1).
//! - [`Secondary`] - the second processor (e.g. MU side B, IPCC processor 2,
//!   HSEM core C2).
//! - [`Tertiary`] - the third processor, where the peripheral supports one
//!   (e.g. HSEM core C3).
//!
//! [`Side`] is sealed: only the zero-sized markers in this module implement it.

use crate::sealed::Sealed;

/// A processor-side marker selecting which register view a doorbell drives.
///
/// Sealed against external implementations; see [`Primary`], [`Secondary`],
/// and [`Tertiary`].
pub trait Side: Sealed + Send + Sync + 'static {
    /// Zero-based side index (`Primary` = 0, `Secondary` = 1, `Tertiary` = 2).
    ///
    /// Drivers map this onto their hardware numbering, e.g. a 1-based `PROC`
    /// register bank is `INDEX + 1`.
    const INDEX: u8;
}

/// The first processor side.
pub struct Primary;

/// The second processor side.
pub struct Secondary;

/// The third processor side, for peripherals that connect three processors.
pub struct Tertiary;

impl Side for Primary {
    const INDEX: u8 = 0;
}

impl Side for Secondary {
    const INDEX: u8 = 1;
}

impl Side for Tertiary {
    const INDEX: u8 = 2;
}
