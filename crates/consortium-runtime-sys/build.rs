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

//! Generates `no_std` Rust bindings for the remoteproc resource-table layout in
//! `include/rsc_table.h`. Runs on the host (like every `build.rs`) even when the
//! crate itself is cross-compiled to a bare-metal target, so it only parses the
//! header with libclang and never links a native library.

use std::{env, path::PathBuf};

/// Renames bindgen's snake_case C type names (e.g. `resource_table`) to
/// PascalCase (`ResourceTable`) so firmware spells them like idiomatic Rust
/// types. The C type names are entirely lowercase; constants such as
/// `FW_RSC_ADDR_ANY` and the `fw_resource_type_RSC_*` variants contain uppercase
/// letters, so they are left untouched.
#[derive(Debug)]
struct PascalCaseTypes;

impl bindgen::callbacks::ParseCallbacks for PascalCaseTypes {
    fn item_name(&self, item_info: bindgen::callbacks::ItemInfo<'_>) -> Option<String> {
        let original = item_info.name;
        // Leave the fixed-width kernel aliases from `wrapper.h` alone (bindgen
        // already disambiguates them to `u8_`/`u32_`); only the resource-table
        // types should become PascalCase.
        if matches!(original, "u8" | "u16" | "u32" | "u64") {
            return None;
        }
        if original.contains(|c: char| c.is_ascii_uppercase()) {
            return None;
        }
        Some(
            original
                .split('_')
                .filter(|seg| !seg.is_empty())
                .map(|seg| {
                    let mut chars = seg.chars();
                    match chars.next() {
                        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                        None => String::new(),
                    }
                })
                .collect(),
        )
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=include/rsc_table.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iinclude")
        .parse_callbacks(Box::new(PascalCaseTypes))
        // Emit `::core` paths so the generated module works in `no_std` firmware.
        .use_core()
        // No libc on the firmware target; map C fixed-width types to `core` ints.
        .ctypes_prefix("::core::ffi")
        // Keep only the resource-table vocabulary; drop the incidental stdint/
        // stddef declarations the wrapper pulls in.
        .allowlist_type("resource_table")
        .allowlist_type("fw_rsc_.*")
        .allowlist_type("fw_resource_type")
        .allowlist_var("RSC_.*")
        .allowlist_var("FW_RSC_ADDR_ANY")
        .derive_default(true)
        .derive_debug(true)
        // Layout tests pull in `std`; the packed kernel structs are the contract
        // we care about, and their sizes are asserted at the Rust call sites.
        .layout_tests(false)
        .generate()
        .expect("failed to generate rsc_table bindings");

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("failed to write rsc_table bindings");
}
