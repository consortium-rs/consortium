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

pub mod mmap;
pub mod remoteproc;
pub mod uio;

// The A-core runtime addresses the shared region through the `[ipc]` layout
// injected by `consortium-builder` (the `CONSORTIUM_IPC_*` env vars). Without
// it the UIO index and region addresses would silently be `0`, so fail the
// build instead — a missing layout is a misconfiguration, not a runtime case.
//
// `env!`'s two-argument form emits this message as a plain `error:` at the
// invocation site (far better DX than the `E0080: evaluation panicked` a
// `const` assert produces). `CONSORTIUM_IPC_BASE` is the sentinel the builder
// always injects, matching `config::CONFIGURED`.
//
// Skipped under `cfg(test)` so the host unit tests (e.g. the IRQ fan-out) build
// without going through the builder. Doctests are disabled in `Cargo.toml`,
// which otherwise force a normal-mode lib build that would trip this guard.
#[cfg(not(test))]
const _: &str = env!(
    "CONSORTIUM_IPC_BASE",
    "consortium-runtime-app requires the IPC shared-memory layout: build through \
     `consortium-builder` (which injects CONSORTIUM_IPC_* from the [ipc] section of \
     Consortium.toml) rather than a plain `cargo build`."
);

pub use consortium_ipc as ipc;
pub use consortium_ipc_transport_memory as ipc_transport;

#[cfg(feature = "tee")]
pub use consortium_tee as tee;

#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_ipcc as ipcc;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_doorbell_mu as mu;
