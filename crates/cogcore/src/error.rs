use std::ffi::NulError;

use thiserror::Error;

use crate::raw;

/// Result type used by the safe Cog wrapper.
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by safe Cog wrapper APIs.
#[derive(Debug, Error)]
pub enum Error {
    /// A Rust string contained an interior NUL byte and could not be passed to C.
    #[error("string contains an interior NUL byte: {0}")]
    Nul(#[from] NulError),
    /// A C API returned an unexpected null pointer.
    #[error("{0} returned a null pointer")]
    Null(&'static str),
    /// A C API reported a `GError`.
    #[error("GLib error domain={} code={}: {}", .0.domain, .0.code, .0.message)]
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
    pub(crate) fn glib(error: *mut cogcore_sys::GError) -> Self {
        Self::Glib(raw::take_gerror(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_mentions_null_context() {
        let message = Error::Null("cog_platform_get").to_string();
        assert!(message.contains("cog_platform_get"));
    }
}
