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

//! Shared HAL traits for Consortium chip support crates.

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
