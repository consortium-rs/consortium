//! Raw FFI bindings to Cog, WPE, WebKit, and GLib/GObject.
//!
//! **Generated, Linux-only, and not meant for direct use.** Depend on
//! [`cogcore`](https://docs.rs/cogcore) instead: it wraps a focused subset of this surface
//! with owned handles, GObject refcounting, and `GError` conversion.
//!
//! `build.rs` probes the system Cog/WPE/WebKit/GLib/libsoup headers with `pkg-config`, runs
//! `bindgen` over `wrapper.h`, and compiles the **vendored** Cog Core C sources under
//! `vendor/include/` into a static archive this crate owns.
//!
//! That last part is deliberate rather than incidental: linking a system `libcogcore` would
//! mean the headers and the library could disagree. The packaged Cog does not export
//! everything the vendored headers declare (`cog_init` and `cog_view_new` among them), so
//! the sources are built here to match. The source list mirrors `cogcore_sources` in the
//! vendored `meson.build`, minus `cog-gamepad-manette.c`, which the checked-in
//! `cog-config.h` disables.
//!
//! Every item is `cfg(target_os = "linux")`, and the crate-wide lint allows exist because
//! bindgen output does not follow Rust naming conventions and not every declaration is
//! referenced in every build.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![cfg(target_os = "linux")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
