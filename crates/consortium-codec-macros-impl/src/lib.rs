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

//! Derive implementations behind `consortium-codec-macros`.
//!
//! The logic for `consortium-codec`'s derives lives here as ordinary
//! `proc_macro2::TokenStream` functions, outside the `proc-macro = true` facade, so
//! expansions can be asserted with unit tests and `insta` snapshots rather than
//! only through `trybuild`.
//!
//! Reusable `syn`/AST utilities belong in `consortium-macros-helpers`; what stays
//! here is codec-specific policy — chiefly which `repr` declarations make an rkyv
//! archive safe to access without validation.
//!
//! This is an implementation detail of `consortium-codec`. Do not depend on it
//! directly; its API is not stable.

mod trusted_archive;

pub use trusted_archive::derive_trusted_archive;
