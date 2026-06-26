/// Represents the interrupt trigger method (trigger type and level/edge specifiers).
///
/// The values assigned to each variant match the standard Linux kernel device tree
/// bindings (`dt-bindings/interrupt-controller/irq.h`) for seamless DTS generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum IrqMethod {
    /// Unspecified or default interrupt trigger configuration.
    None = 0,

    /// Edge-triggered interrupt, active on the rising edge.
    ///
    /// - Linux/GIC: `IRQ_TYPE_EDGE_RISING`
    /// - MCU/NVIC: Rising edge trigger
    EdgeRising = 1,

    /// Edge-triggered interrupt, active on the falling edge.
    ///
    /// - Linux/GIC: `IRQ_TYPE_EDGE_FALLING`
    /// - MCU/NVIC: Falling edge trigger
    EdgeFalling = 2,

    /// Edge-triggered interrupt, active on both rising and falling edges.
    ///
    /// - Linux/GIC: `IRQ_TYPE_EDGE_BOTH`
    /// - MCU/NVIC: Both edges trigger
    EdgeBoth = 3,

    /// Level-sensitive interrupt, active when the signal is high.
    ///
    /// - Linux/GIC: `IRQ_TYPE_LEVEL_HIGH` (Common for GIC SPIs)
    /// - MCU/NVIC: High level detection
    LevelHigh = 4,

    /// Level-sensitive interrupt, active when the signal is low.
    ///
    /// - Linux/GIC: `IRQ_TYPE_LEVEL_LOW`
    /// - MCU/NVIC: Low level detection
    LevelLow = 8,
}

impl Default for IrqMethod {
    fn default() -> Self {
        Self::None
    }
}
