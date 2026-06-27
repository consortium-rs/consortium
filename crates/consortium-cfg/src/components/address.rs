use std::collections::HashMap;

use consortium_cfg_common::config::VisibleCore;

/// UIO mmap'd address provided for MMIO in application processors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApAddress {
    /// ID of the UIO device
    pub uio_id: u32,
    /// Mmap ID in the UIO device of the memory window for this channel
    pub mmap_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    /// application processor
    pub ap: ApAddress,
    /// controller processor
    pub cp: HashMap<VisibleCore, u32>,
}
