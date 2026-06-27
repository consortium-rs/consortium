use super::{McoreName, VisibleCore};
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Party {
    /// A-core, OP-TEE / secure world.
    AcoreSecure,
    /// A-core, Linux / non-secure world.
    AcoreNonSecure,
    /// M-core, e.g. Cortex-M7/M33, with an index for multi-instance cores
    /// and a secure flag for TrustZone-partitioned M-cores.
    Mcore {
        name: McoreName,
        index: Option<u8>,
        secure: bool,
    },
}

impl FromStr for Party {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "optee" => return Ok(Self::AcoreSecure),
            "linux" => return Ok(Self::AcoreNonSecure),
            _ => {}
        }

        let (base, secure) = match value.strip_suffix("-s") {
            Some(stripped) => (stripped, true),
            None => (value, false),
        };
        let (name, index) = parse_mcore(base, value)?;

        Ok(Self::Mcore {
            name,
            index,
            secure,
        })
    }
}

impl TryFrom<String> for Party {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Party {
    pub const fn core(self) -> VisibleCore {
        match self {
            Self::AcoreSecure | Self::AcoreNonSecure => VisibleCore::Ap,
            Self::Mcore { name, .. } => VisibleCore::Mcore(name),
        }
    }
}

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcoreNonSecure => f.write_str("linux"),
            Self::AcoreSecure => f.write_str("optee"),
            Self::Mcore {
                name,
                index,
                secure,
            } => {
                write!(f, "{name}")?;
                if let Some(index) = index {
                    write!(f, ":{index}")?;
                }
                if *secure {
                    f.write_str("-s")?;
                }
                Ok(())
            }
        }
    }
}

impl serde::Serialize for Party {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Party {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

fn parse_mcore(base: &str, original: &str) -> Result<(McoreName, Option<u8>), String> {
    let (name, index) = match base.split_once(':') {
        Some((name, index)) => {
            let index = index
                .parse()
                .map_err(|_| format!("bad M-core index in {original:?}"))?;
            (name, Some(index))
        }
        None => (base, None),
    };

    let name = name
        .parse()
        .map_err(|_| format!("unknown core {name:?} in {original:?}"))?;
    Ok((name, index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_core_parties() {
        assert_eq!("optee".parse::<Party>(), Ok(Party::AcoreSecure));
        assert_eq!("linux".parse::<Party>(), Ok(Party::AcoreNonSecure));
    }

    #[test]
    fn parses_m_core_parties() {
        assert_eq!(
            "cm7".parse::<Party>(),
            Ok(Party::Mcore {
                name: McoreName::Cm7,
                index: None,
                secure: false,
            })
        );
        assert_eq!(
            "cm33:1-s".parse::<Party>(),
            Ok(Party::Mcore {
                name: McoreName::Cm33,
                index: Some(1),
                secure: true,
            })
        );
    }

    #[test]
    fn formats_parties_for_schema_round_trips() {
        assert_eq!(Party::AcoreSecure.to_string(), "optee");
        assert_eq!(
            Party::Mcore {
                name: McoreName::Cm33,
                index: Some(1),
                secure: true,
            }
            .to_string(),
            "cm33:1-s"
        );
    }

    #[test]
    fn projects_party_to_core() {
        assert_eq!(Party::AcoreNonSecure.core(), VisibleCore::Ap);
        assert_eq!(Party::AcoreSecure.core(), VisibleCore::Ap);
        assert_eq!(
            "cm33-s".parse::<Party>().unwrap().core(),
            VisibleCore::Mcore(McoreName::Cm33)
        );
    }
}
