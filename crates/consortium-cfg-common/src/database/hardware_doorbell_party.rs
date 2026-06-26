use super::Party;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareDoorbellParty {
    /// Core name.
    ///
    /// - A-core secure: `optee`.
    /// - A-core non-secure: `linux`.
    /// - M-core: for example, `cm7`, `cm33`, etc. If there are multiple same cortex cores, e.g., two Cortex-M7, use `cm7:0` and `cm7:1`.
    /// - TrustZone: add `-s` suffix, e.g., `cm33-s`, or `cm33:1-s`.
    pub core: Party,
    /// Interrupt number.
    ///
    /// - A-core (GIC): raw SPI number (starts from 0).
    /// - M-core (NVIC): raw NVIC position.
    pub irq: u32,
    /// TRM page documenting the peripheral memory mapping for this core view.
    pub page_memory: u64,
    /// TRM page documenting the interrupt number for this core view.
    pub page_irq: u64,
}
