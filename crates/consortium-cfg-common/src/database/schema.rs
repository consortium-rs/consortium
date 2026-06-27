use super::HardwareDoorbellPeripheral;
use crate::config::VisibleCore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChipDatabase {
    pub name: String,
    pub trm: String,
    pub ap: String,
    pub cp: Vec<Coprocessor>,
    pub memory: MemoryDatabase,
    pub peripheral: PeripheralDatabase,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Coprocessor {
    pub name: String,
    pub count: Option<u8>,
    pub variant: CoprocessorVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoprocessorVariant {
    Remote,
    Manager,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryDatabase {
    pub trm_page: Vec<u32>,
    pub aliases: Vec<MemoryAlias>,
    pub regions: Vec<MemoryRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MemoryAlias {
    pub offset: u32,
    pub secure: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryRegion {
    pub name: String,
    pub r#type: MemoryRegionType,
    pub base: u32,
    pub length: u32,
    pub visible: Vec<VisibleCore>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryRegionType {
    Sram,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeripheralDatabase {
    pub aliases: Vec<PeripheralAlias>,
    pub doorbells: Vec<HardwareDoorbellPeripheral>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PeripheralAlias {
    pub address: u32,
    pub secure: bool,
}
