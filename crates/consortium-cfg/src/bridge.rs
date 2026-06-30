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

//! Lowers a validated [`Session`] into the `components` IR.
//!
//! The IR types in [`crate::components`] each own their own code generation
//! ([`Transceiver::initialize`](crate::components::ipc_shm::Transceiver::initialize),
//! [`BellA::nvic_enable`](crate::components::bella::BellA::nvic_enable),
//! [`ToUio::to_uio`]). This module builds those structs from the parsed
//! `Consortium.toml` plus its resolved chip database, so the builder can drive
//! codegen without hand-assembling addresses, doorbell metadata, and UIO ids.
//!
//! A channel names its transport primary and secondary parties explicitly
//! (`primary = ...` / `secondary = ...`); each must have a declared endpoint.
//!
//! The AP-side address layout is fixed here so the firmware-, NVIC-, and
//! Linux-DTS outputs agree on UIO ids and mmap indices:
//!
//! - `uio0` aggregates every IPC shared-memory region; a region's `mmap_id` is
//!   its index in channel-declaration order.
//! - `uio1..=N` is one device per distinct doorbell, in first-appearance order;
//!   a doorbell always exposes its register window as `mmap_id` 0.

use std::collections::HashMap;

use consortium_cfg_common::config::{MemoryRegion, Party, VisibleCore};
use consortium_cfg_common::database::{ChipDatabase, HardwareDoorbellPeripheral, MemoryRegionType};

use crate::components::address::{Address, ApAddress};
use crate::components::bella::{BellA, HardwareDoorbell};
use crate::components::dbg::{DebugLogs, DebugRing};
use crate::components::ipc_shm::{Channel, DoorbellMeta, ShuHan, Transceiver};
use crate::components::region::Region;
use crate::components::uio::UioDevice;
use crate::transpilers::ToUio;
use crate::{Session, error::CfgError};

/// The `components` IR lowered from a [`Session`].
///
/// Built once per session and shared across target cores; the codegen entry
/// points ([`ShuHan::initialize`], [`BellA::nvic_enable`]) select the relevant
/// core at emit time.
#[derive(Debug, Clone)]
pub struct LoweredIr {
    /// Every IPC transceiver, one per `[[ipc.channel]]`.
    pub shuhan: ShuHan,
    /// Every distinct hardware doorbell, carrying per-core IRQ maps.
    pub bella: BellA,
    /// Linux UIO devices, named for their DTS node. Index 0 is the aggregate
    /// shared-memory device (`ipc-shm`); the rest are one per doorbell.
    pub uio: Vec<(String, UioDevice)>,
    /// Per-coprocessor debug log rings. Generates no Rust; resolves each
    /// logger's `CONSORTIUM_DBG_*` build-time configuration for the builder.
    pub dbg: DebugLogs,
}

impl LoweredIr {
    /// Generates the per-core IPC initialization module.
    ///
    /// Every core gets the `IpcMemoryEndpoints` struct and its `new`
    /// constructor, restricted to the transceivers that core is a party to
    /// (controller firmware images are per-core).
    ///
    /// The AP additionally brings up the Linux [`UioDevice`]s this IR declares
    /// and wraps both in an `init` entry point returning
    /// `ConsortiumEndpoints { uio, ipc_shm }`. The opened devices are kept in
    /// `uio` because `ipc_shm` borrows their mmap windows for the lifetime of
    /// the endpoints.
    ///
    /// [`UioDevice`]: crate::components::uio::UioDevice
    pub fn initialize(&self, core: VisibleCore, ty_crate: &syn::Path) -> proc_macro2::TokenStream {
        use quote::{format_ident, quote};

        let uio_ident = format_ident!("uio");

        // A core only constructs the transceivers it participates in.
        let participating: Vec<_> = self
            .shuhan
            .transceivers()
            .iter()
            .filter(|t| t.channel_primary.0 == core || t.channel_secondary.0 == core)
            .cloned()
            .collect();
        let endpoints = ShuHan::new(participating).initialize(core, &uio_ident, ty_crate);

        if core != VisibleCore::Ap {
            return endpoints;
        }

        // `UioDevice::open(n)` for each declared device, in `self.uio` order
        // (shared-memory device first, then one per doorbell, then `dbg-shm`).
        // The opened handles are moved into `ConsortiumEndpoints` so the mmap
        // windows `IpcMemoryEndpointsConnected` borrows stay alive.
        let opens = (0..self.uio.len()).map(|id| {
            quote! { ::consortium_runtime_app::uio::UioDevice::open(#id)? }
        });

        // The AP host console for the coprocessor debug rings, if any. Its field
        // is declared *before* `uio` so its threads/handles drop (and stop
        // touching the mappings) before the `UioDevice`s unmap them.
        let host = self.dbg.initialize_host(&uio_ident);
        let (dbg_field, dbg_field_name, dbg_init, dbg_error_arm) = match &host {
            Some(host) => {
                let field = &host.field;
                let name = &host.field_name;
                let init = &host.init;
                (
                    quote! { #field, },
                    quote! { #name, },
                    quote! { #init },
                    quote! {
                        Dbg(::consortium_runtime_app::dbg_console::DbgConsoleError),
                    },
                )
            }
            None => (quote! {}, quote! {}, quote! {}, quote! {}),
        };
        let dbg_from = if host.is_some() {
            quote! {
                impl ::core::convert::From<::consortium_runtime_app::dbg_console::DbgConsoleError>
                    for ConsortiumInitError
                {
                    fn from(e: ::consortium_runtime_app::dbg_console::DbgConsoleError) -> Self {
                        ConsortiumInitError::Dbg(e)
                    }
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #endpoints

            /// Aggregate bring-up error for [`init`].
            #[derive(Debug)]
            enum ConsortiumInitError {
                /// Opening or mapping a UIO device failed.
                Uio(::consortium_runtime_app::uio::UioDeviceError),
                /// An IPC transceiver's descriptor handshake failed.
                Ipc(ConsortiumConnectError),
                #dbg_error_arm
            }

            impl ::core::convert::From<::consortium_runtime_app::uio::UioDeviceError>
                for ConsortiumInitError
            {
                fn from(e: ::consortium_runtime_app::uio::UioDeviceError) -> Self {
                    ConsortiumInitError::Uio(e)
                }
            }

            impl ::core::convert::From<ConsortiumConnectError> for ConsortiumInitError {
                fn from(e: ConsortiumConnectError) -> Self {
                    ConsortiumInitError::Ipc(e)
                }
            }

            #dbg_from

            struct ConsortiumEndpoints {
                #dbg_field
                #uio_ident: ::std::vec::Vec<::consortium_runtime_app::uio::UioDevice>,
                ipc_shm: IpcMemoryEndpointsConnected,
            }

            async fn init() -> ::core::result::Result<ConsortiumEndpoints, ConsortiumInitError> {
                let #uio_ident = ::std::vec![ #(#opens),* ];
                let ipc_shm = IpcMemoryEndpoints::new(&#uio_ident)?.connect().await?;
                #dbg_init
                ::core::result::Result::Ok(ConsortiumEndpoints { #dbg_field_name #uio_ident, ipc_shm })
            }
        }
    }
}

impl Session {
    /// Lowers the validated manifest into the [`components`](crate::components)
    /// IR.
    ///
    /// Assumes the session is already validated by [`Session::from_toml`]:
    /// doorbells exist, parties are visible, channels are in bounds, and
    /// regions fit SRAM. Only topology the validator does not constrain is
    /// rejected here.
    ///
    /// # Errors
    ///
    /// Returns [`CfgError::Manifest`] when a channel's `primary`/`secondary`
    /// party has no declared endpoint, when its doorbell name cannot be typed,
    /// or when a memory-pattern channel has no `config.memory`.
    pub fn lower(&self) -> Result<LoweredIr, CfgError> {
        let mut transceivers = Vec::new();
        let mut regions: Vec<MemoryRegion> = Vec::new();

        // Doorbells in first-appearance order; their position fixes the `uioN`
        // id (offset by 1 for the shared-memory device at `uio0`).
        let mut doorbell_order: Vec<String> = Vec::new();
        let mut bells: Vec<HardwareDoorbell> = Vec::new();

        for channel in &self.config.ipc.channels {
            let primary_party = channel.primary;
            let secondary_party = channel.secondary;
            let primary = endpoint(channel, primary_party, "primary")?;
            let secondary = endpoint(channel, secondary_party, "secondary")?;
            let primary_core = primary_party.core();
            let secondary_core = secondary_party.core();

            let meta: DoorbellMeta = channel
                .doorbell
                .parse()
                .map_err(|err: CfgError| CfgError::Manifest(err.to_string()))?;
            let db_doorbell = self.doorbell(&channel.doorbell).ok_or_else(|| {
                CfgError::Manifest(format!("unknown doorbell '{}'", channel.doorbell))
            })?;

            let memory = channel.memory().ok_or_else(|| {
                CfgError::Manifest(format!(
                    "ipc channel '{}' uses the memory pattern but sets no config.memory",
                    channel.name
                ))
            })?;

            // Region lives in `uio0`; its mmap slot is its declaration order.
            let region_mmap = regions.len();
            regions.push(memory);

            // Reserve / reuse this doorbell's `uioN` device.
            let doorbell_key = channel.doorbell.to_ascii_lowercase();
            let doorbell_uio = match doorbell_order.iter().position(|n| *n == doorbell_key) {
                Some(index) => index + 1,
                None => {
                    doorbell_order.push(doorbell_key.clone());
                    doorbell_order.len()
                }
            };

            // The AP resolves addresses through UIO; controller cores embed the
            // physical base. Each base is the chip's canonical offset; the real
            // address adds the security-domain alias for the accessing party
            // (e.g. STM32MP2 maps peripherals non-secure at 0x4000_0000 and
            // secure at 0x5000_0000). Record both participants that are
            // controllers, aliased to that party's world.
            let region_cp =
                controller_bases([primary_party, secondary_party], memory.base, |secure| {
                    memory_alias(&self.database, secure)
                });
            let doorbell_cp = controller_bases(
                [primary_party, secondary_party],
                db_doorbell.memory.base,
                |secure| peripheral_alias(&self.database, secure),
            );

            let region_address = Address {
                ap: ApAddress {
                    uio_id: 0,
                    mmap_id: region_mmap,
                },
                cp: region_cp,
            };
            let doorbell_address = Address {
                ap: ApAddress {
                    uio_id: doorbell_uio,
                    mmap_id: 0,
                },
                cp: doorbell_cp,
            };

            let channel_primary = Channel {
                doorbell_chan: primary.chan as u8,
                type_name: primary.r#type.clone(),
                buffer_name: format!("{}_tx_buf", channel.name),
            };
            let channel_secondary = Channel {
                doorbell_chan: secondary.chan as u8,
                type_name: secondary.r#type.clone(),
                buffer_name: format!("{}_rx_buf", channel.name),
            };

            // Record this doorbell's used channels and the IRQ of each
            // participating party (keyed by core) for NVIC enablement. Only the
            // parties on this channel are recorded, so the secure/non-secure
            // IRQ variant is the one this channel actually uses.
            record_bell(
                &mut bells,
                &doorbell_key,
                &doorbell_address,
                db_doorbell.r#type,
                [
                    (
                        primary_core,
                        party_irq(db_doorbell, primary_party),
                        primary.chan as u8,
                    ),
                    (
                        secondary_core,
                        party_irq(db_doorbell, secondary_party),
                        secondary.chan as u8,
                    ),
                ],
            );

            transceivers.push(Transceiver {
                name: channel.name.clone(),
                doorbell_address,
                mtu: channel.mtu,
                codec: channel.codec,
                region: Region {
                    r#type: MemoryRegionType::Sram,
                    address: region_address,
                    length: memory.length,
                },
                channel_primary: (primary_core, channel_primary),
                channel_secondary: (secondary_core, channel_secondary),
                doorbell: meta,
            });
        }

        // Assemble the UIO device list in the order the addresses reference it:
        // shared memory first, then each doorbell. Linux is non-secure, so every
        // mapped window is offset by the non-secure alias.
        let mut uio = Vec::new();
        let region_alias = memory_alias(&self.database, false);
        if let Some(mut device) = regions.as_slice().to_uio()? {
            for window in &mut device.mmap {
                window.0 += region_alias;
            }
            uio.push(("ipc-shm".to_string(), device));
        }
        let doorbell_alias = peripheral_alias(&self.database, false);
        for name in &doorbell_order {
            let db_doorbell = self
                .doorbell(name)
                .ok_or_else(|| CfgError::Manifest(format!("unknown doorbell '{name}'")))?;
            if let Some(mut device) = db_doorbell.to_uio()? {
                for window in &mut device.mmap {
                    window.0 += doorbell_alias;
                }
                uio.push((name.clone(), device));
            }
        }

        // Debug log rings carry no security world (the table is keyed by a bare
        // core), so each logger embeds its ring through the non-secure SRAM
        // alias, matching both the firmware's own view of memory and Linux's
        // (Linux is non-secure too, so the AP maps the same physical address).
        let dbg_alias = memory_alias(&self.database, false);

        // The AP maps every dbg ring through one aggregate `dbg-shm` UIO device,
        // appended after the IPC/doorbell devices. Sort by core name so the mmap
        // window order is deterministic and agrees with the host codegen, which
        // sorts the same way.
        let dbg_uio_id = uio.len();
        let mut dbg_cores: Vec<_> = self.config.dbg.cores.iter().collect();
        dbg_cores.sort_by_key(|(core, _)| core.to_string());

        let mut dbg_windows = Vec::new();
        let mut dbg_rings = HashMap::new();
        for (mmap_id, (core, entry)) in dbg_cores.into_iter().enumerate() {
            let base = u64::from(entry.memory.base) + u64::from(dbg_alias);
            dbg_windows.push((base as u32, entry.memory.length));
            dbg_rings.insert(
                *core,
                DebugRing {
                    base,
                    length: u64::from(entry.memory.length),
                    mtu: entry.mtu,
                    ap_uio_id: dbg_uio_id,
                    ap_mmap_id: mmap_id,
                    elf_path: self.endpoint_artifact(*core).unwrap_or_default(),
                },
            );
        }
        if !dbg_windows.is_empty() {
            uio.push((
                "dbg-shm".to_string(),
                UioDevice {
                    mmap: dbg_windows,
                    irq: None,
                },
            ));
        }

        Ok(LoweredIr {
            shuhan: ShuHan::new(transceivers),
            bella: BellA { bells },
            uio,
            dbg: DebugLogs::new(dbg_rings, self.config.dbg.mode),
        })
    }

    /// The `[endpoints]` artifact root declared for `core`, used by the host as
    /// the defmt symbol source for that coprocessor's debug ring.
    ///
    /// A core may correspond to a secure and a non-secure party; the ring is per
    /// core, so the first endpoint whose party projects to `core` is used.
    fn endpoint_artifact(&self, core: VisibleCore) -> Option<String> {
        self.config
            .endpoints
            .iter()
            .find(|(party, _)| party.core() == core)
            .map(|(_, root)| root.clone())
    }
}

/// Resolves the endpoint a channel declares for `party`, named by `role` for
/// the error message.
fn endpoint<'c>(
    channel: &'c consortium_cfg_common::config::IpcChannel,
    party: Party,
    role: &str,
) -> Result<&'c consortium_cfg_common::config::IpcEndpoint, CfgError> {
    channel.endpoints.get(&party).ok_or_else(|| {
        CfgError::Manifest(format!(
            "ipc channel '{}' names {role} party '{party}' but has no matching [party.{party}]",
            channel.name
        ))
    })
}

/// The IRQ a doorbell assigns to `party`, if the database lists that exact party
/// (matching secure/non-secure and instance index).
fn party_irq(doorbell: &HardwareDoorbellPeripheral, party: Party) -> Option<u32> {
    doorbell
        .parties
        .iter()
        .find(|entry| entry.core == party)
        .map(|entry| entry.irq)
}

/// The controller-core physical-base map for an address: every participant that
/// is not the AP maps to `base` plus the alias for that party's security world
/// (the AP resolves through UIO instead).
fn controller_bases(
    parties: [Party; 2],
    base: u32,
    alias: impl Fn(bool) -> u32,
) -> HashMap<VisibleCore, u32> {
    parties
        .into_iter()
        .filter(|party| party.core() != VisibleCore::Ap)
        .map(|party| (party.core(), base + alias(party_secure(party))))
        .collect()
}

/// Whether `party` accesses the bus from the secure world.
fn party_secure(party: Party) -> bool {
    match party {
        Party::AcoreSecure => true,
        Party::AcoreNonSecure => false,
        Party::Mcore { secure, .. } => secure,
    }
}

/// The SRAM-region alias offset for the given security world, or `0` if the chip
/// declares none.
fn memory_alias(database: &ChipDatabase, secure: bool) -> u32 {
    database
        .memory
        .aliases
        .iter()
        .find(|alias| alias.secure == secure)
        .map_or(0, |alias| alias.offset)
}

/// The peripheral (doorbell) alias base for the given security world, or `0` if
/// the chip declares none.
fn peripheral_alias(database: &ChipDatabase, secure: bool) -> u32 {
    database
        .peripheral
        .aliases
        .iter()
        .find(|alias| alias.secure == secure)
        .map_or(0, |alias| alias.address)
}

/// Records a doorbell's participating cores, creating the [`HardwareDoorbell`]
/// on first sight and merging later hits. Each entry is a
/// `(core, irq, channel)`: the channel is added to the doorbell's used set, and
/// the IRQ (when known) is mapped to the core for NVIC enablement.
fn record_bell(
    bells: &mut Vec<HardwareDoorbell>,
    key: &str,
    address: &Address,
    r#type: consortium_cfg_common::database::HardwareDoorbellPeripheralType,
    entries: impl IntoIterator<Item = (VisibleCore, Option<u32>, u8)>,
) {
    let bell = match bells.iter_mut().find(|bell| bell.name == key) {
        Some(bell) => bell,
        None => {
            bells.push(HardwareDoorbell {
                name: key.to_string(),
                address: address.clone(),
                r#type,
                channels: Vec::new(),
                irq: HashMap::new(),
            });
            bells.last_mut().expect("just pushed")
        }
    };

    for (core, irq, channel) in entries {
        if !bell.channels.contains(&channel) {
            bell.channels.push(channel);
        }
        if let Some(irq) = irq {
            bell.irq.insert(core, irq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::{Codec, McoreName};

    const STM32: &str = include_str!("../Consortium.example.stm32mp257.toml");

    fn cm33() -> VisibleCore {
        VisibleCore::Mcore(McoreName::Cm33)
    }

    fn cm0p() -> VisibleCore {
        VisibleCore::Mcore(McoreName::Cm0p)
    }

    fn lowered() -> LoweredIr {
        Session::from_toml(STM32)
            .expect("example manifest loads")
            .lower()
            .expect("session lowers")
    }

    fn transceiver<'a>(ir: &'a LoweredIr, name: &str) -> &'a Transceiver {
        ir.shuhan
            .transceivers()
            .iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| panic!("transceiver '{name}' present"))
    }

    #[test]
    fn names_uio_devices_shared_memory_then_doorbells() {
        let ir = lowered();
        let names: Vec<_> = ir.uio.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, ["ipc-shm", "hsem", "ipcc2", "dbg-shm"]);
    }

    #[test]
    fn region_addresses_index_uio0_in_declaration_order() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        let lp = transceiver(&ir, "lp");

        // Regions share uio0; mmap slot follows declaration order.
        assert_eq!(
            sensor.region.address.ap,
            ApAddress {
                uio_id: 0,
                mmap_id: 0
            }
        );
        assert_eq!(
            lp.region.address.ap,
            ApAddress {
                uio_id: 0,
                mmap_id: 1
            }
        );

        // The controller side resolves to the manifest base plus the non-secure
        // SRAM alias (cm33 and cm0p are both non-secure here).
        assert_eq!(sensor.region.address.cp[&cm33()], 0x2040_0000);
        assert_eq!(lp.region.address.cp[&cm0p()], 0x2040_4000);
    }

    #[test]
    fn uio_windows_apply_non_secure_aliases() {
        let ir = lowered();

        // SRAM regions map through the non-secure memory alias (0x2000_0000),
        // so sensor at canonical 0x0040_0000 is exposed at 0x2040_0000.
        let (_, shm) = ir
            .uio
            .iter()
            .find(|(name, _)| name == "ipc-shm")
            .expect("shared-memory uio device");
        assert_eq!(shm.mmap[0].0, 0x2040_0000);

        // The HSEM doorbell window maps through the non-secure peripheral alias
        // (0x4000_0000): canonical 0x0624_0000 becomes 0x4624_0000.
        let (_, hsem) = ir
            .uio
            .iter()
            .find(|(name, _)| name == "hsem")
            .expect("hsem uio device");
        assert_eq!(hsem.mmap[0].0, 0x4624_0000);
    }

    #[test]
    fn controller_doorbell_base_applies_security_alias() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        // cm33 is non-secure, so it sees HSEM through the 0x4000_0000 alias.
        assert_eq!(sensor.doorbell_address.cp[&cm33()], 0x4624_0000);
    }

    #[test]
    fn doorbell_addresses_get_their_own_uio_devices() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        let lp = transceiver(&ir, "lp");

        assert_eq!(
            sensor.doorbell_address.ap,
            ApAddress {
                uio_id: 1,
                mmap_id: 0
            }
        );
        assert_eq!(
            lp.doorbell_address.ap,
            ApAddress {
                uio_id: 2,
                mmap_id: 0
            }
        );
    }

    #[test]
    fn uses_explicit_primary_secondary_parties() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");

        // `primary = "linux"`, `secondary = "cm33"`.
        assert_eq!(sensor.channel_primary.0, VisibleCore::Ap);
        assert_eq!(sensor.channel_primary.1.doorbell_chan, 0);
        assert_eq!(sensor.channel_primary.1.type_name, "SensorData");
        assert_eq!(sensor.channel_secondary.0, cm33());
        assert_eq!(sensor.channel_secondary.1.doorbell_chan, 1);
        assert_eq!(sensor.channel_secondary.1.type_name, "MotionControl");
    }

    #[test]
    fn carries_mtu_and_codec_through() {
        let ir = lowered();
        assert_eq!(transceiver(&ir, "sensor").mtu, 128);
        assert_eq!(transceiver(&ir, "sensor").codec, Codec::Postcard);
        assert_eq!(transceiver(&ir, "lp").codec, Codec::Rkyv);
    }

    #[test]
    fn rejects_party_without_endpoint() {
        // `secondary = "cm33"` names a party with no `[party.cm33]` endpoint.
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"
            shared = "./shared"

            [[ipc.channel]]
            name = "solo"
            doorbell = "hsem"
            pattern = "memory"
            primary = "linux"
            secondary = "cm33"
            party.linux = { chan = 0, type = "Ping" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        let session = Session::from_toml(manifest).expect("manifest validates");
        assert!(matches!(session.lower(), Err(CfgError::Manifest(_))));
    }

    #[test]
    fn bella_collects_doorbell_channels_and_core_irqs() {
        let ir = lowered();

        let hsem = ir
            .bella
            .bells
            .iter()
            .find(|bell| bell.name == "hsem")
            .expect("hsem doorbell");
        // hsem carries the primary (linux, chan 0) and secondary (cm33, chan 1).
        assert!(hsem.channels.contains(&0));
        assert!(hsem.channels.contains(&1));
        // cm33's NVIC line is sourced from the chip database.
        assert_eq!(hsem.irq[&cm33()], 199);

        let ipcc2 = ir
            .bella
            .bells
            .iter()
            .find(|bell| bell.name == "ipcc2")
            .expect("ipcc2 doorbell");
        assert!(ipcc2.irq.contains_key(&cm0p()));
    }

    #[test]
    fn lowers_dbg_ring_with_non_secure_alias_and_env() {
        let ir = lowered();

        // The example declares `[dbg.cm33]` at canonical 0x0040_8000; cm33 sees
        // SRAM through the non-secure alias (0x2000_0000), so its logger embeds
        // 0x2040_8000.
        let ring = ir.dbg.ring(cm33()).expect("cm33 has a debug ring");
        assert_eq!(ring.base, 0x2040_8000);
        assert_eq!(ring.length, 0x2000);
        assert_eq!(ring.mtu, 64);
        assert!(ir.dbg.ring(cm0p()).is_none());

        // The IR resolves the build-time vars the builder injects per core.
        let env: std::collections::HashMap<_, _> = ring.env().into_iter().collect();
        assert_eq!(env["CONSORTIUM_DBG_BASE"], "541097984");
        assert_eq!(env["CONSORTIUM_DBG_LENGTH"], "8192");
    }

    #[test]
    fn lowered_ir_emits_valid_rust() {
        let ir = lowered();
        let uio = quote::format_ident!("uio");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();

        let tokens = ir.shuhan.initialize(VisibleCore::Ap, &uio, &ty_crate);
        syn::parse_file(&tokens.to_string()).expect("lowered ShuHan is valid Rust");

        // The full AP entry point (disconnected/connected endpoints, async
        // `connect`, `ConsortiumInitError`, and the dbg host console weave)
        // must parse as a complete module.
        let ap = ir.initialize(VisibleCore::Ap, &ty_crate);
        let ap_file =
            syn::parse_file(&ap.to_string()).expect("lowered AP init module is valid Rust");
        let ap_src = ap.to_string();
        assert!(ap_src.contains("IpcMemoryEndpointsConnected"));
        assert!(ap_src.contains("async fn connect"));
        assert!(ap_src.contains("descriptor_init"));
        assert!(ap_src.contains("async fn init"));
        // The example declares `[dbg.cm33]`, so the host console is woven in.
        assert!(ap_src.contains("dbg_console :: spawn"));
        // `init` is async and returns the aggregate error.
        assert!(ap_src.contains("ConsortiumInitError"));
        let _ = ap_file;

        // A controller core (Secondary) emits an infallible `new` plus the same
        // `connect` handshake (here via `descriptor_ack`).
        let cm = ir.initialize(cm33(), &ty_crate);
        let cm_src = cm.to_string();
        syn::parse_file(&cm_src).expect("lowered controller init module is valid Rust");
        assert!(cm_src.contains("descriptor_ack"));

        let bella = ir
            .bella
            .nvic_enable(&cm33())
            .expect("cm33 has doorbell irqs");
        let wrapped = quote::quote! { fn _f() { #bella } };
        syn::parse_file(&wrapped.to_string()).expect("lowered Bella is valid Rust");
    }
}
