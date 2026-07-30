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

//! Proc-macro facade for `consortium-ipc`.
//!
//! This crate exists only because a `proc-macro = true` crate can export nothing
//! but macros. It is a thin `TokenStream` shim: every entry point converts to
//! `proc_macro2` and forwards to `consortium-ipc-macros-impl`, where the real
//! logic lives and can be unit-tested and snapshot-tested without a compiler
//! driver.
//!
//! Depend on `consortium-ipc` instead of this crate directly — it re-exports
//! [`IpcSafe`] alongside the trait of the same name.

use proc_macro::TokenStream;

/// Derive macro that validates a struct/enum is safe to use as a cross-core IPC message.
#[proc_macro_derive(IpcSafe)]
pub fn derive_ipc_safe(input: TokenStream) -> TokenStream {
    consortium_ipc_macros_impl::derive_ipc_safe(input.into()).into()
}
