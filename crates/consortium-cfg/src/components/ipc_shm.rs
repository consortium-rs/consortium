use core::fmt;
use std::str::FromStr;

use consortium_cfg_common::{
    config::{Codec, MemoryRegion, VisibleCore},
    database::HardwareDoorbellPeripheralType,
};
use consortium_ipc_transport_memory::ChannelDirection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::error::CfgError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    /// doorbell channel id, verified before codegen
    pub doorbell_chan: u8,
    /// type name
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transceiver {
    /// doorbell base address pointer
    /// - baremetal: raw address
    /// - os: mmap'd
    pub doorbell_base: u32,
    /// max transmission unit
    pub mtu: u32,
    /// codec name
    pub codec: Codec,
    /// address
    pub address: MemoryRegion,
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
        let total_len = self.address.length;
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BellaMeta {
    /// doorbell type
    pub r#type: HardwareDoorbellPeripheralType,
    /// id
    pub id: Option<u8>,
}

impl BellaMeta {
    pub fn as_ident(&self, side: String) -> Ident {
        let id = self.id.unwrap_or(1);
        let name = format!(
            "::consortium_ipc_doorbell_{}::{}Doorbell<::consortium_ipc::side::{side}, {id}>",
            self.r#type.to_string().to_lowercase(),
            self.r#type.to_string()
        );
        Ident::new(&name, proc_macro2::Span::call_site())
    }

    pub fn initialize(
        &self,
        side: String,
        core: &VisibleCore,
        uio_ident: Ident,
        uio_id: u8,
        doorbell_base: TokenStream,
    ) -> (Ident, TokenStream) {
        let doorbell = self.as_ident(side);
        let ident = quote::format_ident!("_doorbell_{self}");
        let binding_uio_stmt = match core {
            VisibleCore::Ap => Some(quote::quote! {
                #uio_ident[#uio_id].bind_nodifier(#ident);
            }),
            _ => None,
        };

        (
            ident,
            quote::quote! {
                #doorbell::new(#doorbell_base);
                #binding_uio_stmt
            },
        )
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
    use super::*;

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
