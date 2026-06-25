#![cfg(target_os = "linux")]
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

#[cfg(feature = "ipc-shm")]
pub mod mmap;
pub mod remoteproc;
#[cfg(feature = "ipc-shm")]
pub mod uio;

// When the A-core runtime uses the shared-memory IPC transport, it reaches the
// region through `/dev/uioN`, not through the physical base address. Require
// the builder-injected UIO number only for that runtime path.
//
// `env!`'s two-argument form emits this message as a plain `error:` at the
// invocation site (far better DX than the `E0080: evaluation panicked` a
// `const` assert produces).
//
// Skipped under `cfg(test)` so the host unit tests (e.g. the IRQ fan-out) build
// without going through the builder. Doctests are disabled in `Cargo.toml`,
// which otherwise force a normal-mode lib build that would trip this guard.
#[cfg(all(feature = "ipc-shm", not(test)))]
const _: &str = env!(
    "CONSORTIUM_IPC_UIO",
    "consortium-runtime-app with `ipc-shm` requires the IPC UIO device index: build through \
     `consortium-builder` (which injects CONSORTIUM_IPC_UIO from the [ipc] section of \
     Consortium.toml) rather than a plain `cargo build`."
);

#[cfg(all(feature = "dbg", not(test)))]
const _: &str = env!(
    "CONSORTIUM_DBG_UIO",
    "consortium-runtime-app with `dbg` requires the debug UIO device index: build through \
     `consortium-builder` (which injects CONSORTIUM_DBG_UIO from the [dbg] section of \
     Consortium.toml) rather than a plain `cargo build`."
);

#[cfg(feature = "ipc-shm")]
pub use consortium_ipc as ipc;
#[cfg(feature = "ipc-shm")]
pub use consortium_ipc_transport_memory as ipc_transport;

#[cfg(feature = "dbg")]
pub use consortium_dbg_defmt as dbg;

#[cfg(feature = "tee")]
pub use consortium_tee as tee;
