# consortium-hmi-pocket

Native PocketJS backend for [`consortium-hmi`](../consortium-hmi).

Hosts a PocketJS bundle in QuickJS, mounts the Consortium command bridge into it, and
rasterizes the UI on the CPU. The alternative to
[`consortium-hmi-webkit`](../consortium-hmi-webkit), with no browser engine and a far
smaller dependency footprint — select with `[hmi] engine = "pocket"`.

Built on the published PocketJS crates (MIT): `pocketjs-core` (retained tree, layout,
DrawList), `pocket-runtime` (op dispatch, asset admission), `pocket-guest-rquickjs`
(QuickJS realm and the generated `ui.*` bindings), and `pocket-raster` (software
DrawList rasterizer). No GPU stack and no `pocket3d` are involved.

> **Versions move together.** The PocketJS crate versions match the npm
> `@pocketjs/framework` release that `examples/pocket-hmi/package.json` pins (`0.16.0`),
> so the Rust host and the bun-built bundle share one binding generation; the guest
> refuses a bundle whose `ui.__hostAbi` differs. Bump the two together, and pin the same
> version in your own HMI project. **MSRV is 1.93**, above the workspace's, because the
> PocketJS crates require it.

## Usage

```rust,ignore
use consortium_hmi::Bridge;
use consortium_hmi_pocket::{FrameRenderer, PocketApp};

let mut bridge = Bridge::new();
bridge.handle("ping", |_| r#""pong""#.into());

// Loads the bundle+pak pair the builder injected.
let mut app = PocketApp::from_env(bridge, tokio::runtime::Handle::current())?;

let mut renderer = FrameRenderer::new(1)?;
let (width, height) = renderer.size(&app);
let mut frame = vec![0; (width * height * 4) as usize];
loop {
    app.frame_from(&mut source)?;                  // drain input, deliver responses, run JS, tick
    renderer.render_xrgb_into(&app, &mut frame)?;  // DrawList -> XRGB8888 pixels
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

| Path            | Host                                                                          |
| --------------- | ----------------------------------------------------------------------------- |
| `FrameRenderer` | `pocket-raster` into a caller-owned RGBA8 or XRGB8888 buffer, 1–4× scale.     |
| `shell` feature | A thin `winit` window host (`run` / `ShellConfig`) presenting via softbuffer. |

Rendering is CPU-only, so the whole stack is testable in CI: QuickJS evaluation, bridge
dispatch, UI layout, and rasterization run with no GPU and no display, and output can be
compared against image goldens — so a rendering regression is a diff rather than a bug
report. XRGB8888 is the layout of a KMS dumb buffer, which is what a DRM/KMS presenter
writes.

The `shell` feature opens a native window (Wayland/X11 on Linux, Cocoa on macOS), drives
`PocketApp::frame_pad` at 60 Hz, rasterizes at the largest integer scale that fits the
window, and letterboxes the result into the surface. It uses the same `FrameRenderer`,
so the window shows the pixels the headless path produces. The `kiosk` example runs a
built `<app>-main.{js,pak}` in it.

## Input

Buttons fold into the PSP bitmask; touch contacts and the pointer fold into the
packed-touch third argument of `globalThis.frame`. A **mouse** is presented to the
guest as a touch contact at the reserved id `POINTER_CONTACT_ID`, present only
while the primary button is held, plus `CIRCLE` in the button mask for the
duration of the hold — the same pairing PocketJS's own `macos-widget` reference
host uses.

That pairing is what makes a click land on the node under the cursor. PocketJS's
`render()` runs app `onFrame` hooks _before_ input edge detection, so an app that
focuses the hit node from `touches()` has already moved focus by the time the
`CIRCLE` edge fires `onPress` in the same frame:

```js
import { touches, hitFocusable, focusNode } from '@pocketjs/framework/input'
onFrame(() => {
  const [c] = touches()
  if (c) focusNode(hitFocusable(c.x, c.y))
})
```

Pass a `null` hit straight through, as above. Guarding the call — `if (node)
focusNode(node)` — leaves focus wherever it last landed, and since `handleFrame`
fires `onPress` on whatever is focused, a press on empty space then activates
the control the pointer merely passed over earlier. The framework's own cursor
mode clears focus on a miss for the same reason.

Contacts survive the frame boundary: one that opens and closes between two frames
is held for exactly one frame rather than vanishing, so a tap or click shorter
than ~16 ms at 60 Hz still reaches the guest exactly once.

Three consequences worth knowing:

- **An app must opt in.** A `focusable`/`onPress` component with no hover-focus
  hook is only reachable by d-pad; PocketJS starts with nothing focused and never
  auto-focuses, so a click before any d-pad press activates nothing.
- **Hover does not reach the guest.** Motion with no button held updates the
  stored position but emits no contact, because a contact means _contact_. Real
  hover needs `input.pointer`, a capability PocketJS specifies but does not yet
  implement in the framework; see the upstream `enableCursor({ source: "pointer" })`
  work. Once that lands, this crate can move to the 4-argument frame entry and
  drop the app-side hook.
- **The logical viewport is capped at 1023 px per axis.** The packed-contact
  format carries 10 bits per axis, so anything larger would render at full size
  and then clamp every contact onto the edge. `PocketApp::new` and
  `[hmi.logical]` validation both reject it instead. This is a PocketJS wire
  limit, so it does not apply to `engine = "cog"`.

## Limitations

- **No production DRM/KMS shell yet.** The `shell` feature is a `winit` host, not a
  seat/`libinput` kiosk shell. `Hmi::validate` therefore rejects
  `engine = "pocket"` together with `backend = "drm"`.
- **Every frame is redrawn.** `FrameRenderer` rasterizes the full DrawList; damage-limited
  presentation (`pocket-present`) is not wired in yet.
- **Prototype status.** The crate is described in-tree as the native prototype; the
  `PocketApp` frame contract is the part intended to be stable, so a production event
  shell can be built on it.

## Pipeline support

`csti build` supports `engine = "pocket"` end to end: bun build, `<app>.js`/`.pak`
verification, `dist/hmi/` staging, and env injection. `[hmi] source` is a bun project
that depends on the npm `@pocketjs/framework` (`csti fetch` runs `bun install` there);
[`examples/pocket-hmi`](../../examples/pocket-hmi) is a minimal one. `[hmi] app` names
the mounted entry: `<source>/<app>.tsx` (or `.ts`) when it exists, otherwise a demo
shipped in the package (e.g. `hero-main`). It is required for this engine. `[hmi.logical]` sets the
logical viewport, defaulting to 480 × 272.

## Features

| Feature | Effect                                                                                  |
| ------- | --------------------------------------------------------------------------------------- |
| `shell` | Adds the `winit`/softbuffer window host (`run`, `ShellConfig`) and the `kiosk` example. |

Default is no features — the headless path only.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
