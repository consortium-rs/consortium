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

//! Implementation of the Consortium runtime entry macros.
//!
//! These operate on [`proc_macro2::TokenStream`] so the thin `proc-macro = true`
//! facade in `consortium-runtime-macros` stays trivial and the logic here is
//! host-testable with `insta` snapshots.
//!
//! `mcu_main` and `app_main` preserve the user's body and return type in an
//! inner async function, stack the appropriate executor on a unit-returning
//! outer entry point, and dispatch `()` or `Result<(), E: Debug>` through the
//! runtime's `MainOutcome` trait. Resource initialization remains explicit in
//! user code.

mod entry;

pub use entry::{app_main, mcu_main};
