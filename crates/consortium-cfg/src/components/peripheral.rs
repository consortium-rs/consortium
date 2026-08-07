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
//! Emits the manifest-owned HAL singleton allowlist, configuration constants,
//! interrupt handlers, and NVIC enablement.
//!
//! Drivers take the chip HAL's peripheral singletons rather than MMIO bases, so
//! generated `init` moves only declared fields out of `hal::init()`. Bases no
//! longer appear here — the HAL's generated bindings own them — but singleton names
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
    /// A declared singleton with no Consortium-owned driver policy.
    Other,
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
        let hal: syn::Path =
            syn::parse_str(&self.hal_crate).expect("hal_crate must be a valid path");
        let mut fields = Vec::new();
        let mut handlers = Vec::new();
        let mut constants = Vec::new();

        for instance in owned {
            let singleton = ident(&instance.singleton);
            fields.push(quote! {
                pub #singleton: #hal::Peri<'static, #hal::peripherals::#singleton>
            });

            match &instance.kind {
                PeripheralKind::I2c {
                    clock_hz,
                    frequency,
                    irq_number,
                    irq_name,
                } => {
                    let handler = ident(&format!("{irq_name}_IRQHandler"));

                    let clock_const = ident(&format!("{}_CLOCK_HZ", instance.singleton));
                    let frequency_const = ident(&format!("{}_FREQUENCY", instance.singleton));
                    constants.push(quote! {
                        pub const #clock_const: u32 = #clock_hz;
                        pub const #frequency_const: u32 = #frequency;
                    });
                    // The driver keeps one interrupt state per instance behind
                    // the singleton type, so the vector only names the instance.
                    handlers.push(quote! {
                        #[cfg(target_arch = "arm")]
                        #[unsafe(no_mangle)]
                        extern "C" fn #handler() {
                            #hal::i2c::on_interrupt::<#hal::peripherals::#singleton>();
                        }
                    });
                    let _ = irq_number;
                }
                PeripheralKind::Gpio | PeripheralKind::Other => {}
            }
        }

        Some(quote! {
            /// Peripheral singletons this endpoint is allowed to construct.
            ///
            /// This type deliberately has no `Drop` implementation so fields may
            /// be moved into drivers independently.
            #[cfg(target_arch = "arm")]
            #[allow(non_snake_case)]
            pub struct Peripherals {
                #(#fields,)*
            }

            #(#constants)*
            #(#handlers)*
        })
    }

    /// NVIC priority and enable statements for generated peripheral handlers.
    pub fn nvic(&self, core: VisibleCore) -> TokenStream {
        let hal: syn::Path =
            syn::parse_str(&self.hal_crate).expect("hal_crate must be a valid path");
        let statements = self.instances.iter().filter_map(|instance| {
            if instance.owner != core {
                return None;
            }
            let PeripheralKind::I2c { irq_number, .. } = instance.kind else {
                return None;
            };
            Some(quote! {
                /// SAFETY: the interrupt number comes from the validated chip
                /// map and its generated handler is installed above.
                unsafe {
                    #hal::hal::nvic::set_priority(#irq_number, 0x80u8);
                    #hal::hal::nvic::enable(#irq_number);
                }
            })
        });
        quote! { #(#statements)* }
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
