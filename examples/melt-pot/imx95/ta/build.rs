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

// Replace the UUID below with the one from your OP-TEE TA UUID generator.
// The UUID must match what the CA uses when opening the session.
const TA_UUID: &str = "12345678-1234-1234-1234-123456789012";

#[cfg(all(target_os = "linux", feature = "ta"))]
fn main() -> Result<(), optee_utee_build::Error> {
    use optee_utee_build::TaConfig;
    let ta_config = TaConfig::new_default_with_cargo_env(TA_UUID)?;
    optee_utee_build::build(optee_utee_build::RustEdition::Edition2024, ta_config)
}

#[cfg(not(all(target_os = "linux", feature = "ta")))]
fn main() {}
