use super::XcoreName;
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct RvCoreName {
    pub xlen: RvXlen,
    #[serde(default)]
    pub extensions: RvExtensions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum RvXlen {
    X32,
    X64,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct RvExtensions {
    #[serde(default)]
    pub i: bool,
    #[serde(default)]
    pub m: bool,
    #[serde(default)]
    pub a: bool,
    #[serde(default)]
    pub f: bool,
    #[serde(default)]
    pub d: bool,
    #[serde(default)]
    pub c: bool,
}

impl RvCoreName {
    pub const fn new(xlen: RvXlen, extensions: RvExtensions) -> Self {
        Self { xlen, extensions }
    }

    fn extension_string(self) -> String {
        let mut value = String::new();
        if self.extensions.i {
            value.push('i');
        }
        if self.extensions.m {
            value.push('m');
        }
        if self.extensions.a {
            value.push('a');
        }
        if self.extensions.f {
            value.push('f');
        }
        if self.extensions.d {
            value.push('d');
        }
        if self.extensions.c {
            value.push('c');
        }
        value
    }
}

impl XcoreName for RvCoreName {
    fn from_abbr(value: &str) -> Result<Self, String> {
        value.parse()
    }

    fn from_full_name(value: &str) -> Result<Self, String> {
        let extensions = value
            .strip_prefix("riscv32")
            .map(|extensions| ("rv32", extensions))
            .or_else(|| {
                value
                    .strip_prefix("riscv64")
                    .map(|extensions| ("rv64", extensions))
            })
            .ok_or_else(|| format!("unknown RISC-V core name {value:?}"))?;
        format!("{}{}", extensions.0, extensions.1).parse()
    }

    fn abbr(&self) -> String {
        self.to_string()
    }

    fn full_name(&self) -> String {
        match self.xlen {
            RvXlen::X32 => format!("riscv32{}", self.extension_string()),
            RvXlen::X64 => format!("riscv64{}", self.extension_string()),
        }
    }

    fn rust_triple(&self) -> String {
        format!("{}-unknown-none-elf", self)
    }

    fn has_trustzone(&self) -> bool {
        false
    }

    fn is_64_bit(&self) -> bool {
        self.xlen == RvXlen::X64
    }
}

impl FromStr for RvCoreName {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(rest) = value.strip_prefix("rv") else {
            return Err(format!("unknown RISC-V core abbreviation {value:?}"));
        };
        let (xlen, extensions) = if let Some(extensions) = rest.strip_prefix("32") {
            (RvXlen::X32, extensions)
        } else if let Some(extensions) = rest.strip_prefix("64") {
            (RvXlen::X64, extensions)
        } else {
            return Err(format!("unknown RISC-V XLEN in {value:?}"));
        };

        let mut parsed = RvExtensions::default();
        for extension in extensions.chars() {
            match extension {
                'i' => parsed.i = true,
                'm' => parsed.m = true,
                'a' => parsed.a = true,
                'f' => parsed.f = true,
                'd' => parsed.d = true,
                'c' => parsed.c = true,
                other => return Err(format!("unknown RISC-V extension {other:?} in {value:?}")),
            }
        }

        Ok(Self {
            xlen,
            extensions: parsed,
        })
    }
}

impl fmt::Display for RvCoreName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.xlen {
            RvXlen::X32 => f.write_str("rv32")?,
            RvXlen::X64 => f.write_str("rv64")?,
        }
        f.write_str(&self.extension_string())
    }
}
