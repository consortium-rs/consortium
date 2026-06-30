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

pub mod remoteproc;
#[cfg(feature = "ipc-shm")]
pub mod uio;

/// Host-side defmt console for coprocessor debug rings. Requires both the
/// `dbg` decode stack and the `ipc-shm` UIO mapping.
#[cfg(all(feature = "dbg", feature = "ipc-shm"))]
pub mod dbg_console;

#[cfg(feature = "ipc")]
pub use consortium_ipc as ipc;
#[cfg(feature = "ipc-shm")]
pub use consortium_ipc_transport_memory as ipc_transport;

#[cfg(feature = "dbg")]
pub use consortium_dbg as dbg;

#[cfg(feature = "tee")]
pub use consortium_tee as tee;
