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

//! Type-A doorbell interrupt enablement code generation.

use std::collections::HashMap;

use consortium_cfg_common::{
    config::{Party, VisibleCore},
    database::{HardwareDoorbellPeripheral, HardwareDoorbellPeripheralType},
};
use consortium_dts::utils::irq_method::IrqMethod;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::components::address::Address;
use crate::components::uio::{ToUio, UioBacking, UioClock, UioDevice};
use crate::error::CfgError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareDoorbell {
    /// Peripheral name.
    pub name: String,
    /// Addressing/location information for the doorbell.
    pub address: Address,
    /// Type of the doorbell peripheral.
    pub r#type: HardwareDoorbellPeripheralType,
    /// Doorbell channels used by this configuration.
    pub channels: Vec<u8>,
    /// Interrupt numbers keyed by visible core.
    pub irq: HashMap<VisibleCore, u32>,
}

/// A collection of doorbells visible to a given core.
///
/// Call the doorbell type `Bella` because we classify it as a type-A doorbell. It is
/// also someone significant to the author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BellA {
    pub bells: Vec<HardwareDoorbell>,
}

impl BellA {
    /// Generates Cortex-M NVIC enablement code for every doorbell interrupt
    /// visible to `core`.
    ///
    /// `hal_crate` is the chip HAL crate path the generated code calls into
    /// (e.g. `::consortium_hal_stm32mp2` or `::consortium_hal_imx9`), as
    /// selected by the lowering's chip-family split.
    ///
    /// Returns `None` when no configured doorbell has an IRQ for `core`.
    pub fn nvic_enable(&self, core: &VisibleCore, hal_crate: &str) -> Option<TokenStream> {
        let mut interrupts = self
            .bells
            .iter()
            .filter_map(|bell| {
                bell.irq
                    .get(core)
                    .and_then(|irq| u16::try_from(*irq).ok())
                    .map(|irq| (interrupt_variant(&bell.name), irq))
            })
            .collect::<Vec<_>>();

        if interrupts.is_empty() {
            return None;
        }

        interrupts.sort_by_key(|(variant, _)| variant.to_string());
        interrupts.dedup_by_key(|(variant, _)| variant.to_string());

        let variants = interrupts
            .iter()
            .map(|(variant, irq)| quote! { #variant = #irq })
            .collect::<Vec<_>>();
        let variant_names = interrupts
            .iter()
            .map(|(variant, _)| variant)
            .collect::<Vec<_>>();

        let hal_crate: TokenStream = hal_crate
            .parse()
            .expect("hal crate path is a valid token stream");

        Some(quote! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            #[repr(u16)]
            pub enum BellaInterrupt {
                #(#variants,)*
            }

            unsafe impl ::cortex_m::interrupt::InterruptNumber for BellaInterrupt {
                fn number(self) -> u16 {
                    self as u16
                }
            }

            #(
                /// SAFETY: interrupt numbers are emitted from the validated
                /// Consortium hardware database for this visible core.
                unsafe {
                    #hal_crate::hal::nvic::set_priority(BellaInterrupt::#variant_names as u16, 0x80);
                }
                ::consortium_runtime_mcu::log::trace!("consortium controller init: NVIC set priority to 0x80 {} (id={})", stringify!(#variant_names), BellaInterrupt::#variant_names as u32);
                ::cortex_m::peripheral::NVIC::unpend(BellaInterrupt::#variant_names);
                ::consortium_runtime_mcu::log::trace!("consortium controller init: NVIC unpend {} (id={})", stringify!(#variant_names), BellaInterrupt::#variant_names as u32);
                /// SAFETY: interrupt numbers are emitted from the validated
                /// Consortium hardware database for this visible core.
                unsafe {
                    ::cortex_m::peripheral::NVIC::unmask(BellaInterrupt::#variant_names);
                }
                ::consortium_runtime_mcu::log::trace!("consortium controller init: NVIC unmask {} (id={})", stringify!(#variant_names), BellaInterrupt::#variant_names as u32);
            )*
        })
    }
}

impl ToUio for HardwareDoorbellPeripheral {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        // The UIO device is driven from Linux, so use that party's GIC SPI.
        // GIC SPI doorbell lines are level-high triggered.
        let irq = self
            .parties
            .iter()
            .find(|party| party.core == Party::AcoreNonSecure)
            .map(|party| (party.irq, IrqMethod::LevelHigh));

        // Carry the doorbell's Linux-side clock (if the chip DB declares one) so
        // the generic-uio node keeps its register block clocked. The
        // `interrupt-parent` label is chip-level, not doorbell-level, so the
        // bridge fills it in from the resolved chip database.
        let clock = self.clock.as_ref().map(|clock| UioClock {
            provider: clock.provider.clone(),
            id: clock.id.clone(),
        });

        // A doorbell always exposes its single register window as `map0`, so
        // there is no `mmap` ordering to violate. These are MMIO peripheral
        // registers, not RAM, so they are described inline with `reg`.
        Ok(Some(UioDevice {
            mmap: vec![(self.memory.base, self.memory.length)],
            irq,
            backing: UioBacking::Registers,
            interrupt_parent: None,
            clock,
        }))
    }
}

fn interrupt_variant(name: &str) -> Ident {
    let mut ident = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            ident.push(ch.to_ascii_uppercase());
        } else {
            ident.push('_');
        }
    }

    if ident.is_empty() || ident.starts_with(|ch: char| ch.is_ascii_digit()) {
        ident.insert(0, '_');
    }

    Ident::new(&ident, Span::call_site())
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::McoreName;
    use consortium_cfg_common::database::{
        HardwareDoorbellParty, HardwareDoorbellPeripheralType, HardwareMemory,
    };

    fn bell(name: &str, core: VisibleCore, irq: u32) -> HardwareDoorbell {
        HardwareDoorbell {
            name: name.to_string(),
            address: Address {
                ap: crate::components::address::ApAddress {
                    uio_id: 0,
                    mmap_id: 0,
                },
                cp: HashMap::new(),
            },
            r#type: HardwareDoorbellPeripheralType::Hsem,
            channels: vec![0],
            irq: HashMap::from([(core, irq)]),
        }
    }

    #[test]
    fn generates_nvic_enablement_for_core_irqs() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let bella = BellA {
            bells: vec![bell("hsem", core, 199), bell("ipcc2", core, 13)],
        };

        let tokens = bella
            .nvic_enable(&core, "::consortium_hal_stm32mp2")
            .unwrap()
            .to_string();

        assert!(tokens.contains("enum BellaInterrupt"));
        assert!(tokens.contains("HSEM = 199u16"));
        assert!(tokens.contains("IPCC2 = 13u16"));
        assert!(tokens.contains("unsafe impl :: cortex_m :: interrupt :: InterruptNumber"));
        assert!(tokens.contains("NVIC :: unmask"));
    }

    #[test]
    fn skips_unavailable_core_irqs() {
        let cm33 = VisibleCore::Mcore(McoreName::Cm33);
        let cm7 = VisibleCore::Mcore(McoreName::Cm7);
        let bella = BellA {
            bells: vec![bell("hsem", cm33, 199)],
        };

        assert!(
            bella
                .nvic_enable(&cm7, "::consortium_hal_stm32mp2")
                .is_none()
        );
    }

    #[test]
    fn sanitizes_interrupt_variant_names() {
        assert_eq!(interrupt_variant("mu-7").to_string(), "MU_7");
        assert_eq!(interrupt_variant("1bad").to_string(), "_1BAD");
    }

    fn party(core: &str, irq: u32) -> HardwareDoorbellParty {
        HardwareDoorbellParty {
            core: core.parse().unwrap(),
            irq,
            page_memory: 0,
            page_irq: 0,
        }
    }

    fn database_doorbell(parties: Vec<HardwareDoorbellParty>) -> HardwareDoorbellPeripheral {
        HardwareDoorbellPeripheral {
            name: "HSEM".to_string(),
            page: 1306,
            r#type: HardwareDoorbellPeripheralType::Hsem,
            memory: HardwareMemory {
                base: 0x0624_0000,
                length: 0x400,
            },
            parties,
            party_page: 1306,
            channels: 16,
            channel_page: 1306,
            clock: None,
        }
    }

    #[test]
    fn doorbell_uio_uses_linux_party_interrupt() {
        let uio = database_doorbell(vec![
            party("optee", 200),
            party("linux", 199),
            party("cm33", 199),
        ])
        .to_uio()
        .unwrap()
        .unwrap();

        assert_eq!(uio.mmap, vec![(0x0624_0000, 0x400)]);
        assert_eq!(uio.irq, Some((199, IrqMethod::LevelHigh)));
    }

    #[test]
    fn doorbell_uio_is_memory_only_without_linux_party() {
        let uio = database_doorbell(vec![party("cm33", 199), party("cm0p", 21)])
            .to_uio()
            .unwrap()
            .unwrap();

        assert_eq!(uio.mmap, vec![(0x0624_0000, 0x400)]);
        assert!(uio.irq.is_none());
    }
}

// /// EASTER EGG. DO NOT DELETE
// ///
// /// # Safety
// ///
// /// DO NOT implement this trait unless you know what you are doing.
// #[allow(unused)]
// unsafe trait EthaN: Sized {}

// /// SAFETY: Ethan knows what he is doing.
// unsafe impl EthaN for BellA {}
