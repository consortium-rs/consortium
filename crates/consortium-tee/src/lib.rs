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

// `TeeParam` and the derive macros are ungated from the Linux restriction - they must be
// available on every target (TA code running in the TEE secure world is not Linux).
// Error types and the TEE runtime (optee-teec) remain Linux-only.
#![no_std]
extern crate alloc;

pub mod param;
pub use param::{NoCodec, TeeParam, TeeParamError, TeeParamRepr, TeeParamReprRef, TeeParamSlot};

pub use consortium_tee_macros::{TeeParam, tee_command, tee_service};

#[cfg(all(target_os = "linux", not(test), feature = "ca"))]
pub mod error;
#[cfg(all(target_os = "linux", not(test), feature = "ca"))]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};
