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
//! plus their supporting `static` interrupt state, `#[no_mangle]` interrupt
//! handlers, and NVIC enablement. The bases and interrupt numbers are resolved
//! from the per-core chip map ([`crate::chipmap`]); this component only shapes
//! them into Rust. All emitted items are gated to `target_arch = "arm"` because
//! they reference the chip HAL crate, which the host trybuild harness never
//! links.

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
    /// Physical base as seen by the owner, with the security alias applied.
    pub base: u32,
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
        let mut statics = Vec::new();
        let mut handlers = Vec::new();
        let mut nvic = Vec::new();
        let mut constructs = Vec::new();

        for instance in owned {
            let field = ident(&instance.key);
            field_names.push(field.clone());
            let base = instance.base;

            match &instance.kind {
                PeripheralKind::I2c {
                    clock_hz,
                    frequency,
                    irq_number,
                    irq_name,
                } => {
                    let state = ident(&format!("{}_STATE", instance.key.to_ascii_uppercase()));
                    let handler = ident(&format!("{irq_name}_IRQHandler"));

                    fields.push(quote! { pub #field: #hal::i2c::I2c });
                    statics.push(quote! {
                        #[cfg(target_arch = "arm")]
                        static #state: #hal::i2c::I2cState = #hal::i2c::I2cState::new();
                    });
                    handlers.push(quote! {
                        #[cfg(target_arch = "arm")]
                        #[unsafe(no_mangle)]
                        extern "C" fn #handler() {
                            #state.on_interrupt();
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
                        /// SAFETY: `base` is this core's validated I2C base from the
                        /// Consortium chip map, owned exclusively by this driver;
                        /// its dedicated state's interrupt is routed above.
                        let #field = unsafe {
                            #hal::i2c::I2c::new(#base as *mut (), #clock_hz, #frequency, &#state)
                        };
                    });
                }
                PeripheralKind::Gpio => {
                    fields.push(quote! { pub #field: #hal::gpio::Gpio });
                    constructs.push(quote! {
                        /// SAFETY: `base` is this core's validated GPIO port base from
                        /// the Consortium chip map, owned exclusively by this driver.
                        let #field = unsafe { #hal::gpio::Gpio::new(#base as *mut ()) };
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

            #(#statics)*
            #(#handlers)*

            /// Brings up this core's owned peripherals: enables their interrupts
            /// and constructs the HAL drivers with their bases baked in.
            ///
            /// Compiled only for Cortex-M targets; the host trybuild harness skips
            /// it along with the rest of the peripheral module.
            #[cfg(target_arch = "arm")]
            fn peripherals() -> Peripherals {
                #(#nvic)*
                #(#constructs)*
                Peripherals {
                    #(#field_names),*
                }
            }
        })
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
