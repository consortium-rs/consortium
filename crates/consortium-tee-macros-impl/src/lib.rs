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

//! Macro implementations behind `consortium-tee-macros`.
//!
//! The logic for `consortium-tee`'s three macros lives here as ordinary
//! `proc_macro2::TokenStream` functions, outside the `proc-macro = true` facade, so
//! expansions can be asserted with unit tests and `insta` snapshots rather than only
//! through `trybuild`. That matters more here than for the other macro families: these
//! macros generate *both sides* of a privilege boundary, and the generated code is not
//! something a reader can easily hold in their head.
//!
//! | Module          | Generates                                                        |
//! | --------------- | ---------------------------------------------------------------- |
//! | `tee_param`   | `impl<C: CodecFor<Self>> TeeParam<C>` for a user type.            |
//! | `tee_command` | `call_<name>` (CA) and `<name>_dispatched` (TA) for one function.  |
//! | `tee_service` | The `Command` enum and `invoke_command` dispatch for a TA.         |
//!
//! The load-bearing design decision, shared by all three: **the codec is a type
//! parameter, never baked into an expansion.** `tee_param` emits an impl generic over `C`
//! and `tee_command` substitutes the `codec = …` argument at the call site. A message type
//! is therefore reusable across commands with different codecs, and shareable with the IPC
//! layer without inheriting its codec.
//!
//! Reusable `syn` predicates — `is_primitive`, `is_slice_ref`, `is_tee_param_path`,
//! `collect_field_types`, and friends — live in `consortium-macros-helpers`, because
//! `tee_command`'s parameter classification and `tee_param`'s field validation need the
//! same AST questions answered. What stays here is TEE-specific policy: which Rust type
//! maps to which parameter slot, and in which direction.
//!
//! This is an implementation detail of `consortium-tee`. Do not depend on it directly; its
//! API is not stable.

mod tee_command;
mod tee_param;
mod tee_service;

pub use tee_command::tee_command;
pub use tee_param::derive_tee_param;
pub use tee_service::tee_service;
