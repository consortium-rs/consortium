//! Safe Rust wrappers for the core Cog/WPE browser API used by Consortium's HMI.
//!
//! Sits directly above `cogcore-sys`, which owns the raw bindgen output for Cog,
//! GLib/GObject, WPE, and WebKit. This crate is the small safe layer callers should
//! normally use: owned object handles with GObject refcounting, Rust string conversion,
//! `GError` conversion, and a focused shell/view API.
//!
//! # Scope
//!
//! | Item                                                      | Covers                    |
//! | --------------------------------------------------------- | ------------------------- |
//! | [`init`]                                                  | Cog initialization         |
//! | [`Platform`]                                              | The global platform and its setup |
//! | [`Shell`]                                                 | Construction, request-handler registration, metadata |
//! | [`View`], [`Viewport`]                                    | View creation, visibility, fullscreen, viewport membership |
//! | [`DirectoryFilesHandler`], [`HostRoutesHandler`], [`PrefixRoutesHandler`] | Request routing |
//! | [`UserContentManager`], [`UserScript`]                    | Injecting scripts, receiving script messages |
//! | [`SecurityManager`]                                       | Security-policy configuration |
//! | [`utils`]                                                 | Cog URI and application-id conversion |
//!
//! **WebKit and WPE types stay opaque here.** Integration code that must cross into them
//! directly uses the raw escape hatches — `View::as_webkit_ptr()`,
//! `Platform::view_backend_ptr()`. Widening the safe surface is preferable to reaching for
//! those, but they exist so a gap in coverage is never a blocker.
//!
//! `gio` and `glib` are re-exported, because public GLib/GIO interop should use those
//! crates' types rather than re-wrapped equivalents.
//!
//! # Safety model
//!
//! This is the contract every wrapper keeps, and the thing to preserve when extending it:
//!
//! - **One owned GObject reference per wrapper**, released on drop. `Clone` takes another
//!   reference, so wrappers are cheap handles rather than deep copies.
//! - **A borrowed pointer is promoted to an owned handle only when the wrapper can safely
//!   call `g_object_ref`.** One that cannot be reffed is not wrapped.
//! - **`GError` is copied into [`GlibError`] and freed immediately**, so no GLib-owned
//!   error outlives the call that produced it.
//! - **Transfer-full strings are copied into `String` and released with `g_free`.**
//! - **Public APIs take Rust strings and reject interior NUL bytes before calling C.**
//!
//! Document each `unsafe` block with the pointer lifetime or ownership invariant it relies
//! on — the workspace denies `undocumented_unsafe_blocks`.
//!
//! # Platform notes
//!
//! Linux-only, and needs Cog, GLib/GIO, WPE, and WebKit development packages. Do not assume
//! `cargo check -p cogcore` runs on a host without them; on macOS this crate cannot build
//! at all. Use `.github/workflows/cogcore.yml`, a provisioned Linux machine, or
//! `just linux-image-full` for binding generation and wrapper validation.
#![cfg(target_os = "linux")]

pub mod content_manager;
pub mod error;
pub mod handlers;
pub mod init;
pub mod platform;
pub mod script;
pub mod security;
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
pub use security::SecurityManager;
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
