# consortium-hmi-webkit

Cog / WPE WebKit backend for [`consortium-hmi`](../consortium-hmi). Linux-only.

Runs a fullscreen kiosk web view and mounts the backend-neutral command bridge into it,
so page JavaScript can call application services.

## Usage

```rust,no_run
use consortium_hmi_webkit::{Bridge, KioskApp};

# async fn run() {
let mut bridge = Bridge::new();
bridge.handle("ping", |_| r#""pong""#.into());

let app = KioskApp::new("my-kiosk", None, None).unwrap();
app.attach_bridge(bridge).unwrap();
app.load().unwrap();
// The application then enters the GLib main loop.
# }
```

Register services in the bridge, attach it **before** loading, then enter the GLib main
loop the application owns.

## How the bridge reaches the page

This is where the backend-neutral bridge meets a concrete guest runtime, and the
interesting part is thread discipline.

1. A JS prelude (`bridge_prelude.js`) is injected as a **document-start user script** and
   registers a `bridge` script-message handler, giving page code a `postMessage`-shaped
   call site.
2. Invocations arrive on **GLib's main context** — WebKit's thread — but a handler's
   `ResponseFuture` generally needs a Tokio runtime to make progress.
3. So the future is spawned onto the Tokio handle the caller supplies, its `(id, json)`
   result is sent back over an unbounded channel, and a task on GLib's main context drains
   that channel and evaluates a resolve script in the view.

**Every WebKit call therefore happens on GLib's thread**, which is the invariant WebKit
requires — and the reason `consortium-hmi` hands back a future rather than polling one
itself. Responses resolve by correlation id, which the prelude's pending-promise map owns,
so concurrent calls are the page's business.

## Content source

| Build     | Loads                                                                  |
| --------- | ---------------------------------------------------------------------- |
| debug     | `CONSORTIUM_HMI_DEBUG_URL` if injected — a dev server, for live reload |
| otherwise | the compiled dist at `CONSORTIUM_HMI_DIST` (defaults to `dist`)        |

Both are injected at compile time by `consortium-builder` from `[hmi]` in
`Consortium.toml`, and read with `option_env!`. The dev-server path is debug-only on
purpose: a kiosk image must not silently depend on a developer's machine being reachable.

The frontend itself is built by the pipeline with `vp` or `npm`, selected by
`CONSORTIUM_HMI_BUILD_CMD` if set.

## Layering

```text
consortium-hmi              backend-neutral Bridge (no GLib, no WebKit, no Tokio)
  └── consortium-hmi-webkit    (this crate) KioskApp + bridge mounting
        └── cogcore               safe subset of Cog/WPE/WebKit
              └── cogcore-sys        raw bindgen output + vendored headers
```

`Bridge` is re-exported, so an application needs one dependency.

## Build requirements

Needs **Linux** with Cog, GLib/GIO, WPE, and WebKit development packages. It cannot be
built on macOS at all, and it is excluded from the macOS-only publish dry run
(`just publish-dry-host`).

On a host without those packages, prefer formatting, source review, and tests that do not
link Cog/WebKit. For real verification use the `cogcore.yml` workflow, a properly
provisioned Linux machine, or `just linux-image-full` — which layers the WPE WebKit/Cog
`.deb` from `consortium-rs/wpe-kiosk-stack-prebuilt` onto the container image.

## Alternative backend

[`consortium-hmi-pocket`](../consortium-hmi-pocket) is the native PocketJS backend, with a
much smaller dependency footprint and no browser engine. Select between them with
`[hmi] engine = "cog"` or `"pocket"`.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
