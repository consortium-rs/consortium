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

//! The A-core Linux runtime: remoteproc control, UIO-backed IPC, host-side debug
//! consoles, and the `#[main]` entry macro.
//!
//! This is the Linux half of a Consortium system. Where `consortium-runtime-mcu`
//! gives firmware its entry point and shared-memory glue, this crate gives the
//! application the same — plus the pieces only Linux has: `/sys/class/remoteproc`
//! for booting a coprocessor, `/dev/uioN` for reaching shared memory and doorbells
//! from userspace, and a `tracing` subscriber to put the resulting logs somewhere.
//!
//! # Modules
//!
//! | Module                      | Role                                            |
//! | --------------------------- | ----------------------------------------------- |
//! | `irq`                     | The portable UIO interrupt dispatch loop.        |
//! | `uio`                     | `/dev/uioN` open, mmap, and device lifecycle.     |
//! | `remoteproc`              | Load firmware and start/stop a coprocessor.       |
//! | `dbg_console`             | Drain and decode coprocessor CDBG debug rings.     |
//! | `ipc`, `ipc_transport`, `dbg`, `tee` | Feature-gated re-exports for generated code. |
//!
//! # Why `irq` is not under the Linux gate
//!
//! Almost everything here is `cfg(target_os = "linux")`, but `irq` is
//! `cfg(unix)`. That is deliberate: the `[fd readiness → user-space fan-out]` slice
//! of the UIO protocol has no Linux-specific API in it, so gating it at `unix`
//! lets its socketpair-backed tests run on a macOS developer machine. The real
//! `/dev/uioN` mapping and device lifecycle stay Linux-only in `uio`.
//!
//! # The UIO interrupt handshake
//!
//! Once `/dev/uioN` is open, the kernel arms the device interrupt and **user space
//! owns the mask/re-arm handshake** from then on. `irq::irq_loop` runs one
//! iteration per M→A interrupt, in an order that is the whole design:
//!
//! 1. wait for the fd to become readable;
//! 2. drain the 4-byte interrupt count so it is not readable again;
//! 3. fan out to every registered doorbell service;
//! 4. re-arm the interrupt with a 4-byte write.
//!
//! Step 3 before step 4 is what keeps a level-sensitive line from immediately
//! re-firing the instant it is unmasked — a masking doorbell (IPCC, MU) masks its
//! asserted lines *during* the fan-out. And the re-arm is deliberately the *last*
//! act of each iteration, never a first enable: the kernel already armed the line
//! at open, and a redundant enable can unbalance a UIO driver's enable/disable
//! accounting and leave the interrupt wedged off.
//!
//! Bind a doorbell accordingly:
//!
//! | Doorbell kind                | Bind with        | Because                        |
//! | ---------------------------- | ---------------- | ------------------------------ |
//! | plain (GPIO)                 | `bind_service`    | edge-driven; the service only wakes |
//! | Type-A bell (IPCC, MU, HSEM) | `bind_service`    | must mask asserted lines first      |
//!
//! Windows are mapped by kind, too: `mmap` for register and doorbell windows,
//! `mmap_zeroed` for data windows — the AP owns first touch of a shared carveout,
//! so a warm reboot's stale descriptor bytes never read as valid.
//!
//! # Entry macro
//!
//! ```rust,ignore
//! #[consortium_runtime_app::main]
//! async fn main() -> Result<(), ConsortiumError> {
//!     let (_p, c) = init(Config::default())?;
//!     let mut telemetry =
//!         consortium_ipc_transport_memory::Transceiver::connect(c.TELEMETRY).await?;
//!     telemetry.send(&Command::Start).await?;
//!     Ok(())
//! }
//! ```
//!
//! The macro stacks `tokio::main` (resolved through the *user* crate's own `tokio`
//! dependency, so it needs `tokio` with `macros`). It accepts `()` or
//! `Result<(), E: Debug>`; an error initializes tracing, logs, and exits non-zero.
//!
//! # Debug consoles
//!
//! `[dbg] mode` picks how the AP consumes coprocessor rings: `auto` spawns a task
//! per ring that decodes frames against the firmware ELF and forwards them to
//! `tracing`; `interface` hands back reader handles for the integrator. A debug
//! ring has no doorbell of its own, so the auto task polls on an interval.
//!
//! # Features
//!
//! Defaults are `dbg`, `ipc-shm`, and `macros`.
//!
//! | Feature   | Effect                                                          |
//! | --------- | --------------------------------------------------------------- |
//! | `ipc`     | Re-export `consortium-ipc`.                                      |
//! | `ipc-shm` | UIO-backed shared-memory IPC; implies `ipc`.                      |
//! | `dbg`     | Debug-ring decoding and `dbg_console`.                          |
//! | `macros`  | The `#[main]` entry macro.                                         |
//! | `tee`     | Re-export `consortium-tee`.                                       |
//! | `hmi`, `ort`, `ffi` | Re-export the HMI bridge, ONNX Runtime, and C ABI crates. |
//! | chip features | `mimx93`/`mimx95`, `stm32mp21x`/`23x`/`25x` — forwarded to the shared-memory transport so it can report doorbell window addresses. |
//!
//! `stm32mp2xx` is a compatibility alias for `stm32mp25x`; new code should name a
//! chip family directly so STM32MP21 can select IPCC without HSEM.

// This crate is Linux-only in production: remoteproc, UIO, TEE, and the entry
// macros all target the A-core Linux runtime. The exception is [`irq`], the
// portable user-space half of the UIO interrupt protocol, which is gated
// `cfg(unix)` so its socketpair-backed tests run on macOS as well as Linux.
// Every other item below is gated `cfg(target_os = "linux")` individually,
// replacing the former crate-level `#![cfg(target_os = "linux")]`.

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
/// their `async fn main()` returning `()` or `Result<(), E: Debug>`.
#[cfg(all(target_os = "linux", feature = "macros"))]
pub use consortium_runtime_macros::app_main as main;

/// Install the application's tracing subscriber.
///
/// Idempotent: an already-installed global subscriber is left in place.
#[cfg(target_os = "linux")]
pub fn init_tracing() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = fmt().with_env_filter(filter).try_init();
}

/// Runtime glue for the entry macro.
#[cfg(all(target_os = "linux", feature = "macros"))]
#[doc(hidden)]
pub mod __rt {
    /// Outcomes accepted from a Consortium entry body.
    pub trait MainOutcome {
        /// Report failure and terminate, or return for a successful unit value.
        fn __consortium_report(self);
    }

    impl MainOutcome for () {
        fn __consortium_report(self) {}
    }

    impl<E: ::core::fmt::Debug> MainOutcome for ::core::result::Result<(), E> {
        fn __consortium_report(self) {
            if let ::core::result::Result::Err(error) = self {
                __consortium_main_fail(error);
            }
        }
    }

    /// Install tracing, log a failed entry outcome, and exit non-zero.
    pub fn __consortium_main_fail<E: ::core::fmt::Debug>(error: E) -> ! {
        crate::init_tracing();
        crate::log::error!("consortium main failed: {:?}", error);
        ::std::process::exit(1);
    }
}
