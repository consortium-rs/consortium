use super::{HardwareDoorbellPeripheral, McoreName};
use core::fmt;
use core::str::FromStr;

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
    pub trm_page: Vec<u64>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibleCore {
    Ap,
    Mcore(McoreName),
}

impl FromStr for VisibleCore {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ap" => Ok(Self::Ap),
            value => value.parse().map(Self::Mcore),
        }
    }
}

impl fmt::Display for VisibleCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ap => f.write_str("ap"),
            Self::Mcore(name) => name.fmt(f),
        }
    }
}

impl serde::Serialize for VisibleCore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for VisibleCore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
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
