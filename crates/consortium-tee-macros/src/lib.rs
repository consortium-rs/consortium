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

//! Proc-macro facade for `consortium-tee`.
//!
//! A `proc-macro = true` crate can export nothing but macros, so this crate is a thin
//! `TokenStream` shim: each entry point converts to `proc_macro2` and forwards to
//! `consortium-tee-macros-impl`, where the logic lives and can be snapshot-tested without
//! a compiler driver.
//!
//! Depend on `consortium-tee` instead of this crate directly — it re-exports all three
//! macros alongside the traits they generate impls for.
//!
//! The three together turn one Rust function into both halves of a TEE call:
//! [`TeeParam`] makes a type carryable in a parameter slot, [`tee_command`] generates the
//! CA caller and TA dispatcher for a function, and [`tee_service`] generates the command
//! enum and dispatch table that ties a TA's commands together.

use proc_macro::TokenStream;

/// Generates a TA's `Command` enum and its `invoke_command` dispatch table.
///
/// ```rust,ignore
/// tee_service! {
///     context ContextStruct        // optional
///     commands { function_a, function_b }
/// }
/// ```
///
/// Produces a `Command` enum with PascalCase variants numbered from 0 plus a
/// `#[default] Unknown` catch-all, and an `invoke_command` that dispatches each variant to
/// the `<function>_dispatched` wrapper [`tee_command`] generated. `Unknown` is reserved,
/// so a command may not be named that.
#[proc_macro]
pub fn tee_service(input: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::tee_service(input.into()).into()
}

/// Generates the CA caller and TA dispatcher for one command handler.
///
/// From a single annotated function this emits `call_<name>` on the CA side (packs
/// arguments into parameter slots, invokes the command, reads output slots back) and
/// `<name>_dispatched` on the TA side (unpacks slots, calls the function, flushes `out`
/// and `inout` slots). The original function body is emitted unchanged.
///
/// ```rust,ignore
/// #[tee_command(codec = PostcardCodec, ctx)]
/// fn my_handler(ctx: &mut Context, config: MyConfig) -> Result<(), TeeError> { /* … */ }
/// ```
///
/// `codec` names the [`consortium_codec::CodecFor`] family used for serialized
/// parameters; omit it for primitive-only commands. `ctx` marks the first parameter as TA
/// context, which is skipped when assigning slots.
///
/// [`consortium_codec::CodecFor`]: https://docs.rs/consortium-codec
#[proc_macro_attribute]
pub fn tee_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::tee_command(attr.into(), item.into()).into()
}

/// Derives `consortium_tee::TeeParam` so a type can ride in a Memref parameter slot.
///
/// Emits `impl<C: CodecFor<Self>> TeeParam<C>` — the codec is deliberately *not* fixed
/// here, but chosen at each `#[tee_command(codec = …)]` call site, so one type is reusable
/// across commands and shareable with the IPC layer.
///
/// Rejects the same field constructs as `IpcSafe` (raw pointers, references, function
/// pointers, `usize`/`isize`); the last matters because a 32-bit TA and a 64-bit CA
/// disagree on the width. `#[tee(max_size = N)]` overrides the default 1024-byte Memref
/// bound.
#[proc_macro_derive(TeeParam, attributes(tee))]
pub fn derive_tee_param(item: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::derive_tee_param(item.into()).into()
}
