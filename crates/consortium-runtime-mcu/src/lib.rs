#![no_std]
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

//! The controller-core firmware runtime: entry macro, resource table, MPU setup, and
//! feature-gated re-exports of the IPC and debug stacks.
//!
//! This is the `no_std` half of a Consortium system. It is deliberately thin — most of
//! what firmware needs is *generated* by `consortium-cfg` into `consortium.gen.rs`, and
//! what remains here is the handful of things generated code cannot supply: an entry
//! point, the ELF section Linux remoteproc reads, cache configuration, and one
//! dependency edge that pulls in the transports.
//!
//! # Modules
//!
//! | Module            | Role                                                       |
//! | ----------------- | ---------------------------------------------------------- |
//! | [`rsc_table`]     | The `.resource_table` section a Linux host parses at boot.   |
//! | [`mpu`]           | Mark the shared carveout non-cacheable on a cached core.      |
//! | [`log`]           | Logging facade re-export for generated code.                 |
//!
//! # Entry macro
//!
//! ```rust,ignore
//! #[consortium_runtime_mcu::main]
//! async fn main(context: Context, spawner: Spawner) {
//!     let telemetry = context.ipc_shm.telemetry;
//!     // ...
//! }
//!
//! // Optional: report a bring-up failure yourself.
//! #[consortium_runtime_mcu::fail]
//! fn on_fail(err: ConsortiumInitError) { defmt::error!("init failed: {}", err); }
//! ```
//!
//! The macro stacks `embassy_executor::main` — resolved through the *user* crate's own
//! dependency, so firmware still needs `embassy-executor` with `executor-thread` and
//! `platform-cortex-m` — then calls the generated `init()` and binds its result as
//! `context`.
//!
//! Without a `#[fail]` handler, a bring-up failure parks the core with `wfi`. The
//! dispatch is a glob-import shadow: `#[main]` glob-imports a fallback from
//! [`__rt`], and a module-scope `#[fail]` item shadows it. `#[main]`, `#[fail]`,
//! `init`, and `Context` must therefore share the module the generated file is
//! `include!`d into.
//!
//! # What the generated `init()` already does
//!
//! Firmware used to hand-write all of this; it is now sealed:
//!
//! - resets the CDBG ring (`ConsortiumLogger::init`) when the core has a
//!   `[dbg.<core>]` logger, so a warm reset does not resume into a stale ring;
//! - brings up the chip HAL's Embassy time driver and emits the matching timer
//!   vector and `defmt::timestamp!` — disable with `[profile] time_driver = false`
//!   when firmware owns its own time base;
//! - constructs, `connect`s, and splits each IPC transport into a transceiver at
//!   `context.ipc_shm.<name>`;
//! - hands out `context.peripherals` on cores owning `[peripheral.*]`.
//!
//! The two-phase `IpcMemoryEndpoints::new` / `connect` API stays available for
//! non-macro callers.
//!
//! # Interrupts are still yours
//!
//! Neither this crate nor the doorbell or HAL crates install `#[interrupt]` handlers.
//! The application owns the vector table and forwards each relevant IRQ into the
//! exported wake path — `hsem::notify()`, `ipcc::notify_rx_occupied_interrupt()`,
//! `mu::notify::<N>()`, `uart::on_buffered_interrupt::<P>()`. Enable the chip *HAL's*
//! `rt` feature so the `#[interrupt]` macro and PAC vector table are available. See
//! the melt-pot `mcu/src/main.rs` files for worked examples.
//!
//! # Scratch buffers
//!
//! [`static_buf!`](static_buf) declares a `StaticCell` whose `.init(..)` yields the
//! `&'static mut [u8]` every channel and buffered-UART constructor wants, keeping the
//! raw `static mut` out of application code.
//!
//! # Features
//!
//! Defaults are `ipc-shm` and `macros`.
//!
//! | Feature     | Effect                                                        |
//! | ----------- | ------------------------------------------------------------- |
//! | `ipc-shm`   | Shared-memory IPC (`ipc` is an alias).                         |
//! | `ipc-uart`  | The COBS/CRC-framed UART transport instead, or as well.         |
//! | `macros`    | The `#[main]` / `#[fail]` entry macros.                         |
//! | `dbg`       | Re-export `consortium-dbg` for the CDBG logger.                  |
//! | `defmt`     | Route generated-code logging to `defmt`.                         |
//! | chip features | `mimx93`/`mimx95`, `stm32mp21x`/`23x`/`25x` — forwarded to the shared-memory transport. |
//!
//! Enabling `defmt` here is not sufficient on its own: `defmt`'s macros expand to
//! literal `defmt::` paths, so **the firmware crate that includes the generated module
//! must also depend on `defmt` directly.**
//!
//! `stm32mp2xx` is a compatibility alias for `stm32mp25x`; new code should name a chip
//! family directly so STM32MP21 can select IPCC without HSEM.

pub mod mpu;
pub mod rsc_table;

/// Logging facade re-export for generated controller code. The pipeline emits
/// `::consortium_runtime_mcu::log::trace!(...)` in the generated `init`, routed
/// to `defmt` when this crate's `defmt` feature is on (and the firmware crate
/// depends on `defmt`), otherwise a no-op.
pub mod log {
    pub use consortium_log::{debug, error, info, trace, warn};
}

#[cfg(feature = "dbg")]
pub use consortium_dbg as dbg;

#[cfg(feature = "ipc-shm")]
pub use consortium_ipc_transport_memory;

#[cfg(feature = "ipc-uart")]
pub use consortium_ipc_transport_uart;

/// Entry-point macros. Firmware writes `#[consortium_runtime_mcu::main]` on its
/// `async fn main(context: Context, spawner: Spawner)` and, optionally,
/// `#[consortium_runtime_mcu::fail]` on an init-failure handler.
#[cfg(feature = "macros")]
pub use consortium_runtime_macros::{mcu_fail as fail, mcu_main as main};

/// Runtime glue for the entry macros. Not a stable API — the generated
/// `#[main]` code glob-imports `__consortium_init_fail` from here so a user
/// `#[fail]` handler can shadow it.
#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod __rt {
    /// Fallback init-failure handler used when no `#[fail]` handler is defined.
    /// Logs and parks the core; a `#[fail]` handler shadows this via the glob
    /// import the `#[main]` expansion emits.
    pub fn __consortium_init_fail<E>(_err: E) -> ! {
        // The `log` re-export expands to `defmt::` paths when `defmt` is linked,
        // which this crate can only resolve behind its own `defmt` feature; keep
        // the fallback silent otherwise. Rich reporting belongs in a user
        // `#[consortium_runtime_mcu::fail]` handler, which owns the error type.
        #[cfg(feature = "defmt")]
        crate::log::error!("consortium: init failed; no `#[fail]` handler, parking core");
        loop {
            ::cortex_m::asm::wfi();
        }
    }
}

/// Macro for declaring a static scratch buffer with proper alignment.
///
/// # Example
///
/// ```ignore
/// static_buf!(MY_BUF, 1024);  // Declares static mut MY_BUF: [u8; 1024]
/// ```
#[macro_export]
macro_rules! static_buf {
    ($name:ident, $size:expr) => {
        static $name: $crate::static_cell::StaticCell<[u8; $size]> =
            $crate::static_cell::StaticCell::new([0u8; $size]);
    };
}

pub use static_cell;
