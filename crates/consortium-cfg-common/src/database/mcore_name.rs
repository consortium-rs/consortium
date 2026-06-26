use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McoreName {
    Cm0p,
    Cm4,
    Cm7,
    Cm33,
}

impl McoreName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cm0p => "cm0p",
            Self::Cm4 => "cm4",
            Self::Cm7 => "cm7",
            Self::Cm33 => "cm33",
        }
    }
}

impl FromStr for McoreName {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "cm0p" => Ok(Self::Cm0p),
            "cm4" => Ok(Self::Cm4),
            "cm7" => Ok(Self::Cm7),
            "cm33" => Ok(Self::Cm33),
            other => Err(format!("unknown M-core name {other:?}")),
        }
    }
}

impl fmt::Display for McoreName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
