use super::XcoreName;
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RcoreName {
    Cr5f,
}

impl RcoreName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cr5f => "cr5f",
        }
    }

    pub const fn as_full_name(self) -> &'static str {
        match self {
            Self::Cr5f => "cortex-r5f",
        }
    }

    pub const fn as_rust_triple(self) -> &'static str {
        match self {
            Self::Cr5f => "armv7r-none-eabihf",
        }
    }

    pub const fn has_trustzone(self) -> bool {
        false
    }

    pub const fn is_64_bit(self) -> bool {
        false
    }
}

impl XcoreName for RcoreName {
    fn from_abbr(value: &str) -> Result<Self, String> {
        value.parse()
    }

    fn from_full_name(value: &str) -> Result<Self, String> {
        match value {
            "cortex-r5f" => Ok(Self::Cr5f),
            other => Err(format!("unknown R-core name {other:?}")),
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

impl FromStr for RcoreName {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "cr5f" | "r5f" => Ok(Self::Cr5f),
            other => Err(format!("unknown R-core abbreviation {other:?}")),
        }
    }
}

impl fmt::Display for RcoreName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
