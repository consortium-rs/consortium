use std::{error, ffi::NulError, fmt};

use crate::raw;

/// Result type used by the safe Cog wrapper.
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by safe Cog wrapper APIs.
#[derive(Debug)]
pub enum Error {
    /// A Rust string contained an interior NUL byte and could not be passed to C.
    Nul(NulError),
    /// A C API returned an unexpected null pointer.
    Null(&'static str),
    /// A C API reported a `GError`.
    Glib(GlibError),
}

/// Owned copy of a GLib `GError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlibError {
    /// GLib error domain quark.
    pub domain: u32,
    /// Domain-specific error code.
    pub code: i32,
    /// Human-readable error message.
    pub message: String,
}

impl Error {
    pub(crate) fn glib(error: *mut libcogcore_sys::GError) -> Self {
        Self::Glib(raw::take_gerror(error))
    }
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Self {
        Self::Nul(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nul(error) => write!(f, "string contains an interior NUL byte: {error}"),
            Self::Null(context) => write!(f, "{context} returned a null pointer"),
            Self::Glib(error) => write!(
                f,
                "GLib error domain={} code={}: {}",
                error.domain, error.code, error.message
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_mentions_null_context() {
        let message = Error::Null("cog_platform_get").to_string();
        assert!(message.contains("cog_platform_get"));
    }
}
