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

#[cfg(all(target_os = "linux", feature = "ta"))]
fn main() -> Result<(), optee_utee_build::Error> {
    use consortium_common::config::TEE_UUID_ENV;
    use optee_utee_build::TaConfig;

    println!("cargo:rerun-if-env-changed={TEE_UUID_ENV}");

    let ta_uuid = std::env::var(TEE_UUID_ENV).unwrap_or_else(|_| {
        panic!("missing {TEE_UUID_ENV}; build this TA through consortium-builder")
    });
    let ta_config = TaConfig::new_default_with_cargo_env(&ta_uuid)?;
    optee_utee_build::build(optee_utee_build::RustEdition::Edition2024, ta_config)
}

#[cfg(not(all(target_os = "linux", feature = "ta")))]
fn main() {}
