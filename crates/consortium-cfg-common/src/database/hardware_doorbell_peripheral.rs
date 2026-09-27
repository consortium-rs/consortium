use core::fmt;

use super::HardwareDoorbellParty;

// A stray key here is silently dropped otherwise, and these tables are
// hand-authored from the reference manual -- `pattern: gi` sat unread on the
// i.MX 93 MU1 entry until this was added.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardwareDoorbellPeripheral {
    pub name: String,
    pub page: u32,
    pub r#type: HardwareDoorbellPeripheralType,
    /// Length of the whole register block, spanning every per-side window.
    ///
    /// The block has no single base: each party reaches its own window at
    /// [`HardwareDoorbellParty::memory`].
    pub length: u32,
    pub parties: Vec<HardwareDoorbellParty>,
    pub party_page: u32,
    pub channels: u32,
    pub channel_page: u32,
    /// Optional device-tree clock this doorbell's register block needs kept
    /// enabled from the Linux side, emitted as `clocks = <&provider id>` on the
    /// generic-uio node. STM32MP2 IPCC/HSEM reference the SCMI clock domain;
    /// i.MX MUs live in an always-on domain and carry no clock.
    #[serde(default)]
    pub clock: Option<DoorbellClock>,
    /// Length of one per-side register window, when the block exposes several.
    ///
    /// An i.MX MU's `length` spans both processor views, and each view is one
    /// window of this length. Absent means the block has a single window
    /// spanning all of `length` (STM32 IPCC/HSEM).
    #[serde(default)]
    pub window_length: Option<u32>,
}

impl HardwareDoorbellPeripheral {
    /// Length of one party's register window.
    ///
    /// Falls back to the whole block for single-window peripherals.
    pub fn party_window_length(&self) -> u32 {
        self.window_length.unwrap_or(self.length)
    }
}

/// A device-tree clock reference (`<&provider id>`) for a doorbell register
/// block. `id` is a clock-binding macro name (e.g. `CK_SCMI_IPCC1`) emitted
/// verbatim, so the vendor base tree's `#include`d clock header resolves it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DoorbellClock {
    /// Device-tree label of the clock provider (e.g. `scmi_clk`).
    pub provider: String,
    /// Clock-binding macro name for the clock cell (e.g. `CK_SCMI_IPCC1`).
    pub id: String,
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
