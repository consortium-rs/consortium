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

impl Channel {
    pub fn initialize(
        &self,
        dir: Direction,
        chan_ident: &Ident,
        transport_ident: &Ident,
        codec: Codec,
        mtu: u32,
        ty_crate: &Path,
    ) -> TokenStream {
        let direction_ty = match dir {
            Direction::Tx => quote::quote! { ::consortium_ipc::Tx },
            Direction::Rx => quote::quote! { ::consortium_ipc::Rx },
        };

        let type_ident = format_ident!("{}", self.type_name);
        let codec_token = codec_path(codec);

        // The MTU-sized scratch buffer the channel encodes/decodes through. A
        // function-local `static` gives it a `'static` lifetime without a heap
        // or a caller-provided buffer; it is borrowed exactly once during init.
        let buf_static = format_ident!("{}", self.buffer_name.to_uppercase());
        let mtu = Literal::usize_suffixed(mtu as usize);
        let buf = quote::quote! {
            {
                static mut #buf_static: [u8; #mtu] = [0u8; #mtu];
                // SAFETY: function-local static scratch buffer, borrowed once
                // during initialization and never aliased afterwards.
                unsafe { &mut *::core::ptr::addr_of_mut!(#buf_static) }
            }
        };

        quote::quote! {
            ::consortium_ipc::Channel::<#direction_ty, #ty_crate::#type_ident, _, #codec_token>::new(
                #chan_ident,
                #transport_ident,
                #buf,
            )
        }
    }
}

/// The codec type path for `codec`, used by both endpoint construction and the
/// transceiver type signature.
fn codec_path(codec: Codec) -> TokenStream {
    match codec {
        Codec::Postcard => quote::quote! { ::consortium_codec::PostcardCodec },
        Codec::Prost => quote::quote! { ::consortium_codec::ProstCodec },
        Codec::Rkyv => quote::quote! { ::consortium_codec::RkyvCodec },
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

        let pair_len = slot_len.checked_mul(2).unwrap_or(u32::MAX);

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

    /// Generates the initialization code for the transceiver, including: region, doorbell, transport, channel, and transceiver.
    pub fn initialize(&self, core: VisibleCore, uio_ident: &Ident, ty_crate: &Path) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");

        let region_ident = format_ident!("_consortium_ipc_shm_{}_region", self.name);
        let region_init = self.region.initialize(core, uio_ident);
        let region_code = quote::quote! {
            let #region_ident = #region_init;
        };

        let doorbell_ident = format_ident!(
            "_consortium_ipc_shm_{}_doorbell_{}",
            self.name,
            self.doorbell.to_string()
        );
        let doorbell_code = self.doorbell.initialize(
            side,
            core,
            uio_ident,
            self.region.address.ap.uio_id,
            self.doorbell_address.memory_lookup(core, uio_ident),
            &doorbell_ident,
        );

        let chan_tx_ident = format_ident!("_consortium_ipc_shm_{}_chan_tx", self.name);
        let chan_rx_ident = format_ident!("_consortium_ipc_shm_{}_chan_rx", self.name);
        let chan_id_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1.doorbell_chan,
        };
        let chan_id_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1.doorbell_chan,
            DoorbellRingerSide::Secondary => &self.channel_primary.1.doorbell_chan,
        };
        let chan_code = quote::quote! {
            // SAFETY: doorbell channel is verified to be valid in the config.
            let #chan_tx_ident = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_tx) };
            // SAFETY: doorbell channel is verified to be valid in the config.
            let #chan_rx_ident = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_rx) };
        };

        let transport_side_ty = match side {
            DoorbellRingerSide::Primary => quote::quote! { ::consortium_ipc::Primary },
            DoorbellRingerSide::Secondary => quote::quote! { ::consortium_ipc::Secondary },
        };

        let transport_ident = format_ident!("_consortium_ipc_shm_{}_transport", self.name);
        let transport_tx_ident = format_ident!("_consortium_ipc_shm_{}_transport_tx", self.name);
        let transport_rx_ident = format_ident!("_consortium_ipc_shm_{}_transport_rx", self.name);
        // The transport's descriptor base is the shared-memory region base
        // (offset 0x00), not the doorbell register window.
        let transport_address = self.region.address.memory_lookup(core, uio_ident);
        let transport_code = quote::quote! {
            // SAFETY: the base is offset 0x00 of the validated shared-memory
            // region, and the channels/doorbell/region are this side's.
            let #transport_ident = unsafe {
                ::consortium_ipc_transport_memory::SharedMemoryTransport::<#transport_side_ty, _, _>::new(
                    #chan_tx_ident,
                    #chan_rx_ident,
                    #doorbell_ident,
                    #region_ident,
                    (#transport_address) as *mut u8,
                )
            };

            let (#transport_tx_ident, #transport_rx_ident) = #transport_ident.split();
        };

        let channel_tx_ident = format_ident!("_consortium_ipc_shm_{}_channel_tx", self.name);
        let channel_rx_ident = format_ident!("_consortium_ipc_shm_{}_channel_rx", self.name);

        let channel_tx = match side {
            DoorbellRingerSide::Primary => &self.channel_primary.1,
            DoorbellRingerSide::Secondary => &self.channel_secondary.1,
        };
        let channel_rx = match side {
            DoorbellRingerSide::Primary => &self.channel_secondary.1,
            DoorbellRingerSide::Secondary => &self.channel_primary.1,
        };

        let channel_tx_init = channel_tx.initialize(
            Direction::Tx,
            &chan_tx_ident,
            &transport_tx_ident,
            self.codec,
            self.mtu,
            ty_crate,
        );
        let channel_rx_init = channel_rx.initialize(
            Direction::Rx,
            &chan_rx_ident,
            &transport_rx_ident,
            self.codec,
            self.mtu,
            ty_crate,
        );

        let channel_code = quote::quote! {
            let #channel_tx_ident = #channel_tx_init;
            let #channel_rx_ident = #channel_rx_init;
        };

        let transceiver_code = quote::quote! {
            ::consortium_ipc::Transceiver::new(
                #channel_tx_ident,
                #channel_rx_ident
            )
        };

        quote::quote! {
            {
                #region_code
                #doorbell_code
                #chan_code
                #transport_code
                #channel_code
                #transceiver_code
            }
        }
    }

    pub fn ty_signature(&self, core: VisibleCore, ty_crate: &Path) -> TokenStream {
        let side = self
            .side(core)
            .expect("core must be either primary or secondary for the transceiver");
        // The doorbell type carries its side and 1-based instance id so the
        // transport halves are fully concrete (the field type names no `_`).
        let side_ident = match side {
            DoorbellRingerSide::Primary => format_ident!("Primary"),
            DoorbellRingerSide::Secondary => format_ident!("Secondary"),
        };
        let db_id = Literal::usize_suffixed(self.doorbell.id.unwrap_or(1) as usize);
        let doorbell_type = match self.doorbell.r#type {
            HardwareDoorbellPeripheralType::Mu => {
                quote::quote! { ::consortium_ipc_doorbell_mu::MuDoorbell<::consortium_ipc::#side_ident, #db_id> }
            }
            HardwareDoorbellPeripheralType::Hsem => {
                quote::quote! { ::consortium_ipc_doorbell_hsem::HsemDoorbell<::consortium_ipc::#side_ident, #db_id> }
            }
            HardwareDoorbellPeripheralType::Ipcc => {
                quote::quote! { ::consortium_ipc_doorbell_ipcc::IpccDoorbell<::consortium_ipc::#side_ident, #db_id> }
            }
        };
        let region_type = match self.region.r#type {
            MemoryRegionType::Sram => {
                quote::quote! { ::consortium_shared_memory::regions::SramRegion }
            }
        };

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

        let codec_type = codec_path(self.codec);

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
    pub fn as_path(&self, side: DoorbellRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
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
        let side_path = quote::quote! { ::consortium_ipc::#side_ident };
        quote::quote! {
            ::#crate_ident::#type_ident::<#side_path, #id>
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
            // SAFETY: the base is the UIO-mapped doorbell register window,
            // resolved from the validated Consortium hardware database.
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

    pub fn initialize(&self, core: VisibleCore, uio_ident: &Ident, ty_crate: &Path) -> TokenStream {
        // 1. struct field defs: `name: Type`
        let struct_def = self
            .transceivers
            .iter()
            .map(|t| {
                let ident = Ident::new(t.name.as_str(), Span::call_site());
                let ty = t.ty_signature(core, ty_crate);
                quote::quote! { #ident: #ty }
            })
            .collect::<Vec<_>>();

        // 2. field inits for `new`'s body: `name: <init expr>`
        let field_inits = self
            .transceivers
            .iter()
            .map(|t| {
                let ident = Ident::new(t.name.as_str(), Span::call_site());
                let init = t.initialize(core, uio_ident, ty_crate);
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
                        #(#field_inits),*
                    })
                },
            )
        } else {
            (
                quote::quote! { fn new() -> Self },
                quote::quote! {
                    Self {
                        #(#field_inits),*
                    }
                },
            )
        };

        quote::quote! {
            // struct definition
            struct IpcMemoryEndpoints {
                #(#struct_def),*
            }

            // impl new
            impl IpcMemoryEndpoints {
                #new_sig {
                    #new_body
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
        let tokens = channel(0)
            .initialize(
                Direction::Tx,
                &chan,
                &transport,
                Codec::Postcard,
                128,
                &ty_crate,
            )
            .to_string();

        assert!(tokens.contains(":: consortium_ipc :: Channel"));
        assert!(tokens.contains(":: consortium_ipc :: Tx"));
        // The scratch buffer is an inline function-local static sized to the MTU.
        assert!(tokens.contains("static mut PING_BUF : [u8 ; 128usize]"));
        assert!(tokens.contains("addr_of_mut ! (PING_BUF)"));
    }

    #[test]
    fn channel_rx_uses_rx_direction_marker() {
        let chan = format_ident!("chan");
        let transport = format_ident!("transport");
        let ty_crate: Path = syn::parse_str("::shared").unwrap();
        let tokens = channel(1)
            .initialize(
                Direction::Rx,
                &chan,
                &transport,
                Codec::Postcard,
                128,
                &ty_crate,
            )
            .to_string();

        assert!(tokens.contains(":: consortium_ipc :: Rx"));
        assert!(!tokens.contains(":: consortium_ipc :: Tx"));
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
