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

//! Sealed Embassy time-driver code generation for controller cores.
//!
//! Firmware used to hand-write the time-driver bring-up, its interrupt vector,
//! and the `defmt::timestamp!` source in `main.rs`. Those are chip-specific, so
//! they cannot live in the (chip-agnostic) `#[consortium_runtime_mcu::main]`
//! macro; instead the generated `init` drives a chip HAL `timer_driver::bringup`
//! and this module emits the matching vector + timestamp into
//! `consortium.gen.rs`. See [`TimeDriverCodegen`].
//!
//! The timer selection is the reference-board default baked into each chip
//! HAL's `bringup` (STM32MP2 TIM2, i.MX9 LPIT1). Integrators that own their own
//! time base disable this with `[profile] time_driver = false`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// The chip family whose time driver we generate, distinguished by HAL crate.
#[derive(Clone, Copy)]
enum Family {
    /// STM32MP2: cortex-m-rt `#[interrupt] fn TIM2()` vector.
    Stm32mp2,
    /// i.MX9: CMSIS-named `LPIT1_IRQHandler` vector.
    Imx9,
}

impl Family {
    /// Classify by the HAL crate path (`::consortium_hal_stm32mp2` /
    /// `::consortium_hal_imx9`). Returns `None` for families with no sealed time
    /// driver yet (e.g. RISC-V), which suppresses all emission.
    fn from_hal_crate(hal_crate: &str) -> Option<Self> {
        if hal_crate.contains("stm32mp2") {
            Some(Family::Stm32mp2)
        } else if hal_crate.contains("imx9") {
            Some(Family::Imx9)
        } else {
            None
        }
    }
}

/// The module-scope fragments the sealed time driver contributes to one
/// controller core's generated module. All are empty when the driver is
/// disabled or the chip family has no sealed driver.
pub struct TimeDriverCodegen {
    /// `defmt::timestamp!` source. Empty unless a `[dbg.<core>]` logger makes
    /// `defmt` present on this core.
    pub timestamp: TokenStream,
    /// The timer interrupt vector, forwarding to the HAL's `on_timer_interrupt`.
    pub isr: TokenStream,
    /// The `init`-body statement that brings the driver up (calls the HAL
    /// `timer_driver::bringup`).
    pub bringup: TokenStream,
}

impl TimeDriverCodegen {
    /// An all-empty set, for disabled / unsupported cores.
    fn empty() -> Self {
        Self {
            timestamp: quote! {},
            isr: quote! {},
            bringup: quote! {},
        }
    }
}

/// Build the sealed time-driver fragments for one controller core.
///
/// * `hal_crate` — the chip HAL crate path (`::consortium_hal_*`).
/// * `enabled` — the `[profile] time_driver` flag.
/// * `dbg_present` — whether this core has a `[dbg.<core>]` logger (so `defmt`
///   is linked and a `timestamp!` source is required).
pub fn time_driver_codegen(hal_crate: &str, enabled: bool, dbg_present: bool) -> TimeDriverCodegen {
    if !enabled {
        return TimeDriverCodegen::empty();
    }
    let Some(family) = Family::from_hal_crate(hal_crate) else {
        return TimeDriverCodegen::empty();
    };
    let hal: syn::Path = parse_str(hal_crate).expect("hal_crate must be a valid path");

    let bringup = quote! {
        /// SAFETY: `init` runs once during controller bring-up, in privileged
        /// mode, before any `embassy-time` use; `bringup` takes exclusive
        /// ownership of the time-driver timer and its NVIC line.
        unsafe {
            #hal::timer_driver::bringup();
        }
        ::consortium_runtime_mcu::log::trace!("consortium controller init: time driver up");
    };

    // The vector is bare-metal only; the host trybuild harness (which never
    // links the chip `rt` vector table) skips it, matching the `init` gate.
    let isr = match family {
        Family::Stm32mp2 => quote! {
            #[cfg(target_arch = "arm")]
            #[#hal::interrupt]
            fn TIM2() {
                #hal::timer_driver::on_timer_interrupt();
            }
        },
        Family::Imx9 => quote! {
            /// SAFETY: installs the CMSIS-named LPIT1 vector the i.MX9 runtime
            /// dispatches; the handler only forwards to the time driver.
            #[cfg(target_arch = "arm")]
            #[unsafe(no_mangle)]
            extern "C" fn LPIT1_IRQHandler() {
                #hal::timer_driver::on_timer_interrupt();
            }
        },
    };

    // `defmt` requires exactly one timestamp source per binary. Emit it only
    // when `defmt` is actually linked (a `[dbg.<core>]` logger is present) and
    // only for bare-metal builds, so the host harness stays `defmt`-free.
    let timestamp = if dbg_present {
        quote! {
            #[cfg(target_arch = "arm")]
            defmt::timestamp!("{=u32}", {
                use ::core::sync::atomic::{AtomicU32, Ordering};
                static COUNT: AtomicU32 = AtomicU32::new(0);
                COUNT.fetch_add(1, Ordering::Relaxed)
            });
        }
    } else {
        quote! {}
    };

    TimeDriverCodegen {
        timestamp,
        isr,
        bringup,
    }
}
