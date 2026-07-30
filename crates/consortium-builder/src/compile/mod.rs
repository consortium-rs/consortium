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

//! One compiler per artifact kind, behind a single trait.
//!
//! A Consortium build produces four very differently-built things:
//!
//! | Module          | Artifact                  | Built with                     |
//! | --------------- | ------------------------- | ------------------------------ |
//! | `firmware`    | coprocessor ELF            | `cargo` for a bare-metal target |
//! | `application` | Linux executable           | `cargo`, usually cross-compiled  |
//! | `optee`       | signed OP-TEE TA           | the OP-TEE TA dev kit            |
//! | `web`         | HMI frontend bundle        | `vp` / `npm` / `bun`             |
//!
//! [`Compiler`] is what makes them uniform: each implementor owns its `Configuration`
//! (all build parameters), a typed `Error`, and an `Output`. Its methods take the config
//! rather than `&self`, so every implementor has the same `.build(cfg)` /
//! `.output_path(&cfg)` shape and the pipeline needs no per-kind special casing.
//!
//! `env` holds the carriers for compile-time environment injection; `deploy` renders
//! the deployment artifacts (currently a systemd unit).

pub mod application;
pub mod deploy;
pub mod env;
pub mod firmware;
pub mod optee;
pub(crate) mod utils;
pub mod web;

use std::path::PathBuf;

/// Unified async compilation trait for firmware, application, and OP-TEE targets.
///
/// Each implementor owns its `Configuration` type (carries all build parameters),
/// a typed `Error`, and an `Output` (usually `PathBuf` to the built artifact).
///
/// The trait methods are free functions (take `config` by value or reference) so
/// callers can use any implementor with the same `.build(cfg)` / `.output_path(&cfg)` UX.
pub trait Compiler: Sized {
    /// All build parameters for this compiler.
    type Configuration;

    /// Typed error returned from build operations.
    type Error: std::error::Error + Send + Sync + 'static;

    /// The primary artifact produced (usually `PathBuf`).
    type Output;

    /// Run the full build and return the path to the primary artifact.
    fn build(
        config: Self::Configuration,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + Send;

    /// Return the expected output path without building.
    fn output_path(
        config: &Self::Configuration,
    ) -> impl std::future::Future<Output = Result<PathBuf, Self::Error>> + Send;
}
