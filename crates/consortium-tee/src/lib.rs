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

/// Marker trait for types that can be passed through a TEE memref slot via a codec.
///
/// Derive it alongside the serialization traits required by your codec:
///
/// ```rust,ignore
/// #[derive(TeeParam, Serialize, Deserialize)]
/// struct MyConfig {
///     threshold: u32,
///     mode: u8,
/// }
/// ```
///
/// When using `#[tee_command]`, any `TeeParam` parameters must be accompanied by a
/// `codec` attribute specifying which codec to use:
///
/// ```rust,ignore
/// #[tee_command(codec = PostcardCodec)]
/// fn process_config(ctx: &mut Context, config: MyConfig) -> Result<(), TeeError> {
///     // ...
/// }
/// ```
///
/// `MAX_SIZE` controls how large a buffer the CA pre-allocates for inout/output slots.
/// The default from `#[derive(TeeParam)]` is 1024 bytes; override with a manual impl.
pub trait TeeParam {
    /// Maximum serialized size in bytes.
    const MAX_SIZE: usize;
}

pub use consortium_tee_proc_macros::{TeeParam, tee_command, tee_service};

#[cfg(all(target_os = "linux", feature = "ca"))]
pub mod error;
#[cfg(all(target_os = "linux", feature = "ca"))]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};
