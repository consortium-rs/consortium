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
