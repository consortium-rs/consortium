//! A wrapper for channel identifiers, with platform-specific validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chan(u8);

/// For channels known at compile time
/// Validation happens at compile time via const fn
impl Chan {
    pub fn new<P: ChanValidate>(id: u8) -> Result<Self, P::Error> {
        P::validate(id)
    }

    /// # Safety
    ///
    /// This constructor does not perform any validation on the provided `id`. It is the caller's responsibility to ensure that the `id` is valid for the intended platform and use case.
    ///
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
    ///
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

#[cfg(test)]
mod tests {
    use super::*;

    struct Max16;
    impl ChanValidate for Max16 {
        type Error = &'static str;
        fn validate(id: u8) -> Result<Chan, Self::Error> {
            if id < 16 {
                Ok(unsafe { Chan::new_unchecked(id) })
            } else {
                Err("channel id out of range")
            }
        }
    }

    #[test]
    fn new_unchecked_stores_id() {
        let ch = unsafe { Chan::new_unchecked(5) };
        assert_eq!(ch.id(), 5);
    }

    #[test]
    fn new_unchecked_zero() {
        let ch = unsafe { Chan::new_unchecked(0) };
        assert_eq!(ch.id(), 0);
    }

    #[test]
    fn new_unchecked_max() {
        let ch = unsafe { Chan::new_unchecked(255) };
        assert_eq!(ch.id(), 255);
    }

    #[test]
    fn from_chan_gives_inner_byte() {
        let ch = unsafe { Chan::new_unchecked(7) };
        assert_eq!(u8::from(ch), 7);
    }

    #[test]
    fn validate_accepts_valid_id() {
        let ch = Chan::new::<Max16>(0).unwrap();
        assert_eq!(ch.id(), 0);
        let ch = Chan::new::<Max16>(15).unwrap();
        assert_eq!(ch.id(), 15);
    }

    #[test]
    fn validate_rejects_out_of_range() {
        assert!(Chan::new::<Max16>(16).is_err());
        assert!(Chan::new::<Max16>(255).is_err());
    }

    #[test]
    fn chan_equality() {
        let a = unsafe { Chan::new_unchecked(3) };
        let b = unsafe { Chan::new_unchecked(3) };
        let c = unsafe { Chan::new_unchecked(4) };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn chan_is_copy() {
        let a = unsafe { Chan::new_unchecked(1) };
        let b = a; // copy
        assert_eq!(a.id(), b.id());
    }
}
