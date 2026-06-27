use core::fmt;

use super::HardwareDoorbellParty;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareDoorbellPeripheral {
    pub name: String,
    pub page: u32,
    pub r#type: HardwareDoorbellPeripheralType,
    pub memory: HardwareMemory,
    pub parties: Vec<HardwareDoorbellParty>,
    pub party_page: u32,
    pub channels: u32,
    pub channel_page: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HardwareMemory {
    pub base: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HardwareDoorbellPeripheralType {
    Mu,
    Hsem,
    Ipcc,
}

impl fmt::Display for HardwareDoorbellPeripheralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareDoorbellPeripheralType::Mu => write!(f, "Mu"),
            HardwareDoorbellPeripheralType::Hsem => write!(f, "Hsem"),
            HardwareDoorbellPeripheralType::Ipcc => write!(f, "Ipcc"),
        }
    }
}
