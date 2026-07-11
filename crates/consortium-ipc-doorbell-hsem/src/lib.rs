#![cfg_attr(not(feature = "std"), no_std)]
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

//! HSEM (Hardware Semaphore) doorbell driver for Consortium IPC.
//!
//! A [`HsemDoorbell`] drives the HSEM peripheral from one processor [`side`],
//! with the side selected at the type level: [`Primary`] is core C1, [`Secondary`]
//! is C2 and [`Tertiary`] is C3. HSEM is single-instance, so the instance const
//! `N` is always `1` (e.g. `HsemDoorbell<Primary, 1>`). Its channels are the 16
//! hardware semaphores.
//!
//! [`side`]: consortium_ipc::Side

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Hsem;
pub use chan::{HSEM_CHAN_COUNT, HSEM_CHAN_OCCUPIED, HsemChanValidate};
pub use consortium_ipc::{Primary, Secondary, Side, Tertiary};
pub use doorbell::HsemDoorbell;
#[cfg(not(feature = "std"))]
pub use doorbell::notify;
pub use error::HsemError;
