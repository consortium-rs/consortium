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

//! C ABI surface for embedding Consortium runtimes in foreign hosts.
//!
//! # Status: reserved, not implemented
//!
//! This crate is a **placeholder**. It currently exports nothing.
//!
//! The intent is an `extern "C"` surface so a non-Rust host — a C or C++ application, or
//! a language with a C FFI — can drive a Consortium runtime: open a link, send and
//! receive typed messages, invoke a TEE command. `consortium-runtime-app` already
//! reserves an `ffi` feature that would re-export it.
//!
//! Nothing depends on it yet, and its shape is undecided. Two constraints are worth
//! recording before it grows:
//!
//! - **Message types cannot cross a C boundary as generics.** The IPC layer's
//!   `IpcSafe`/`CodecFor` design is compile-time; an FFI surface has to expose either
//!   byte buffers or a generated per-message shim.
//! - **Async does not cross either.** Every send and receive in the runtime is a future
//!   driven by Tokio or Embassy, so a C API needs a blocking wrapper, a callback, or a
//!   poll-style handle — a decision that determines the whole surface.
