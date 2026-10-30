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
use crate::components::ipc_shm::{DoorbellMeta, DoorbellRingerSide};
use crate::components::irq::InterruptLine;
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
    /// 1-based instance number (`mu7` is 7, `ipcc1` is 1). `None` for a block
    /// with a single instance, such as HSEM.
    pub id: Option<u8>,
    /// Doorbell channels used by this configuration.
    pub channels: Vec<u8>,
    /// Interrupt lines keyed by visible core.
    pub irq: HashMap<VisibleCore, InterruptLine>,
    /// Which IPC side each visible core drives this doorbell from. Decides the
    /// side marker the generated vector forwards with.
    pub side: HashMap<VisibleCore, DoorbellRingerSide>,
}

impl HardwareDoorbell {
    /// This doorbell's type and instance, as the IPC codegen spells them.
    fn meta(&self) -> DoorbellMeta {
        DoorbellMeta {
            r#type: self.r#type,
            id: self.id,
        }
    }
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
    /// Fills in each controller-core line's PAC `Interrupt` variant, so
    /// [`nvic_enable`](Self::nvic_enable) can name it rather than pass a bare
    /// number.
    ///
    /// `resolve` is the per-core lookup — in practice the bundled chip map — and
    /// returns `None` where no variant applies: a core with no bundled dump, or
    /// one the PAC's shipped interrupt table does not describe.
    pub fn resolve_variants(
        &mut self,
        mut resolve: impl FnMut(VisibleCore, u16) -> Option<String>,
    ) {
        for bell in &mut self.bells {
            for (core, irq) in &mut bell.irq {
                if !matches!(core, VisibleCore::Mcore(_)) {
                    continue;
                }
                irq.variant = u16::try_from(irq.number)
                    .ok()
                    .and_then(|number| resolve(*core, number));
            }
        }
    }

    /// The `#[interrupt]` vectors `core` needs for its doorbells, as bare items
    /// for the generated vector module.
    ///
    /// Each forwards to its doorbell crate's bare-metal service entry point,
    /// which masks the asserted lines and wakes the waiting futures. Firmware
    /// used to hand-write these in `main.rs`; every fact they need — the line's
    /// name, the instance, the side — is already in the manifest, and getting
    /// one wrong fails silently as a doorbell that never wakes.
    ///
    /// A doorbell whose line has no PAC `Interrupt` variant on this core is
    /// skipped: without a variant there is no name to hang the vector on (see
    /// [`InterruptLine`]), so that core keeps writing it by hand.
    pub fn vectors(&self, core: &VisibleCore, hal_crate: &str) -> Vec<TokenStream> {
        let hal: TokenStream = hal_crate
            .parse()
            .expect("hal crate path is a valid token stream");

        // Two channels may share one doorbell, and a shared line gets exactly
        // one vector; sort so the emitted order does not follow manifest order.
        let mut lines = self
            .bells
            .iter()
            .filter_map(|bell| {
                let irq = bell.irq.get(core)?;
                let variant = irq.variant.as_ref()?;
                let side = bell.side.get(core)?;
                Some((
                    variant.clone(),
                    bell.meta().service_call(*side),
                    irq.guard(&hal),
                ))
            })
            .collect::<Vec<_>>();
        lines.sort_by(|(left, ..), (right, ..)| left.cmp(right));
        lines.dedup_by(|(left, ..), (right, ..)| left == right);

        lines
            .into_iter()
            .map(|(variant, service, guard)| {
                let line = Ident::new(&variant, Span::call_site());
                quote! {
                    #guard
                    #[#hal::interrupt]
                    fn #line() {
                        #service
                    }
                }
            })
            .collect()
    }

    /// Generates Cortex-M NVIC enablement code for every doorbell interrupt
    /// visible to `core`.
    ///
    /// `hal_crate` is the chip HAL crate path the generated code calls into
    /// (e.g. `::consortium_hal_stm32mp2` or `::consortium_hal_imx9`), as
    /// selected by the lowering's chip-family split.
    ///
    /// Lines are driven through that crate's `nvic` helpers and named per
    /// [`InterruptLine::tokens`], so generated bring-up declares no interrupt
    /// enum of its own and never names `cortex_m`.
    ///
    /// Returns `None` when no configured doorbell has an IRQ for `core`.
    pub fn nvic_enable(&self, core: &VisibleCore, hal_crate: &str) -> Option<TokenStream> {
        let hal: TokenStream = hal_crate
            .parse()
            .expect("hal crate path is a valid token stream");

        let mut interrupts = self
            .bells
            .iter()
            .filter_map(|bell| bell.irq.get(core))
            .filter_map(|irq| irq.tokens(&hal).map(|named| (irq.number, named)))
            .collect::<Vec<_>>();

        if interrupts.is_empty() {
            return None;
        }

        interrupts.sort_by_key(|(number, _)| *number);
        interrupts.dedup_by_key(|(number, _)| *number);

        let statements = interrupts.iter().map(|(number, (line, label))| {
            // The PAC's `Interrupt` is a fieldless enum and `nvic::Irq` is
            // `Copy`, so the same expression can be spelled once per call.
            quote! {
                #hal::hal::nvic::unpend(#line);
                /// SAFETY: the interrupt comes from the validated Consortium
                /// hardware database for this visible core, and the application
                /// owns the matching vector.
                unsafe {
                    #hal::hal::nvic::set_priority(#line, 0x80u8);
                    #hal::hal::nvic::enable(#line);
                }
                ::consortium_runtime_mcu::log::trace!(
                    "consortium controller init: NVIC enabled {} at priority 0x80 (id={})",
                    #label,
                    #number
                );
            }
        });

        Some(quote! { #(#statements)* })
    }
}

impl ToUio for HardwareDoorbellPeripheral {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        // The UIO device is driven from Linux, so use that party's GIC SPI.
        // GIC SPI doorbell lines are level-high triggered.
        let linux = self
            .parties
            .iter()
            .find(|party| party.core == Party::AcoreNonSecure);
        let irq = linux.map(|party| (party.irq, IrqMethod::LevelHigh));

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
        // Map only the window Linux actually drives. An i.MX MU spans both
        // processor views, and mapping the far side into userspace hands the AP
        // registers belonging to the coprocessor; the vendor tree maps one
        // window too. Single-window blocks resolve to the whole thing, and
        // every party shares that base, so without a Linux party any one will do.
        let Some(party) = linux.or_else(|| self.parties.first()) else {
            return Ok(None);
        };
        let window = (party.memory, self.party_window_length());

        Ok(Some(UioDevice {
            mmap: vec![window],
            irq,
            backing: UioBacking::Registers,
            dma_coherent: Vec::new(),
            interrupt_parent: None,
            clock,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::McoreName;
    use consortium_cfg_common::database::{
        DoorbellWindow, HardwareDoorbellParty, HardwareDoorbellPeripheralType,
    };

    fn bell(name: &str, core: VisibleCore, irq: u32) -> HardwareDoorbell {
        typed_bell(
            name,
            core,
            irq,
            HardwareDoorbellPeripheralType::Ipcc,
            Some(1),
        )
    }

    fn typed_bell(
        name: &str,
        core: VisibleCore,
        irq: u32,
        r#type: HardwareDoorbellPeripheralType,
        id: Option<u8>,
    ) -> HardwareDoorbell {
        HardwareDoorbell {
            name: name.to_string(),
            address: Address {
                ap: crate::components::address::ApAddress {
                    uio_id: 0,
                    mmap_id: 0,
                },
                cp: HashMap::new(),
            },
            r#type,
            id,
            channels: vec![0],
            irq: HashMap::from([(core, InterruptLine::new(irq))]),
            side: HashMap::from([(core, DoorbellRingerSide::Secondary)]),
        }
    }

    #[test]
    fn generates_nvic_enablement_for_core_irqs() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let mut bella = BellA {
            bells: vec![bell("ipcc1", core, 171), bell("ipcc2", core, 175)],
        };
        bella.resolve_variants(|_, number| match number {
            171 => Some("IPCC1_RX".to_string()),
            175 => Some("IPCC2_RX".to_string()),
            _ => None,
        });

        let tokens = bella
            .nvic_enable(&core, "::consortium_hal_stm32mp2")
            .unwrap()
            .to_string();

        // The chip HAL re-exports the PAC's `Interrupt`, so generated bring-up
        // names the same variant as the vector table and declares no enum of
        // its own.
        assert!(!tokens.contains("BellaInterrupt"));
        assert!(!tokens.contains("cortex_m"));
        assert!(tokens.contains(":: consortium_hal_stm32mp2 :: Interrupt :: IPCC1_RX"));
        assert!(tokens.contains(":: consortium_hal_stm32mp2 :: Interrupt :: IPCC2_RX"));
        assert!(tokens.contains(
            ":: hal :: nvic :: set_priority (:: consortium_hal_stm32mp2 :: Interrupt :: IPCC1_RX , \
             0x80u8)"
        ));
    }

    #[test]
    fn generates_doorbell_vectors_forwarding_to_each_crate() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let mut bella = BellA {
            bells: vec![
                typed_bell(
                    "ipcc1",
                    core,
                    171,
                    HardwareDoorbellPeripheralType::Ipcc,
                    Some(1),
                ),
                typed_bell(
                    "ipcc2",
                    core,
                    175,
                    HardwareDoorbellPeripheralType::Ipcc,
                    Some(2),
                ),
            ],
        };
        bella.resolve_variants(|_, number| match number {
            171 => Some("IPCC1_RX".to_string()),
            175 => Some("IPCC2_RX".to_string()),
            _ => None,
        });

        let tokens = bella
            .vectors(&core, "::consortium_hal_stm32mp2")
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        assert_eq!(tokens.len(), 2);
        assert!(tokens[0].contains("[:: consortium_hal_stm32mp2 :: interrupt] fn IPCC1_RX ()"));
        // Each instance forwards with its own number, and no MMIO base appears
        // in generated code: the register model derives the window.
        assert!(tokens[0].contains(
            ":: consortium_ipc_transport_memory :: bell :: service_interrupt :: < :: \
             consortium_ipc_transport_memory :: bell :: ipcc :: Ipcc < :: consortium_ipc :: \
             Secondary , 1usize > > ()"
        ));
        assert!(tokens[1].contains("fn IPCC2_RX ()"));
        assert!(tokens[1].contains(
            ":: consortium_ipc_transport_memory :: bell :: ipcc :: Ipcc < :: consortium_ipc :: \
             Secondary , 2usize >"
        ));
    }

    /// HSEM has no instance number, so it services instance 1 — through the same
    /// generic entry as IPCC and MU, naming its bank so the vector can mask.
    #[test]
    fn generates_hsem_doorbell_vector_for_instance_one() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let mut bella = BellA {
            bells: vec![typed_bell(
                "hsem",
                core,
                199,
                HardwareDoorbellPeripheralType::Hsem,
                None,
            )],
        };
        bella.resolve_variants(|_, _| Some("HSEM".to_string()));

        let tokens = bella.vectors(&core, "::consortium_hal_stm32mp2")[0].to_string();

        assert!(tokens.contains("fn HSEM ()"));
        assert!(tokens.contains(
            ":: consortium_ipc_transport_memory :: bell :: service_interrupt :: < :: \
             consortium_ipc_transport_memory :: bell :: hsem :: Hsem < :: consortium_ipc :: \
             Secondary , 1usize > > ()"
        ));
    }

    #[test]
    fn generates_mu_doorbell_vector_with_its_instance() {
        let core = VisibleCore::Mcore(McoreName::Cm7);
        let mut bella = BellA {
            bells: vec![typed_bell(
                "mu7",
                core,
                207,
                HardwareDoorbellPeripheralType::Mu,
                Some(7),
            )],
        };
        bella.resolve_variants(|_, _| Some("MU7_B".to_string()));

        let tokens = bella.vectors(&core, "::consortium_hal_imx9")[0].to_string();

        assert!(tokens.contains("fn MU7_B ()"));
        assert!(tokens.contains(
            ":: consortium_ipc_transport_memory :: bell :: service_interrupt :: < :: \
             consortium_ipc_transport_memory :: bell :: mu :: Mu < :: consortium_ipc :: \
             Secondary , 7usize > > ()"
        ));
    }

    /// Two channels over one doorbell share a single line, so it gets one vector.
    #[test]
    fn deduplicates_a_shared_doorbell_line() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let mut bella = BellA {
            bells: vec![bell("ipcc1", core, 171), bell("ipcc1", core, 171)],
        };
        bella.resolve_variants(|_, _| Some("IPCC1_RX".to_string()));

        assert_eq!(bella.vectors(&core, "::consortium_hal_stm32mp2").len(), 1);
    }

    /// Without a PAC variant there is no name to hang `#[interrupt]` on, so the
    /// core keeps writing that vector by hand.
    #[test]
    fn emits_no_vector_without_a_pac_variant() {
        let core = VisibleCore::Mcore(McoreName::Cm0p);
        let bella = BellA {
            bells: vec![bell("ipcc2", core, 13)],
        };

        assert!(bella.vectors(&core, "::consortium_hal_stm32mp2").is_empty());
    }

    /// A core the PAC's shipped interrupt table does not describe (STM32MP2's
    /// CM0+) must not name an `Interrupt` variant: the same line name carries
    /// the CM33's number there.
    #[test]
    fn falls_back_to_the_raw_line_without_a_pac_variant() {
        let core = VisibleCore::Mcore(McoreName::Cm0p);
        let mut bella = BellA {
            bells: vec![bell("ipcc2", core, 13)],
        };
        bella.resolve_variants(|_, _| None);

        let tokens = bella
            .nvic_enable(&core, "::consortium_hal_stm32mp2")
            .unwrap()
            .to_string();

        assert!(!tokens.contains("Interrupt ::"));
        assert!(tokens.contains(
            ":: hal :: nvic :: enable (:: consortium_hal_stm32mp2 :: hal :: nvic :: Irq (13u16))"
        ));
    }

    #[test]
    fn resolve_variants_skips_the_ap() {
        let mut bella = BellA {
            bells: vec![bell("ipcc1", VisibleCore::Ap, 171)],
        };
        bella.resolve_variants(|_, _| Some("IPCC1_RX".to_string()));

        assert!(bella.bells[0].irq[&VisibleCore::Ap].variant.is_none());
    }

    #[test]
    fn skips_unavailable_core_irqs() {
        let cm33 = VisibleCore::Mcore(McoreName::Cm33);
        let cm7 = VisibleCore::Mcore(McoreName::Cm7);
        let bella = BellA {
            bells: vec![bell("ipcc1", cm33, 171)],
        };

        assert!(
            bella
                .nvic_enable(&cm7, "::consortium_hal_stm32mp2")
                .is_none()
        );
    }

    fn party(core: &str, irq: u32) -> HardwareDoorbellParty {
        HardwareDoorbellParty {
            core: core.parse().unwrap(),
            irq,
            window: DoorbellWindow::A,
            memory: 0x0049_0000,
            page_memory: 0,
            page_irq: 0,
        }
    }

    fn database_doorbell(parties: Vec<HardwareDoorbellParty>) -> HardwareDoorbellPeripheral {
        HardwareDoorbellPeripheral {
            name: "IPCC1".to_string(),
            page: 1305,
            r#type: HardwareDoorbellPeripheralType::Ipcc,
            length: 0x400,
            parties,
            party_page: 1305,
            channels: 32,
            channel_page: 1305,
            clock: None,
            window_length: None,
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

        assert_eq!(uio.mmap, vec![(0x0049_0000, 0x400)]);
        assert_eq!(uio.irq, Some((199, IrqMethod::LevelHigh)));
    }

    #[test]
    fn doorbell_uio_is_memory_only_without_linux_party() {
        let uio = database_doorbell(vec![party("cm33", 199), party("cm0p", 21)])
            .to_uio()
            .unwrap()
            .unwrap();

        assert_eq!(uio.mmap, vec![(0x0049_0000, 0x400)]);
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
