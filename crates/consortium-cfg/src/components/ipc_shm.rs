use core::fmt;
use std::str::FromStr;

use consortium_cfg_common::{
    config::{Codec, VisibleCore},
    database::HardwareDoorbellPeripheralType,
};
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::Ident;

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
    ) -> TokenStream {
        let buf = format_ident!("{}", self.buffer_name);
        let direction_ty = match dir {
            Direction::Tx => quote::quote! { ::consortium_ipc::Tx },
            Direction::Rx => quote::quote! { ::consortium_ipc::Rx },
        };

        let type_ident = format_ident!("{}", self.type_name);
        let codec_token = match codec {
            Codec::Postcard => quote::quote! { ::consortium_codec_postcard::PostcardCodec },
            Codec::Prost => quote::quote! { ::consortium_codec_prost::ProstCodec },
            Codec::Rkyv => quote::quote! { ::consortium_codec_rkyv::RkyvCodec },
        };

        quote::quote! {
            ::consortium_ipc::Channel::<#direction_ty, #type_ident, _, #codec_token>::new(
                #chan_ident,
                #transport_ident,
                #buf,
            )
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
    pub bella: BellaMeta,
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

    /// Generates the initialization code for the transceiver, including: region, doorbell, transport, channel, and transceiver.
    pub fn initialize(
        &self,
        core: &VisibleCore,
        uio_ident: &Ident,
        side: BellaRingerSide,
    ) -> TokenStream {
        let region_ident = format_ident!("_consortium_ipc_shm_{}_region", self.name);
        let region_init = self.region.initialize(core, uio_ident);
        let region_code = quote::quote! {
            let #region_ident = #region_init;
        };

        let doorbell_ident = format_ident!(
            "_consortium_ipc_shm_{}_doorbell_{}",
            self.name,
            self.bella.to_string()
        );
        let doorbell_code = self.bella.initialize(
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
            BellaRingerSide::Primary => &self.channel_primary.1.doorbell_chan,
            BellaRingerSide::Secondary => &self.channel_secondary.1.doorbell_chan,
        };
        let chan_id_rx = match side {
            BellaRingerSide::Primary => &self.channel_secondary.1.doorbell_chan,
            BellaRingerSide::Secondary => &self.channel_primary.1.doorbell_chan,
        };
        let chan_code = quote::quote! {
            // SAFETY: doorbell channel is verified to be valid in the config.
            let #chan_tx_ident = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_tx) };
            // SAFETY: doorbell channel is verified to be valid in the config.
            let #chan_rx_ident = unsafe { ::consortium_ipc::Chan::new_unchecked(#chan_id_rx) };
        };

        let transport_side_ty = match side {
            BellaRingerSide::Primary => quote::quote! { ::consortium_ipc::side::Primary },
            BellaRingerSide::Secondary => quote::quote! { ::consortium_ipc::side::Secondary },
        };

        let transport_ident = format_ident!("_consortium_ipc_shm_{}_transport", self.name);
        let transport_tx_ident = format_ident!("_consortium_ipc_shm_{}_transport_tx", self.name);
        let transport_rx_ident = format_ident!("_consortium_ipc_shm_{}_transport_rx", self.name);
        let transport_address = self.doorbell_address.memory_lookup(core, uio_ident);
        let transport_code = quote::quote! {
            let #transport_ident = ::consortium_ipc_transport_memory::Transport::<#transport_side_ty, _, _>::new(
                #chan_tx_ident,
                #chan_rx_ident,
                #doorbell_ident,
                #region_ident,
                #transport_address
            );

            let (#transport_tx_ident, #transport_rx_ident) = #transport_ident.split();
        };

        let channel_tx_ident = format_ident!("_consortium_ipc_shm_{}_channel_tx", self.name);
        let channel_rx_ident = format_ident!("_consortium_ipc_shm_{}_channel_rx", self.name);

        let channel_tx = match side {
            BellaRingerSide::Primary => &self.channel_primary.1,
            BellaRingerSide::Secondary => &self.channel_secondary.1,
        };
        let channel_rx = match side {
            BellaRingerSide::Primary => &self.channel_secondary.1,
            BellaRingerSide::Secondary => &self.channel_primary.1,
        };

        let channel_tx_init = channel_tx.initialize(
            Direction::Tx,
            &chan_tx_ident,
            &transport_tx_ident,
            self.codec,
        );
        let channel_rx_init = channel_rx.initialize(
            Direction::Rx,
            &chan_rx_ident,
            &transport_rx_ident,
            self.codec,
        );

        let channel_code = quote::quote! {
            let #channel_tx_ident = #channel_tx_init;
            let #channel_rx_ident = #channel_rx_init;
        };

        let transceiver_ident = format_ident!("_consortium_ipc_shm_{}_transceiver", self.name);
        let transceiver_code = quote::quote! {
            let #transceiver_ident = ::consortium_ipc::Transceiver::new(
                #channel_tx_ident,
                #channel_rx_ident
            );
        };

        quote::quote! {
            #region_code
            #doorbell_code
            #chan_code
            #transport_code
            #channel_code
            #transceiver_code
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BellaMeta {
    /// doorbell type
    pub r#type: HardwareDoorbellPeripheralType,
    /// id
    pub id: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum BellaRingerSide {
    Primary,
    Secondary,
}

impl BellaMeta {
    pub fn as_path(&self, side: BellaRingerSide) -> TokenStream {
        let id = self.id.unwrap_or(1) as usize;
        let crate_ident = format_ident!(
            "consortium_ipc_doorbell_{}",
            self.r#type.to_string().to_lowercase()
        );
        let type_ident = format_ident!("{}Doorbell", self.r#type.to_string());
        let side_ident = format_ident!(
            "{}",
            match side {
                BellaRingerSide::Primary => "Primary",
                BellaRingerSide::Secondary => "Secondary",
            }
        );
        let side_path = quote::quote! { ::consortium_ipc::side::#side_ident };
        quote::quote! {
            ::#crate_ident::#type_ident::<#side_path, #id>
        }
    }

    pub fn initialize(
        &self,
        side: BellaRingerSide,
        core: &VisibleCore,
        uio_ident: &Ident,
        uio_id: usize,
        bella_base: TokenStream,
        bella_ident: &Ident,
    ) -> TokenStream {
        let doorbell = self.as_path(side);

        let binding_uio_stmt = match core {
            VisibleCore::Ap => Some(quote::quote! {
                #uio_ident[#uio_id].bind_notifier(#bella_ident.notifier());
            }),
            _ => None,
        };

        quote::quote! {
            // SAFETY: the base is the UIO-mapped doorbell register window,
            // resolved from the validated Consortium hardware database.
            let #bella_ident = unsafe { #doorbell::new(#bella_base) };
            #binding_uio_stmt
        }
    }
}

impl FromStr for BellaMeta {
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

impl fmt::Display for BellaMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type.to_string().to_lowercase())?;
        if let Some(id) = self.id {
            write!(f, "{id}")?;
        }
        Ok(())
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
        let tokens = channel(0)
            .initialize(Direction::Tx, &chan, &transport, Codec::Postcard)
            .to_string();

        assert!(tokens.contains(":: consortium_ipc :: Channel"));
        assert!(tokens.contains(":: consortium_ipc :: Tx"));
        assert!(tokens.contains("new (chan , transport , ping_buf ,)"));
    }

    #[test]
    fn channel_rx_uses_rx_direction_marker() {
        let chan = format_ident!("chan");
        let transport = format_ident!("transport");
        let tokens = channel(1)
            .initialize(Direction::Rx, &chan, &transport, Codec::Postcard)
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
            bella: BellaMeta {
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
        let bella = BellaMeta {
            r#type: HardwareDoorbellPeripheralType::Mu,
            id: Some(7),
        };

        let tokens = bella.as_path(BellaRingerSide::Primary).to_string();
        assert!(tokens.contains(":: consortium_ipc_doorbell_mu :: MuDoorbell"));
        assert!(tokens.contains(":: consortium_ipc :: side :: Primary"));
        assert!(tokens.contains("7usize"));
    }

    #[test]
    fn doorbell_path_defaults_missing_id_to_one() {
        let bella = BellaMeta {
            r#type: HardwareDoorbellPeripheralType::Hsem,
            id: None,
        };

        let tokens = bella.as_path(BellaRingerSide::Secondary).to_string();
        assert!(tokens.contains(":: consortium_ipc_doorbell_hsem :: HsemDoorbell"));
        assert!(tokens.contains(":: consortium_ipc :: side :: Secondary"));
        assert!(tokens.contains("1usize"));
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for s in ["mu7", "hsem", "ipcc1", "mu0"] {
            let bella: BellaMeta = s.parse().unwrap();
            assert_eq!(bella.to_string(), s);
        }
    }

    #[test]
    fn rejects_invalid_doorbell_id() {
        assert!("mu999".parse::<BellaMeta>().is_err());
    }

    #[test]
    fn parses_typed_doorbell_with_id() {
        assert_eq!(
            "mu7".parse::<BellaMeta>().unwrap(),
            BellaMeta {
                r#type: HardwareDoorbellPeripheralType::Mu,
                id: Some(7),
            }
        );
        assert_eq!(
            "ipcc1".parse::<BellaMeta>().unwrap(),
            BellaMeta {
                r#type: HardwareDoorbellPeripheralType::Ipcc,
                id: Some(1),
            }
        );
    }

    #[test]
    fn parses_typed_doorbell_without_id() {
        assert_eq!(
            "hsem".parse::<BellaMeta>().unwrap(),
            BellaMeta {
                r#type: HardwareDoorbellPeripheralType::Hsem,
                id: None,
            }
        );
    }

    #[test]
    fn rejects_unknown_doorbell_type() {
        assert!("foo3".parse::<BellaMeta>().is_err());
    }
}
