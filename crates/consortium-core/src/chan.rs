//! A wrapper for channel identifiers, with platform-specific validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chan(u8);

/// For channels known at compile time
/// Validation happens at compile time via const fn
impl Chan {
    pub fn new<P: ChanValidate>(id: u8) -> Result<Self, P::Error> {
        P::validate(id)
    }

    /// In driver implementations, the implementor should refer to the platform-specific documentation for the valid range of channel IDs.
    /// Hence, this function is unsafe and should be used with caution.
    ///
    /// In real implementation, it should be like:
    ///
    /// ```compile_fail
    /// use consortium_core::Chan;
    ///
    /// // Here we assume a `Channel` for `stm32mp2` series.
    ///
    /// impl ChanValidate for stm32mp::Channel {
    ///    type Error = &'static str;
    ///
    ///    fn validate(id: u8) -> Result<Chan, Self::Error> {
    ///        if id < 16 {
    ///            Ok(unsafe { Chan::new_unchecked(id) })
    ///        } else {
    ///            Err("Invalid channel ID")
    ///        }
    ///    }
    /// }
    /// ```
    pub unsafe fn new_unchecked(id: u8) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u8 {
        self.0
    }
}

/// Platform-specific validation
pub trait ChanValidate {
    type Error;
    fn validate(id: u8) -> Result<Chan, Self::Error>;
}

impl From<Chan> for u8 {
    fn from(chan: Chan) -> Self {
        chan.0
    }
}
