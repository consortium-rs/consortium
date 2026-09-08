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
//! ([`ShuHan::initialize`](crate::components::ipc_shm::ShuHan::initialize),
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

use consortium_cfg_common::config::{McoreName, Party, VisibleCore};
use consortium_cfg_common::database::{
    ChipDatabase, DoorbellWindow, HardwareDoorbellPeripheral, MemoryRegionType,
};

use crate::components::address::{Address, ApAddress};
use crate::components::bella::{BellA, HardwareDoorbell};
use crate::components::dbg::{DebugLogs, DebugRing};
use crate::components::ipc_shm::{Channel, DoorbellMeta, DoorbellRingerSide, ShuHan, Transceiver};
use crate::components::irq::InterruptLine;
use crate::components::memory_to_uio::SharedWindow;
use crate::components::peripheral::{PeripheralInstance, PeripheralKind, Peripherals};
use crate::components::region::Region;
use crate::components::remoteproc::{
    LinkerRegionAttributes, LinkerSectionBinding, RemoteprocMemoryLayout, RemoteprocMemoryWindow,
    RemoteprocNamedRegion,
};
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
    /// Peripheral instance names each controller core's chip map lists, keyed by
    /// core. Code generation can only name a HAL peripheral singleton for an
    /// instance that appears here; cores with no bundled map have no entry, and
    /// fall back to address-based construction.
    pub core_peripherals: HashMap<VisibleCore, Vec<String>>,
    /// Per-remote-core linker/ELF/DTS memory contracts.
    pub remoteproc: HashMap<VisibleCore, RemoteprocMemoryLayout>,
    /// Whether controller cores get the sealed Embassy time driver, timer
    /// vector, and `defmt::timestamp!` (mirrors `[profile] time_driver`).
    pub time_driver: bool,
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
            core_peripherals: HashMap::new(),
            remoteproc: HashMap::new(),
            time_driver: true,
        }
    }

    /// Generates the per-core IPC initialization module.
    ///
    /// Every core gets channel markers, instance bindings, and a transport
    /// installer restricted to the transceivers it participates in.
    ///
    /// The AP additionally emits `Config`, platform `Peripherals`, and a
    /// fallible synchronous `init` that opens the declared [`UioDevice`]s.
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
        // Peripheral singletons only exist for cores with a bundled chip map;
        // the doorbell falls back to its mapped-base constructor elsewhere.
        let available = self
            .core_peripherals
            .get(&core)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let endpoints = crate::components::ipc_shm::initialize_instances(
            &ShuHan::new(participating.clone()),
            core,
            &uio_ident,
            ty_crate,
            &self.peripherals.hal_crate,
            available,
        );

        match core {
            // The AP falls through to its UIO/`init` bring-up below.
            VisibleCore::Ap => {}
            // Cortex-M controllers additionally get an `init` entry point that
            // programs the MPU and NVIC before constructing the endpoints.
            VisibleCore::Mcore(name) => {
                let doorbells =
                    ShuHan::new(participating.clone()).doorbell_singletons(core, available);
                return self.controller_init(core, name, &participating, endpoints, doorbells);
            }
            // Other controller families (Cortex-R, RISC-V) have no Cortex-M
            // MPU/NVIC weave yet; emit just the endpoint module.
            _ => return endpoints,
        }

        // `UioDevice::open(n, enable_irq)` for each declared device, in `self.uio` order
        // (shared-memory device first, then one per doorbell, then `dbg-shm`).
        // Opened handles are Arc-owned by platform resources and cloned into
        // each transport that uses one of their mappings.
        let opens = self.uio.iter().enumerate().map(|(id, (_, device))| {
            let enable_irq = device.irq.is_some();
            quote! {
                ::std::sync::Arc::new(
                    ::consortium_runtime_app::uio::UioDevice::open(
                        #id,
                        #enable_irq && config.uio_irqs,
                    )?
                )
            }
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
                #dbg_error_arm
            }

            impl ::core::convert::From<::consortium_runtime_app::uio::UioDeviceError>
                for ConsortiumInitError
            {
                fn from(e: ::consortium_runtime_app::uio::UioDeviceError) -> Self {
                    ConsortiumInitError::Uio(e)
                }
            }

            #dbg_from

            /// Runtime bring-up policy. Every enabled manifest step defaults on.
            #[non_exhaustive]
            pub struct Config {
                pub tracing: bool,
                pub uio_irqs: bool,
                pub dbg_console: bool,
            }

            impl ::core::default::Default for Config {
                fn default() -> Self {
                    Self { tracing: true, uio_irqs: true, dbg_console: true }
                }
            }

            /// Platform resources owned by the application endpoint.
            ///
            /// This type deliberately has no `Drop` implementation so fields can
            /// be moved independently. The debug field is declared first so it
            /// stops touching mappings before the UIO devices unmap.
            pub struct Peripherals {
                #dbg_field
                pub #uio_ident: ::std::vec::Vec<
                    ::std::sync::Arc<::consortium_runtime_app::uio::UioDevice>
                >,
            }

            fn init(
                config: Config,
            ) -> ::core::result::Result<(Peripherals, Channels), ConsortiumInitError> {
                if config.tracing {
                    ::consortium_runtime_app::init_tracing();
                }
                ::consortium_runtime_app::log::trace!(
                    "consortium init: opening {} uio device(s)", #uio_count
                );
                let #uio_ident = ::std::vec![ #(#opens),* ];
                ::consortium_runtime_app::log::trace!(
                    "consortium init: installing ipc endpoints"
                );
                __consortium_install_channels(&#uio_ident)?;
                let channels = Channels::take();
                #dbg_init
                ::consortium_runtime_app::log::trace!("consortium init: resources ready");
                ::core::result::Result::Ok((
                    Peripherals { #dbg_field_name #uio_ident },
                    channels,
                ))
            }

            /// Error union suitable for `#[consortium_runtime_app::main]`.
            ///
            /// `MainOutcome` reports the payload through derived `Debug`, which
            /// dead-code analysis intentionally does not count as a field read.
            #[derive(Debug)]
            #[allow(dead_code)]
            enum ConsortiumError {
                Init(ConsortiumInitError),
                Connect(::consortium_ipc_transport_memory::ConnectError),
            }

            impl ::core::convert::From<ConsortiumInitError> for ConsortiumError {
                fn from(e: ConsortiumInitError) -> Self { Self::Init(e) }
            }

            impl ::core::convert::From<::consortium_ipc_transport_memory::ConnectError>
                for ConsortiumError
            {
                fn from(e: ::consortium_ipc_transport_memory::ConnectError) -> Self {
                    Self::Connect(e)
                }
            }
        }
    }

    /// Wraps a Cortex-M endpoint module with its Embassy-shaped resource types
    /// and synchronous `init`.
    ///
    /// The generated `init`:
    ///
    /// - resets the CDBG ring via `ConsortiumLogger::init()` when a
    ///   `[dbg.<core>]` logger is present, so the first `defmt` frame lands in a
    ///   fresh descriptor;
    /// - programs one Normal, shareable, non-cacheable MPU carveout per
    ///   distinct uncached shared IPC region (cacheable windows retain the
    ///   default write-back attributes and use explicit maintenance), for the
    ///   Cortex-M families that ship a [`consortium_runtime_mcu::mpu`] helper
    ///   (ARMv7-M/ARMv8-M). ARMv6-M (`cm0p`) has no data cache and no helper, so
    ///   it gets NVIC setup only;
    /// - invalidates and enables the Cortex-M7 data cache when at least one IPC
    ///   window uses `config.cache = true`;
    /// - unmasks every doorbell NVIC line visible to this core (via
    ///   [`BellA::nvic_enable`](crate::components::bella::BellA::nvic_enable));
    /// - brings up the sealed Embassy time driver (its timer vector and
    ///   `defmt::timestamp!` are emitted alongside) unless `[profile]
    ///   time_driver = false`; and
    /// - installs each unsplit IPC transport behind its channel instance; and
    /// - returns the peripheral and channel singleton allowlists.
    ///
    /// The platform resource surface is gated to `target_arch = "arm"` so host
    /// trybuild can compile the portable instance declarations.
    fn controller_init(
        &self,
        core: VisibleCore,
        name: McoreName,
        participating: &[Transceiver],
        endpoints: proc_macro2::TokenStream,
        doorbell_arguments: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        use quote::quote;

        // One non-cacheable carveout per distinct shared-region base this core
        // maps. ARMv6-M has no cache to disable, so it configures no regions.
        let mut mpu_stmts = Vec::new();
        if mcore_has_mpu_helper(name) {
            let mut seen = std::collections::BTreeSet::new();
            for transceiver in participating {
                // A cached window keeps the core's default write-back attributes;
                // `CachedRegion` does the maintenance instead. Skipping before the
                // dedup insert keeps MPU region numbering gap-free.
                if transceiver.region.cache {
                    continue;
                }
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
                        ::consortium_runtime_mcu::log::trace!(
                            "consortium controller init: MPU carveout {} base={:#010x} size={:#010x}",
                            #region, #base, #size
                        );
                    }
                });
            }
        }

        let enables_dcache = name == McoreName::Cm7
            && participating
                .iter()
                .any(|transceiver| transceiver.region.cache);
        let enable_dcache = if enables_dcache {
            quote! {
                // `enable_dcache` is idempotent and invalidates all cache lines
                // before setting SCB.CCR.DC, so a reset-started CM7 cannot retain
                // stale data while switching to cached shared-window access.
                __core.SCB.enable_dcache(&mut __core.CPUID);
                ::consortium_runtime_mcu::log::trace!(
                    "consortium controller init: Cortex-M7 data cache enabled"
                );
            }
        } else {
            quote! {}
        };

        // MPU carveouts need `MPU`; D-cache bring-up needs `SCB` and `CPUID`.
        // NVIC unmasking is a free function, so a core needing neither skips the
        // steal entirely.
        let steal = if mpu_stmts.is_empty() && !enables_dcache {
            quote! {}
        } else {
            quote! {
                /// SAFETY: `init` runs once during controller bring-up, in
                /// privileged mode, before any other code touches the MPU or
                /// cache-control peripherals.
                let mut __core = unsafe { ::cortex_m::Peripherals::steal() };
            }
        };

        // The doorbell NVIC priority/unpend/enable statements for this core.
        let nvic = self.bella.nvic_enable(&core, &self.peripherals.hal_crate);

        // Peripheral singleton allowlist and interrupt vectors.
        let peripherals = self.peripherals.initialize(core);
        let peripheral_vectors = self.peripherals.vectors(core);
        let peripheral_arguments = self.peripherals.arguments(core);
        let has_peripherals = !peripheral_arguments.is_empty();
        let peripheral_nvic = self.peripherals.nvic(core);
        let peripheral_value = if has_peripherals {
            quote! { Peripherals { #(#peripheral_arguments: __p.#peripheral_arguments),* } }
        } else {
            quote! { Peripherals {} }
        };

        // A `[dbg.<core>]` logger means `defmt` is linked: reset the CDBG ring
        // first thing (before any trace) and emit a `defmt::timestamp!` source.
        let dbg_present = self.dbg.ring(core).is_some();
        let logger_init = if dbg_present {
            quote! {
                // Reset the CDBG ring on every boot so a warm reset does not
                // leave a stale, valid-looking descriptor behind. Must run
                // before the first `defmt` frame below.
                ::consortium_dbg::logger::ConsortiumLogger::init();
            }
        } else {
            quote! {}
        };

        // Sealed Embassy time driver: `bringup` statement (into `init`), the
        // timer vector, and the `defmt::timestamp!` source. Empty when
        // `[profile] time_driver = false` or the chip family has no driver yet.
        let time = crate::components::time::time_driver_codegen(
            &self.peripherals.hal_crate,
            self.time_driver,
            dbg_present,
        );
        let crate::components::time::TimeDriverCodegen {
            timestamp,
            isr: time_isr,
            bringup: time_bringup,
        } = time;

        // The chip HAL's singletons are taken once and then moved out field by
        // field: to the doorbells, the time driver, and `peripherals()`. Cores
        // that need none of them skip the call, so `init` stays free of an
        // unused binding (and of the HAL's once-only panic).
        let hal: syn::Path =
            syn::parse_str(&self.peripherals.hal_crate).expect("hal_crate must be a valid path");
        let hal_peripherals =
            if has_peripherals || !doorbell_arguments.is_empty() || !time_bringup.is_empty() {
                quote! { let __p = #hal::init(); }
            } else {
                quote! {}
            };

        // Every vector this core owns goes in one module. cortex-m-rt's
        // `#[interrupt]` expands to a `interrupt::<NAME>;` existence check, so
        // the attribute needs an `interrupt` path in scope — importing it here
        // rather than at the include site keeps the generated file from
        // depending on (or colliding with) the firmware crate's own `use`.
        // Vectors are bare-metal only: the host trybuild harness never links the
        // chip `rt` vector table, matching the `init` gate.
        let mut vectors = self.bella.vectors(&core, &self.peripherals.hal_crate);
        vectors.extend(peripheral_vectors);
        if !time_isr.is_empty() {
            vectors.insert(0, time_isr);
        }
        let vectors = if vectors.is_empty() {
            quote! {}
        } else {
            quote! {
                /// Interrupt vectors for this endpoint's generated bring-up.
                #[cfg(target_arch = "arm")]
                mod __consortium_vectors {
                    use #hal::interrupt;

                    #(#vectors)*
                }
            }
        };

        quote! {
            #endpoints

            #timestamp
            #vectors

            /// Runtime bring-up policy. Enabled manifest steps default on.
            #[cfg(target_arch = "arm")]
            #[non_exhaustive]
            pub struct Config {
                pub logger: bool,
                pub mpu: bool,
                pub nvic: bool,
                pub time_driver: bool,
            }

            #[cfg(target_arch = "arm")]
            impl ::core::default::Default for Config {
                fn default() -> Self {
                    Self { logger: true, mpu: true, nvic: true, time_driver: true }
                }
            }

            /// Brings up synchronous platform resources and installs each
            /// unconnected IPC transport into its generated endpoint state.
            #[cfg(target_arch = "arm")]
            fn init(config: Config) -> (Peripherals, Channels) {
                if config.logger { #logger_init }
                ::consortium_runtime_mcu::log::trace!("consortium controller init: mpu/nvic bring-up");
                #steal
                if config.mpu { #(#mpu_stmts)* }
                #enable_dcache
                if config.nvic {
                    #nvic
                    #peripheral_nvic
                }
                #hal_peripherals
                if config.time_driver { #time_bringup }
                __consortium_install_channels(#(__p.#doorbell_arguments),*);
                let channels = Channels::take();
                let peripherals = #peripheral_value;
                ::consortium_runtime_mcu::log::trace!("consortium controller init: resources ready");
                (peripherals, channels)
            }

            /// Error union suitable for `#[consortium_runtime_mcu::main]`.
            #[cfg(target_arch = "arm")]
            #[cfg_attr(target_os = "none", derive(Debug, defmt::Format))]
            #[cfg_attr(not(target_os = "none"), derive(Debug))]
            #[allow(dead_code)]
            enum ConsortiumError {
                Connect(::consortium_ipc_transport_memory::ConnectError),
            }

            #[cfg(target_arch = "arm")]
            impl ::core::convert::From<::consortium_ipc_transport_memory::ConnectError>
                for ConsortiumError
            {
                fn from(e: ::consortium_ipc_transport_memory::ConnectError) -> Self {
                    Self::Connect(e)
                }
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
        let remoteproc = self.lower_remoteproc()?;
        let mut transceivers = Vec::new();
        let mut regions: Vec<SharedWindow> = Vec::new();

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
            regions.push(SharedWindow {
                memory,
                cache: channel.cache(),
            });

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
            let doorbell_cp = controller_doorbell_bases(
                db_doorbell,
                [primary_party, secondary_party],
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
                meta,
                [
                    (
                        primary_core,
                        party_irq(db_doorbell, primary_party),
                        primary.chan as u8,
                        party_window(db_doorbell, primary_party, DoorbellRingerSide::Primary),
                    ),
                    (
                        secondary_core,
                        party_irq(db_doorbell, secondary_party),
                        secondary.chan as u8,
                        party_window(db_doorbell, secondary_party, DoorbellRingerSide::Secondary),
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
                    cache: channel.cache(),
                    address: region_address,
                    length: memory.length,
                },
                channel_primary: (primary_core, channel_primary),
                channel_secondary: (secondary_core, channel_secondary),
                doorbell: meta,
                doorbell_window: (
                    party_window(db_doorbell, primary_party, DoorbellRingerSide::Primary),
                    party_window(db_doorbell, secondary_party, DoorbellRingerSide::Secondary),
                ),
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
                // The interrupt-parent (GIC) label is a chip-level property; the
                // STM32MP2 root sets no default, so a root-level doorbell node
                // needs it explicit or its IRQ never binds.
                device.interrupt_parent = Some(self.database.interrupt_parent.clone());
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
                    dma_coherent: Vec::new(),
                    interrupt_parent: None,
                    clock: None,
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

        // The doorbell database records only a raw NVIC position, so recover
        // each controller-core line's name from that core's chip map — the same
        // dump the PAC's `Interrupt` enum is generated from, making the name the
        // variant generated bring-up should spell. A core the PAC's shipped
        // table does not describe (`has_pac_interrupts`) keeps the raw number,
        // because there the variant of that name carries another core's line.
        let mut bella = BellA { bells };
        let mut maps: HashMap<VisibleCore, Option<crate::chipmap::ChipMap>> = HashMap::new();
        bella.resolve_variants(|core, number| {
            let map = maps
                .entry(core)
                .or_insert_with(|| {
                    crate::chipmap::load(&self.config.profile.chip, &core.to_string()).ok()
                })
                .as_ref()?;
            if !map.has_pac_interrupts() {
                return None;
            }
            map.irq_by_number(number).map(|entry| entry.name.clone())
        });

        let peripherals = self.lower_peripherals()?;
        consortium_log::trace!(
            "lowering complete: {} transceivers, {} uio devices, {} peripheral instances",
            transceivers.len(),
            uio.len(),
            peripherals.instances.len()
        );
        let core_peripherals =
            Self::lower_core_peripherals(&self.config.profile.chip, &transceivers, &peripherals);
        Ok(LoweredIr {
            shuhan: ShuHan::new(transceivers),
            bella,
            uio,
            dbg: DebugLogs::new(dbg_rings, self.config.dbg.mode),
            peripherals,
            core_peripherals,
            remoteproc,
            time_driver: self.config.profile.time_driver,
        })
    }

    /// Applies optional manifest fields to the chip database's remoteproc
    /// defaults and produces the single layout consumed by every later stage.
    pub(crate) fn lower_remoteproc(
        &self,
    ) -> Result<HashMap<VisibleCore, RemoteprocMemoryLayout>, CfgError> {
        for core in self.config.remoteproc.keys() {
            if !self.database.remoteproc.contains_key(core) {
                return Err(CfgError::Manifest(format!(
                    "[remoteproc.{core}] has no loader contract for chip '{}'",
                    self.config.profile.chip
                )));
            }
            let visible: VisibleCore = core.parse().map_err(|_| {
                CfgError::Manifest(format!("[remoteproc.{core}] is not a visible core name"))
            })?;
            if !self
                .config
                .endpoints
                .keys()
                .any(|party| party.core() == visible)
            {
                return Err(CfgError::Manifest(format!(
                    "[remoteproc.{core}] requires an [endpoints] firmware entry for `{core}`"
                )));
            }
        }

        self.database
            .remoteproc
            .iter()
            .filter_map(|(name, default)| {
                let core: VisibleCore = match name.parse() {
                    Ok(core) => core,
                    Err(_) => {
                        return Some(Err(CfgError::Manifest(format!(
                            "chip database remoteproc entry '{name}' is not a visible core"
                        ))));
                    }
                };
                if !self
                    .config
                    .endpoints
                    .keys()
                    .any(|party| party.core() == core)
                {
                    return None;
                }
                let overrides = self.config.remoteproc.get(name);
                let window = |default: consortium_cfg_common::database::RemoteprocWindow,
                              override_window: Option<
                    &consortium_cfg_common::config::RemoteprocWindowOverride,
                >| {
                    RemoteprocMemoryWindow {
                        origin: override_window
                            .and_then(|value| value.origin)
                            .unwrap_or(default.origin),
                        length: override_window
                            .and_then(|value| value.length)
                            .unwrap_or(default.length),
                        reserve: override_window
                            .and_then(|value| value.reserve)
                            .unwrap_or(default.reserve),
                    }
                };
                let regions = overrides
                    .into_iter()
                    .flat_map(|value| &value.regions)
                    .map(|(region_name, region)| RemoteprocNamedRegion {
                        name: region_name.clone(),
                        attributes: LinkerRegionAttributes::new(
                            region.read,
                            region.write,
                            region.execute,
                        ),
                        window: RemoteprocMemoryWindow {
                            origin: region.origin,
                            length: region.length,
                            reserve: region.reserve,
                        },
                        sections: region
                            .sections
                            .iter()
                            .map(|section| LinkerSectionBinding {
                                output: section.output.clone(),
                                inputs: section.inputs.clone(),
                                keep: section.keep,
                                insert_after: section.insert_after.clone(),
                            })
                            .collect(),
                    })
                    .collect();
                let layout = RemoteprocMemoryLayout {
                    core,
                    compatible: default.compatible.clone(),
                    vector_section: default.vector_section.clone(),
                    vector: RemoteprocNamedRegion {
                        name: default.vector.name.clone(),
                        attributes: LinkerRegionAttributes::new(
                            default.vector.read,
                            default.vector.write,
                            default.vector.execute,
                        ),
                        window: window(default.vector.window, overrides.map(|value| &value.vector)),
                        sections: Vec::new(),
                    },
                    flash: window(default.flash, overrides.map(|value| &value.flash)),
                    shmem: RemoteprocNamedRegion {
                        name: default.shmem.name.clone(),
                        attributes: LinkerRegionAttributes::new(
                            default.shmem.read,
                            default.shmem.write,
                            default.shmem.execute,
                        ),
                        window: window(default.shmem.window, overrides.map(|value| &value.shmem)),
                        sections: Vec::new(),
                    },
                    ram: window(default.ram, overrides.map(|value| &value.ram)),
                    regions,
                };
                if let Err(error) = layout.validate() {
                    return Some(Err(CfgError::Manifest(format!(
                        "[remoteproc.{name}] is invalid: {error}"
                    ))));
                }
                Some(Ok((core, layout)))
            })
            .collect()
    }

    /// The peripheral instance names each controller core's bundled chip map
    /// lists, which is exactly the set code generation may name as a HAL
    /// peripheral singleton. Cores with no bundled map get no entry, and their
    /// generated code keeps the address-based construction.
    fn lower_core_peripherals(
        chip: &str,
        transceivers: &[Transceiver],
        peripherals: &Peripherals,
    ) -> HashMap<VisibleCore, Vec<String>> {
        // Every controller core code generation runs for: a party to some
        // transceiver, or the owner of some peripheral.
        let mut cores: Vec<VisibleCore> = transceivers
            .iter()
            .flat_map(|t| [t.channel_primary.0, t.channel_secondary.0])
            .chain(peripherals.instances.iter().map(|p| p.owner))
            .filter(|core| matches!(core, VisibleCore::Mcore(_)))
            .collect();
        cores.sort_by_key(|core| core.to_string());
        cores.dedup();

        cores
            .into_iter()
            .filter_map(|core| {
                let map = crate::chipmap::load(chip, &core.to_string()).ok()?;
                Some((core, map.peripheral_names()))
            })
            .collect()
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
            // The HAL's peripheral singletons carry the non-secure base the
            // chip dump lists. A secure owner used to be rebased onto the secure
            // alias here; refuse rather than hand the driver the wrong window.
            if peripheral.secure {
                consortium_log::trace!(
                    "peripheral '{}' is secure; the singleton path has no secure alias",
                    name
                );
                return Err(CfgError::Manifest(format!(
                    "peripheral '{name}' sets `secure = true`, but generated bring-up takes the \
                     chip HAL's peripheral singleton, which names only the non-secure alias; \
                     drive secure peripherals from firmware instead"
                )));
            }
            let singleton = entry.name.clone();

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
                    // The PAC variant is only nameable where the shipped table
                    // is this core's; elsewhere the line stays a raw number and
                    // no vector is generated for it.
                    if !map.has_pac_interrupts() {
                        consortium_log::trace!(
                            "core '{}' is not described by the chip PAC's interrupt table; \
                             peripheral '{}' gets its NVIC enable but no generated vector, so \
                             the firmware must install '{}' itself",
                            core,
                            name,
                            irq.name,
                        );
                    }
                    let line = InterruptLine {
                        number: u32::from(irq.number),
                        variant: map.has_pac_interrupts().then(|| irq.name.clone()),
                    };
                    PeripheralKind::I2c {
                        clock_hz,
                        frequency,
                        irq: line,
                    }
                }
                "gpio" | "rgpio" => PeripheralKind::Gpio,
                _ => PeripheralKind::Other,
            };

            instances.push(PeripheralInstance {
                owner: peripheral.owner,
                key: name.clone(),
                singleton,
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

/// The doorbell register window `party` drives, as the codegen's side marker.
///
/// Only a block that declares a `window_length` has per-side windows. For a
/// single-window doorbell (HSEM, IPCC) the marker means the *core bank* rather
/// than a window, and the bank is the descriptor role — so those fall back to
/// `role` and their generated output is unchanged by this mapping.
fn party_window(
    doorbell: &HardwareDoorbellPeripheral,
    party: Party,
    role: DoorbellRingerSide,
) -> DoorbellRingerSide {
    if doorbell.window_length.is_none() {
        return role;
    }
    let window = doorbell
        .parties
        .iter()
        .find(|entry| entry.core == party)
        .map_or(DoorbellWindow::A, |entry| entry.window);
    match window {
        DoorbellWindow::A => DoorbellRingerSide::Primary,
        DoorbellWindow::B => DoorbellRingerSide::Secondary,
    }
}

/// The controller-core physical-base map for a doorbell: like
/// [`controller_bases`], but each party is offset to *its own* register window
/// rather than sharing the block base.
fn controller_doorbell_bases(
    doorbell: &HardwareDoorbellPeripheral,
    parties: [Party; 2],
    alias: impl Fn(bool) -> u32,
) -> HashMap<VisibleCore, u32> {
    parties
        .into_iter()
        .filter(|party| party.core() != VisibleCore::Ap)
        .map(|party| {
            let base = doorbell
                .parties
                .iter()
                .find(|entry| entry.core == party)
                .map_or(doorbell.memory.base, |entry| {
                    doorbell.party_window_base(entry)
                });
            (party.core(), base + alias(party_secure(party)))
        })
        .collect()
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
/// `(core, irq, channel, side)`: the channel is added to the doorbell's used
/// set, the IRQ (when known) is mapped to the core for NVIC enablement, and the
/// side is mapped to the core so the generated vector forwards with the right
/// marker.
fn record_bell(
    bells: &mut Vec<HardwareDoorbell>,
    key: &str,
    address: &Address,
    meta: DoorbellMeta,
    entries: impl IntoIterator<Item = (VisibleCore, Option<u32>, u8, DoorbellRingerSide)>,
) {
    let bell = match bells.iter_mut().find(|bell| bell.name == key) {
        Some(bell) => bell,
        None => {
            bells.push(HardwareDoorbell {
                name: key.to_string(),
                address: address.clone(),
                r#type: meta.r#type,
                id: meta.id,
                channels: Vec::new(),
                irq: HashMap::new(),
                side: HashMap::new(),
            });
            bells.last_mut().expect("just pushed")
        }
    };

    for (core, irq, channel, side) in entries {
        if !bell.channels.contains(&channel) {
            bell.channels.push(channel);
        }
        if let Some(irq) = irq {
            bell.irq.insert(core, InterruptLine::new(irq));
        }
        bell.side.insert(core, side);
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
        // (dbg-shm @0x2006_2000, ipc-shm @0x2006_4000) precede the peripheral
        // doorbells (ipcc1 @0x4049_0000, ipcc2 @0x4625_0000).
        assert_eq!(names, ["dbg-shm", "ipc-shm", "ipcc1", "ipcc2"]);
    }

    #[test]
    fn region_addresses_index_uio0_in_declaration_order() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        let lp = transceiver(&ir, "lp");

        // Both IPC regions share the ipc-shm device (uio1, after dbg-shm at
        // uio0); mmap slot follows declaration order.
        assert_eq!(
            sensor.region.address.ap,
            ApAddress {
                uio_id: 1,
                mmap_id: 0
            }
        );
        assert_eq!(
            lp.region.address.ap,
            ApAddress {
                uio_id: 1,
                mmap_id: 1
            }
        );

        // The controller side resolves to the manifest base plus the non-secure
        // SRAM alias (cm33 and cm0p are both non-secure here). Sensor lives in
        // SRAM2 (0x0006_4000), lp in LPSRAM3 (0x000C_4000).
        assert_eq!(sensor.region.address.cp[&cm33()], 0x2006_4000);
        assert_eq!(lp.region.address.cp[&cm0p()], 0x200C_4000);
    }

    #[test]
    fn uio_windows_apply_non_secure_aliases() {
        let ir = lowered();

        // SRAM regions map through the non-secure memory alias (0x2000_0000),
        // so sensor at canonical 0x0006_4000 is exposed at 0x2006_4000.
        let (_, shm) = ir
            .uio
            .iter()
            .find(|(name, _)| name == "ipc-shm")
            .expect("shared-memory uio device");
        assert_eq!(shm.mmap[0].0, 0x2006_4000);

        // The IPCC1 doorbell window maps through the non-secure peripheral alias
        // (0x4000_0000): canonical 0x0049_0000 becomes 0x4049_0000.
        let (_, ipcc1) = ir
            .uio
            .iter()
            .find(|(name, _)| name == "ipcc1")
            .expect("ipcc1 uio device");
        assert_eq!(ipcc1.mmap[0].0, 0x4049_0000);
    }

    #[test]
    fn controller_doorbell_base_applies_security_alias() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        // cm33 is non-secure, so it sees IPCC1 through the 0x4000_0000 alias.
        assert_eq!(sensor.doorbell_address.cp[&cm33()], 0x4049_0000);
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

    /// The i.MX 93 CM33 owns MU side A while Linux owns side B, the mirror of
    /// i.MX 95. A manifest still spells the descriptor roles the documented way
    /// (`primary = "linux"`), so the two must be lowered independently: getting
    /// this wrong emits `MuDoorbell::<Secondary, _>::new(p.MU1_MUA)`, which
    /// fails `MuDoorbell::new`'s `Instance<Side = S>` bound.
    #[test]
    fn imx93_doorbell_window_is_mirrored_from_the_descriptor_role() {
        let manifest = r#"
            [profile]
            chip = "mimx9352"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [endpoints]
            linux = "./app"
            cm33  = "./firmware"

            [[ipc.channel]]
            name          = "sensor"
            doorbell      = "mu1"
            pattern       = "memory"
            primary       = "linux"
            secondary     = "cm33"
            party.linux   = { chan = 0, type = "SensorData" }
            party.cm33    = { chan = 1, type = "MotionControl" }
            config.memory = { base = 0x00480000, length = 0x4000 }
            mtu           = 128
        "#;
        let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
        let sensor = transceiver(&ir, "sensor");

        // Roles follow the manifest...
        assert_eq!(sensor.channel_primary.0, VisibleCore::Ap);
        assert_eq!(sensor.channel_secondary.0, cm33());
        // ...while the windows are mirrored: Linux on MUB, the CM33 on MUA.
        assert_eq!(
            sensor.doorbell_window,
            (DoorbellRingerSide::Secondary, DoorbellRingerSide::Primary)
        );
        // The CM33 is on side A, so its physical base is the block base itself
        // (0x0422_0000 through the 0x4000_0000 non-secure alias) — the same
        // window its chip dump exposes as `MU1_MUA`. Linux takes MUB above it.
        assert_eq!(sensor.doorbell_address.cp[&cm33()], 0x4422_0000);
    }

    /// i.MX 95 is the unmirrored case, and the CM33 there reaches MU1 through
    /// MUB even though it is the channel's `secondary` — the window offset must
    /// apply to it too, or it is handed Linux's half of the block.
    #[test]
    fn imx95_controller_doorbell_base_is_its_own_window() {
        let manifest = r#"
            [profile]
            chip = "mimx9596xvt"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [endpoints]
            linux = "./app"
            cm33  = "./firmware"

            [[ipc.channel]]
            name          = "sensor"
            doorbell      = "mu1"
            pattern       = "memory"
            primary       = "linux"
            secondary     = "cm33"
            party.linux   = { chan = 0, type = "SensorData" }
            party.cm33    = { chan = 1, type = "MotionControl" }
            config.memory = { base = 0x00488000, length = 0x4000 }
            mtu           = 128
        "#;
        let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
        let sensor = transceiver(&ir, "sensor");
        assert_eq!(
            sensor.doorbell_window,
            (DoorbellRingerSide::Primary, DoorbellRingerSide::Secondary)
        );
        assert_eq!(sensor.doorbell_address.cp[&cm33()], 0x4423_0000);
    }

    /// The generated vector must name the same window it forwards.
    ///
    /// The vector's name comes from the chip map (looked up by the party's NVIC
    /// number) while the side marker comes from the party's declared window, so
    /// an `MU1_A` handler forwarding `Secondary` would mask the *other* half of
    /// the block and starve the executor. On i.MX 93 the CM33 is on MUA, so
    /// this is `MU1_A` forwarding `Primary`.
    #[test]
    fn imx93_doorbell_vector_names_the_window_it_forwards() {
        let manifest = r#"
            [profile]
            chip = "mimx9352"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [endpoints]
            linux = "./app"
            cm33  = "./firmware"

            [[ipc.channel]]
            name          = "sensor"
            doorbell      = "mu1"
            pattern       = "memory"
            primary       = "linux"
            secondary     = "cm33"
            party.linux   = { chan = 0, type = "SensorData" }
            party.cm33    = { chan = 1, type = "MotionControl" }
            config.memory = { base = 0x00480000, length = 0x4000 }
            mtu           = 128
        "#;
        let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
        let tokens = ir.bella.vectors(&cm33(), "::consortium_hal_imx9")[0].to_string();

        assert!(tokens.contains("fn MU1_A ()"), "{tokens}");
        assert!(
            tokens.contains(
                ":: consortium_ipc_doorbell_mu :: notify_gi_interrupt :: < :: consortium_ipc :: \
                 Primary , 1usize > ()"
            ),
            "{tokens}"
        );
    }

    /// A single-window doorbell's side marker names the core *bank*, not a
    /// register window, so it must keep following the descriptor role. This is
    /// what keeps the STM32 output identical across the window change.
    #[test]
    fn single_window_doorbell_side_still_follows_the_role() {
        let ir = lowered();
        let sensor = transceiver(&ir, "sensor");
        assert_eq!(
            sensor.doorbell_window,
            (DoorbellRingerSide::Primary, DoorbellRingerSide::Secondary)
        );
    }

    /// The i.MX 93 CM33 runs from TCM rather than a DDR carveout, so its
    /// windows carry `reserve: false` and the pipeline still gets a complete
    /// memory contract — without one, `csti build` fails outright at
    /// `write_generated_memory_x`.
    #[test]
    fn imx93_cm33_remoteproc_lowers_to_a_tcm_layout() {
        let manifest = r#"
            [profile]
            chip = "mimx9352"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [endpoints]
            linux = "./app"
            cm33  = "./firmware"

            [[ipc.channel]]
            name          = "sensor"
            doorbell      = "mu1"
            pattern       = "memory"
            primary       = "linux"
            secondary     = "cm33"
            party.linux   = { chan = 0, type = "SensorData" }
            party.cm33    = { chan = 1, type = "MotionControl" }
            config.memory = { base = 0x00480000, length = 0x4000 }
            mtu           = 128
        "#;
        let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
        let layout = &ir.remoteproc[&"cm33".parse().unwrap()];

        assert_eq!(layout.compatible, "fsl,imx93-cm33");
        assert_eq!(layout.vector.window.origin, 0x1FFC_0000);
        assert_eq!(layout.ram.origin, 0x2000_0000);
        // TCM is not DRAM, so nothing here is carved out of the kernel map.
        assert!(!layout.vector.window.reserve);
        assert!(!layout.flash.reserve);
        assert!(!layout.shmem.window.reserve);
        assert!(!layout.ram.reserve);

        // The contract has to render, or the firmware crate gets no `memory.x`.
        let memory_x = layout.linker_memory().unwrap().render();
        assert!(memory_x.contains("m_interrupts"), "{memory_x}");
        assert!(memory_x.contains("m_rsc_tbl_in_tcm"), "{memory_x}");
    }

    #[test]
    fn remoteproc_defaults_and_partial_overrides_lower_once() {
        let manifest = r#"
            [profile]
            chip = "mimx9596xvt"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [endpoints]
            cm7 = "./mcu"

            [remoteproc.cm7]
            ram = { origin = 0x81000000, length = 0x00400000 }
        "#;
        let ir = Session::from_toml(manifest)
            .expect("manifest validates")
            .lower()
            .expect("session lowers");
        let layout = &ir.remoteproc[&"cm7".parse().unwrap()];

        assert_eq!(layout.flash.origin, 0x8000_1000);
        assert_eq!(layout.flash.length, 0x007f_f000);
        assert_eq!(layout.ram.origin, 0x8100_0000);
        assert_eq!(layout.ram.length, 0x0040_0000);
        assert_eq!(layout.vector_section, ".interrupts");
        assert_eq!(layout.vector.name, "m_interrupts");
        assert_eq!(layout.vector.window.length, 0x1000);
        assert_eq!(layout.shmem.name, "m_rsc_tbl_in_dram");
        assert_eq!(layout.shmem.window.origin, 0x8822_0000);
        let memory = layout.linker_memory().unwrap().render();
        assert!(memory.contains("m_interrupts (rx) : ORIGIN = 0x80000000, LENGTH = 0x1000"));
        assert!(memory.contains("FLASH (rx) : ORIGIN = 0x80001000, LENGTH = 0x7ff000"));
        assert!(memory.contains("RAM (rwx) : ORIGIN = 0x81000000, LENGTH = 0x400000"));
        assert!(memory.contains("m_rsc_tbl_in_dram (rw) : ORIGIN = 0x88220000"));
        assert!(memory.contains("} > m_rsc_tbl_in_dram\n} INSERT AFTER .rodata;"));

        let stm = Session::from_toml(
            r#"
                [profile]
                chip = "stm32mp257fak3"
                shared = "./shared"
                deploy_root = "/opt/consortium"

                [endpoints]
                cm33 = "./mcu"
            "#,
        )
        .unwrap()
        .lower()
        .unwrap();
        let stm_layout = &stm.remoteproc[&"cm33".parse().unwrap()];
        assert_eq!(stm_layout.vector_section, ".isr_vectors");
        let stm_memory = stm_layout.linker_memory().unwrap().render();
        assert!(stm_memory.contains("NS_VECTOR_TBL (rwx) : ORIGIN = 0x80100000, LENGTH = 0x1000"));
        assert!(stm_memory.contains("FLASH (rx) : ORIGIN = 0x80101000, LENGTH = 0x7ff000"));
        assert!(stm_memory.contains("SHMEM (rwx) : ORIGIN = 0x812f8000, LENGTH = 0x8000"));
        assert!(stm_memory.contains("RAM (rwx) : ORIGIN = 0x80a00000, LENGTH = 0x100000"));
        assert!(!stm_memory.contains("VIRTIO_SHMEM"));
        assert!(stm_memory.contains("} > SHMEM\n} INSERT AFTER .rodata;"));
    }

    #[test]
    fn example_memory_x_files_match_structured_layouts() {
        let cases = [
            (
                include_str!("../../../examples/melt-pot/imx95/Consortium.toml"),
                "cm7",
                include_str!("../../../examples/melt-pot/imx95/mcu/memory.x"),
            ),
            (
                include_str!("../../../examples/melt-pot/imx93/Consortium.toml"),
                "cm33",
                include_str!("../../../examples/melt-pot/imx93/mcu/memory.x"),
            ),
            (
                include_str!("../../../examples/melt-pot/stm32mp25/Consortium.toml"),
                "cm33",
                include_str!("../../../examples/melt-pot/stm32mp25/mcu/memory.x"),
            ),
        ];
        for (manifest, core, expected) in cases {
            let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
            let layout = &ir.remoteproc[&core.parse().unwrap()];
            assert_eq!(layout.linker_memory().unwrap().render(), expected);
        }
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
            doorbell = "ipcc1"
            pattern = "memory"
            primary = "linux"
            secondary = "cm33"
            party.linux = { chan = 0, type = "Ping" }
            config.memory = { base = 0x00040000, length = 0x00008000 }
            mtu = 128
        "#;

        let session = Session::from_toml(manifest).expect("manifest validates");
        assert!(matches!(session.lower(), Err(CfgError::Manifest(_))));
    }

    #[test]
    fn bella_collects_doorbell_channels_and_core_irqs() {
        let ir = lowered();

        let ipcc1 = ir
            .bella
            .bells
            .iter()
            .find(|bell| bell.name == "ipcc1")
            .expect("ipcc1 doorbell");
        // ipcc1 carries the primary (linux, chan 0) and secondary (cm33, chan 1).
        assert!(ipcc1.channels.contains(&0));
        assert!(ipcc1.channels.contains(&1));
        assert_eq!(ipcc1.id, Some(1));
        // cm33's NVIC line is sourced from the chip database, and lowering
        // resolves its name through the bundled chip map.
        assert_eq!(ipcc1.irq[&cm33()].number, 171);
        assert_eq!(ipcc1.irq[&cm33()].variant.as_deref(), Some("IPCC1_RX"));

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

        // The example declares `[dbg.cm33]` at canonical 0x0006_2000; cm33 sees
        // SRAM through the non-secure alias (0x2000_0000), so its logger embeds
        // 0x2006_2000.
        let ring = ir.dbg.ring(cm33()).expect("cm33 has a debug ring");
        assert_eq!(ring.base, 0x2006_2000);
        assert_eq!(ring.length, 0x2000);
        assert_eq!(ring.mtu, 64);
        assert!(ir.dbg.ring(cm0p()).is_none());

        // The IR resolves the build-time vars the builder injects per core.
        let env: std::collections::HashMap<_, _> = ring.env().into_iter().collect();
        assert_eq!(env["CONSORTIUM_DBG_BASE"], "537272320");
        assert_eq!(env["CONSORTIUM_DBG_LENGTH"], "8192");
    }

    #[test]
    fn lowered_ir_emits_valid_rust() {
        let ir = lowered();
        let uio = quote::format_ident!("uio");
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();

        let tokens = crate::components::ipc_shm::initialize_instances(
            &ir.shuhan,
            VisibleCore::Ap,
            &uio,
            &ty_crate,
            &ir.peripherals.hal_crate,
            &[],
        );
        syn::parse_file(&tokens.to_string()).expect("lowered ShuHan is valid Rust");

        // The full AP resource entry point must parse as a complete module.
        let ap = ir.initialize(VisibleCore::Ap, &ty_crate);
        let ap_file =
            syn::parse_file(&ap.to_string()).expect("lowered AP init module is valid Rust");
        let ap_src = ap.to_string();
        assert!(ap_src.contains("channels !"));
        assert!(ap_src.contains("impl_channel !"));
        assert!(ap_src.contains("fn init"));
        assert!(ap_src.contains("Config"));
        assert!(ap_src.contains("Peripherals"));
        assert!(!ap_src.contains("impl Drop for Peripherals"));
        assert!(!ap_src.contains("impl Drop for Channels"));
        // The example declares `[dbg.cm33]`, so the host console is woven in.
        assert!(ap_src.contains("dbg_console :: spawn"));
        // AP platform resource construction remains fallible.
        assert!(ap_src.contains("ConsortiumInitError"));
        let _ = ap_file;

        // A controller core emits a Secondary instance and synchronous init.
        let cm = ir.initialize(cm33(), &ty_crate);
        let cm_src = cm.to_string();
        syn::parse_file(&cm_src).expect("lowered controller init module is valid Rust");
        assert!(cm_src.contains("fn init"));
        assert!(!cm_src.contains("async fn init"));
        assert!(cm_src.contains("Secondary"));

        let bella = ir
            .bella
            .nvic_enable(&cm33(), &ir.peripherals.hal_crate)
            .expect("cm33 has doorbell irqs");
        let wrapped = quote::quote! { fn _f() { #bella } };
        syn::parse_file(&wrapped.to_string()).expect("lowered Bella is valid Rust");
    }

    #[test]
    fn cached_window_does_not_leave_gap_in_mpu_region_numbers() {
        let manifest = r#"
            [profile]
            chip = "mimx9596xvt"
            shared = "./shared"
            deploy_root = "/opt/consortium"

            [[ipc.channel]]
            name = "cached"
            doorbell = "mu7"
            pattern = "memory"
            primary = "linux"
            secondary = "cm7"
            party.linux = { chan = 0, type = "Ping" }
            party.cm7 = { chan = 1, type = "Pong" }
            config.memory = { base = 0x00488000, length = 0x4000 }
            config.cache = true
            mtu = 128

            [[ipc.channel]]
            name = "uncached"
            doorbell = "mu7"
            pattern = "memory"
            primary = "linux"
            secondary = "cm7"
            party.linux = { chan = 2, type = "Ping" }
            party.cm7 = { chan = 3, type = "Pong" }
            config.memory = { base = 0x0048c000, length = 0x4000 }
            mtu = 128
        "#;
        let ir = Session::from_toml(manifest).unwrap().lower().unwrap();
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
        let cm7 = VisibleCore::Mcore(McoreName::Cm7);
        let source = ir.initialize(cm7, &ty_crate).to_string();

        let disable_cache = source
            .split("disable_cache")
            .nth(1)
            .expect("uncached window emits an MPU carveout");
        assert!(disable_cache.contains("0u8"));
        assert!(!disable_cache.contains("1u8"));
        assert_eq!(source.matches("enable_dcache").count(), 1);
    }
}
