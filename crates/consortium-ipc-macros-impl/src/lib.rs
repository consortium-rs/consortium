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

//! Derive implementations behind `consortium-ipc-macros`.
//!
//! This crate holds the logic for `consortium-ipc`'s derives as ordinary
//! `proc_macro2::TokenStream` functions. Keeping it out of the `proc-macro = true`
//! facade means the expansions can be exercised by plain unit tests and `insta`
//! snapshots instead of only through `trybuild`.
//!
//! Reusable `syn`/AST utilities (field-type-tree walking, generic collection)
//! belong in `consortium-macros-helpers`; what stays here is the IPC-specific
//! policy — which types are wire-safe and what the generated `unsafe impl` must
//! assert.
//!
//! This is an implementation detail of `consortium-ipc`. Do not depend on it
//! directly; its API is not stable.

mod ipc_safe;

pub use ipc_safe::derive_ipc_safe;
