use core::fmt;
use std::str::FromStr;

use consortium_cfg_common::{
    config::{Codec, VisibleCore},
    database::{HardwareDoorbellPeripheralType, MemoryRegionType},
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
    /// type name
    pub type_name: String,
    /// buffer name
    pub buffer_name: String,
}

/// The two fragments a channel emits: a module-level scratch-buffer `static`
/// and the `Channel::new(...)` expression that borrows it.
///
/// The `static` is kept separate from the expression so callers can hoist it to
/// module scope (see [`ShuHan::initialize`]) rather than nesting it inside the
/// `connect` method body.
pub struct ChannelInit {
    /// The module-level `static mut <BUF>: [u8; MTU]` declaration.
    pub buffer: TokenStream,
    /// The `::consortium_ipc::Channel::<...>::new(...)` expression, which
    /// borrows the [`buffer`](Self::buffer) static by name.
    pub channel: TokenStream,
}

impl Channel {
    pub fn initialize(
        &self,
        dir: Direction,
        chan_ident: &Ident,
        transport_ident: &Ident,
        codec: Codec,
        mtu: u32,
        ty_crate: &Path,
    ) -> ChannelInit {
        let direction_ty = match dir {
            Direction::Tx => quote::quote! { ::consortium_ipc::Tx },
            Direction::Rx => quote::quote! { ::consortium_ipc::Rx },
        };

        let type_ident = format_ident!("{}", self.type_name);
        let codec_token = codec_path(codec, mtu);

        // The MTU-sized scratch buffer the channel encodes/decodes through. A
        // module-level `static` gives it a `'static` lifetime without a heap or
        // a caller-provided buffer; it is borrowed exactly once during init.
        let buf_static = format_ident!("{}", self.buffer_name.to_uppercase());
        let mtu = Literal::usize_suffixed(mtu as usize);
        let buffer = quote::quote! {
            static mut #buf_static: [u8; #mtu] = [0u8; #mtu];
        };

        let channel = quote::quote! {
            ::consortium_ipc::Channel::<#direction_ty, #ty_crate::#type_ident, _, #codec_token>::new(
                #chan_ident,
                #transport_ident,
                {
                    /// SAFETY: module-level static scratch buffer, borrowed once
                    /// during initialization and never aliased afterwards.
                    unsafe { &mut *::core::ptr::addr_of_mut!(#buf_static) }
                },
            )
        };

        ChannelInit { buffer, channel }
    }
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

pub enum Direction {
    Tx,
    Rx,
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
}

impl Transceiver {
    pub fn max_channels(&self) -> u32 {
        // Length <= Descriptor + message slot * max channels.
        let total_len = self.region.length;
        let desc_len = consortium_ipc_transport_memory::DESCRIPTOR_LENGTH as u32;
        let header_len = consortium_ipc_transport_memory::slot::SLOT_HEADER as u32;

        let available = total_len.saturating_sub(desc_len);

        let slot_len = header_len.saturating_add(self.mtu);

        let pair_len = slot_len.saturating_mul(2);

        if pair_len == 0 {
            return 0;
        }

        let pairs = available / pair_len;
        pairs
            .saturating_mul(2)
            .min(consortium_ipc_transport_memory::CHANNEL_NUM_MAX as u32)
    }

    /// get the side
    fn side(&self, core: VisibleCore) -> Option<DoorbellRingerSide> {
        if self.channel_primary.0 == core {
            Some(DoorbellRingerSide::Primary)
        } else if self.channel_secondary.0 == core {
            Some(DoorbellRingerSide::Secondary)
        } else {
            None
        }
    }

    /// The unsplit transport type for `core`, naming every generic concretely.
    ///
    /// This is both the disconnected `IpcMemoryEndpoints` field type and the
    /// receiver of the descriptor handshake associated functions in
    /// [`connect_field_init`](Self::connect_field_init).
    pub fn transport_ty(&self, core: VisibleCore) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");
        let doorbell_type = self.doorbell.as_type(side);
        let region_type = self.region_ty();
        let side_type = match side {
            DoorbellRingerSide::Primary => quote::quote! { ::consortium_ipc::Primary },
            DoorbellRingerSide::Secondary => quote::quote! { ::consortium_ipc::Secondary },
        };
        quote::quote! {
            ::consortium_ipc_transport_memory::SharedMemoryTransport<#side_type, #doorbell_type, #region_type>
        }
    }

    /// The shared-memory region type for this transceiver, in type position.
    fn region_ty(&self) -> TokenStream {
        match self.region.r#type {
            MemoryRegionType::Sram => {
                quote::quote! { ::consortium_shared_memory::regions::SramRegion }
            }
        }
    }

    /// The disconnected `IpcMemoryEndpoints` field initializer: builds the
    /// region, doorbell, channel ids, and unsplit transport, evaluating to the
    /// transport. The descriptor handshake is deferred to
    /// [`connect`](super::ipc_shm::ShuHan), so this performs no IPC.
    pub fn build_field_init(&self, core: VisibleCore, uio_ident: &Ident) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");

        let region_ident = format_ident!("region");
        let region_init = self.region.initialize(core, uio_ident);

        let doorbell_ident = format_ident!("doorbell");
        let doorbell_code = self.doorbell.initialize(
            side,
            core,
            uio_ident,
            self.region.address.ap.uio_id,
            self.doorbell_address.memory_lookup(core, uio_ident),
            &doorbell_ident,
        );

        let chan_id_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1.doorbell_chan,
        };
        let chan_id_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_primary.1.doorbell_chan,
        };

        let transport_side_ty = match side {
            DoorbellRingerSide::Primary => quote::quote! { ::consortium_ipc::Primary },
            DoorbellRingerSide::Secondary => quote::quote! { ::consortium_ipc::Secondary },
        };
        // The transport's descriptor base is the shared-memory region base
        // (offset 0x00), not the doorbell register window.
        let transport_address = self.region.address.memory_lookup(core, uio_ident);

        quote::quote! {{
            /// SAFETY: automatically generated by Consortium after matching with database
            let #region_ident = #region_init;
            #doorbell_code
            /// SAFETY: doorbell channel is verified to be valid in the config.
            let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_tx) };
            /// SAFETY: doorbell channel is verified to be valid in the config.
            let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_rx) };
            /// SAFETY: the base is offset 0x00 of the validated shared-memory
            /// region, and the channels/doorbell/region are this side's.
            unsafe {
                ::consortium_ipc_transport_memory::SharedMemoryTransport::<#transport_side_ty, _, _>::new(
                    chan_tx,
                    chan_rx,
                    #doorbell_ident,
                    #region_ident,
                    (#transport_address) as *mut u8,
                )
            }
        }}
    }

    /// The connected `IpcMemoryEndpointsConnected` field initializer: runs the
    /// descriptor handshake on the stored transport (`descriptor_init` when this
    /// side is Primary, `descriptor_ack` when Secondary), then splits it and
    /// builds the typed [`Transceiver`]. Evaluates to the transceiver; uses
    /// `?` against `ConsortiumConnectError` and `.await`.
    ///
    /// Returns `(buffers, expr)`: `buffers` holds the two channels' module-level
    /// scratch-buffer `static`s, which [`ShuHan::initialize`] emits at module
    /// scope; `expr` is the field-initializer expression that borrows them.
    pub fn connect_field_init(
        &self,
        core: VisibleCore,
        field_ident: &Ident,
        ty_crate: &Path,
    ) -> (TokenStream, TokenStream) {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");
        let transport_ty = self.transport_ty(core);

        let channel_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1,
        };
        let channel_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1,
            DoorbellRingerSide::Secondary => &self.channel_primary.1,
        };

        let t_tx = format_ident!("t_tx");
        let t_rx = format_ident!("t_rx");
        let chan_tx = format_ident!("chan_tx");
        let chan_rx = format_ident!("chan_rx");
        let tx = channel_tx.initialize(
            Direction::Tx,
            &chan_tx,
            &t_tx,
            self.codec,
            self.mtu,
            ty_crate,
        );
        let rx = channel_rx.initialize(
            Direction::Rx,
            &chan_rx,
            &t_rx,
            self.codec,
            self.mtu,
            ty_crate,
        );
        // The two scratch-buffer statics are hoisted to module scope by the
        // caller; only the `Channel::new(...)` expressions stay in the body.
        let (tx_buffer, channel_tx_init) = (tx.buffer, tx.channel);
        let (rx_buffer, channel_rx_init) = (rx.buffer, rx.channel);

        // A bidirectional transceiver carves out two channel-table slots: slot 0
        // is PrimaryToSecondary (TX), slot 1 is SecondaryToPrimary (RX).
        let transfer_size = Literal::usize_suffixed(self.mtu as usize);
        // Map any doorbell-generic transport error into the no_std-safe connect
        // error, dropping the doorbell payload (whose type varies per channel).
        let map_err = quote::quote! {
            |__e| match __e {
                ::consortium_ipc_transport_memory::Error::Descriptor(__d) => {
                    ConsortiumConnectError::Descriptor(__d)
                }
                ::consortium_ipc_transport_memory::Error::Doorbell(_) => {
                    ConsortiumConnectError::Doorbell
                }
                _ => ConsortiumConnectError::Other,
            }
        };
        let handshake = match side {
            DoorbellRingerSide::Primary => quote::quote! {
                /// SAFETY: base covers the descriptor; this side is Primary, so it
                /// writes the descriptor and waits for the secondary's ack.
                unsafe {
                    <#transport_ty>::descriptor_init(
                        transport.base(), 2u8, #transfer_size, transport.doorbell(), #chan_tx, #chan_rx,
                    ).await
                }.map_err(#map_err)?;
            },
            DoorbellRingerSide::Secondary => quote::quote! {
                /// SAFETY: base covers the descriptor; this side is Secondary, so it
                /// validates and acks the primary-written descriptor.
                unsafe {
                    <#transport_ty>::descriptor_ack(
                        transport.base(), 2usize, #transfer_size, transport.doorbell(), #chan_rx, #chan_tx,
                    ).await
                }.map_err(#map_err)?;
            },
        };

        let buffers = quote::quote! {
            #tx_buffer
            #rx_buffer
        };
        let expr = quote::quote! {{
            let transport = self.#field_ident;
            let (#chan_tx, #chan_rx) = transport.channels();
            #handshake
            let (#t_tx, #t_rx) = transport.split();
            let channel_tx = #channel_tx_init;
            let channel_rx = #channel_rx_init;
            ::consortium_ipc::Transceiver::new(channel_tx, channel_rx)
        }};
        (buffers, expr)
    }

    pub fn ty_signature(&self, core: VisibleCore, ty_crate: &Path) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");
        // The doorbell type carries its side and 1-based instance id so the
        // transport halves are fully concrete (the field type names no `_`).
        let doorbell_type = self.doorbell.as_type(side);
        let region_type = self.region_ty();

        let side_type = match side {
            DoorbellRingerSide::Primary => {
                quote::quote! { ::consortium_ipc_transport_memory::Primary }
            }
            DoorbellRingerSide::Secondary => {
                quote::quote! { ::consortium_ipc_transport_memory::Secondary }
            }
        };
        // `split` yields distinct send/receive half types; the transceiver names
        // both.
        let transport_tx_type = quote::quote! {
            ::consortium_ipc_transport_memory::SharedMemoryTransportTx<#side_type, #doorbell_type, #region_type>
        };
        let transport_rx_type = quote::quote! {
            ::consortium_ipc_transport_memory::SharedMemoryTransportRx<#side_type, #doorbell_type, #region_type>
        };

        let codec_type = codec_path(self.codec, self.mtu);

        let chan_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1,
        };
        let chan_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1,
            DoorbellRingerSide::Secondary => &self.channel_primary.1,
        };

        let ty_tx = Ident::new(&chan_tx.type_name, proc_macro2::Span::call_site());
        let ty_rx = Ident::new(&chan_rx.type_name, proc_macro2::Span::call_site());

        quote::quote! {
            ::consortium_ipc::Transceiver<#ty_crate::#ty_tx, #ty_crate::#ty_rx, #transport_tx_type, #transport_rx_type, #codec_type>
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoorbellMeta {
    /// doorbell type
    pub r#type: HardwareDoorbellPeripheralType,
    /// id
    pub id: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum DoorbellRingerSide {
    Primary,
    Secondary,
}

impl DoorbellMeta {
    /// The doorbell's crate path, type name, and IPC side marker idents.
    ///
    /// Shared by [`as_path`](Self::as_path) (expression position) and
    /// [`as_type`](Self::as_type) (type position), which differ only in turbofish.
    fn idents(&self, side: DoorbellRingerSide) -> (Ident, Ident, Ident) {
        let crate_ident = format_ident!(
            "consortium_ipc_doorbell_{}",
            self.r#type.to_string().to_lowercase()
        );
        let type_ident = format_ident!("{}Doorbell", self.r#type.to_string());
        let side_ident = format_ident!(
            "{}",
            match side {
                DoorbellRingerSide::Primary => "Primary",
                DoorbellRingerSide::Secondary => "Secondary",
            }
        );
        (crate_ident, type_ident, side_ident)
    }

    /// The doorbell type in expression/path position (turbofish form), for
    /// `<doorbell>::new(...)` construction.
    pub fn as_path(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let (crate_ident, type_ident, side_ident) = self.idents(side);
        quote::quote! {
            ::#crate_ident::#type_ident::<::consortium_ipc::#side_ident, #id>
        }
    }

    /// The doorbell type in type position (no turbofish), for naming the
    /// doorbell inside a transport type signature.
    pub fn as_type(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let (crate_ident, type_ident, side_ident) = self.idents(side);
        quote::quote! {
            ::#crate_ident::#type_ident<::consortium_ipc::#side_ident, #id>
        }
    }

    pub fn initialize(
        &self,
        side: DoorbellRingerSide,
        core: VisibleCore,
        uio_ident: &Ident,
        uio_id: usize,
        doorbell_base: TokenStream,
        doorbell_ident: &Ident,
    ) -> TokenStream {
        let doorbell = self.as_path(side);

        let binding_uio_stmt = match core {
            VisibleCore::Ap => Some(quote::quote! {
                #uio_ident[#uio_id].bind_notifier(#doorbell_ident.notifier());
            }),
            _ => None,
        };

        quote::quote! {
            /// SAFETY: the base is the UIO-mapped doorbell register window,
            /// resolved from the validated Consortium hardware database.
            let #doorbell_ident = unsafe { #doorbell::new(#doorbell_base) };
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

    /// Generates the per-core IPC endpoint module: a *disconnected*
    /// `IpcMemoryEndpoints` whose `new` only builds the transports, and a
    /// `IpcMemoryEndpointsConnected` produced by an `async fn connect` that runs
    /// the descriptor handshake before exposing the typed transceivers.
    ///
    /// Splitting construction from connection makes "use a transceiver before the
    /// descriptor is initialized" unrepresentable: `send`/`recv` live only on the
    /// connected type, which cannot exist without the handshake having run.
    pub fn initialize(&self, core: VisibleCore, uio_ident: &Ident, ty_crate: &Path) -> TokenStream {
        // Disconnected fields hold the unsplit transport; connected fields hold
        // the typed transceiver.
        let disconnected_def = self.transceivers.iter().map(|t| {
            let ident = Ident::new(t.name.as_str(), Span::call_site());
            let ty = t.transport_ty(core);
            quote::quote! { #ident: #ty }
        });
        let connected_def = self.transceivers.iter().map(|t| {
            let ident = Ident::new(t.name.as_str(), Span::call_site());
            let ty = t.ty_signature(core, ty_crate);
            quote::quote! { #ident: #ty }
        });

        // `new`-body field inits (build only) and `connect`-body field inits
        // (handshake + split + typed channels).
        let build_inits = self.transceivers.iter().map(|t| {
            let ident = Ident::new(t.name.as_str(), Span::call_site());
            let init = t.build_field_init(core, uio_ident);
            quote::quote! { #ident: #init }
        });
        // Each transceiver's scratch-buffer statics are hoisted to module scope;
        // the field initializer only borrows them.
        let mut connect_buffers = Vec::new();
        let connect_inits = self
            .transceivers
            .iter()
            .map(|t| {
                let ident = Ident::new(t.name.as_str(), Span::call_site());
                let (buffers, init) = t.connect_field_init(core, &ident, ty_crate);
                connect_buffers.push(buffers);
                quote::quote! { #ident: #init }
            })
            .collect::<Vec<_>>();

        // The AP resolves region/doorbell bases through UIO, so its `new` borrows
        // the opened devices and is fallible (`mmap` can fail). Controller cores
        // embed physical bases and never touch UIO, so theirs is infallible.
        let (new_sig, new_body) = if core == VisibleCore::Ap {
            (
                quote::quote! {
                    fn new(
                        #uio_ident: &[::consortium_runtime_app::uio::UioDevice],
                    ) -> ::core::result::Result<Self, ::consortium_runtime_app::uio::UioDeviceError>
                },
                quote::quote! {
                    ::core::result::Result::Ok(Self {
                        #(#build_inits),*
                    })
                },
            )
        } else {
            (
                quote::quote! { fn new() -> Self },
                quote::quote! {
                    Self {
                        #(#build_inits),*
                    }
                },
            )
        };

        quote::quote! {
            /// Descriptor-handshake failure, with the doorbell-specific payload
            /// erased so a single error type spans transceivers that use
            /// different doorbells. `no_std`-safe (no `alloc`).
            #[derive(Debug)]
            enum ConsortiumConnectError {
                /// The descriptor header/channel table was invalid or never reached
                /// the ready state.
                Descriptor(::consortium_ipc_transport_memory::DescriptorError),
                /// The doorbell signal failed during the handshake.
                Doorbell,
                /// Any other transport error during the handshake.
                Other,
            }

            /// IPC endpoints that have been *constructed* but not yet connected.
            /// Call [`connect`](IpcMemoryEndpoints::connect) to run the descriptor
            /// handshake and obtain usable transceivers.
            struct IpcMemoryEndpoints {
                #(#disconnected_def),*
            }

            // Per-channel scratch buffers the typed channels encode/decode
            // through. Declared at module scope (not inside `connect`) so each
            // has a `'static` lifetime and a single, clearly-placed definition.
            #(#connect_buffers)*

            /// Connected IPC endpoints: each field is a ready-to-use transceiver.
            struct IpcMemoryEndpointsConnected {
                #(#connected_def),*
            }

            impl IpcMemoryEndpoints {
                #new_sig {
                    #new_body
                }

                /// Runs the descriptor handshake for every transceiver and returns
                /// the connected endpoints. Each transceiver's primary side writes
                /// and the secondary side acks the descriptor before any traffic.
                async fn connect(
                    self,
                ) -> ::core::result::Result<IpcMemoryEndpointsConnected, ConsortiumConnectError> {
                    ::core::result::Result::Ok(IpcMemoryEndpointsConnected {
                        #(#connect_inits),*
                    })
                }
            }
        }
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
            type_name: "Ping".to_string(),
            buffer_name: "ping_buf".to_string(),
        }
    }

    #[test]
    fn channel_tx_uses_tx_direction_marker() {
        let chan = format_ident!("chan");
        let transport = format_ident!("transport");
        let ty_crate: Path = syn::parse_str("::shared").unwrap();
        let init = channel(0).initialize(
            Direction::Tx,
            &chan,
            &transport,
            Codec::Postcard,
            128,
            &ty_crate,
        );
        let buffer = init.buffer.to_string();
        let channel = init.channel.to_string();

        assert!(channel.contains(":: consortium_ipc :: Channel"));
        assert!(channel.contains(":: consortium_ipc :: Tx"));
        // The scratch buffer is a module-level static sized to the MTU, borrowed
        // by name from the channel expression.
        assert!(buffer.contains("static mut PING_BUF : [u8 ; 128usize]"));
        assert!(channel.contains("addr_of_mut ! (PING_BUF)"));
    }

    #[test]
    fn channel_rx_uses_rx_direction_marker() {
        let chan = format_ident!("chan");
        let transport = format_ident!("transport");
        let ty_crate: Path = syn::parse_str("::shared").unwrap();
        let channel = channel(1)
            .initialize(
                Direction::Rx,
                &chan,
                &transport,
                Codec::Postcard,
                128,
                &ty_crate,
            )
            .channel
            .to_string();

        assert!(channel.contains(":: consortium_ipc :: Rx"));
        assert!(!channel.contains(":: consortium_ipc :: Tx"));
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
                address,
                length,
            },
            channel_primary: (VisibleCore::Ap, channel(0)),
            channel_secondary: (cm33, channel(1)),
            doorbell: DoorbellMeta {
                r#type: HardwareDoorbellPeripheralType::Hsem,
                id: None,
            },
        }
    }

    const DESC: u32 = consortium_ipc_transport_memory::DESCRIPTOR_LENGTH as u32;

    #[test]
    fn max_channels_is_zero_when_only_descriptor_fits() {
        // No room beyond the descriptor: no message slots can be carved out.
        assert_eq!(transceiver(DESC, 4).max_channels(), 0);
    }

    #[test]
    fn max_channels_counts_slot_pairs() {
        // slot_len = SLOT_HEADER(12) + mtu(4) = 16; a Tx/Rx pair is 32 bytes.
        // Three pairs of room => 6 channels.
        let pair_len = (consortium_ipc_transport_memory::slot::SLOT_HEADER as u32 + 4) * 2;
        assert_eq!(transceiver(DESC + pair_len * 3, 4).max_channels(), 6);
    }

    #[test]
    fn max_channels_saturates_at_channel_num_max() {
        let max = consortium_ipc_transport_memory::CHANNEL_NUM_MAX as u32;
        assert_eq!(transceiver(u32::MAX, 4).max_channels(), max);
    }

    #[test]
    fn doorbell_path_resolves_crate_type_and_side() {
        let doorbell = DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Mu,
            id: Some(7),
        };

        let tokens = doorbell.as_path(DoorbellRingerSide::Primary).to_string();
        assert!(tokens.contains(":: consortium_ipc_doorbell_mu :: MuDoorbell"));
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
        assert!(tokens.contains(":: consortium_ipc_doorbell_hsem :: HsemDoorbell"));
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
