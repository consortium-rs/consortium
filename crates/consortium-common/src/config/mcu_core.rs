#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CortexMcuCore {
    CortexM0p,
    CortexM33,
    CortexM7,
    CortexM4,
    CortexM4f,
}

impl CortexMcuCore {
    pub fn as_key(s: &str) -> Option<Self> {
        match s {
            "cortex-m0p" => Some(Self::CortexM0p),
            "cortex-m33" => Some(Self::CortexM33),
            "cortex-m7" => Some(Self::CortexM7),
            "cortex-m4" => Some(Self::CortexM4),
            "cortex-m4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    pub fn abbr(&self) -> &'static str {
        match self {
            Self::CortexM0p => "cm0p",
            Self::CortexM33 => "cm33",
            Self::CortexM7 => "cm7",
            Self::CortexM4 => "cm4",
            Self::CortexM4f => "cm4f",
        }
    }

    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "cm0p" => Some(Self::CortexM0p),
            "cm33" => Some(Self::CortexM33),
            "cm7" => Some(Self::CortexM7),
            "cm4" => Some(Self::CortexM4),
            "cm4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    pub fn to_rust_target(&self) -> &'static str {
        match self {
            Self::CortexM0p => "thumbv6m-none-eabi",
            Self::CortexM33 => "thumbv8m.main-none-eabi",
            Self::CortexM7 | Self::CortexM4 | Self::CortexM4f => "thumbv7em-none-eabi",
        }
    }
}
