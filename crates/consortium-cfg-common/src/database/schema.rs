use super::HardwareDoorbellPeripheral;
use crate::config::VisibleCore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChipDatabase {
    pub name: String,
    pub trm: String,
    pub ap: String,
    /// Device-tree label of this chip's primary interrupt controller (the GIC):
    /// `intc` on STM32MP2, `gic` on i.MX 9. Emitted as `interrupt-parent = <&…>`
    /// on the register-backed (doorbell) UIO nodes so their GIC SPI resolves —
    /// the STM32MP2 root sets no default interrupt-parent, so a doorbell node at
    /// `/` would otherwise bind with no IRQ.
    pub interrupt_parent: String,
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
