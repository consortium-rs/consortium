//! Safe Rust wrappers for the core Cog browser API.
//!
//! This crate intentionally wraps a small, useful subset of `cogcore-sys`.
//! WebKit and WPE types are left opaque for now and can be reached through raw
//! pointer escape hatches when an integration needs them.

pub mod content_manager;
pub mod error;
pub mod handlers;
pub mod init;
pub mod platform;
pub mod script;
pub mod shell;
pub mod utils;
pub mod view;
pub mod viewport;

mod object;
mod raw;

pub use content_manager::UserContentManager;
pub use error::{Error, GlibError, Result};
pub use handlers::{DirectoryFilesHandler, HostRoutesHandler, PrefixRoutesHandler, RequestHandler};
pub use init::init;
pub use platform::Platform;
pub use script::UserScript;
pub use shell::Shell;
pub use view::View;
pub use viewport::Viewport;

pub use gio;
pub use glib;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_clone<T: Clone>() {}

    #[test]
    fn public_wrappers_are_clone_handles() {
        assert_clone::<DirectoryFilesHandler>();
        assert_clone::<HostRoutesHandler>();
        assert_clone::<PrefixRoutesHandler>();
        assert_clone::<RequestHandler>();
        assert_clone::<Shell>();
        assert_clone::<View>();
        assert_clone::<Viewport>();
    }
}
