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

#[cfg(test)]
extern crate std;

pub use consortium_hal as hal;
pub use consortium_pac_imx9 as pac;

/// Interrupt lines for the selected chip, re-exported from the PAC for direct
/// HAL consumers.
pub use pac::interrupts;
pub use pac::interrupts::Interrupt;
/// The `#[interrupt]` attribute macro, available when the `rt` feature is on.
#[cfg(feature = "rt")]
pub use pac::interrupts::interrupt;

#[cfg(feature = "adc")]
pub mod adc;

#[cfg(feature = "can")]
pub mod can;

#[cfg(feature = "dma")]
pub mod dma;

#[cfg(feature = "gpio")]
pub mod gpio;

#[cfg(feature = "i2c")]
pub mod i2c;

#[cfg(feature = "i3c")]
pub mod i3c;

#[cfg(feature = "timer")]
pub mod lptmr;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "time-driver-lpit")]
pub mod timer_driver;

#[cfg(feature = "timer")]
pub mod timer;

#[cfg(feature = "uart")]
pub mod uart;
