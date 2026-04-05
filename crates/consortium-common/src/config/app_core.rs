#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CortexAppCore {
    CortexA7,
    CortexA9,
    CortexA53,
    CortexA35,
    CortexA55,
    CortexA72,
}

impl CortexAppCore {
    pub fn as_key(s: &str) -> Option<Self> {
        match s {
            "cortex-a7" => Some(Self::CortexA7),
            "cortex-a9" => Some(Self::CortexA9),
            "cortex-a53" => Some(Self::CortexA53),
            "cortex-a35" => Some(Self::CortexA35),
            "cortex-a55" => Some(Self::CortexA55),
            "cortex-a72" => Some(Self::CortexA72),
            _ => None,
        }
    }

    pub fn abbr(&self) -> &'static str {
        match self {
            Self::CortexA7 => "ca7",
            Self::CortexA9 => "ca9",
            Self::CortexA53 => "ca53",
            Self::CortexA35 => "ca35",
            Self::CortexA55 => "ca55",
            Self::CortexA72 => "ca72",
        }
    }

    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "ca7" => Some(Self::CortexA7),
            "ca9" => Some(Self::CortexA9),
            "ca53" => Some(Self::CortexA53),
            "ca35" => Some(Self::CortexA35),
            "ca55" => Some(Self::CortexA55),
            "ca72" => Some(Self::CortexA72),
            _ => None,
        }
    }

    pub fn to_rust_target(&self) -> &'static str {
        match self {
            // ARMv7-A (32-bit)
            Self::CortexA7 | Self::CortexA9 => "armv7-unknown-linux-gnueabihf",
            // ARMv8-A (64-bit)
            Self::CortexA53 | Self::CortexA35 | Self::CortexA55 | Self::CortexA72 => "aarch64-unknown-linux-gnu",
        }
    }
}