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

//! MU (Messaging Unit) doorbell driver for Consortium IPC.
//!
//! A [`MuDoorbell`] drives one MU instance from one processor [`side`], with the
//! instance number and side selected at the type level (e.g. `MuDoorbell<Primary,
//! 7>` is the CA55 view of MU7, the i.MX95 CA55 <-> CM7 default route). Its
//! channels span two line kinds of that instance: channels `0..4` are the
//! general-purpose interrupt (GI) lines and channels `4..8` are the
//! receive-full (RF) lines.
//!
//! [`side`]: consortium_ipc::Side

pub mod abstraction;
pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use abstraction::Mu;
pub use chan::{
    MU_CHAN_COUNT, MU_CHAN_OCCUPIED, MU_GI_CHAN_COUNT, MU_INSTANCE_COUNT, MU_RF_CHAN_COUNT,
    MuChanValidate,
};
pub use consortium_ipc::{Primary, Secondary, Side};
pub use doorbell::MuDoorbell;
#[cfg(not(feature = "std"))]
pub use doorbell::notify;
pub use error::MuError;
