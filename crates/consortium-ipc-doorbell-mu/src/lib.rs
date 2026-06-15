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

pub mod chan;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use chan::MuChanValidate;
#[cfg(feature = "imx95")]
pub use chan::{IMX95_CA55_CM7_RX_CHAN, IMX95_CA55_CM7_TX_CHAN};
pub use doorbell::MuDoorbell;
pub use error::MuError;
