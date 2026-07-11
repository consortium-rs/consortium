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
//! - Every device — the aggregate IPC shared-memory device, one device per
//!   distinct doorbell, and the aggregate dbg-ring device — is assigned its
//!   `uioN` id by ascending mapped base, so the ids follow the memory map the
//!   kernel enumerates.
//! - Within the IPC device, a region's `mmap_id` is its index in
//!   channel-declaration order; a doorbell always exposes its register window as
//!   `mmap_id` 0.

use std::collections::HashMap;

use consortium_cfg_common::config::{McoreName, MemoryRegion, Party, VisibleCore};
use consortium_cfg_common::database::{ChipDatabase, HardwareDoorbellPeripheral, MemoryRegionType};

use crate::components::address::{Address, ApAddress};
use crate::components::bella::{BellA, HardwareDoorbell};
use crate::components::dbg::{DebugLogs, DebugRing};
use crate::components::ipc_shm::{Channel, DoorbellMeta, ShuHan, Transceiver};
use crate::components::peripheral::{PeripheralInstance, PeripheralKind, Peripherals};
use crate::components::region::Region;
use crate::components::uio::{ToUio, UioBacking, UioDevice};
use consortium_data::DataError;

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
    /// Owned peripherals (I2C, GPIO) to bring up per controller core.
    pub peripherals: Peripherals,
}

impl LoweredIr {
    /// A minimal IR carrying only a UIO device list, for tests that exercise the
    /// device-tree rendering (which reads nothing else).
    ///
    /// Test-only; hidden from the public API surface but reachable from other
    /// crates' tests (e.g. `consortium-builder`).
    #[doc(hidden)]
    pub fn empty_with_uio(uio: Vec<(String, UioDevice)>) -> Self {
        use crate::components::bella::BellA;
        use crate::components::dbg::DebugLogs;
        use crate::components::ipc_shm::ShuHan;
        use crate::components::peripheral::Peripherals;
        use consortium_cfg_common::config::DbgMode;
        use std::collections::HashMap;

        Self {
            shuhan: ShuHan::new(Vec::new()),
            bella: BellA { bells: Vec::new() },
            uio,
            dbg: DebugLogs::new(HashMap::new(), DbgMode::default()),
            peripherals: Peripherals {
                hal_crate: String::new(),
                instances: Vec::new(),
            },
        }
    }

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
        let endpoints = ShuHan::new(participating.clone()).initialize(core, &uio_ident, ty_crate);

        match core {
            // The AP falls through to its UIO/`init` bring-up below.
            VisibleCore::Ap => {}
            // Cortex-M controllers additionally get an `init` entry point that
            // programs the MPU and NVIC before constructing the endpoints.
            VisibleCore::Mcore(name) => {
                return self.controller_init(core, name, &participating, endpoints);
            }
            // Other controller families (Cortex-R, RISC-V) have no Cortex-M
            // MPU/NVIC weave yet; emit just the endpoint module.
            _ => return endpoints,
        }

        // `UioDevice::open(n)` for each declared device, in `self.uio` order
        // (shared-memory device first, then one per doorbell, then `dbg-shm`).
        // The opened handles are moved into `ConsortiumEndpoints` so the mmap
        // windows `IpcMemoryEndpointsConnected` borrows stay alive.
        let opens = (0..self.uio.len()).map(|id| {
            quote! { ::consortium_runtime_app::uio::UioDevice::open(#id)? }
        });
        // Literal device count for the generated `init` trace line.
        let uio_count = self.uio.len();

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
            ///
            /// The variant payloads are surfaced through `Debug` in the
            /// consuming binary (a downstream crate this module is `include!`d
            /// into), which dead-code analysis cannot see, so allow the
            /// otherwise-"never read" fields.
            #[derive(Debug)]
            #[allow(dead_code)]
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
                ::consortium_runtime_app::log::trace!(
                    "consortium init: opening {} uio device(s)", #uio_count
                );
                let #uio_ident = ::std::vec![ #(#opens),* ];
                ::consortium_runtime_app::log::trace!(
                    "consortium init: connecting ipc endpoints"
                );
                let ipc_shm = IpcMemoryEndpoints::new(&#uio_ident)?.connect().await?;
                #dbg_init
                ::consortium_runtime_app::log::trace!("consortium init: endpoints ready");
                ::core::result::Result::Ok(ConsortiumEndpoints { #dbg_field_name #uio_ident, ipc_shm })
            }
        }
    }

    /// Wraps a Cortex-M controller core's endpoint module with an `init` entry
    /// point that brings up the core's memory protection and interrupt
    /// controller before constructing the (disconnected) IPC endpoints.
    ///
    /// The generated `init`:
    ///
    /// - programs one Normal, shareable, non-cacheable MPU carveout per
    ///   distinct shared IPC region (so this core and its peer observe a
    ///   coherent view without manual cache maintenance), for the cached
    ///   Cortex-M families that ship a [`consortium_runtime_mcu::mpu`] helper
    ///   (ARMv7-M/ARMv8-M). ARMv6-M (`cm0p`) has no data cache and no helper, so
    ///   it gets NVIC setup only; and
    /// - unmasks every doorbell NVIC line visible to this core (via
    ///   [`BellA::nvic_enable`](crate::components::bella::BellA::nvic_enable)).
    ///
    /// It is gated to `target_arch = "arm"` so the host trybuild harness, which
    /// never links `cortex-m` or `consortium-runtime-mcu`, skips it while still
    /// compiling the endpoint module.
    fn controller_init(
        &self,
        core: VisibleCore,
        name: McoreName,
        participating: &[Transceiver],
        endpoints: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        use quote::quote;

        // One non-cacheable carveout per distinct shared-region base this core
        // maps. ARMv6-M has no cache to disable, so it configures no regions.
        let mut mpu_stmts = Vec::new();
        if mcore_has_mpu_helper(name) {
            let mut seen = std::collections::BTreeSet::new();
            for transceiver in participating {
                let Some(base) = transceiver.region.address.cp.get(&core).copied() else {
                    continue;
                };
                if !seen.insert(base) {
                    continue;
                }
                let size = transceiver.region.length;
                let region = seen.len() as u8 - 1;
                mpu_stmts.push(quote! {
                    /// SAFETY: `init` holds exclusive privileged MPU access during
                    /// bring-up; `base`/`size` are this core's validated shared
                    /// region from the Consortium hardware layout.
                    unsafe {
                        ::consortium_runtime_mcu::mpu::disable_cache(
                            &mut __core.MPU,
                            #region,
                            #base,
                            #size,
                        );
                    }
                });
            }
        }

        // `disable_cache` needs the stolen `MPU` peripheral; NVIC unmasking is a
        // free function, so cores with no MPU carveout skip the steal entirely.
        let steal = if mpu_stmts.is_empty() {
            quote! {}
        } else {
            quote! {
                /// SAFETY: `init` runs once during controller bring-up, in
                /// privileged mode, before any other code touches the MPU.
                let mut __core = unsafe { ::cortex_m::Peripherals::steal() };
            }
        };

        // The doorbell NVIC enum/impl plus `unmask` statements for this core.
        let nvic = self.bella.nvic_enable(&core);

        // Peripheral (I2C/GPIO) bring-up: the `Peripherals` struct, per-instance
        // interrupt state/handlers, and a `peripherals()` constructor. Empty for
        // cores that own no codegen-supported peripheral.
        let peripherals = self.peripherals.initialize(core);

        quote! {
            #endpoints

            /// Brings up this controller core's memory protection and interrupt
            /// controller, then constructs its (disconnected) IPC endpoints.
            /// Call `.connect().await` on the result to run the descriptor
            /// handshake.
            ///
            /// Compiled only for Cortex-M targets; the host trybuild harness
            /// (which never links `cortex-m`) skips it.
            #[cfg(target_arch = "arm")]
            fn init() -> IpcMemoryEndpoints {
                unsafe { core::ptr::write_volatile(0x20064000 as *mut u32, 0x31) };
                ::consortium_runtime_mcu::log::trace!("consortium controller init: mpu/nvic bring-up");
                unsafe { core::ptr::write_volatile(0x20064000 as *mut u32, 0x32) };
                #steal
                unsafe { core::ptr::write_volatile(0x20064000 as *mut u32, 0x33) };
                #(#mpu_stmts)*
                unsafe { core::ptr::write_volatile(0x20064000 as *mut u32, 0x34) };
                #nvic
                ::consortium_runtime_mcu::log::trace!("consortium controller init: endpoints constructed");
                unsafe { core::ptr::write_volatile(0x20064000 as *mut u32, 0x35) };
                IpcMemoryEndpoints::new()
            }

            #peripherals
        }
    }
}

/// Whether `name`'s Cortex-M architecture ships a `disable_cache` MPU helper in
/// [`consortium_runtime_mcu::mpu`]. ARMv7-M (`cm4f`/`cm7`) and ARMv8-M (`cm33`)
/// do; ARMv6-M (`cm0p`) has no data cache and no helper.
fn mcore_has_mpu_helper(name: McoreName) -> bool {
    !matches!(name, McoreName::Cm0p)
}

/// The chip HAL crate the generated peripheral handles are typed against,
/// selected by chip family: STM32MP2 uses `consortium-hal-stm32mp2`, and the
/// i.MX 9 families use `consortium-hal-imx9`.
fn hal_crate_path(chip: &str) -> String {
    if chip.to_ascii_lowercase().starts_with("stm32") {
        "::consortium_hal_stm32mp2".to_string()
    } else {
        "::consortium_hal_imx9".to_string()
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
        consortium_log::trace!(
            "lowering manifest: {} ipc channels, {} peripherals",
            self.config.ipc.channels.len(),
            self.config.peripheral.len()
        );
        let mut transceivers = Vec::new();
        let mut regions: Vec<MemoryRegion> = Vec::new();

        // Doorbells in first-appearance order. This only fixes a provisional
        // `uioN` layout; the whole device list is re-sorted into memory order
        // after assembly below, which rewrites every recorded id.
        let mut doorbell_order: Vec<String> = Vec::new();
        let mut bells: Vec<HardwareDoorbell> = Vec::new();

        for channel in &self.config.ipc.channels {
            consortium_log::trace!("lowering ipc channel '{}'", channel.name);
            let primary_party = channel.primary;
            let secondary_party = channel.secondary;
            let primary = endpoint(channel, primary_party, "primary")?;
            let secondary = endpoint(channel, secondary_party, "secondary")?;
            let primary_core = primary_party.core();
            let secondary_core = secondary_party.core();

            let meta: DoorbellMeta = channel.doorbell.parse().map_err(|err: CfgError| {
                consortium_log::trace!(
                    "cannot type doorbell '{}' for channel '{}': {}",
                    channel.doorbell,
                    channel.name,
                    err
                );
                CfgError::Manifest(err.to_string())
            })?;
            let db_doorbell = self.doorbell(&channel.doorbell).ok_or_else(|| {
                consortium_log::trace!(
                    "unknown doorbell '{}' for channel '{}'",
                    channel.doorbell,
                    channel.name
                );
                CfgError::Manifest(format!("unknown doorbell '{}'", channel.doorbell))
            })?;

            let memory = channel.memory().ok_or_else(|| {
                consortium_log::trace!(
                    "channel '{}' uses memory pattern but sets no config.memory",
                    channel.name
                );
                CfgError::Manifest(format!(
                    "ipc channel '{}' uses the memory pattern but sets no config.memory",
                    channel.name
                ))
            })?;

            // Region lives in `uio0`; its mmap slot is its declaration order.
            let region_mmap = regions.len();
            regions.push(memory);

            // Reserve / reuse this doorbell's provisional `uioN` device (offset
            // by 1 for the shared-memory device at pre-sort `uio0`).
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

        // Assemble the UIO device list in a provisional order: shared memory
        // first, then each doorbell. Linux is non-secure, so every mapped window
        // is offset by the non-secure alias. The final `uioN` ids are fixed by
        // the memory-order sort after the dbg device is appended.
        let mut uio = Vec::new();
        let region_alias = memory_alias(&self.database, false);
        consortium_log::trace!("lowering {} shared-memory regions to UIO", regions.len());
        if let Some(mut device) = regions.as_slice().to_uio()? {
            for window in &mut device.mmap {
                window.0 += region_alias;
            }
            uio.push(("ipc-shm".to_string(), device));
        }
        let doorbell_alias = peripheral_alias(&self.database, false);
        for name in &doorbell_order {
            consortium_log::trace!("lowering doorbell '{}' to UIO", name);
            let db_doorbell = self.doorbell(name).ok_or_else(|| {
                consortium_log::trace!("unknown doorbell '{}' during UIO assembly", name);
                CfgError::Manifest(format!("unknown doorbell '{name}'"))
            })?;
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
                    backing: UioBacking::ReservedMemory,
                },
            ));
        }

        // Order every UIO device — shared memory, doorbells, and the aggregate
        // dbg-ring device alike — by ascending mapped base, so the `uioN` ids
        // follow the memory map the kernel enumerates rather than the assembly
        // order above. The provisional ids recorded in the transceiver/doorbell
        // addresses and dbg rings are then rewritten to their sorted positions.
        let device_base =
            |device: &UioDevice| device.mmap.iter().map(|(base, _)| *base).min().unwrap_or(0);
        let mut remap = vec![0usize; uio.len()];
        let old_index: HashMap<String, usize> = uio
            .iter()
            .enumerate()
            .map(|(id, (name, _))| (name.clone(), id))
            .collect();
        uio.sort_by_key(|(_, device)| device_base(device));
        for (new_id, (name, _)) in uio.iter().enumerate() {
            remap[old_index[name]] = new_id;
        }
        for transceiver in &mut transceivers {
            transceiver.region.address.ap.uio_id = remap[transceiver.region.address.ap.uio_id];
            transceiver.doorbell_address.ap.uio_id = remap[transceiver.doorbell_address.ap.uio_id];
        }
        for bell in &mut bells {
            bell.address.ap.uio_id = remap[bell.address.ap.uio_id];
        }
        for ring in dbg_rings.values_mut() {
            ring.ap_uio_id = remap[ring.ap_uio_id];
        }

        let peripherals = self.lower_peripherals()?;
        consortium_log::trace!(
            "lowering complete: {} transceivers, {} uio devices, {} peripheral instances",
            transceivers.len(),
            uio.len(),
            peripherals.instances.len()
        );
        Ok(LoweredIr {
            shuhan: ShuHan::new(transceivers),
            bella: BellA { bells },
            uio,
            dbg: DebugLogs::new(dbg_rings, self.config.dbg.mode),
            peripherals,
        })
    }

    /// Resolves the `[peripheral.<name>]` entries owned by controller cores into
    /// the [`Peripherals`] IR, using the per-core chip map for bases and
    /// interrupt numbers.
    ///
    /// Only I2C and GPIO blocks are brought up today; other blocks (and cores
    /// without a bundled chip map) are skipped, yielding no generated handle.
    fn lower_peripherals(&self) -> Result<Peripherals, CfgError> {
        let chip = &self.config.profile.chip;
        let mut instances = Vec::new();

        for (name, peripheral) in &self.config.peripheral {
            let VisibleCore::Mcore(_) = peripheral.owner else {
                continue;
            };
            let core = peripheral.owner.to_string();
            consortium_log::trace!("lowering peripheral '{}' on core '{}'", name, core);
            let map = match crate::chipmap::load(chip, &core) {
                Ok(map) => map,
                Err(DataError::ChipMapUnavailable { .. }) => {
                    consortium_log::trace!(
                        "no chip map for core '{}'; peripheral '{}' gets no bring-up",
                        core,
                        name
                    );
                    continue;
                }
                Err(err) => {
                    consortium_log::trace!("chip map load failed for core '{}': {}", core, err);
                    return Err(err.into());
                }
            };
            let Some(entry) = map.peripheral(name) else {
                consortium_log::trace!("peripheral '{}' absent from chip map for '{}'", name, core);
                continue;
            };
            let base = peripheral_absolute_base(&self.database, entry.base, peripheral.secure);

            let kind = match entry.kind() {
                "i2c" | "lpi2c" => {
                    let irq = map.irq(name).ok_or_else(|| {
                        consortium_log::trace!(
                            "i2c peripheral '{}' has no interrupt entry in chip map for '{}'",
                            name,
                            core
                        );
                        CfgError::Manifest(format!(
                            "i2c peripheral '{name}' has no interrupt entry in the chip map for '{core}'"
                        ))
                    })?;
                    let clock_hz = peripheral.clock.ok_or_else(|| {
                        consortium_log::trace!(
                            "i2c peripheral '{}' missing required `clock` field",
                            name
                        );
                        CfgError::Manifest(format!(
                            "i2c peripheral '{name}' needs `clock` (its input clock in Hz) to generate bring-up"
                        ))
                    })?;
                    let frequency = peripheral.frequency.unwrap_or(100_000);
                    PeripheralKind::I2c {
                        clock_hz,
                        frequency,
                        irq_number: irq.number,
                        irq_name: irq.name.clone(),
                    }
                }
                "gpio" | "rgpio" => PeripheralKind::Gpio,
                // Unsupported block: no generated bring-up.
                _ => continue,
            };

            instances.push(PeripheralInstance {
                owner: peripheral.owner,
                key: name.clone(),
                base,
                kind,
            });
        }

        // Deterministic emit order regardless of the config map iteration order.
        instances.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(Peripherals {
            hal_crate: hal_crate_path(chip),
            instances,
        })
    }

    /// The `[endpoints]` artifact root declared for `core`, used to *seed* that
    /// coprocessor's debug-ring [`elf_path`](crate::components::dbg::DebugRing::elf_path).
    ///
    /// This is only a placeholder: it names the firmware's source directory, not
    /// its compiled ELF. The build pipeline overwrites it with the real
    /// build-host artifact path via
    /// [`DebugLogs::with_resolved_elf_paths`](crate::components::dbg::DebugLogs::with_resolved_elf_paths)
    /// once Stage 1 has compiled the firmware, before generating the AP codegen
    /// that `include_bytes!`es the image. Keeping `lower` a pure function of the
    /// manifest means the substitution happens in the builder, not here.
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
        consortium_log::trace!(
            "channel '{}' names {} party '{}' with no matching [party.{}]",
            channel.name,
            role,
            party,
            party
        );
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

/// The absolute peripheral base a controller core sees, given the non-secure
/// absolute base from the chip map.
///
/// The chip dumps carry each peripheral's non-secure CMSIS address, so
/// a non-secure owner uses it unchanged. A secure owner rebases it from the
/// non-secure world to the secure world (e.g. STM32MP2 `0x4000_0000` →
/// `0x5000_0000`); chips without peripheral aliases have no such split and the
/// base is returned unchanged.
fn peripheral_absolute_base(database: &ChipDatabase, base: u32, secure: bool) -> u32 {
    if secure {
        let non_secure = peripheral_alias(database, false);
        let secure_world = peripheral_alias(database, true);
        base.wrapping_sub(non_secure).wrapping_add(secure_world)
    } else {
        base
    }
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
    fn orders_uio_devices_by_ascending_memory_base() {
        let ir = lowered();
        let names: Vec<_> = ir.uio.iter().map(|(name, _)| name.as_str()).collect();
        // Every device is ordered by its mapped base: the SRAM carveouts
        // (ipc-shm @0x2040_0000, dbg-shm @0x2040_8000) precede the peripheral
        // doorbells (hsem @0x4624_0000, ipcc2 @0x4625_0000).
        assert_eq!(names, ["ipc-shm", "dbg-shm", "hsem", "ipcc2"]);
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

        // Doorbells sort after both SRAM carveouts, so hsem is uio2 and ipcc2
        // is uio3 (ipc-shm @uio0, dbg-shm @uio1).
        assert_eq!(
            sensor.doorbell_address.ap,
            ApAddress {
                uio_id: 2,
                mmap_id: 0
            }
        );
        assert_eq!(
            lp.doorbell_address.ap,
            ApAddress {
                uio_id: 3,
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
            deploy_root = "/opt/consortium"

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
