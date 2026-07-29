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

//! Consortium HAL for STM32MP2 family devices.
//!
//! Peripherals are owned, not addressed. [`init`] hands out the chip's
//! singletons once as a [`Peripherals`] struct, and each driver's constructor
//! consumes the one it drives:
//!
//! ```ignore
//! let p = consortium_hal_stm32mp2::init();
//! let i2c = consortium_hal_stm32mp2::i2c::I2c::new(p.I2C1, 64_000_000, 400_000);
//! ```
//!
//! Because the base address and NVIC line travel with the singleton, no driver
//! takes an MMIO pointer and no constructor is `unsafe`. Bring-up that runs
//! outside [`init`] — a second-stage loader, a test harness — can still reach a
//! peripheral through `peripherals::<NAME>::steal()`, which carries the
//! exclusivity obligation the safe path discharges.

#[cfg(test)]
extern crate std;

pub use consortium_hal as hal;
pub use consortium_pac_stm32mp2 as pac;
/// Ownership handle and marker trait for peripheral singletons, re-exported so
/// drivers in other crates can name them without depending on Embassy directly.
pub use embassy_hal_internal::{Peri, PeripheralType};

// Peripheral singletons and their driver bindings, generated from the chip
// databases by `scripts/pac/gen_peripherals.py`. Chip features are mutually
// exclusive; enabling several is a duplicate-definition error, matching the PAC.
// `stm32mp23x` and `stm32mp25x` share the STM32MP257 map, as they do in the PAC.
#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
#[path = "peripherals/stm32mp25.rs"]
mod chip;
#[cfg(feature = "stm32mp21x")]
#[path = "peripherals/stm32mp21.rs"]
mod chip;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use chip::{Peripherals, peripherals};

/// Take the chip's peripheral singletons.
///
/// # Panics
///
/// Panics if called more than once — the singletons are exactly that, and
/// handing them out twice would let two drivers claim the same registers.
#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub fn init() -> Peripherals {
    Peripherals::take()
}

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

#[cfg(feature = "lptim")]
pub mod lptim;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "time-driver-gp-tim")]
pub mod timer_driver;

#[cfg(feature = "timer")]
pub mod timer;

#[cfg(feature = "uart")]
pub mod uart;
