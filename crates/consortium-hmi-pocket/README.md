# consortium-hmi-pocket

Native PocketJS backend for [`consortium-hmi`](../consortium-hmi).

Hosts a PocketJS bundle in QuickJS, mounts the Consortium command bridge into it, and
renders the UI with wgpu. The alternative to
[`consortium-hmi-webkit`](../consortium-hmi-webkit), with no browser engine and a far
smaller dependency footprint — select with `[hmi] engine = "pocket"`.

> **`publish = false`.** Its `pocket-mod`, `pocket-ui-wgpu`, and `pocket3d` dependencies
> come from the upstream **`v0.7.0` git tag**, and cargo refuses to publish a package with
> a git dependency. This stays unpublished until PocketJS reaches crates.io. The
> `submodules/pocketjs` checkout is still needed for the bun/JS side.

## Usage

```rust,ignore
use consortium_hmi_pocket::{Bridge, PocketApp};

let mut bridge = Bridge::new();
bridge.handle("ping", |_| r#""pong""#.into());

// Loads the bundle+pak pair the builder injected.
let mut app = PocketApp::from_env(bridge, tokio::runtime::Handle::current())?;

loop {
    app.frame(&mut source)?;   // drain input, run JS, deliver responses, render
}
```

`PocketApp::from_files` takes explicit paths; `from_env` reads
`CONSORTIUM_HMI_POCKET_BUNDLE` and `CONSORTIUM_HMI_POCKET_PAK` (on-target
`{deploy_root}/hmi/<app>.{js,pak}`) plus `CONSORTIUM_HMI_LOGICAL_WIDTH` /
`_HEIGHT`, all injected at compile time by `consortium-builder`.

## Single-threaded by design

`PocketApp` is intentionally single-threaded. **The display/event-loop thread owns it**;
Tokio handlers run elsewhere and return through a channel drained by `frame` or
`pump_bridge`.

That is the same single-threaded UI rule `consortium-hmi-input`'s pull-based
`InputSource` enforces, and it means QuickJS is only ever entered from the frame loop —
never from a worker thread or a device callback.

### How the bridge is mounted

Two layers, split deliberately:

- The **native surface** stays a minimal queueing primitive: JS enqueues an invocation,
  completions come back over a channel keyed by ticket. It knows nothing about promises.
- A **JS adapter** (`service_prelude.js`), installed before the product bundle, owns
  correlation: the pending map, matching completions to callers, and integration with
  PocketJS's host-injected effect driver.

Keeping correlation in JS is what lets the native side stay a queue — and it matches
`consortium-hmi`'s premise that correlation ids belong to the guest. Responses are
delivered at a **frame boundary** rather than whenever they resolve.

## Rendering

| Path                    | Host                                              |
| ----------------------- | ------------------------------------------------- |
| `HeadlessRenderer`      | Offscreen wgpu target. No compositor, no `DISPLAY`. |
| `shell` feature         | A thin `winit` window host (`run` / `ShellConfig`). |

The headless path is what makes the whole stack testable in CI: QuickJS evaluation,
bridge dispatch, UI layout, and GPU submission all run with no display, and output can be
compared against image goldens — so a rendering regression is a diff rather than a bug
report.

The `shell` feature opens a native window (Wayland/X11 on Linux, Cocoa on macOS), drives
`PocketApp::frame`, and blits the logical-size UI to the surface. Both drive the same
frame contract, so a change verified headlessly holds for the shell too. The `kiosk`
example runs a built `<app>-main.{js,pak}` in it.

## Limitations

- **No production DRM/KMS shell yet.** The `shell` feature is a `winit` host, not a
  seat/`libinput` kiosk shell. `Hmi::validate` therefore rejects
  `engine = "pocket"` together with `backend = "drm"`.
- **`publish = false`,** as above.
- **Prototype status.** The crate is described in-tree as the native prototype; the
  `PocketApp` frame contract is the part intended to be stable, so a production event
  shell can be built on it.

## Pipeline support

`csti build` supports `engine = "pocket"` end to end: bun build, `<app>.js`/`.pak`
verification, `dist/hmi/` staging, and env injection. `[hmi] app` names the mounted
`-main` entry (e.g. `hero-main`) and is required for this engine. `[hmi.logical]` sets the
logical viewport, defaulting to 480 × 272.

## Features

| Feature | Effect                                                                 |
| ------- | ---------------------------------------------------------------------- |
| `shell` | Adds the `winit` window host (`run`, `ShellConfig`) and the `kiosk` example. |

Default is no features — the headless path only.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
