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

//! Consortium HAL for i.MX 9 family devices.

pub use consortium_hal as hal;
pub use consortium_pac_imx9 as pac;

#[cfg(feature = "dma")]
pub mod dma;

#[cfg(feature = "time-driver-lpit")]
pub mod timer_driver;

#[cfg(feature = "uart")]
pub mod uart;
