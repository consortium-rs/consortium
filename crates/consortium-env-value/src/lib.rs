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

//! `const`-evaluable parsing helpers for compile-time configuration values.
//!
//! The Consortium builder injects shared-memory layout parameters (IPC and
//! debug regions) as environment variables before invoking `cargo build`.
//! Consuming crates read them with `option_env!`/`env!` and parse them in a
//! `const` context using these helpers, so the values are baked into the
//! firmware/application binary with no runtime file or environment access.
//!
//! This crate is `no_std` and dependency-free so `no_std` consumers (e.g.
//! `consortium-dbg`'s logger on a bare-metal target) can use it without pulling
//! in the std-only schema crate. `consortium-cfg-common` re-exports it as its
//! `env_value` module for host-side consumers.
//!
//! Both decimal (`262049792`) and hexadecimal (`0x204C_0000`) forms are
//! accepted, and `_` digit separators are ignored, so the raw `Consortium.toml`
//! representation can be passed through verbatim.

#![no_std]

/// Parses an unsigned integer from a string in a `const` context.
///
/// Accepts decimal or `0x`/`0X`-prefixed hexadecimal, ignoring `_` separators.
/// Any unexpected byte terminates parsing and the accumulated value is returned,
/// which keeps the function total (usable in `const` without panics).
pub const fn parse_u64(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut radix = 10u64;
    if bytes.len() >= 2 && bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') {
        radix = 16;
        i = 2;
    }
    let mut acc = 0u64;
    while i < bytes.len() {
        let c = bytes[i];
        let digit = match c {
            b'0'..=b'9' => (c - b'0') as u64,
            b'a'..=b'f' => (c - b'a' + 10) as u64,
            b'A'..=b'F' => (c - b'A' + 10) as u64,
            b'_' => {
                i += 1;
                continue;
            }
            _ => break,
        };
        acc = acc * radix + digit;
        i += 1;
    }
    acc
}

/// Like [`parse_u64`] but truncates to `u32`, for UIO indices and MTU values.
pub const fn parse_u32(s: &str) -> u32 {
    parse_u64(s) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_decimal_and_hex() {
        assert_eq!(parse_u64("0"), 0);
        assert_eq!(parse_u64("4096"), 4096);
        assert_eq!(parse_u64("0x1000"), 4096);
        assert_eq!(parse_u64("0x204C0000"), 0x204C_0000);
        assert_eq!(parse_u64("0x204C_0000"), 0x204C_0000);
        assert_eq!(parse_u64("0X1000"), 0x1000);
        assert_eq!(parse_u32("64"), 64);
    }
}
