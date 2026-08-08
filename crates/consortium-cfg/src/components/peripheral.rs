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
//! interrupt vectors ([`Peripherals::vectors`]), and NVIC enablement.
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

use crate::components::irq::InterruptLine;

/// A codegen-supported peripheral kind, resolved for one instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeripheralKind {
    /// I2C master: functional clock (Hz), bus frequency (Hz), and its interrupt
    /// line.
    ///
    /// The line is the single source for both the generated `#[interrupt]`
    /// vector's name and the NVIC enable. Keeping a second, ungated copy of the
    /// name here is what let the two disagree: the vector would be spelled from
    /// the chip map while the enable came from the PAC-gated
    /// [`InterruptLine`], so a core the PAC does not describe got a handler in
    /// the wrong table slot.
    I2c {
        clock_hz: u32,
        frequency: u32,
        irq: InterruptLine,
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
                    ..
                } => {
                    let clock_const = ident(&format!("{}_CLOCK_HZ", instance.singleton));
                    let frequency_const = ident(&format!("{}_FREQUENCY", instance.singleton));
                    constants.push(quote! {
                        pub const #clock_const: u32 = #clock_hz;
                        pub const #frequency_const: u32 = #frequency;
                    });
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
        })
    }

    /// The interrupt vectors `core`'s owned peripherals need, as bare
    /// `#[interrupt]` items for the generated vector module.
    ///
    /// The handler is named for the line's PAC `Interrupt` variant, which is
    /// what places it in that slot of the vector table. The driver keeps one
    /// interrupt state per instance behind the singleton type, so the body only
    /// names the instance.
    ///
    /// A core the PAC's shipped interrupt table does not describe is skipped,
    /// exactly as [`BellA::vectors`](crate::components::bella::BellA::vectors)
    /// skips its doorbells. Emitting the vector there would be worse than
    /// emitting nothing: `#[interrupt] fn I2C8()` resolves through the *covered*
    /// core's table (212 on STM32MP2's CM33) while [`nvic`](Self::nvic) enables
    /// this core's raw line (15 on its CM0+), so the enabled interrupt would
    /// dispatch somewhere else entirely and the handler would never run. Such a
    /// core keeps writing its vectors by hand.
    pub fn vectors(&self, core: VisibleCore) -> Vec<TokenStream> {
        let hal: syn::Path =
            syn::parse_str(&self.hal_crate).expect("hal_crate must be a valid path");
        let hal_tokens: TokenStream = self
            .hal_crate
            .parse()
            .expect("hal crate path is a valid token stream");
        self.instances
            .iter()
            .filter(|instance| instance.owner == core)
            .filter_map(|instance| {
                let PeripheralKind::I2c { irq, .. } = &instance.kind else {
                    return None;
                };
                let line = ident(irq.variant.as_ref()?);
                let singleton = ident(&instance.singleton);
                let guard = irq.guard(&hal_tokens);
                Some(quote! {
                    #guard
                    #[#hal::interrupt]
                    fn #line() {
                        #hal::i2c::on_interrupt::<#hal::peripherals::#singleton>();
                    }
                })
            })
            .collect()
    }

    /// NVIC priority and enable statements for generated peripheral handlers.
    ///
    /// The line is named per [`InterruptLine::tokens`]: the chip HAL's
    /// re-exported PAC `Interrupt` variant where that describes this core, and
    /// the HAL's raw-number carrier otherwise.
    pub fn nvic(&self, core: VisibleCore) -> TokenStream {
        let hal: TokenStream = self
            .hal_crate
            .parse()
            .expect("hal crate path is a valid token stream");
        let statements = self.instances.iter().filter_map(|instance| {
            if instance.owner != core {
                return None;
            }
            let PeripheralKind::I2c { irq, .. } = &instance.kind else {
                return None;
            };
            let (line, _) = irq.tokens(&hal)?;
            Some(quote! {
                /// SAFETY: the interrupt comes from the validated chip map and
                /// its generated handler is installed above.
                unsafe {
                    #hal::hal::nvic::set_priority(#line, 0x80u8);
                    #hal::hal::nvic::enable(#line);
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

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::McoreName;

    fn cm33() -> VisibleCore {
        VisibleCore::Mcore(McoreName::Cm33)
    }

    fn cm0p() -> VisibleCore {
        VisibleCore::Mcore(McoreName::Cm0p)
    }

    /// One owned I2C whose line may or may not be named by the PAC.
    fn peripherals(owner: VisibleCore, irq: InterruptLine) -> Peripherals {
        Peripherals {
            hal_crate: "::consortium_hal_stm32mp2".to_string(),
            instances: vec![PeripheralInstance {
                owner,
                key: "i2c8".to_string(),
                singleton: "I2C8".to_string(),
                kind: PeripheralKind::I2c {
                    clock_hz: 24_000_000,
                    frequency: 100_000,
                    irq,
                },
            }],
        }
    }

    #[test]
    fn emits_a_guarded_vector_for_a_pac_covered_core() {
        let line = InterruptLine {
            number: 212,
            variant: Some("I2C8".to_string()),
        };
        let vectors = peripherals(cm33(), line).vectors(cm33());

        assert_eq!(vectors.len(), 1);
        let tokens = vectors[0].to_string();
        assert!(tokens.contains("[:: consortium_hal_stm32mp2 :: interrupt] fn I2C8 ()"));
        // The guard ties the variant to the number the chip map gave it.
        assert!(
            tokens.contains(":: consortium_hal_stm32mp2 :: Interrupt :: I2C8 as u16 == 212u16")
        );
    }

    /// STM32MP2's CM0+ is not described by the PAC's interrupt table: `I2C8` is
    /// line 15 there and 212 on the CM33 the PAC ships. Emitting
    /// `#[interrupt] fn I2C8()` would install the handler in the CM33's slot
    /// while `nvic` enabled the CM0+ line, so the interrupt would be enabled and
    /// never delivered. No vector may be generated; the firmware installs it.
    #[test]
    fn emits_no_vector_for_a_core_the_pac_does_not_describe() {
        let ir = peripherals(cm0p(), InterruptLine::new(15));

        assert!(ir.vectors(cm0p()).is_empty());
        // The NVIC enable still happens, through the raw-number carrier.
        let nvic = ir.nvic(cm0p()).to_string();
        assert!(nvic.contains("hal :: nvic :: enable"), "{nvic}");
        assert!(nvic.contains("Irq (15u16)"), "{nvic}");
        assert!(!nvic.contains("Interrupt ::"), "{nvic}");
    }

    #[test]
    fn vectors_are_scoped_to_the_owning_core() {
        let line = InterruptLine {
            number: 212,
            variant: Some("I2C8".to_string()),
        };
        assert!(peripherals(cm33(), line).vectors(cm0p()).is_empty());
    }
}
