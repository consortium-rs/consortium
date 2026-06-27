use super::XcoreName;
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcoreName {
    Ca35,
    Ca55,
    Ca53,
    Ca7,
}

impl AcoreName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ca35 => "ca35",
            Self::Ca55 => "ca55",
            Self::Ca53 => "ca53",
            Self::Ca7 => "ca7",
        }
    }

    pub const fn as_full_name(self) -> &'static str {
        match self {
            Self::Ca35 => "cortex-a35",
            Self::Ca55 => "cortex-a55",
            Self::Ca53 => "cortex-a53",
            Self::Ca7 => "cortex-a7",
        }
    }

    pub const fn as_rust_triple(self) -> &'static str {
        match self {
            Self::Ca35 | Self::Ca55 | Self::Ca53 => "aarch64-unknown-none",
            Self::Ca7 => "armv7a-none-eabihf",
        }
    }

    pub const fn has_trustzone(self) -> bool {
        true
    }

    pub const fn is_64_bit(self) -> bool {
        matches!(self, Self::Ca35 | Self::Ca55 | Self::Ca53)
    }
}

impl XcoreName for AcoreName {
    fn from_abbr(value: &str) -> Result<Self, String> {
        value.parse()
    }

    fn from_full_name(value: &str) -> Result<Self, String> {
        match value {
            "cortex-a35" => Ok(Self::Ca35),
            "cortex-a55" => Ok(Self::Ca55),
            "cortex-a53" => Ok(Self::Ca53),
            "cortex-a7" => Ok(Self::Ca7),
            other => Err(format!("unknown A-core name {other:?}")),
        }
    }

    fn abbr(&self) -> String {
        self.as_str().to_string()
    }

    fn full_name(&self) -> String {
        self.as_full_name().to_string()
    }

    fn rust_triple(&self) -> String {
        self.as_rust_triple().to_string()
    }

    fn has_trustzone(&self) -> bool {
        (*self).has_trustzone()
    }

    fn is_64_bit(&self) -> bool {
        (*self).is_64_bit()
    }
}

impl FromStr for AcoreName {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ca35" | "a35" => Ok(Self::Ca35),
            "ca55" | "a55" => Ok(Self::Ca55),
            "ca53" | "a53" => Ok(Self::Ca53),
            "ca7" | "a7" => Ok(Self::Ca7),
            other => Err(format!("unknown A-core abbreviation {other:?}")),
        }
    }
}

impl fmt::Display for AcoreName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
