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

pub use consortium_ipc as ipc;
pub use consortium_ipc_transport_memory as ipc_transport;
pub use consortium_tee as tee;

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_doorbell_mu as mu;
