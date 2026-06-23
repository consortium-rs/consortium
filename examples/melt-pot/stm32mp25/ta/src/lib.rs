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

// TA builds are no_std (OP-TEE provides alloc via optee-utee).
// CA builds (feature "ca") are std — used from the Linux app.
#![cfg_attr(feature = "ta", no_std)]
// The whole crate is Linux-only: the workspace checks it on the host,
// and the OP-TEE build toolchain also targets a Linux sysroot.
#![cfg(target_os = "linux")]

#[cfg(feature = "ta")]
extern crate alloc;

use consortium_tee::{tee_command, tee_service};

/// Safe temperature range: -20 °C … 85 °C expressed in millidegrees.
const TEMP_LIMIT_LO: i32 = -20_000;
const TEMP_LIMIT_HI: i32 = 85_000;

/// Return `true` when `temperature_mdc` is within the safe operating range.
///
/// `#[tee_command]` generates two companions:
///
/// - `check_temperature_dispatched` (TA side) — unpacks `optee_utee::Parameters`,
///   calls this function, and writes the `bool` result back into the output slot.
///
/// - `call_check_temperature` (CA side, `#[cfg(feature = "ca")]`) — packs
///   `temperature_mdc` via `TeeParam`, calls `session.invoke_command`, and reads
///   the returned `bool` back from the output slot.
#[tee_command]
pub fn check_temperature(temperature_mdc: i32) -> bool {
    (TEMP_LIMIT_LO..=TEMP_LIMIT_HI).contains(&temperature_mdc)
}

tee_service! {
    commands { check_temperature }
}
