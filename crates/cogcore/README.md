# cogcore

Safe Rust wrappers for the core Cog/WPE browser API used by Consortium's HMI subsystem.

This crate sits above `cogcore-sys`. The `-sys` crate owns raw bindgen output for Cog, GLib/GObject, WPE, and WebKit. This crate provides the small safe layer callers should normally use: object handles, GObject refcounting, Rust string conversion, `GError` conversion, and a focused browser shell/view API.

## Scope

The current wrapper covers the core Cog flow:

- `init()` for Cog initialization.
- `Platform` for the global platform and platform setup.
- `Shell` for shell construction, request handler registration, and shell metadata.
- `View` and `Viewport` for view creation, visibility, fullscreen, and viewport
  membership.
- `DirectoryFilesHandler`, `HostRoutesHandler`, and `PrefixRoutesHandler` for
  request routing.
- Utility helpers for Cog URI and application ID conversion.

WebKit and WPE types remain opaque in this crate. When integration code needs to cross into WebKit/WPE directly, use the raw pointer escape hatches such as `View::as_webkit_ptr()` and `Platform::view_backend_ptr()`.

## Example

```rust,no_run
use cogcore::{
    DirectoryFilesHandler, Platform, Shell, View, Viewport, init,
};

fn main() -> cogcore::Result<()> {
    init(None, None)?;

    let shell = Shell::new("consortium-hmi", false)?;
    let platform = Platform::get()?;
    platform.setup(&shell, None)?;

    let viewport = Viewport::new()?;
    let view = View::new()?;
    viewport.add(&view);
    viewport.set_visible_view(&view);
    view.set_visible();

    let files = gio::File::for_path("/opt/consortium/hmi");
    let handler = DirectoryFilesHandler::new(&files)?;
    shell.set_request_handler("app", handler.as_request_handler())?;

    Ok(())
}
```

## Safety Model

Safe wrapper types own one GObject reference and release it on drop. Cloning a wrapper takes another GObject reference. Borrowed pointers returned by Cog are promoted to owned wrapper handles only when the wrapper can safely call `g_object_ref`.

Returned `GError` values are copied into `GlibError` and freed immediately. Returned transfer-full strings are copied into `String` and released with `g_free`. Public APIs accept Rust strings and reject interior NUL bytes before calling C.

## Platform Notes

Cog/WPE development is Linux-oriented. On macOS, prefer formatting and static review for this crate; do not assume `cargo check -p cogcore` or `cargo test -p cogcore` can run without Cog, GLib/GIO, WPE, and WebKit development packages.

Use `.github/workflows/cogcore.yml` or a Linux machine with the Cog/WebKit stack installed for binding generation and wrapper validation.

Useful validation commands on a suitable Linux host:

```bash
cargo check -p cogcore
cargo test -p cogcore
cargo clippy -p cogcore --all-targets
```
