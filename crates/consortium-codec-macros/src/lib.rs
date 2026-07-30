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

//! Proc-macro facade for `consortium-codec`.
//!
//! A `proc-macro = true` crate can export nothing but macros, so this crate is a
//! thin `TokenStream` shim: each entry point converts to `proc_macro2` and forwards
//! to `consortium-codec-macros-impl`, where the logic lives and can be
//! snapshot-tested without a compiler driver.
//!
//! Depend on `consortium-codec` with the `rkyv` feature instead of this crate
//! directly — it re-exports [`TrustedArchive`] next to the trait of the same name.

use proc_macro::TokenStream;

/// Derive macro that validates a struct/enum is safe for unchecked rkyv archive access.
///
/// See `consortium_codec::rkyv::TrustedArchive` for the full safety contract.
#[proc_macro_derive(TrustedArchive)]
pub fn derive_trusted_archive(input: TokenStream) -> TokenStream {
    consortium_codec_macros_impl::derive_trusted_archive(input.into()).into()
}
