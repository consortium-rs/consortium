use super::HardwareDoorbellParty;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareDoorbellPeripheral {
    pub name: String,
    pub page: u64,
    pub r#type: HardwareDoorbellPeripheralType,
    pub memory: HardwareMemory,
    pub parties: Vec<HardwareDoorbellParty>,
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
