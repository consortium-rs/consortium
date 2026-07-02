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
//! in a `const` context. This replaces the previous `extern "C"` linker symbols
//! (`CONSORTIUM_DBG_SRAM_BASE`/`CONSORTIUM_DBG_SRAM_SIZE`) and the matching
//! `#[no_mangle]` statics the firmware had to define.
//!
//! When the variables are absent (a plain `cargo build`), [`CONFIGURED`] is
//! `false`, [`BASE`] is `0`, and the [`crate::logger::ConsortiumLogger`] becomes
//! a no-op. Firmware that needs real logging should guard with
//! [`assert_configured`].

use consortium_env_value::parse_u64;

const fn env_or_zero(value: Option<&str>) -> &str {
    match value {
        Some(s) => s,
        None => "0",
    }
}

/// `true` when the builder injected the `[dbg]` shared-memory layout.
pub const CONFIGURED: bool = option_env!("CONSORTIUM_DBG_BASE").is_some();

/// Physical base address of the debug region, as seen by the MCU (raw memory).
pub const BASE: u64 = parse_u64(env_or_zero(option_env!("CONSORTIUM_DBG_BASE")));

/// Length of the debug region in bytes.
pub const LENGTH: u64 = parse_u64(env_or_zero(option_env!("CONSORTIUM_DBG_LENGTH")));

/// Debug region base as a raw mutable pointer for the MCU side.
pub const BASE_PTR: *mut u8 = BASE as *mut u8;

/// Debug region length truncated to the descriptor's `u16` size field.
pub const SIZE: u16 = LENGTH as u16;
