use super::XcoreName;
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McoreName {
    Cm0p,
    Cm4f,
    Cm7,
    Cm33,
}

impl McoreName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cm0p => "cm0p",
            Self::Cm4f => "cm4f",
            Self::Cm7 => "cm7",
            Self::Cm33 => "cm33",
        }
    }

    pub const fn as_full_name(self) -> &'static str {
        match self {
            Self::Cm0p => "cortex-m0+",
            Self::Cm4f => "cortex-m4f",
            Self::Cm7 => "cortex-m7",
            Self::Cm33 => "cortex-m33",
        }
    }

    pub const fn as_rust_triple(self) -> &'static str {
        match self {
            Self::Cm0p => "thumbv6m-none-eabi",
            Self::Cm4f | Self::Cm7 => "thumbv7em-none-eabihf",
            Self::Cm33 => "thumbv8m.main-none-eabihf",
        }
    }

    pub const fn has_trustzone(self) -> bool {
        matches!(self, Self::Cm33)
    }

    pub const fn is_64_bit(self) -> bool {
        false
    }
}

impl XcoreName for McoreName {
    fn from_abbr(value: &str) -> Result<Self, String> {
        value.parse()
    }

    fn from_full_name(value: &str) -> Result<Self, String> {
        match value {
            "cortex-m0+" => Ok(Self::Cm0p),
            "cortex-m4f" => Ok(Self::Cm4f),
            "cortex-m7" => Ok(Self::Cm7),
            "cortex-m33" => Ok(Self::Cm33),
            other => Err(format!("unknown M-core name {other:?}")),
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

impl FromStr for McoreName {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "cm0p" => Ok(Self::Cm0p),
            "cm4f" => Ok(Self::Cm4f),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AcoreName, RcoreName, RvCoreName, RvExtensions, RvXlen};

    #[test]
    fn reports_m_core_metadata() {
        assert_eq!(McoreName::from_full_name("cortex-m33"), Ok(McoreName::Cm33));
        assert_eq!(McoreName::Cm33.abbr(), "cm33");
        assert_eq!(McoreName::Cm33.full_name(), "cortex-m33");
        assert_eq!(McoreName::Cm33.rust_triple(), "thumbv8m.main-none-eabihf");
        assert!(McoreName::Cm33.has_trustzone());
        assert!(McoreName::Cm33.is_32_bit());
    }

    #[test]
    fn reports_a_core_metadata() {
        assert_eq!(AcoreName::from_full_name("cortex-a55"), Ok(AcoreName::Ca55));
        assert_eq!(AcoreName::Ca55.abbr(), "ca55");
        assert_eq!(AcoreName::Ca55.full_name(), "cortex-a55");
        assert_eq!(AcoreName::Ca55.rust_triple(), "aarch64-unknown-none");
        assert!(AcoreName::Ca55.has_trustzone());
        assert!(AcoreName::Ca55.is_64_bit());
        assert!(AcoreName::Ca7.is_32_bit());
    }

    #[test]
    fn reports_r_core_metadata() {
        assert_eq!(RcoreName::from_full_name("cortex-r5f"), Ok(RcoreName::Cr5f));
        assert_eq!(RcoreName::Cr5f.abbr(), "cr5f");
        assert_eq!(RcoreName::Cr5f.full_name(), "cortex-r5f");
        assert_eq!(RcoreName::Cr5f.rust_triple(), "armv7r-none-eabihf");
        assert!(!RcoreName::Cr5f.has_trustzone());
        assert!(RcoreName::Cr5f.is_32_bit());
    }

    #[test]
    fn reports_riscv_core_metadata() {
        let core: RvCoreName = "rv32imafc".parse().unwrap();
        assert_eq!(
            core,
            RvCoreName::new(
                RvXlen::X32,
                RvExtensions {
                    i: true,
                    m: true,
                    a: true,
                    f: true,
                    d: false,
                    c: true,
                }
            )
        );
        assert_eq!(core.abbr(), "rv32imafc");
        assert_eq!(core.full_name(), "riscv32imafc");
        assert_eq!(RvCoreName::from_full_name("riscv32imafc"), Ok(core));
        assert_eq!(core.rust_triple(), "rv32imafc-unknown-none-elf");
        assert!(!core.has_trustzone());
        assert!(core.is_32_bit());
    }
}
