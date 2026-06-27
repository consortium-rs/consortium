/// Common metadata for CPU core names used by Consortium configuration.
pub trait XcoreName: Sized {
    /// Parses the short configuration spelling, such as `a55` or `cm33`.
    fn from_abbr(value: &str) -> Result<Self, String>;

    /// Parses the canonical full core name, such as `cortex-a55`.
    fn from_full_name(value: &str) -> Result<Self, String>;

    /// Short configuration spelling.
    fn abbr(&self) -> String;

    /// Canonical full core name.
    fn full_name(&self) -> String;

    /// Rust target triple used for bare-metal builds for this core family.
    fn rust_triple(&self) -> String;

    /// Whether this core family can support TrustZone-style secure/non-secure
    /// partitioning.
    fn has_trustzone(&self) -> bool;

    /// Whether this core is a 64-bit execution target.
    fn is_64_bit(&self) -> bool;

    /// Whether this core is a 32-bit execution target.
    fn is_32_bit(&self) -> bool {
        !self.is_64_bit()
    }

    /// Consumes the core name and returns its short configuration spelling.
    fn into_abbr(self) -> String {
        self.abbr()
    }

    /// Consumes the core name and returns its canonical full core name.
    fn into_full_name(self) -> String {
        self.full_name()
    }
}
