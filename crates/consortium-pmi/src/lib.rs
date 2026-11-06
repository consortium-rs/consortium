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

#![cfg_attr(not(feature = "std"), no_std)]

//! Platform management for Consortium: how a core obtains resources it does not
//! own.
//!
//! On a heterogeneous SoC the clocks, pads, power domains and resets a core
//! needs are not always the core's to touch. On STM32MP2 and i.MX 93 a
//! controller core writes RCC/CCM/IOMUXC itself; on i.MX 95 a System Manager
//! owns all of them and has to be *asked*, over SCMI; a RISC-V platform may
//! answer RPMI instead. Drivers and generated `init()` should not care which.
//!
//! This crate is that seam. It defines one provider trait per resource kind and
//! nothing else. Backends implement them:
//!
//! | Backend                  | Crate                     | Chips                      |
//! | ------------------------ | ------------------------- | -------------------------- |
//! | SCMI client              | `consortium-pmi-scmi`     | i.MX 95/952 System Manager |
//! | Direct register access   | the chip HAL (future)     | STM32MP2, i.MX 93          |
//!
//! # Identifiers
//!
//! [`ClockId`], [`PinId`], [`FunctionId`], [`PowerDomainId`] and [`ResetId`]
//! are opaque `u32` newtypes. Their numbering belongs to the backend: an SCMI
//! backend passes them through as the platform's domain identifiers, and a
//! register backend maps them to its own tables. Code that names a resource
//! therefore names it for one platform; `consortium-cfg` is where that mapping
//! is meant to live.
//!
//! # Async
//!
//! Every operation returns `impl Future<Output = Result<_, Self::Error>> +
//! MaybeSend` (RPITIT), like `consortium-ipc`'s transports. A message-based
//! backend waits for the platform's reply; a register backend completes on the
//! first poll. Methods take `&mut self`: one provider handle carries one
//! request at a time.
//!
//! # Features
//!
//! - `std` — [`MaybeSend`] requires `Send`.
//! - `defmt` — derive `defmt::Format` on the identifier and configuration types.

pub mod clock;
pub mod pinctrl;
pub mod power;
pub mod reset;

pub use consortium_core::MaybeSend;
pub use {
    clock::{ClockId, ClockProvider},
    pinctrl::{FunctionId, PinConfig, PinId, PinctrlProvider},
    power::{PowerDomainId, PowerProvider, PowerState},
    reset::{ResetId, ResetProvider},
};

/// Declares an opaque `u32` resource identifier.
macro_rules! resource_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name(pub u32);

        impl From<u32> for $name {
            fn from(raw: u32) -> Self {
                Self(raw)
            }
        }

        impl From<$name> for u32 {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}
pub(crate) use resource_id;
