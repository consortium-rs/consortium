use super::{AcoreName, McoreName, RcoreName, RvCoreName};
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibleCore {
    Ap,
    Mcore(McoreName),
    Rcore(RcoreName),
    Rvcore(RvCoreName),
}

impl FromStr for VisibleCore {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "ap" {
            return Ok(Self::Ap);
        }

        value
            .parse()
            .map(Self::Mcore)
            .or_else(|_| value.parse().map(Self::Rcore))
            .or_else(|_| value.parse().map(Self::Rvcore))
            .map_err(|_| format!("unknown visible core {value:?}"))
    }
}

impl fmt::Display for VisibleCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ap => f.write_str("ap"),
            Self::Mcore(name) => name.fmt(f),
            Self::Rcore(name) => name.fmt(f),
            Self::Rvcore(name) => name.fmt(f),
        }
    }
}

impl serde::Serialize for VisibleCore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for VisibleCore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Core {
    Acore(AcoreName),
    Mcore(McoreName),
    Rcore(RcoreName),
    Rvcore(RvCoreName),
}

impl FromStr for Core {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .parse()
            .map(Self::Acore)
            .or_else(|_| value.parse().map(Self::Mcore))
            .or_else(|_| value.parse().map(Self::Rcore))
            .or_else(|_| value.parse().map(Self::Rvcore))
            .map_err(|_| format!("unknown core {value:?}"))
    }
}

impl fmt::Display for Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acore(name) => name.fmt(f),
            Self::Mcore(name) => name.fmt(f),
            Self::Rcore(name) => name.fmt(f),
            Self::Rvcore(name) => name.fmt(f),
        }
    }
}

impl serde::Serialize for Core {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Core {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_visible_core_names() {
        assert_eq!("ap".parse::<VisibleCore>(), Ok(VisibleCore::Ap));
        assert_eq!(
            "cm33".parse::<VisibleCore>(),
            Ok(VisibleCore::Mcore(McoreName::Cm33))
        );
        assert_eq!(
            "cr5f".parse::<VisibleCore>(),
            Ok(VisibleCore::Rcore(RcoreName::Cr5f))
        );
        assert_eq!(
            "rv32imac".parse::<VisibleCore>(),
            Ok(VisibleCore::Rvcore("rv32imac".parse().unwrap()))
        );
        assert!("ca55".parse::<VisibleCore>().is_err());
    }

    #[test]
    fn parses_concrete_core_names() {
        assert_eq!("ca55".parse::<Core>(), Ok(Core::Acore(AcoreName::Ca55)));
        assert_eq!("a55".parse::<Core>(), Ok(Core::Acore(AcoreName::Ca55)));
        assert_eq!("cm33".parse::<Core>(), Ok(Core::Mcore(McoreName::Cm33)));
        assert_eq!("cr5f".parse::<Core>(), Ok(Core::Rcore(RcoreName::Cr5f)));
        assert_eq!("r5f".parse::<Core>(), Ok(Core::Rcore(RcoreName::Cr5f)));
        assert_eq!(
            "rv32imac".parse::<Core>(),
            Ok(Core::Rvcore("rv32imac".parse().unwrap()))
        );
        assert!("ap".parse::<Core>().is_err());
    }
}
