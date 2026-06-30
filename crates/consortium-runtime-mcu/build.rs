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

//! Exposes `armv7m`/`armv8m` cfgs to gate the per-architecture MPU helpers.
//!
//! The `cortex-m` crate selects its `mpu::RegisterBlock` layout on its own
//! internal `armv8m` cfg, which is not visible to downstream crates. We mirror
//! the relevant part of that detection here so [`crate::mpu`] compiles the
//! `RBAR`/`RASR` code on ARMv7-M cores (e.g. `thumbv7em`) and the
//! `RBAR`/`RLAR`/`MAIR` code on ARMv8-M cores (e.g. `thumbv8m.main`).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(armv7m)");
    println!("cargo::rustc-check-cfg=cfg(armv8m)");

    let target = std::env::var("TARGET").unwrap_or_default();
    // ARMv7-M family: Cortex-M3 (thumbv7m) and Cortex-M4/M7 (thumbv7em). These
    // share the `reg`/`rasr` MPU register block.
    if target.starts_with("thumbv7em") || target.starts_with("thumbv7m") {
        println!("cargo::rustc-cfg=armv7m");
    }
    // ARMv8-M family: Cortex-M23 (thumbv8m.base) and Cortex-M33/M55
    // (thumbv8m.main). These use the base/limit + MAIR register block.
    if target.starts_with("thumbv8m") {
        println!("cargo::rustc-cfg=armv8m");
    }
}
