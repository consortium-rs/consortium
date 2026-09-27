use crate::config::Party;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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
    ///
    /// These are two different numbering spaces, and the `linux`/`optee` entries
    /// are consumed verbatim as the `interrupts` cell of the generated UIO node.
    /// Copying an M-core NVIC position into an A-core slot therefore produces a
    /// device tree that binds the wrong line, with no diagnostic.
    pub irq: u32,
    /// Which of the peripheral's per-side register windows this party drives.
    ///
    /// An i.MX MU exposes the same instance through two windows — MUA and MUB,
    /// one per processor — and which side a core gets is a silicon fact, not a
    /// property of its role in the channel. On i.MX 95 the A-core is on MUA and
    /// the CM7 on MUB; on i.MX 93 it is the other way round. STM32 IPCC/HSEM
    /// expose one block with a bank per core, so they leave this at the default.
    ///
    /// This selects the generated doorbell's side marker; the window's address
    /// is [`memory`](Self::memory).
    #[serde(default)]
    pub window: DoorbellWindow,
    /// Base of the register window this party drives, relative to the chip's
    /// peripheral alias.
    ///
    /// Single-window blocks (STM32 IPCC/HSEM) repeat the block base for every
    /// party; an i.MX MU gives each party the base of its own MUA/MUB window.
    pub memory: u32,
    /// TRM page documenting the peripheral memory mapping for this core view.
    pub page_memory: u32,
    /// TRM page documenting the interrupt number for this core view.
    pub page_irq: u32,
}

/// One of a doorbell peripheral's per-side register windows.
///
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DoorbellWindow {
    /// An i.MX MU's MUA window, and the default for single-window blocks.
    #[default]
    A,
    /// An i.MX MU's MUB window.
    B,
}
