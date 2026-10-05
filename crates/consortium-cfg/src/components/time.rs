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
//!
//! A core whose chip map does not list that timer gets no time driver, the same
//! as a family with no sealed driver; a core with no bundled chip map keeps the
//! family default. The STM32MP25 CM0+ is the case in point:
//! it reaches only the SmartRun domain (LPTIM3–5, no TIM2). `[profile]
//! time_driver` is manifest-wide, so rejecting such a core would force the
//! driver off on every other core too.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// The chip family whose time driver we generate, distinguished by HAL crate.
#[derive(Clone, Copy)]
enum Family {
    /// STM32MP2: TIM2.
    Stm32mp2,
    /// i.MX9: LPIT1.
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

    /// The peripheral instance the HAL's `bringup` drives.
    fn timer(self) -> &'static str {
        match self {
            Family::Stm32mp2 => "TIM2",
            Family::Imx9 => "LPIT1",
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
    ///
    /// A bare `#[interrupt]` item: the caller places it in the generated vector
    /// module, which supplies the `interrupt` import the attribute expands
    /// against and the `target_arch = "arm"` gate.
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
/// * `available` — the peripheral instances this core's chip map lists, or
///   `None` for a core with no bundled map. A listed map without the family's
///   timer suppresses all emission.
pub fn time_driver_codegen(
    hal_crate: &str,
    enabled: bool,
    dbg_present: bool,
    available: Option<&[String]>,
) -> TimeDriverCodegen {
    if !enabled {
        return TimeDriverCodegen::empty();
    }
    let Some(family) = Family::from_hal_crate(hal_crate) else {
        return TimeDriverCodegen::empty();
    };
    let timer = family.timer();
    if available.is_some_and(|names| !names.iter().any(|name| name == timer)) {
        return TimeDriverCodegen::empty();
    }
    let hal: syn::Path = parse_str(hal_crate).expect("hal_crate must be a valid path");

    // `bringup` consumes the timer's peripheral singleton, so the reservation
    // is enforced by the type system: nothing else can build a driver over it.
    let timer = quote::format_ident!("{timer}");
    let bringup = quote! {
        #hal::timer_driver::bringup(__p.#timer);
        ::consortium_runtime_mcu::log::trace!("consortium controller init: time driver up");
    };

    // cortex-m-rt's `#[interrupt]` names the line by its PAC `Interrupt`
    // variant, which is what puts the handler in that slot of the vector table.
    // The bare-metal gate and the `interrupt` import come from the vector module
    // the caller places this in.
    let isr = quote! {
        #[#hal::interrupt]
        fn #timer() {
            #hal::timer_driver::on_timer_interrupt();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn emits_when_the_core_reaches_the_timer() {
        let time = time_driver_codegen(
            "::consortium_hal_stm32mp2",
            true,
            true,
            Some(&names(&["IPCC1", "TIM2"])),
        );
        assert!(time.bringup.to_string().contains("bringup (__p . TIM2)"));
        assert!(time.isr.to_string().contains("fn TIM2 ()"));
        assert!(!time.timestamp.is_empty());
    }

    #[test]
    fn skips_a_core_without_the_timer() {
        // The STM32MP25 CM0+ reaches only the SmartRun domain.
        let time = time_driver_codegen(
            "::consortium_hal_stm32mp2",
            true,
            true,
            Some(&names(&["IPCC2", "LPTIM3", "LPTIM4", "LPTIM5"])),
        );
        assert!(time.bringup.is_empty());
        assert!(time.isr.is_empty());
        assert!(time.timestamp.is_empty());
    }

    #[test]
    fn keeps_the_default_for_a_core_with_no_chip_map() {
        // e.g. the i.MX95 CM33, which has no bundled map.
        let time = time_driver_codegen("::consortium_hal_imx9", true, false, None);
        assert!(time.bringup.to_string().contains("bringup (__p . LPIT1)"));
        assert!(time.timestamp.is_empty());
    }
}
