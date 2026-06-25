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

//! Sealing primitive for marker traits.
//!
//! [`Sealed`] is only implemented for the types defined inside this crate.
//! Because [`crate::side::Side`] and [`crate::channel::Direction`] require
//! `Sealed` as a supertrait, downstream crates can use those marker traits as
//! bounds but cannot implement them for their own types.

/// Private supertrait used to seal [`crate::side::Side`] and
/// [`crate::channel::Direction`] against external implementations.
pub trait Sealed {}

impl Sealed for crate::side::Primary {}
impl Sealed for crate::side::Secondary {}
impl Sealed for crate::side::Tertiary {}

impl Sealed for crate::channel::Tx {}
impl Sealed for crate::channel::Rx {}
