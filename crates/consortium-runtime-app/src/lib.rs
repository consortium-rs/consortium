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

// This crate is Linux-only in production: remoteproc, UIO, TEE, and the entry
// macros all target the A-core Linux runtime. The exception is [`irq`], the
// portable user-space half of the UIO interrupt protocol, which is gated
// `cfg(unix)` so its socketpair-backed tests run on macOS as well as Linux.
// Every other item below is gated `cfg(target_os = "linux")` individually,
// replacing the former crate-level `#![cfg(target_os = "linux")]`.

/// Portable UIO interrupt dispatch (`[fd readiness → doorbell fan-out]`). Kept
/// out from under the Linux gate so its tests build and run on any Unix; the
/// real `/dev/uioN` device wiring lives in [`uio`] (Linux only).
#[cfg(unix)]
pub mod irq;

#[cfg(target_os = "linux")]
pub mod remoteproc;
#[cfg(all(target_os = "linux", feature = "ipc-shm"))]
pub mod uio;

/// Logging facade re-export for generated AP code. The pipeline emits
/// `::consortium_runtime_app::log::trace!(...)` in the generated `init`, routed
/// here to `tracing`, so endpoint crates need no direct logging dependency.
#[cfg(target_os = "linux")]
pub mod log {
    pub use consortium_log::{debug, error, info, trace, warn};
}

/// Host-side defmt console for coprocessor debug rings. Requires both the
/// `dbg` decode stack and the `ipc-shm` UIO mapping.
#[cfg(all(target_os = "linux", feature = "dbg", feature = "ipc-shm"))]
pub mod dbg_console;

#[cfg(all(target_os = "linux", feature = "ipc"))]
pub use consortium_ipc as ipc;
#[cfg(all(target_os = "linux", feature = "ipc-shm"))]
pub use consortium_ipc_transport_memory as ipc_transport;

#[cfg(all(target_os = "linux", feature = "dbg"))]
pub use consortium_dbg as dbg;

#[cfg(all(target_os = "linux", feature = "tee"))]
pub use consortium_tee as tee;

/// Entry-point macros. Applications write `#[consortium_runtime_app::main]` on
/// their `async fn main(context: Context)` and, optionally,
/// `#[consortium_runtime_app::fail]` on an init-failure handler.
#[cfg(all(target_os = "linux", feature = "macros"))]
pub use consortium_runtime_macros::{app_fail as fail, app_main as main};

/// Runtime glue for the entry macros. Not a stable API — the generated
/// `#[main]` code glob-imports [`__consortium_init_fail`] from here so a user
/// `#[fail]` handler can shadow it.
#[cfg(all(target_os = "linux", feature = "macros"))]
#[doc(hidden)]
pub mod __rt {
    /// Fallback init-failure handler used when no `#[fail]` handler is defined.
    /// Logs and exits non-zero; a `#[fail]` handler shadows this via the glob
    /// import the `#[main]` expansion emits.
    pub fn __consortium_init_fail<E>(_err: E) -> ! {
        crate::log::error!("consortium: init failed; no `#[fail]` handler, exiting");
        ::std::process::exit(1);
    }
}
