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

//! Shared HAL vocabulary for Consortium chip support crates: the peripheral
//! singleton machinery, a handful of narrow traits, and NVIC helpers.
//!
//! This crate is deliberately small. It holds only what the chip HALs
//! (`consortium-hal-imx9`, `consortium-hal-stm32mp2`) and their generated code need
//! to *agree* on. Every register layout, clock tree, and driver lives in a chip HAL,
//! because that is where the differences are and a shared trait that tried to
//! abstract them would either leak or lie.
//!
//! # Peripherals are owned, not addressed
//!
//! The load-bearing idea here is that a driver takes ownership of a peripheral
//! rather than an address. [`peripherals!`] declares a chip's zero-sized singleton
//! markers and the `Peripherals` struct that hands them out **once**; a driver
//! constructor consumes the one it drives:
//!
//! ```rust,ignore
//! let p = consortium_hal_imx9::init();
//! let i2c = consortium_hal_imx9::i2c::I2c::new(p.LPI2C1, 24_000_000, 400_000);
//! ```
//!
//! Because the base address and NVIC line travel with the singleton, no driver takes
//! an MMIO pointer and no constructor is `unsafe` — the aliasing question is settled
//! by the type system instead of by a comment. `peripherals::<NAME>::steal()` is the
//! escape hatch for bring-up outside `init()`, and it carries the exclusivity
//! obligation the safe path discharges.
//!
//! Per-instance driver state hangs off the marker rather than a named `static`, so an
//! application forwards an interrupt as
//! `<hal>::i2c::on_interrupt::<peripherals::LPI2C1>()`.
//!
//! # Traits
//!
//! | Trait               | Answers                                              |
//! | ------------------- | ---------------------------------------------------- |
//! | [`Peripheral`]      | What is this instance called in generated metadata?    |
//! | [`ClockControl`]    | Enable and reset the peripheral clock.                 |
//! | [`InterruptSource`] | Which interrupt line does it raise?                    |
//! | [`MonotonicTimer`]  | Read ticks and program the next alarm.                 |
//!
//! [`MonotonicTimer`] is what a chip HAL's Embassy time driver is built on
//! (STM32MP2 TIM2, i.MX9 LPIT1); `consortium-cfg` emits its bring-up into the
//! generated `init()` unless `[profile] time_driver = false`.
//!
//! The [`nvic`] module adds interrupt masking for lines known only at runtime, which
//! typed PAC `Interrupt` enums cannot express.
//!
//! # `Peri` and `PeripheralType`
//!
//! Both are re-exported from `embassy_hal_internal`, so chip HALs and their
//! dependents (the doorbell crates, the UART transport glue) can name a singleton
//! handle without taking an Embassy dependency of their own.
//!
//! `no_std`, no features.

pub mod nvic;

/// Ownership handle and marker trait for peripheral singletons, re-exported so
/// chip HALs and their dependents can name them without depending on Embassy
/// directly.
pub use embassy_hal_internal::{Peri, PeripheralType};

/// Declare a chip's peripheral singletons and the `Peripherals` struct that
/// hands them out once.
///
/// This is `embassy_hal_internal::peripherals!` with two deliberate changes:
///
/// - **No per-type `defmt::Format` derive.** Consortium's chips have hundreds of
///   peripherals, and the derive emits a `.defmt.*` link section per type; on a
///   `defmt`-enabled host build that overruns the Mach-O section limit and the
///   HAL stops compiling on macOS. The markers are zero-sized and never
///   formatted, so nothing is lost.
/// - **No `#[unsafe(no_mangle)]` on the once-guard.** Two chip HALs can end up in
///   one binary (the `consortium-cfg` UI tests link both), and an unmangled
///   guard symbol would collide between them.
///
/// Invoke it at the chip HAL's crate root; `scripts/pac/gen_peripherals.py`
/// emits the invocation.
#[macro_export]
macro_rules! peripherals {
    ($($(#[$cfg:meta])? $name:ident),* $(,)?) => {
        /// Zero-sized marker types for the peripheral singletons.
        pub mod peripherals {
            $(
                $(#[$cfg])?
                #[allow(non_camel_case_types)]
                #[doc = concat!(stringify!($name), " peripheral")]
                #[derive(Debug)]
                pub struct $name {
                    _private: (),
                }

                $(#[$cfg])?
                impl $name {
                    /// Unsafely create an instance of this peripheral out of thin
                    /// air.
                    ///
                    /// # Safety
                    ///
                    /// You must ensure that you're only using one instance of
                    /// this type at a time.
                    #[inline]
                    pub const unsafe fn steal() -> $crate::Peri<'static, Self> {
                        // SAFETY: forwarded to the caller, who guarantees this
                        // is the only live instance of the peripheral.
                        unsafe { $crate::Peri::new_unchecked(Self { _private: () }) }
                    }
                }

                $(#[$cfg])?
                impl ::core::marker::Copy for $name {}

                $(#[$cfg])?
                impl ::core::clone::Clone for $name {
                    fn clone(&self) -> Self {
                        *self
                    }
                }

                $(#[$cfg])?
                impl $crate::PeripheralType for $name {}
            )*
        }

        /// Every peripheral singleton this chip exposes to the running core.
        ///
        /// Obtained once from the HAL's `init()`.
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(
                #[doc = concat!(stringify!($name), " peripheral")]
                $(#[$cfg])?
                pub $name: $crate::Peri<'static, peripherals::$name>,
            )*
        }

        impl Peripherals {
            /// Returns all the peripherals *once*.
            ///
            /// # Panics
            ///
            /// Panics if called more than once.
            #[inline]
            pub(crate) fn take() -> Self {
                ::critical_section::with(|_| {
                    static mut TAKEN: bool = false;

                    // SAFETY: inside a critical section, so this is the only
                    // access to `TAKEN` for its duration.
                    unsafe {
                        if TAKEN {
                            panic!("the chip's peripherals have already been taken");
                        }
                        TAKEN = true;
                        Self::steal()
                    }
                })
            }

            /// Unsafely create an instance of every peripheral out of thin air.
            ///
            /// # Safety
            ///
            /// You must ensure that you're only using one instance of each type
            /// at a time.
            #[inline]
            pub unsafe fn steal() -> Self {
                Self {
                    $(
                        $(#[$cfg])?
                        // SAFETY: forwarded to the caller, who guarantees each
                        // peripheral has only one live instance.
                        $name: unsafe { peripherals::$name::steal() },
                    )*
                }
            }
        }
    };
}

/// Marker for a concrete peripheral instance.
pub trait Peripheral {
    /// Human-readable peripheral name used by generated chip metadata.
    const NAME: &'static str;
}

/// Minimal clock/reset hooks for chip-specific peripheral drivers.
pub trait ClockControl {
    /// Enable the peripheral clock.
    fn enable(&self);

    /// Reset the peripheral when the chip supports a local reset.
    fn reset(&self);
}

/// Interrupt source that can be bound by board startup code.
pub trait InterruptSource {
    /// Human-readable interrupt name.
    const INTERRUPT: &'static str;
}

/// Minimal monotonic timer abstraction shared by chip HAL time drivers.
pub trait MonotonicTimer {
    /// Return monotonically increasing ticks.
    fn now(&self) -> u64;

    /// Program the next hardware alarm.
    fn set_alarm(&self, at: u64) -> bool;
}
