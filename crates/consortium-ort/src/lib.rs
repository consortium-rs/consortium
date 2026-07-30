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

//! ONNX Runtime integration for Consortium.
//!
//! # Status: reserved, not implemented
//!
//! This crate is a **placeholder**. It currently exports nothing; the `ort` dependency is
//! declared so the intended configuration is pinned and the build is proven, but no
//! wrapper exists yet.
//!
//! The intent is to let a Consortium application run inference on the A-core alongside
//! its real-time IPC work — a coprocessor samples sensors and forwards frames over a
//! channel, the application runs a model on them. `consortium-runtime-app` already
//! reserves an `ort` feature that would re-export this.
//!
//! `ort` is pulled in with `default-features = false` plus `api-24`, `ndarray`, and
//! `std`. That is deliberate for a cross-compiled embedded Linux target: the default
//! features would fetch or link an ONNX Runtime binary chosen for the *host*, whereas an
//! embedded target needs the one in its own sysroot.
//!
//! The `tracing` feature forwards to `ort/tracing`, so ORT diagnostics land in the same
//! subscriber `consortium-runtime-app` installs.
