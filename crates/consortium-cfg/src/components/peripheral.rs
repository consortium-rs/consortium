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

//! Peripheral bring-up code generation.
//!
//! Emits, for the peripherals a controller core owns, the HAL driver handles
//! plus their `#[no_mangle]` interrupt handlers and NVIC enablement.
//!
//! Drivers take the chip HAL's peripheral singletons rather than MMIO bases, so
//! the generated `peripherals` constructor is a plain function over `Peri`
//! arguments that `init` feeds from `hal::init()`. Bases no longer appear here
//! at all — the HAL's generated bindings own them — but the singleton *names*
//! and interrupt numbers still come from the per-core chip map
//! ([`crate::chipmap`]). All emitted items are gated to `target_arch = "arm"`
//! because they reference the chip HAL crate, which the host trybuild harness
//! never links.

use consortium_cfg_common::config::VisibleCore;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// A codegen-supported peripheral kind, resolved for one instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeripheralKind {
    /// I2C master: functional clock (Hz), bus frequency (Hz), and its interrupt.
    I2c {
        clock_hz: u32,
        frequency: u32,
        irq_number: u16,
        irq_name: String,
    },
    /// GPIO port (no interrupt bring-up yet).
    Gpio,
}

/// One owned peripheral instance to bring up on its core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeripheralInstance {
    /// Core that owns (and constructs) this peripheral.
    pub owner: VisibleCore,
    /// Config key, used verbatim as the handle field/ident (e.g. `lpi2c1`).
    pub key: String,
    /// Chip HAL peripheral singleton for this instance, spelled as the chip
    /// database does (e.g. `LPI2C1`). Names the type the driver is handed.
    pub singleton: String,
    /// The resolved driver kind and its parameters.
    pub kind: PeripheralKind,
}

/// Every owned peripheral instance across cores, plus the chip HAL crate path
/// the generated handles are typed against.
#[derive(Debug, Clone)]
pub struct Peripherals {
    /// Chip HAL crate path, e.g. `::consortium_hal_imx9`.
    pub hal_crate: String,
    /// Instances for every core; [`Peripherals::initialize`] filters by core.
    pub instances: Vec<PeripheralInstance>,
}

impl Peripherals {
    /// Emit the peripheral bring-up module for `core`, or `None` when that core
    /// owns no codegen-supported peripherals (so the generated file is
    /// unchanged for peripheral-free cores).
    pub fn initialize(&self, core: VisibleCore) -> Option<TokenStream> {
        let owned: Vec<&PeripheralInstance> =
            self.instances.iter().filter(|p| p.owner == core).collect();
        if owned.is_empty() {
            return None;
        }

        let hal: syn::Path =
            syn::parse_str(&self.hal_crate).expect("hal_crate must be a valid path");
        let mut fields = Vec::new();
        let mut field_names = Vec::new();
        let mut parameters = Vec::new();
        let mut handlers = Vec::new();
        let mut nvic = Vec::new();
        let mut constructs = Vec::new();

        for instance in owned {
            let field = ident(&instance.key);
            field_names.push(field.clone());
            let singleton = ident(&instance.singleton);
            parameters.push(quote! {
                #field: #hal::Peri<'static, #hal::peripherals::#singleton>
            });

            match &instance.kind {
                PeripheralKind::I2c {
                    clock_hz,
                    frequency,
                    irq_number,
                    irq_name,
                } => {
                    let handler = ident(&format!("{irq_name}_IRQHandler"));

                    fields.push(quote! { pub #field: #hal::i2c::I2c });
                    // The driver keeps one interrupt state per instance behind
                    // the singleton type, so the vector only names the instance.
                    handlers.push(quote! {
                        #[cfg(target_arch = "arm")]
                        #[unsafe(no_mangle)]
                        extern "C" fn #handler() {
                            #hal::i2c::on_interrupt::<#hal::peripherals::#singleton>();
                        }
                    });
                    nvic.push(quote! {
                        /// SAFETY: the interrupt number comes from the validated
                        /// Consortium chip map; a handler for it is installed above,
                        /// and bring-up runs once before other code masks interrupts.
                        unsafe {
                            #hal::hal::nvic::set_priority(#irq_number, 0x80u8);
                            #hal::hal::nvic::enable(#irq_number);
                        }
                    });
                    constructs.push(quote! {
                        let #field = #hal::i2c::I2c::new(#field, #clock_hz, #frequency);
                    });
                }
                PeripheralKind::Gpio => {
                    fields.push(quote! { pub #field: #hal::gpio::Gpio });
                    constructs.push(quote! {
                        let #field = #hal::gpio::Gpio::new(#field);
                    });
                }
            }
        }

        Some(quote! {
            /// Peripherals this core owns, constructed by [`peripherals`].
            #[cfg(target_arch = "arm")]
            pub struct Peripherals {
                #(#fields,)*
            }

            #(#handlers)*

            /// Brings up this core's owned peripherals: enables their interrupts
            /// and consumes each singleton into its HAL driver.
            ///
            /// Taking the singletons one by one rather than the whole
            /// `hal::Peripherals` lets `init` also hand fields to the time driver
            /// and the doorbells without fighting partial moves.
            ///
            /// Compiled only for Cortex-M targets; the host trybuild harness skips
            /// it along with the rest of the peripheral module.
            #[cfg(target_arch = "arm")]
            fn peripherals(#(#parameters),*) -> Peripherals {
                #(#nvic)*
                #(#constructs)*
                Peripherals {
                    #(#field_names),*
                }
            }
        })
    }

    /// The singleton idents `peripherals` takes on `core`, in parameter order,
    /// so `init` can spell the matching `hal::init()` fields.
    pub fn arguments(&self, core: VisibleCore) -> Vec<Ident> {
        self.instances
            .iter()
            .filter(|p| p.owner == core)
            .map(|p| ident(&p.singleton))
            .collect()
    }
}

/// Build a Rust identifier from a config key or symbol, sanitizing characters
/// the manifest allows but Rust does not.
fn ident(raw: &str) -> Ident {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() || out.starts_with(|ch: char| ch.is_ascii_digit()) {
        out.insert(0, '_');
    }
    Ident::new(&out, Span::call_site())
}
