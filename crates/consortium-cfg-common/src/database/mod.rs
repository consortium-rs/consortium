pub mod hardware_doorbell_party;
pub mod hardware_doorbell_peripheral;
pub mod schema;

pub use hardware_doorbell_party::{DoorbellWindow, HardwareDoorbellParty};
pub use hardware_doorbell_peripheral::{
    DoorbellClock, HardwareDoorbellPeripheral, HardwareDoorbellPeripheralType,
};
pub use schema::{
    ChipDatabase, Coprocessor, CoprocessorVariant, MemoryAlias, MemoryDatabase, MemoryRegion,
    MemoryRegionType, PeripheralAlias, PeripheralDatabase, RemoteprocDatabase, RemoteprocRegion,
    RemoteprocVector, RemoteprocWindow, VectorPlacement,
};
