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

//! Error contract for the native PocketJS backend.

use std::path::PathBuf;

use thiserror::Error;

/// Result type used across the native PocketJS backend.
pub type Result<T> = std::result::Result<T, PocketError>;

/// Boxed upstream error captured from PocketJS, wgpu, or QuickJS.
type Source = Box<dyn std::error::Error + Send + Sync>;

/// Errors raised while hosting or driving a PocketJS application.
#[derive(Debug, Error)]
pub enum PocketError {
    /// A logical viewport dimension was zero.
    #[error("PocketJS logical viewport dimensions must be non-zero")]
    ZeroViewport,
    /// The host target or ABI did not identify a real contract.
    #[error("PocketJS host target and ABI must identify a real contract")]
    InvalidHostContract,
    /// The evaluated bundle never installed `globalThis.frame`.
    #[error("PocketJS bundle evaluated but did not install globalThis.frame")]
    MissingFrame,
    /// Reading the PocketJS bundle from disk failed.
    #[error("reading PocketJS bundle {path}")]
    ReadBundle {
        /// Bundle path that could not be read.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// Reading the PocketJS pak from disk failed.
    #[error("reading PocketJS pak {path}")]
    ReadPak {
        /// Pak path that could not be read.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// An upstream PocketJS, QuickJS, or wgpu call failed.
    #[error("{context}")]
    Host {
        /// Human-readable description of the failed operation.
        context: &'static str,
        /// Underlying error reported by the host stack.
        #[source]
        source: Source,
    },
}

impl PocketError {
    /// Wraps an upstream host error with a static description of the operation.
    pub(crate) fn host(context: &'static str, source: impl Into<Source>) -> Self {
        Self::Host {
            context,
            source: source.into(),
        }
    }
}
