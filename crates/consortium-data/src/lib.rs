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

//! Static chip data for the Consortium project, embedded at compile time.
//!
//! This crate owns the chip-family databases (`database/memory/*.yaml`), the
//! generated per-core chip dumps (`database/chip/*.json`), the extracted
//! peripheral register definitions (`database/peri/`), and the vendor
//! device-tree sources (`database/dts/`).
//! It also owns the resolvers that turn a `[profile] chip` part number into a
//! parsed [`database::ChipDatabase`] or a per-core [`chipmap::ChipMap`].

pub mod chipmap;
pub mod database;
pub mod dts;
mod error;

pub use error::DataError;
