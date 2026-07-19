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
/// `#[main]` code glob-imports [`__consortium_init_fail`] from here so a user
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
