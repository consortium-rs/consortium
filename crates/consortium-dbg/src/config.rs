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

//! The debug (CDBG) ring this core logs into.
//!
//! The ring uses the **same** shared-memory format as the IPC region. Its base and
//! length used to be compile-time constants parsed from `CONSORTIUM_DBG_*`
//! environment variables; now generated `init()` passes them to
//! `ConsortiumLogger::init`, which records them here once, so generated code is
//! the single place the layout is written down.
//!
//! Until `init` runs, [`region`] is `None` and the logger and panic handler are
//! no-ops, which is also what a firmware crate built without generated code gets.
//!
//! Plain atomic loads and stores only: ARMv6-M (the STM32MP25 CM0+) has no atomic
//! read-modify-write, and none is needed for a value written once before logging
//! starts.

use core::sync::atomic::{AtomicU16, AtomicUsize, Ordering};

/// Ring base address, or `0` while unset.
static BASE: AtomicUsize = AtomicUsize::new(0);
/// Ring length in bytes; only meaningful once [`BASE`] is set.
static SIZE: AtomicU16 = AtomicU16::new(0);

/// Record the ring. Called once, by `ConsortiumLogger::init`, after the ring has
/// been reset and before the first `defmt` frame.
pub(crate) fn set(base: *mut u8, size: u16) {
    SIZE.store(size, Ordering::Relaxed);
    // Publishes `SIZE` to every reader that observes the non-zero base.
    BASE.store(base as usize, Ordering::Release);
}

/// The ring's `(base, size)`, or `None` before `ConsortiumLogger::init`.
pub(crate) fn region() -> Option<(*mut u8, u16)> {
    match BASE.load(Ordering::Acquire) {
        0 => None,
        base => Some((base as *mut u8, SIZE.load(Ordering::Relaxed))),
    }
}
