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

#![no_std]

//! Trusted Execution Environment contracts for Consortium: typed OP-TEE commands
//! whose parameter marshalling is generated rather than hand-written.
//!
//! A TEE call crosses a privilege boundary between a Client Application (CA) in
//! Linux userspace and a Trusted Application (TA) in the secure world. The GlobalPlatform
//! ABI for that crossing is four untyped parameter slots, each either a pair of `u32`s
//! (a *Value*) or a shared buffer (a *Memref*). Both sides must agree, by convention
//! only, on what each slot holds — and a disagreement is a security-relevant bug in the
//! most sensitive code in the system.
//!
//! This crate replaces that convention with a Rust signature. One function definition
//! generates both ends of the call.
//!
//! # Architecture
//!
//! ```text
//!                      one #[tee_command] fn
//!                              │
//!            ┌─────────────────┴─────────────────┐
//!    CA side (`ca`)                        TA side (`ta`)
//!    call_<name>()                         <name>_dispatched()
//!      pack args → slots                     slots → unpack args
//!      invoke_command                        call the real fn
//!      read output slots back                flush out/inout slots
//! ```
//!
//! | Item                | Role                                                        |
//! | ------------------- | ----------------------------------------------------------- |
//! | [`TeeParamSlot`]    | The wire shape of one slot: `Primitive { a, b }` or `Serialized(bytes)`. |
//! | [`TeeParam<C>`]     | "This Rust type can ride in a slot", parameterized by codec.  |
//! | [`TeeParam`] derive | Generates the serialized impl for a user struct.              |
//! | [`tee_command`]     | Generates the CA caller and TA dispatcher for one function.    |
//! | [`tee_service`]     | Generates the `Command` enum and the TA's `invoke_command` dispatch. |
//!
//! [`TeeParam<C>`]: param::TeeParam
//!
//! # Two slot kinds, chosen by the type
//!
//! | Rust value                     | Slot                          | Cost               |
//! | ------------------------------ | ----------------------------- | ------------------ |
//! | `u8`…`i64`, `bool`, `(u32, u32)` | [`Primitive`] — a TEE *Value*  | no serialization    |
//! | a `#[derive(TeeParam)]` struct  | [`Serialized`] — a TEE *Memref* | one codec round-trip |
//!
//! [`Primitive`]: param::TeeParamSlot::Primitive
//! [`Serialized`]: param::TeeParamSlot::Serialized
//!
//! Primitives are not forced through a buffer: a `u16` becomes
//! `Primitive { a: v as u32, b: 0 }`, so a command taking two integers allocates
//! nothing and copies nothing across the boundary.
//!
//! # The codec is not baked into the derive
//!
//! [`TeeParam`] is generic over its codec `C`, and `#[derive(TeeParam)]` emits
//! `impl<C: CodecFor<Self>> TeeParam<C>` rather than committing to one family. The choice
//! is made once, at the call site:
//!
//! ```rust,ignore
//! use consortium_tee::{TeeParam, tee_command};
//!
//! #[derive(TeeParam, serde::Serialize, serde::Deserialize)]
//! struct MotorCommand { motor_id: u8, target_rpm: i32 }
//!
//! #[tee_command(codec = PostcardCodec)]
//! fn set_motor(cmd: MotorCommand) -> Result<(), TeeError> { /* … */ }
//! ```
//!
//! That keeps a message type reusable across commands with different codecs, and lets a
//! type be shared with the IPC layer without inheriting its codec choice. Primitive
//! types implement `TeeParam<C>` for *any* `C`, so a primitive-only command needs no
//! `codec` argument at all — it defaults to the [`NoCodec`] marker, which
//! implements no `CodecFor`, so forgetting `codec` on a command that needs one is a
//! compile error naming the missing impl.
//!
//! # Buffer sizes are declared, not discovered
//!
//! A Memref buffer has to be allocated by the CA *before* the call, so the serialized
//! size must be known up front. [`TeeParam::MAX_SIZE`]
//! carries it; `#[derive(TeeParam)]` defaults to 1024 bytes and `#[tee(max_size = N)]`
//! overrides. Set it deliberately — too small fails the call, too large wastes shared
//! memory on every invocation.
//!
//! [`NoCodec`]: param::NoCodec
//! [`TeeParam::MAX_SIZE`]: param::TeeParam::MAX_SIZE
//!
//! # Field rules
//!
//! `#[derive(TeeParam)]` rejects the same constructs as `IpcSafe`, for the same reason:
//! raw pointers, references, function pointers, and `usize`/`isize`. The last is not
//! hypothetical here — a 32-bit TA and a 64-bit CA genuinely disagree on the width.
//!
//! # Features
//!
//! `no_std` with `alloc`. Default is `ca`.
//!
//! | Feature | Effect                                                                 |
//! | ------- | ---------------------------------------------------------------------- |
//! | `ca`    | Client-Application side: `optee-teec`, Tokio, and the `error` types.   |
//! | `ta`    | Trusted-Application side dispatchers. Host-agnostic, so host-side macro and parameter tests use it. |
//! | `defmt` | Route diagnostics to `defmt` on TA builds.                              |
//!
//! CA error exports are additionally gated to Linux non-test builds, because
//! `optee-teec` needs a real `/dev/tee*`. `defmt` is deliberately *not* implied by `ta`:
//! it requires a linked global logger an OP-TEE TA may not have.
//!
//! # Testing
//!
//! ```bash
//! cargo test -p consortium-tee --no-default-features --features ta
//! just test tee host
//! ```
//!
//! When changing the macros, update the `trybuild` cases and `insta` snapshots in
//! `consortium-tee-macros-impl` before relying on the proc-macro facade.

extern crate alloc;

pub mod param;
pub use param::{NoCodec, TeeParam, TeeParamError, TeeParamRepr, TeeParamReprRef, TeeParamSlot};

pub use consortium_tee_macros::{TeeParam, tee_command, tee_service};

#[cfg(all(target_os = "linux", not(test), feature = "ca"))]
pub mod error;
#[cfg(all(target_os = "linux", not(test), feature = "ca"))]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};
