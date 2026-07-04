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
    use optee_utee_build::TaConfig;
    let ta_config = TaConfig::new_default_with_cargo_env("197c710c-bdab-11eb-8f3f-17a5f698d23b")?;
    optee_utee_build::build(ta_config)
}

#[cfg(not(all(target_os = "linux", feature = "ta")))]
fn main() {
    // No OP-TEE header generation is needed for host checks or non-Linux targets.
}
