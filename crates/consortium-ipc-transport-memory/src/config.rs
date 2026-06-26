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

//! Compile-time IPC shared-memory layout.
//!
//! The Consortium builder injects the `[ipc]` section of `Consortium.toml` as
//! `CONSORTIUM_IPC_*` environment variables before invoking `cargo build`. This
//! module reads them with `option_env!` and parses them in a `const` context,
//! so the single shared descriptor + shared slots are addressed identically
//! from both cores: the MCU uses [`BASE`] as a raw physical address, while the
//! Linux application maps the same region through UIO device [`UIO`].
//!
//! When the variables are absent (a plain `cargo build` rather than a build
//! driven by `consortium-builder`), [`CONFIGURED`] is `false` and the address
//! constants fall back to `0`. Binaries that require a real region should guard
//! with [`assert_configured`] (or a `const` assert on [`CONFIGURED`]) to get a
//! clear "build via consortium-builder" message instead of a silent zero base.

use consortium_cfg_common::env_value::{parse_u32, parse_u64};

const fn env_or_zero(value: Option<&str>) -> &str {
    match value {
        Some(s) => s,
        None => "0",
    }
}

/// `true` when the builder injected the `[ipc]` shared-memory layout.
pub const CONFIGURED: bool = option_env!("CONSORTIUM_IPC_BASE").is_some();

/// Physical base address of the IPC region, as seen by the MCU (raw memory).
pub const BASE: u64 = parse_u64(env_or_zero(option_env!("CONSORTIUM_IPC_BASE")));

/// Length of the IPC region in bytes.
pub const LENGTH: u64 = parse_u64(env_or_zero(option_env!("CONSORTIUM_IPC_LENGTH")));

/// UIO device index the Linux application mmaps to reach the IPC region.
pub const UIO: u32 = parse_u32(env_or_zero(option_env!("CONSORTIUM_IPC_UIO")));

/// Maximum transfer unit (payload bytes per slot) for the IPC channel.
pub const MTU: u32 = parse_u32(env_or_zero(option_env!("CONSORTIUM_IPC_MTU")));

/// `const` guard that fails the build with a helpful message when the IPC
/// shared-memory layout was not injected by `consortium-builder`.
///
/// Invoke as `const _: () = consortium_ipc_transport_memory::config::assert_configured();`
/// in a binary that requires a real region.
pub const fn assert_configured() {
    // Intentionally asserts on a `const`: this guard only evaluates when the `const fn`
    // is called from a binary that requires a real region, not at every build.
    #[allow(clippy::assertions_on_constants)]
    {
        assert!(
            CONFIGURED,
            "IPC shared-memory layout is not configured: build through `consortium-builder` \
             (which injects CONSORTIUM_IPC_* from the [ipc] section of Consortium.toml) \
             rather than a plain `cargo build`."
        );
    }
}
