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

//! Compile-time debug (CDBG) shared-memory layout.
//!
//! The debug ring uses the **same** shared-memory format as the IPC region; the
//! builder injects the `[dbg]` section of `Consortium.toml` as `CONSORTIUM_DBG_*`
//! environment variables, which this module reads with `option_env!` and parses
//! in a `const` context. The builder emits numeric values as decimal strings;
//! this replaces the previous `extern "C"` linker symbols
//! (`CONSORTIUM_DBG_SRAM_BASE`/`CONSORTIUM_DBG_SRAM_SIZE`) and the matching
//! `#[no_mangle]` statics the firmware had to define.
//!
//! When the variables are absent (a plain `cargo build`), [`CONFIGURED`] is
//! `false`, [`BASE`] is `0`, and the `logger::ConsortiumLogger` becomes
//! a no-op. That is what lets firmware crates be `cargo check`ed without running
//! the build pipeline.
//!
//! The trade-off is that a real firmware image built without the pipeline logs
//! nothing, silently. Firmware that must not ship that way should assert
//! [`CONFIGURED`] at compile time:
//!
//! ```rust,ignore
//! const _: () = assert!(
//!     consortium_dbg::config::CONFIGURED,
//!     "debug logging requires `csti build` to inject CONSORTIUM_DBG_*",
//! );
//! ```

const fn env_or_zero(value: Option<&str>) -> &str {
    match value {
        Some(s) => s,
        None => "0",
    }
}

/// `true` when the builder injected the `[dbg]` shared-memory layout.
pub const CONFIGURED: bool = option_env!("CONSORTIUM_DBG_BASE").is_some();

/// Physical base address of the debug region, as seen by the MCU (raw memory).
pub const BASE: u64 = const_str::parse!(env_or_zero(option_env!("CONSORTIUM_DBG_BASE")), u64);

/// Length of the debug region in bytes.
pub const LENGTH: u64 = const_str::parse!(env_or_zero(option_env!("CONSORTIUM_DBG_LENGTH")), u64);

/// Debug region base as a raw mutable pointer for the MCU side.
pub const BASE_PTR: *mut u8 = BASE as *mut u8;

/// Debug region length truncated to the descriptor's `u16` size field.
pub const SIZE: u16 = LENGTH as u16;
