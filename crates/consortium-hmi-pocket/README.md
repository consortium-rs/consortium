# consortium-hmi-pocket

Native PocketJS backend for [`consortium-hmi`](../consortium-hmi).

Hosts a PocketJS bundle in QuickJS, mounts the Consortium command bridge into it, and
renders the UI with wgpu. The alternative to
[`consortium-hmi-webkit`](../consortium-hmi-webkit), with no browser engine and a far
smaller dependency footprint — select with `[hmi] engine = "pocket"`.

> **`publish = false`.** Its `pocket-mod`, `pocket-ui-wgpu`, and `pocket3d` dependencies
> come from an **upstream commit**, and cargo refuses to publish a package with a git
> dependency. This stays unpublished until PocketJS reaches crates.io. The pin is a SHA
> rather than a tag because the host touch entry this crate needs landed after `v0.7.0`
> and upstream has not cut a release since; it returns to a tag once one includes it. The
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

| Path               | Host                                                |
| ------------------ | --------------------------------------------------- |
| `HeadlessRenderer` | Offscreen wgpu target. No compositor, no `DISPLAY`. |
| `shell` feature    | A thin `winit` window host (`run` / `ShellConfig`). |

The headless path is what makes the whole stack testable in CI: QuickJS evaluation,
bridge dispatch, UI layout, and GPU submission all run with no display, and output can be
compared against image goldens — so a rendering regression is a diff rather than a bug
report.

The `shell` feature opens a native window (Wayland/X11 on Linux, Cocoa on macOS), drives
`PocketApp::frame`, and blits the logical-size UI to the surface. Both drive the same
frame contract, so a change verified headlessly holds for the shell too. The `kiosk`
example runs a built `<app>-main.{js,pak}` in it.

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

| Feature | Effect                                                                       |
| ------- | ---------------------------------------------------------------------------- |
| `shell` | Adds the `winit` window host (`run`, `ShellConfig`) and the `kiosk` example. |

Default is no features — the headless path only.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
