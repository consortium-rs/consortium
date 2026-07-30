# cogcore

Safe Rust wrappers for the core Cog/WPE browser API used by Consortium's HMI subsystem.

This crate sits above `cogcore-sys`. The `-sys` crate owns raw bindgen output for Cog, GLib/GObject, WPE, and WebKit. This crate provides the small safe layer callers should normally use: object handles, GObject refcounting, Rust string conversion, `GError` conversion, and a focused browser shell/view API.

The consumer is [`consortium-hmi-webkit`](../consortium-hmi-webkit), which builds a kiosk
app on top of it:

```text
consortium-hmi-webkit      KioskApp + bridge mounting
  └── cogcore                 (this crate) safe subset
        └── cogcore-sys          raw bindgen output + vendored Cog Core C
```

## Scope

| Item                                                                      | Covers                          |
| ------------------------------------------------------------------------- | ------------------------------- |
| `init()`                                                                  | Cog initialization               |
| `Platform`                                                                | The global platform and its setup |
| `Shell`                                                                   | Construction, request-handler registration, metadata |
| `View`, `Viewport`                                                        | View creation, visibility, fullscreen, viewport membership |
| `DirectoryFilesHandler`, `HostRoutesHandler`, `PrefixRoutesHandler`        | Request routing                  |
| `UserContentManager`, `UserScript`                                        | Injecting scripts, receiving script messages |
| `SecurityManager`                                                         | Security-policy configuration     |
| `utils`                                                                   | Cog URI and application-ID conversion |

`UserContentManager` and `UserScript` are what `consortium-hmi-webkit` uses to inject its
bridge prelude as a document-start script and register the `bridge` script-message
handler.

`gio` and `glib` are re-exported, because public GLib/GIO interop should use those crates'
types rather than re-wrapped equivalents.

WebKit and WPE types remain opaque in this crate. When integration code needs to cross into
WebKit/WPE directly, use the raw pointer escape hatches such as `View::as_webkit_ptr()` and
`Platform::view_backend_ptr()`. Widening the safe surface is preferable to reaching for
those, but they exist so a gap in coverage is never a blocker.

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

This is the contract every wrapper keeps, and the thing to preserve when extending the
crate:

- **One owned GObject reference per wrapper**, released on drop. `Clone` takes another
  reference, so wrappers are cheap handles rather than deep copies.
- **A borrowed pointer returned by Cog is promoted to an owned handle only when the wrapper
  can safely call `g_object_ref`.** One that cannot be reffed is not wrapped.
- **`GError` values are copied into `GlibError` and freed immediately**, so no GLib-owned
  error outlives the call that produced it.
- **Transfer-full strings are copied into `String` and released with `g_free`.**
- **Public APIs accept Rust strings and reject interior NUL bytes before calling C.**

When adding a wrapper, document each `unsafe` block with the pointer lifetime or ownership
invariant it relies on — the workspace denies `undocumented_unsafe_blocks`. A unit test
asserts that every public wrapper type is `Clone`, which is the handle-semantics contract
made checkable.

## Platform Notes

Cog/WPE development is Linux-oriented. On macOS, prefer formatting and static review for this crate; do not assume `cargo check -p cogcore` or `cargo test -p cogcore` can run without Cog, GLib/GIO, WPE, and WebKit development packages.

Use `.github/workflows/cogcore.yml` or a Linux machine with the Cog/WebKit stack installed for binding generation and wrapper validation.

Useful validation commands on a suitable Linux host:

```bash
cargo check -p cogcore
cargo test -p cogcore
cargo clippy -p cogcore --all-targets
```
