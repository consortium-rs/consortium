#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
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

//! STM32 inter-processor communication controller (IPCC) doorbell support.
//!
//! The IPCC carries no payload. It only exposes per-channel status flags used
//! to transfer ownership of shared memory between processor 1 and processor 2.
//! This crate uses simplex signaling: [`IpccDoorbell::ring`] sets the local
//! transmit channel occupied, and [`IpccDoorbell::wait`] waits for the remote
//! transmit channel to become occupied before clearing it.
//!
//! The processor side and IPCC instance are selected at the type level:
//! [`Primary`] drives processor 1, [`Secondary`] processor 2; `N` is the instance
//! (`IpccDoorbell<Primary, 1>` is processor 1 on IPCC1). Channels are the 16
//! IPCC channels.
//!
//! [`Primary`]: consortium_ipc::Primary
//! [`Secondary`]: consortium_ipc::Secondary

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Ipcc;
pub use chan::{IPCC_CHAN_COUNT, IPCC_CHAN_OCCUPIED, IpccChanValidate};
pub use consortium_ipc::{Primary, Secondary, Side};
#[cfg(not(feature = "std"))]
pub use doorbell::notify_rx_occupied_interrupt;
pub use doorbell::{IpccDoorbell, IpccWaitFut};
pub use error::IpccError;
