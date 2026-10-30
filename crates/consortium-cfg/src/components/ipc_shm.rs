//! Shared-memory IPC code generation: the largest component, and the one that
//! removes the most hand-written firmware.
//!
//! What a manifest's `[[ipc.channel]]` entries become here:
//!
//! - a [`Channel`] per direction, carrying its validated doorbell channel id and the
//!   message type name;
//! - a [`Transceiver`] pairing two channels over one shared window;
//! - a [`ShuHan`] aggregating every transceiver on a core; [`initialize_instances`]
//!   emits channel markers, instance metadata, and transport installation.
//!
//! # Doorbell ownership across transceivers
//!
//! On a controller core a doorbell is a *singleton* the driver consumes, so it can
//! move exactly once — but two transceivers may legitimately share one doorbell block
//! (disjoint channels of the same MU or IPCC). [`DoorbellSource`] resolves that: the
//! first user takes the constructor parameter ([`Owned`](DoorbellSource::Owned)) and
//! later ones borrow from it ([`Shared`](DoorbellSource::Shared)).
//!
//! The AP has no singletons at all — it reaches its doorbells through UIO `mmap`, so
//! every AP-side source is [`Uio`](DoorbellSource::Uio). A controller core with no
//! bundled chip map, or whose map does not reach the window, falls back to the same
//! address-based construction it has always used.

use core::fmt;
use std::str::FromStr;

use consortium_cfg_common::{
    config::{Codec, VisibleCore},
    database::HardwareDoorbellPeripheralType,
};
use proc_macro2::{Literal, Span, TokenStream};
use quote::format_ident;
use syn::{Ident, Path};

use crate::{
    components::{address::Address, region::Region},
    error::CfgError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    /// doorbell channel id, verified before codegen
    pub doorbell_chan: u8,
    /// doorbell channel this party rings to acknowledge the peer's messages
    ///
    /// `Some` exactly when the manifest sets `config.ack`. Acknowledgement needs a line of
    /// its own because each doorbell line carries one waiter: the receive half is already
    /// parked on the data line, so an acknowledgement arriving there would have nowhere to
    /// land.
    pub ack_chan: Option<u8>,
    /// type name
    pub type_name: String,
    /// buffer name
    pub buffer_name: String,
}

/// The codec type path for `codec`, used by both endpoint construction and the
/// transceiver type signature.
///
/// `scratch` is the channel MTU; only [`RkyvCodec`](consortium_codec::RkyvCodec)
/// is generic over it, sizing the on-stack serialization scratch in the
/// `no_std` (no-`alloc`) encode path. The other codecs ignore it.
fn codec_path(codec: Codec, scratch: u32) -> TokenStream {
    match codec {
        Codec::Postcard => quote::quote! { ::consortium_codec::PostcardCodec },
        Codec::Prost => quote::quote! { ::consortium_codec::ProstCodec },
        Codec::Rkyv => {
            let scratch = Literal::usize_suffixed(scratch as usize);
            quote::quote! { ::consortium_codec::RkyvCodec<#scratch> }
        }
    }
}

/// Generates channel singleton markers, their instance bindings, and the
/// transport installer called by generated `init`.
pub fn initialize_instances(
    shuhan: &ShuHan,
    core: VisibleCore,
    uio_ident: &Ident,
    ty_crate: &Path,
    hal_crate: &str,
    available: &[String],
) -> TokenStream {
    let (doorbell_params, sources) = shuhan.doorbell_sources(core, available);
    let markers = shuhan
        .transceivers
        .iter()
        .map(|t| Ident::new(&t.name.to_ascii_uppercase(), Span::call_site()))
        .collect::<Vec<_>>();

    let instance_impls = shuhan
        .transceivers
        .iter()
        .zip(markers.iter())
        .map(|(t, marker)| {
            let side = t
                .side(core)
                .expect("core must participate in generated transceiver");
            let side_ty = match side {
                DoorbellRingerSide::Primary => quote::quote! { ::consortium_ipc::Primary },
                DoorbellRingerSide::Secondary => quote::quote! { ::consortium_ipc::Secondary },
            };
            // `impl_channel!` takes `$side` and `$doorbell` as independent type
            // parameters: the first is the descriptor role, the second carries
            // the doorbell's own register window.
            let window = t
                .window_side(core)
                .expect("core must participate in generated transceiver");
            let doorbell_ty = t.doorbell.as_type(window);
            let region_ty = t.region_ty(core);
            let codec_ty = codec_path(t.codec, t.mtu);
            let (tx, rx) = match side {
                DoorbellRingerSide::Primary => (&t.channel_primary.1, &t.channel_secondary.1),
                DoorbellRingerSide::Secondary => (&t.channel_secondary.1, &t.channel_primary.1),
            };
            let tx_ty = Ident::new(&tx.type_name, Span::call_site());
            let rx_ty = Ident::new(&rx.type_name, Span::call_site());
            let chan_tx = tx.doorbell_chan;
            let chan_rx = rx.doorbell_chan;
            let mtu = Literal::usize_suffixed(t.mtu as usize);
            // `ack:` is emitted only for a channel the manifest opted in; the plain arm of
            // `impl_channel!` defaults the capability off, so an unchanged manifest keeps
            // producing byte-identical code.
            let ack = match (tx.ack_chan, rx.ack_chan) {
                (Some(ack_tx), Some(ack_rx)) => quote::quote! { ack: (#ack_tx, #ack_rx), },
                _ => quote::quote! {},
            };
            quote::quote! {
                ::consortium_ipc_transport_memory::impl_channel!(
                    #marker,
                    #side_ty,
                    #doorbell_ty,
                    #region_ty,
                    #ty_crate::#tx_ty,
                    #ty_crate::#rx_ty,
                    #codec_ty,
                    #chan_tx,
                    #chan_rx,
                    #mtu,
                    #ack
                );
            }
        })
        .collect::<Vec<_>>();

    let installs = shuhan
        .transceivers
        .iter()
        .zip(sources.iter())
        .zip(markers.iter())
        .map(|((t, source), marker)| {
            let transport = t.build_field_init(core, uio_ident, source.clone(), hal_crate);
            quote::quote! {
                ::consortium_ipc_transport_memory::install::<channels::#marker>(#transport);
            }
        })
        .collect::<Vec<_>>();

    let installer = if core == VisibleCore::Ap {
        quote::quote! {
            fn __consortium_install_channels(
                #uio_ident: &[::std::sync::Arc<::consortium_runtime_app::uio::UioDevice>],
            ) -> ::core::result::Result<(), ::consortium_runtime_app::uio::UioDeviceError> {
                #(#installs)*
                ::core::result::Result::Ok(())
            }
        }
    } else {
        let hal: syn::Path = syn::parse_str(hal_crate).expect("hal_crate must be a valid path");
        let parameters = doorbell_params.iter().map(|singleton| {
            let param = doorbell_param(singleton);
            quote::quote! {
                #param: #hal::Peri<'static, #hal::peripherals::#singleton>
            }
        });
        quote::quote! {
            #[cfg(target_arch = "arm")]
            fn __consortium_install_channels(#(#parameters),*) {
                #(#installs)*
            }
        }
    };

    quote::quote! {
        ::consortium_ipc_transport_memory::channels!(#(#markers),*);
        #(#instance_impls)*
        #installer
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transceiver {
    /// the transceiver name, used for codegen
    pub name: String,
    /// doorbell base address pointer
    pub doorbell_address: Address,
    /// max transmission unit
    pub mtu: u32,
    /// codec name
    pub codec: Codec,
    /// address
    pub region: Region,
    /// primary to secondary channel
    pub channel_primary: (VisibleCore, Channel),
    /// secondary to primary channel
    pub channel_secondary: (VisibleCore, Channel),
    /// type-a doorbell metadata
    pub doorbell: DoorbellMeta,
    /// Which of the doorbell's per-side register windows each party drives,
    /// primary first.
    ///
    /// Distinct from the primary/secondary *role* above: the role is the
    /// descriptor protocol's, chosen by the manifest, while the window is a
    /// fact read from the chip database. They coincide on i.MX 93 and i.MX 95
    /// (A-core on MUA, M-core on MUB) for the documented `primary = "linux"`,
    /// but a manifest that swaps the roles keeps the windows, so conflating the
    /// two fails `MuDoorbell::new`'s `Instance<Side = S>` bound.
    pub doorbell_window: (DoorbellRingerSide, DoorbellRingerSide),
}

impl Transceiver {
    pub fn max_channels(&self) -> u32 {
        // Length <= Descriptor + message slot * max channels.
        let total_len = u64::from(self.region.length);
        let slot_base = consortium_ipc_transport_memory::slot_offset(0, self.mtu as usize) as u64;
        let slot_size = u64::from(consortium_ipc_transport_memory::slot::SLOT_HEADER as u32)
            + u64::from(self.mtu);
        let first_slot_end = slot_base.saturating_add(slot_size);
        if total_len < first_slot_end {
            return 0;
        }

        let stride = consortium_ipc_transport_memory::slot_stride(self.mtu as usize) as u64;
        let slots = 1 + (total_len - first_slot_end) / stride;
        let slots = slots.min(consortium_ipc_transport_memory::CHANNEL_NUM_MAX as u64);
        (slots / 2 * 2) as u32
    }

    /// The descriptor role `core` plays: which end of the shared-memory
    /// protocol it is, and which channel id it transmits on.
    fn side(&self, core: VisibleCore) -> Option<DoorbellRingerSide> {
        if self.channel_primary.0 == core {
            Some(DoorbellRingerSide::Primary)
        } else if self.channel_secondary.0 == core {
            Some(DoorbellRingerSide::Secondary)
        } else {
            None
        }
    }

    /// The doorbell register window `core` drives.
    ///
    /// Everything that names the doorbell type, its singleton, or its interrupt
    /// entry point must use this rather than [`side`](Self::side) — see
    /// [`doorbell_window`](Self::doorbell_window).
    fn window_side(&self, core: VisibleCore) -> Option<DoorbellRingerSide> {
        match self.side(core)? {
            DoorbellRingerSide::Primary => Some(self.doorbell_window.0),
            DoorbellRingerSide::Secondary => Some(self.doorbell_window.1),
        }
    }

    /// The shared-memory region type for this transceiver, in type position.
    ///
    /// The AP's is a `UioRegion`, which owns the UIO devices behind the link;
    /// controller cores embed the window directly.
    fn region_ty(&self, core: VisibleCore) -> TokenStream {
        match (core, self.region.cache) {
            (VisibleCore::Ap, _) => quote::quote! { ::consortium_runtime_app::uio::UioRegion },
            (_, false) => quote::quote! { ::consortium_shared_memory::regions::UncachedRegion },
            (_, true) => quote::quote! { ::consortium_shared_memory::regions::CachedRegion },
        }
    }

    /// Build the region, doorbell, channel ids, and unsplit transport installed
    /// by synchronous generated init. This performs no peer handshake.
    pub fn build_field_init(
        &self,
        core: VisibleCore,
        uio_ident: &Ident,
        source: DoorbellSource,
        hal_crate: &str,
    ) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");

        let region_ident = format_ident!("region");
        let region_init = self.region.initialize(core, uio_ident);
        // On the AP the region also owns the doorbell's UIO device: the doorbell's
        // register window is mapped from it, and the transport holding the region
        // rings and waits through that window. Controller cores embed the region
        // through an `unsafe` constructor whose obligation the database discharges.
        let region_stmt = if core == VisibleCore::Ap {
            let doorbell_uio = self.doorbell_address.ap.uio_id;
            quote::quote! {
                let #region_ident = #region_init.keep(#uio_ident[#doorbell_uio].clone());
            }
        } else {
            quote::quote! {
                /// SAFETY: automatically generated by Consortium after matching with database
                let #region_ident = #region_init;
            }
        };

        let doorbell_ident = format_ident!("doorbell");
        // The IRQ service binds to the *doorbell's* UIO device, not the
        // shared-memory region's: only the doorbell device is declared with an
        // interrupt, so only it is opened with `enable_irq` and runs an
        // `irq_loop`. Binding the service to the region device registers it on a
        // handle whose IRQ listener never runs, and the AP never wakes.
        let window = self
            .window_side(core)
            .expect("core must be either primary or secondary for the transceiver");
        let doorbell_code = self.doorbell.initialize(&DoorbellSite {
            side: window,
            core,
            uio_ident,
            uio_id: self.doorbell_address.ap.uio_id,
            base: self.doorbell_address.memory_lookup(core, uio_ident),
            ident: &doorbell_ident,
            source: &source,
            hal_crate,
        });

        let chan_id_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1.doorbell_chan,
        };
        let chan_id_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_primary.1.doorbell_chan,
        };

        // Acknowledgement mirrors the data lines: this core rings its own, and waits on
        // the peer's.
        let (own, peer) = match side {
            DoorbellRingerSide::Primary => (&self.channel_primary.1, &self.channel_secondary.1),
            DoorbellRingerSide::Secondary => (&self.channel_secondary.1, &self.channel_primary.1),
        };
        let (constructor, ack_channels) = match (own.ack_chan, peer.ack_chan) {
            (Some(ack_tx), Some(ack_rx)) => (
                format_ident!("new_acked"),
                quote::quote! {
                    /// SAFETY: doorbell channel is verified to be valid in the config.
                    let chan_ack_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(#ack_tx) };
                    /// SAFETY: doorbell channel is verified to be valid in the config.
                    let chan_ack_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(#ack_rx) };
                },
            ),
            _ => (format_ident!("new"), quote::quote! {}),
        };
        let ack_arguments = match (own.ack_chan, peer.ack_chan) {
            (Some(_), Some(_)) => quote::quote! { chan_ack_tx, chan_ack_rx, },
            _ => quote::quote! {},
        };

        let transport_side_ty = match side {
            DoorbellRingerSide::Primary => quote::quote! { ::consortium_ipc::Primary },
            DoorbellRingerSide::Secondary => quote::quote! { ::consortium_ipc::Secondary },
        };
        quote::quote! {{
            #region_stmt
            #doorbell_code
            /// SAFETY: doorbell channel is verified to be valid in the config.
            let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_tx) };
            /// SAFETY: doorbell channel is verified to be valid in the config.
            let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_rx) };
            #ack_channels
            /// SAFETY: the region is this channel's validated shared-memory
            /// window, whose offset 0x00 holds the descriptor, and the
            /// channels/doorbell are this side's.
            unsafe {
                ::consortium_ipc_transport_memory::SharedMemoryTransport::<#transport_side_ty, _, _>::#constructor(
                    chan_tx,
                    chan_rx,
                    #ack_arguments
                    #doorbell_ident,
                    #region_ident,
                )
            }
        }}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoorbellMeta {
    /// doorbell type
    pub r#type: HardwareDoorbellPeripheralType,
    /// id
    pub id: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorbellRingerSide {
    Primary,
    Secondary,
}

/// Everything one transceiver's doorbell construction needs at its emit site.
///
/// `uio_id` must index the UIO device that *owns the doorbell's interrupt* —
/// the same device `base` is mapped from. Only devices declaring an `irq` are
/// opened with `enable_irq`, so only they run an `irq_loop` that can dispatch
/// the bound service.
pub struct DoorbellSite<'a> {
    /// Which of the doorbell's register windows the core being generated
    /// drives — not its descriptor role. See
    /// [`Transceiver::doorbell_window`](Transceiver::doorbell_window).
    pub side: DoorbellRingerSide,
    /// The core being generated.
    pub core: VisibleCore,
    /// Ident of the opened-UIO-device slice in scope (AP only).
    pub uio_ident: &'a Ident,
    /// Index of the doorbell's own UIO device (AP only).
    pub uio_id: usize,
    /// Expression evaluating to the doorbell's register-window base (AP only).
    pub base: TokenStream,
    /// Ident to bind the constructed doorbell to.
    pub ident: &'a Ident,
    /// Where the doorbell handle comes from on this core.
    pub source: &'a DoorbellSource,
    /// Chip HAL crate path, for the `steal` fallback on a shared block.
    pub hal_crate: &'a str,
}

/// Where one transceiver's doorbell handle comes from on the core being
/// generated.
#[derive(Debug, Clone)]
pub enum DoorbellSource {
    /// Linux/AP: an `mmap` of the doorbell's UIO device, which has no
    /// peripheral singleton.
    Uio,
    /// Controller core, first user of this block: takes the singleton passed to
    /// the generated transport installer.
    Owned(Ident),
    /// Controller core, a later transceiver on a block another one already
    /// owns: steals the singleton, since the two drive disjoint channels.
    Shared(Ident),
}

impl DoorbellMeta {
    /// The chip HAL peripheral singleton naming this doorbell's register window
    /// on `side`, spelled as the chip database does.
    ///
    /// MU windows are per-side (`MU7_MUA` / `MU7_MUB`); HSEM and IPCC expose one
    /// block with a bank per core, so their singleton is side-independent and
    /// the side stays a type parameter of the doorbell.
    pub fn singleton(&self, side: DoorbellRingerSide) -> Ident {
        let id = self.id.unwrap_or(1);
        match self.r#type {
            HardwareDoorbellPeripheralType::Mu => {
                let window = match side {
                    DoorbellRingerSide::Primary => "MUA",
                    DoorbellRingerSide::Secondary => "MUB",
                };
                format_ident!("MU{id}_{window}")
            }
            HardwareDoorbellPeripheralType::Hsem => format_ident!("HSEM"),
            HardwareDoorbellPeripheralType::Ipcc => format_ident!("IPCC{id}"),
        }
    }

    /// The doorbell's register-model path (`bell::ipcc::Ipcc<Side, N>`) and its
    /// `BellA` alias (`bell::ipcc::IpccDoorbell`), in the transport crate.
    ///
    /// Every hardware doorbell is a Type-A bell, so the names follow from the type
    /// alone: module `bell::<type>`, register model `<Type>`, doorbell
    /// `<Type>Doorbell`.
    fn idents(&self, side: DoorbellRingerSide) -> (TokenStream, Ident, Ident) {
        let name = self.r#type.to_string();
        let module = format_ident!("{}", name.to_lowercase());
        let side_ident = format_ident!(
            "{}",
            match side {
                DoorbellRingerSide::Primary => "Primary",
                DoorbellRingerSide::Secondary => "Secondary",
            }
        );
        (
            quote::quote! { ::consortium_ipc_transport_memory::bell::#module },
            format_ident!("{name}Doorbell"),
            side_ident,
        )
    }

    /// The doorbell type in expression/path position (turbofish form), for
    /// `<doorbell>::new(...)` construction.
    pub fn as_path(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let (module, type_ident, side_ident) = self.idents(side);
        quote::quote! {
            #module::#type_ident::<::consortium_ipc::#side_ident, #id>
        }
    }

    /// The vector body that services this doorbell's interrupt from `side`.
    ///
    /// One generic entry point for every block, parameterized with its register
    /// model: it masks the asserted lines before waking the futures waiting on
    /// them. The register model derives the window from the transport's vetted
    /// address table, so no MMIO base ever appears in generated code.
    pub fn service_call(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let (module, _, side_ident) = self.idents(side);
        let model = format_ident!("{}", self.r#type.to_string());
        quote::quote! {
            ::consortium_ipc_transport_memory::bell::service_interrupt::<
                #module::#model<::consortium_ipc::#side_ident, #id>
            >();
        }
    }

    /// The doorbell type in type position (no turbofish), for naming the
    /// doorbell inside a transport type signature.
    pub fn as_type(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let (module, type_ident, side_ident) = self.idents(side);
        quote::quote! {
            #module::#type_ident<::consortium_ipc::#side_ident, #id>
        }
    }

    /// Emit the doorbell construction, and on the AP its UIO IRQ binding.
    pub fn initialize(&self, site: &DoorbellSite<'_>) -> TokenStream {
        let DoorbellSite {
            side,
            core,
            uio_ident,
            uio_id: doorbell_uio_id,
            base: doorbell_base,
            ident: doorbell_ident,
            source,
            hal_crate,
        } = site;
        let (side, core) = (*side, *core);
        let doorbell = self.as_path(side);

        // On the AP the UIO IRQ fanout runs a per-doorbell service on every M→A
        // interrupt. Every Type-A bell binds its own IRQ entry point — cloned so
        // the original can still move into the transport — which masks the
        // asserted lines *before* waking, so a level-sensitive line does not
        // immediately re-fire the UIO IRQ.
        let binding_uio_stmt = (core == VisibleCore::Ap).then(|| {
            quote::quote! {
                let __service_db = #doorbell_ident.clone();
                #uio_ident[#doorbell_uio_id].bind_service(::std::sync::Arc::new(
                    move || __service_db.service_interrupt(),
                ));
            }
        });

        // The AP reaches the doorbell through an `mmap` of its UIO device, which
        // has no peripheral singleton; controller cores own the real peripheral,
        // so they hand the doorbell its singleton and never name an address.
        let construct = match source {
            DoorbellSource::Uio => quote::quote! {
                /// SAFETY: the base is the UIO-mapped doorbell register window,
                /// resolved from the validated Consortium hardware database.
                let #doorbell_ident = unsafe { #doorbell::new_mapped(#doorbell_base) };
            },
            DoorbellSource::Owned(param) => quote::quote! {
                let #doorbell_ident = #doorbell::new(#param);
            },
            DoorbellSource::Shared(singleton) => {
                let hal: syn::Path =
                    syn::parse_str(hal_crate).expect("hal_crate must be a valid path");
                quote::quote! {
                    /// SAFETY: another transceiver on this core already owns this
                    /// doorbell block; the two address disjoint channels of it, as
                    /// the manifest's duplicate-channel validation guarantees.
                    let #doorbell_ident = #doorbell::new(unsafe {
                        #hal::peripherals::#singleton::steal()
                    });
                }
            }
        };

        quote::quote! {
            #construct
            #binding_uio_stmt
        }
    }
}

impl FromStr for DoorbellMeta {
    type Err = CfgError;

    /// Parses a doorbell peripheral name into a typed initiator.
    ///
    /// The name is an alphabetic peripheral type optionally followed by a
    /// numeric instance id, e.g. `mu7` -> `Mu`/`Some(7)`, `hsem` ->
    /// `Hsem`/`None`, `ipcc1` -> `Ipcc`/`Some(1)`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.find(|ch: char| ch.is_ascii_digit()).unwrap_or(s.len());
        let (prefix, suffix) = s.split_at(split);

        let r#type = match prefix {
            "mu" => HardwareDoorbellPeripheralType::Mu,
            "hsem" => HardwareDoorbellPeripheralType::Hsem,
            "ipcc" => HardwareDoorbellPeripheralType::Ipcc,
            _ => return Err(CfgError::Manifest(format!("unknown doorbell type '{s}'"))),
        };

        let id = if suffix.is_empty() {
            None
        } else {
            Some(
                suffix
                    .parse()
                    .map_err(|_| CfgError::Manifest(format!("invalid doorbell id in '{s}'")))?,
            )
        };

        Ok(Self { r#type, id })
    }
}

impl fmt::Display for DoorbellMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type.to_string().to_lowercase())?;
        if let Some(id) = self.id {
            write!(f, "{id}")?;
        }
        Ok(())
    }
}

/// The transport-installer parameter that carries `singleton`.
fn doorbell_param(singleton: &Ident) -> Ident {
    format_ident!("__db_{}", singleton.to_string().to_lowercase())
}

/// SHared Memory Unified, Hardware abstraction node.
#[derive(Debug, Clone)]
pub struct ShuHan {
    transceivers: Vec<Transceiver>,
}

impl ShuHan {
    /// Builds a node from its transceivers.
    pub fn new(transceivers: Vec<Transceiver>) -> Self {
        Self { transceivers }
    }

    /// The transceivers this node generates.
    pub fn transceivers(&self) -> &[Transceiver] {
        &self.transceivers
    }

    /// Where each transceiver's doorbell comes from on `core`, plus the distinct
    /// singletons a controller core's transport installer must be handed.
    ///
    /// Two transceivers may share one doorbell block (disjoint channels of the
    /// same MU or IPCC) and a singleton moves only once, so the first user takes
    /// the parameter and later ones steal it. The AP has no singletons at all —
    /// it reaches its doorbells through UIO `mmap`.
    fn doorbell_sources(
        &self,
        core: VisibleCore,
        available: &[String],
    ) -> (Vec<Ident>, Vec<DoorbellSource>) {
        let mut owned: Vec<Ident> = Vec::new();
        let sources = self
            .transceivers
            .iter()
            .map(|t| {
                if core == VisibleCore::Ap {
                    return DoorbellSource::Uio;
                }
                let window = t
                    .window_side(core)
                    .expect("core must be either primary or secondary for the transceiver");
                let singleton = t.doorbell.singleton(window);
                // A core with no bundled chip map (or one whose map does not
                // reach this window) has no singleton to hand over, so it keeps
                // the address-based construction it has always used.
                if !available
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&singleton.to_string()))
                {
                    return DoorbellSource::Uio;
                }
                if owned.contains(&singleton) {
                    return DoorbellSource::Shared(singleton);
                }
                owned.push(singleton.clone());
                DoorbellSource::Owned(doorbell_param(&singleton))
            })
            .collect();
        (owned, sources)
    }

    /// The distinct doorbell singletons `core` must hand to
    /// the generated transport installer, in parameter order.
    pub fn doorbell_singletons(&self, core: VisibleCore, available: &[String]) -> Vec<Ident> {
        self.doorbell_sources(core, available).0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::components::address::ApAddress;
    use consortium_cfg_common::database::MemoryRegionType;
    use quote::format_ident;

    fn channel(doorbell_chan: u8) -> Channel {
        Channel {
            doorbell_chan,
            ack_chan: None,
            type_name: "Ping".to_string(),
            buffer_name: "ping_buf".to_string(),
        }
    }

    fn acked_channel(doorbell_chan: u8, ack_chan: u8) -> Channel {
        Channel {
            ack_chan: Some(ack_chan),
            ..channel(doorbell_chan)
        }
    }

    fn transceiver(length: u32, mtu: u32) -> Transceiver {
        let cm33 = VisibleCore::Mcore(consortium_cfg_common::config::McoreName::Cm33);
        let address = Address {
            ap: ApAddress {
                uio_id: 0,
                mmap_id: 0,
            },
            cp: HashMap::from([(cm33, 0x3400_0000)]),
        };
        Transceiver {
            name: "ping".to_string(),
            doorbell_address: address.clone(),
            mtu,
            codec: Codec::Postcard,
            region: Region {
                r#type: MemoryRegionType::Sram,
                cache: false,
                address,
                length,
            },
            channel_primary: (VisibleCore::Ap, channel(0)),
            channel_secondary: (cm33, channel(1)),
            doorbell: DoorbellMeta {
                r#type: HardwareDoorbellPeripheralType::Ipcc,
                id: Some(1),
            },
            doorbell_window: (DoorbellRingerSide::Primary, DoorbellRingerSide::Secondary),
        }
    }

    /// Emitted bring-up must reach for `new_acked` and hand it both extra lines, mirrored
    /// per side: each core rings its own acknowledgement line and waits on the peer's.
    /// Getting that mapping backwards produces code that compiles and then never wakes.
    #[test]
    fn acked_channels_emit_the_acknowledging_constructor() {
        let cm33 = VisibleCore::Mcore(consortium_cfg_common::config::McoreName::Cm33);
        let mut transceiver = transceiver(0x4000, 128);
        transceiver.channel_primary.1 = acked_channel(0, 2);
        transceiver.channel_secondary.1 = acked_channel(1, 3);

        let uio = format_ident!("uio");
        for (core, own, peer) in [(VisibleCore::Ap, 2u8, 3u8), (cm33, 3, 2)] {
            let source = match core {
                VisibleCore::Ap => DoorbellSource::Uio,
                _ => DoorbellSource::Owned(format_ident!("__db_ipcc1")),
            };
            let code = transceiver
                .build_field_init(core, &uio, source, "::consortium_hal_stm32mp2")
                .to_string();

            assert!(code.contains("new_acked"), "{core}: {code}");
            assert!(
                code.contains(&format!(
                    "chan_ack_tx = unsafe {{ :: consortium_ipc :: Chan :: new_unchecked ({own}u8)"
                )),
                "{core} must ring its own line: {code}"
            );
            assert!(
                code.contains(&format!(
                    "chan_ack_rx = unsafe {{ :: consortium_ipc :: Chan :: new_unchecked ({peer}u8)"
                )),
                "{core} must wait on the peer's line: {code}"
            );
        }
    }

    /// The additive half of the same claim: without `config.ack` nothing changes.
    #[test]
    fn plain_channels_still_emit_the_plain_constructor() {
        let code = transceiver(0x4000, 128)
            .build_field_init(
                VisibleCore::Ap,
                &format_ident!("uio"),
                DoorbellSource::Uio,
                "::consortium_hal_stm32mp2",
            )
            .to_string();

        assert!(!code.contains("new_acked"));
        assert!(!code.contains("chan_ack"));
    }

    const SLOT_BASE: u32 = consortium_ipc_transport_memory::slot_offset(0, 4) as u32;
    const SLOT_STRIDE: u32 = consortium_ipc_transport_memory::slot_stride(4) as u32;

    #[test]
    fn max_channels_is_zero_when_only_descriptor_fits() {
        // No room beyond the descriptor: no message slots can be carved out.
        assert_eq!(transceiver(SLOT_BASE, 4).max_channels(), 0);
    }

    #[test]
    fn max_channels_counts_slot_pairs() {
        // Each 16-byte logical slot occupies one 64-byte cache-line stride.
        // Three pairs of room => 6 channels.
        let pair_len = SLOT_STRIDE * 2;
        assert_eq!(transceiver(SLOT_BASE + pair_len * 3, 4).max_channels(), 6);
    }

    #[test]
    fn max_channels_counts_pair_without_trailing_padding() {
        let exact = consortium_ipc_transport_memory::required_region_len(2, 128) as u32;
        assert_eq!(exact, 0x24c);
        assert_eq!(transceiver(exact, 128).max_channels(), 2);
        assert_eq!(transceiver(exact - 1, 128).max_channels(), 0);
    }

    #[test]
    fn max_channels_saturates_at_channel_num_max() {
        let max = consortium_ipc_transport_memory::CHANNEL_NUM_MAX as u32;
        assert_eq!(transceiver(u32::MAX, 4).max_channels(), max);
    }

    #[test]
    fn cached_region_type_is_controller_only() {
        let mut transceiver = transceiver(SLOT_BASE + 4096, 128);
        transceiver.region.cache = true;
        let cm33 = VisibleCore::Mcore(consortium_cfg_common::config::McoreName::Cm33);

        assert!(
            transceiver
                .region_ty(cm33)
                .to_string()
                .contains("CachedRegion")
        );
        assert!(
            transceiver
                .region_ty(VisibleCore::Ap)
                .to_string()
                .contains("UioRegion")
        );
    }

    /// Only UIO devices declaring an `irq` are opened with `enable_irq` and run
    /// an `irq_loop`, and that is the doorbell's device — not the shared-memory
    /// region's. Binding the service to the region device registers it on a
    /// handle whose IRQ listener never runs, so the AP parks in `wait` forever
    /// while the controller keeps ringing.
    #[test]
    fn ap_binds_doorbell_service_to_the_doorbell_uio_device() {
        let mut t = transceiver(SLOT_BASE + 4096, 128);
        t.region.address.ap = ApAddress {
            uio_id: 1,
            mmap_id: 0,
        };
        t.doorbell_address.ap = ApAddress {
            uio_id: 2,
            mmap_id: 0,
        };
        // `transceiver` uses IPCC, a masking doorbell, so the AP binds its
        // masking service.
        let uio = format_ident!("uio");
        let init = t
            .build_field_init(
                VisibleCore::Ap,
                &uio,
                DoorbellSource::Uio,
                "::consortium_hal_stm32mp2",
            )
            .to_string();

        assert!(init.contains("uio [2usize] . bind_service"), "{init}");
        assert!(!init.contains("uio [1usize] . bind_service"), "{init}");
        // The region and the doorbell window still map from their own devices,
        // and the region owns the doorbell's device so its window stays mapped.
        assert!(
            init.contains(
                "UioRegion :: zeroed (& uio [1usize] , 0usize) ? . keep (uio [2usize] . clone ())"
            ),
            "{init}"
        );
        assert!(init.contains("uio [2usize] . mmap"), "{init}");
    }

    /// HSEM is a Type-A bell like the others, so the AP binds its masking
    /// service rather than a plain notifier.
    #[test]
    fn ap_binds_the_masking_service_for_hsem() {
        let mut t = transceiver(SLOT_BASE + 4096, 128);
        t.doorbell_address.ap = ApAddress {
            uio_id: 2,
            mmap_id: 0,
        };
        t.doorbell = DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Hsem,
            id: None,
        };

        let uio = format_ident!("uio");
        let init = t
            .build_field_init(
                VisibleCore::Ap,
                &uio,
                DoorbellSource::Uio,
                "::consortium_hal_stm32mp2",
            )
            .to_string();

        assert!(init.contains("uio [2usize] . bind_service"), "{init}");
        assert!(init.contains("service_interrupt ()"), "{init}");
        assert!(!init.contains("bind_notifier"), "{init}");
    }

    #[test]
    fn doorbell_path_resolves_crate_type_and_side() {
        let doorbell = DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Mu,
            id: Some(7),
        };

        let tokens = doorbell.as_path(DoorbellRingerSide::Primary).to_string();
        assert!(tokens.contains(":: consortium_ipc_transport_memory :: bell :: mu :: MuDoorbell"));
        assert!(tokens.contains(":: consortium_ipc :: Primary"));
        assert!(tokens.contains("7usize"));
    }

    #[test]
    fn doorbell_path_defaults_missing_id_to_one() {
        let doorbell = DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Hsem,
            id: None,
        };

        let tokens = doorbell.as_path(DoorbellRingerSide::Secondary).to_string();
        assert!(
            tokens.contains(":: consortium_ipc_transport_memory :: bell :: hsem :: HsemDoorbell")
        );
        assert!(tokens.contains(":: consortium_ipc :: Secondary"));
        assert!(tokens.contains("1usize"));
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for s in ["mu7", "hsem", "ipcc1", "mu0"] {
            let doorbell: DoorbellMeta = s.parse().unwrap();
            assert_eq!(doorbell.to_string(), s);
        }
    }

    #[test]
    fn rejects_invalid_doorbell_id() {
        assert!("mu999".parse::<DoorbellMeta>().is_err());
    }

    #[test]
    fn parses_typed_doorbell_with_id() {
        assert_eq!(
            "mu7".parse::<DoorbellMeta>().unwrap(),
            DoorbellMeta {
                r#type: HardwareDoorbellPeripheralType::Mu,
                id: Some(7),
            }
        );
        assert_eq!(
            "ipcc1".parse::<DoorbellMeta>().unwrap(),
            DoorbellMeta {
                r#type: HardwareDoorbellPeripheralType::Ipcc,
                id: Some(1),
            }
        );
    }

    #[test]
    fn parses_typed_doorbell_without_id() {
        assert_eq!(
            "hsem".parse::<DoorbellMeta>().unwrap(),
            DoorbellMeta {
                r#type: HardwareDoorbellPeripheralType::Hsem,
                id: None,
            }
        );
    }

    #[test]
    fn rejects_unknown_doorbell_type() {
        assert!("foo3".parse::<DoorbellMeta>().is_err());
    }
}
