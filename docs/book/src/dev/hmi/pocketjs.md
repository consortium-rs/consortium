# PocketJS Repository and Integration Notes

This page records what PocketJS provides, what is ready to reuse, and what is
still required to make it Consortium's primary HMI runtime.

The review is pinned to git commit
[`26d418103143cec26faa332529cbea86cfa8c530`](https://github.com/pocket-stack/pocketjs/tree/26d418103143cec26faa332529cbea86cfa8c530)
(`v0.6.0-1-g26d4181`, reviewed 2026-07-19). The local checkout is the
`pocketjs` submodule at `submodules/pocketjs`. PocketJS is moving quickly, so
verify these notes against the pinned sources before implementing a phase.

## What PocketJS is

PocketJS is a native retained-mode UI stack, not a small browser. Applications
use Solid or Vue Vapor JSX and a compile-time Tailwind subset, but there is no
DOM, CSS engine, browser networking stack, or WebView at runtime. A compiler
turns an application into two main artifacts:

- `<app>.js`: a bundled guest program executed by QuickJS on native hosts;
- `<app>.pak`: compiled styles, baked font atlases, images, sprites, and other
  binary assets.

The guest issues synchronous `ui.*` operations to a Rust UI core. The core owns
the tree, flex layout, text, focus, animation, textures, and draw-list
generation. A host renders that draw list with a platform backend. This keeps
the hot UI state and per-frame work in Rust while preserving a reactive JSX
authoring model.

```text
Solid/Vue Vapor JSX
        |
        v
PocketJS compiler -------> app.js + app.pak
                                |
                                v
                         QuickJS guest
                         /            \
                ui.* surface          consortium.* surface
                     |                         |
                     v                         v
             pocketjs-core              command router
                     |                 /      |       \
                  DrawList           IPC     TEE     Linux services
                     |
                     v
              wgpu renderer
                     |
              Wayland / headless
              (DRM/KMS is future work)
```

The separation between `ui.*` and `consortium.*` is important. PocketJS's
`HostOps` is a rendering protocol and should not acquire product-specific IPC,
TEE, filesystem, or process operations. `pocket-mod` already supports mounting
additional named QuickJS surfaces, which is the appropriate extension point
for Consortium services.

## Repository map

| Path                             | Purpose                                                                                                 | Consortium relevance                                                           |
| -------------------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `src/`                           | Solid/Vue renderers, components, input, animation, platform checks, virtual clock, and effect shell     | Frontend SDK and the guest-side half of the host contract                      |
| `compiler/`, `schema/`, `spec/`  | JSX/style/font/asset compiler, manifest schema, target registry, and generated wire constants           | Build integration and a future Consortium Linux target profile                 |
| `core/`                          | `no_std + alloc` retained UI core using Taffy, with draw-list and deterministic software raster support | Reusable renderer-independent UI state; no Linux or QuickJS dependency         |
| `wasm/`, `host-web/`             | WASM core mirror and browser development host                                                           | Fast frontend iteration, but not the intended production HMI runtime           |
| `host-sim/`, `test/`             | Deterministic headless simulation, input tapes, goldens, and contract tests                             | Strong basis for HMI acceptance tests and replaying hardware/IPC traces        |
| `pocket3d/crates/pocket-mod`     | `rquickjs` guest lifecycle, named surface mounting, and one guest turn per tick                         | Native JS host and the extension point for `consortium.*`                      |
| `pocket3d/crates/pocket-ui-wgpu` | Pak loading, `ui.*` mounting, and DrawList-to-wgpu rendering                                            | Closest reusable native Linux rendering layer                                  |
| `pocket3d/examples/uihost`       | winit window and headless example using `pocket-mod` and `pocket-ui-wgpu`                               | Bring-up reference, not yet a production embedded shell                        |
| `native/`, `native-vita/`        | PSP and Vita QuickJS/render/input hosts                                                                 | Mature examples of native packaging and input delivery; not Linux dependencies |
| `aot/`                           | Separate ahead-of-time Game Boy Advance experiment                                                      | Outside the Consortium HMI path                                                |

The main design references are `README.md`, `DESIGN.md`, `RUNTIMES.md`,
`DETERMINISM.md`, `site/content/docs/native-contract.md`, and
`site/content/docs/platform-contracts.md` in the submodule.

## Useful contracts

### UI and host ABI

`src/host.ts` defines `HostOps`, the synchronous QuickJS-to-native UI surface.
The operation numbers and shared binary formats are pinned in `spec/spec.ts`
and generated into the Rust core. Manifest-driven bundles also embed a target
name and host ABI and refuse to run when the native host reports a different
contract.

The current stock target registry contains only `psp` and `vita`. The desktop
wgpu surface reports `__host = "desktop"`, does not report `__hostAbi`, and the
`uihost` example normally consumes low-level compiler output without an
embedded target contract. Therefore the desktop example demonstrates the
runtime pieces, but it is not yet a manifest-backed Linux target that
Consortium can package as-is.

### Frame and effect model

The host gives the guest one turn per frame. PocketJS uses a virtual clock and
delivers external effect results at frame boundaries, which makes a run a
deterministic fold over input. This is a good fit for Consortium: IPC responses,
TEE results, and Linux service events can be queued by Tokio and delivered to
QuickJS only when the UI thread begins its next frame.

The guest must never be called from a Tokio worker or a device callback.
QuickJS and the retained UI are single-threaded. Cross-thread work should use
bounded command and completion queues, with all QuickJS access retained on the
display/event-loop thread.

### Build artifacts

The stable custom-host projection in `src/manifest/host-build-inputs.ts`
provides the app output name, target, host ABI, logical and physical viewport,
presentation mode, and raster density. It can also produce the `POCKETJS_*`
environment expected by a native host build.

Consortium currently has a different assumption: its HMI compiler runs `vp
build` or `npm run build`, verifies a non-empty directory, copies that directory
to `dist/www`, and injects `CONSORTIUM_HMI_DIST` for Cog to serve. PocketJS
needs an engine-specific artifact contract for its `.js` and `.pak` files;
those should not be disguised as a website directory.

## Prototype status

The initial adapter split now exists in the workspace:

- `consortium-hmi` is a backend-neutral, cloneable command registry. It parses
  invocation envelopes and returns `Send` response futures without depending
  on Tokio, Cog, GLib, WebKit, QuickJS, or a renderer.
- `consortium-hmi-webkit` contains `KioskApp` and adapts the registry to
  WebKit's script-message channel using a caller-selected Tokio runtime.
- `consortium-hmi-pocket` loads JS/pak artifacts, mounts `ui` and a separate
  Consortium QuickJS surface, installs `__BRIDGE__` plus PocketJS's host effect
  driver, delivers completed commands at frame boundaries, and exposes an
  offscreen wgpu renderer for smoke tests and goldens.
- `consortium-hmi-input` is the backend-neutral, `no_std` input layer: an
  `InputEvent` model in logical UI coordinates plus a pull-based `InputSource`
  trait. `InputEvent` derives `IpcSafe`/`serde`, so a real-time core can
  serialize events with a Consortium codec and forward them over IPC to the
  Linux HMI host. The pocket crate's `PadState` folds events into PocketJS's
  `frame(buttons, analog)` contract; the winit shell and future
  `libinput`/`evdev` and IPC readers all present as an `InputSource`.

This is deliberately a headless/native-host prototype. It does not yet make
`engine = "pocketjs"` valid, build PocketJS artifacts through `csti`, or own a
Wayland/DRM window and Linux input loop.

## Fit and current gaps

| Concern               | Reusable now                                                                                                                                                                                           | Missing for a production Consortium HMI                                                                                                                           |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UI core               | `pocketjs-core`, Taffy layout, text, focus, animation, draw list                                                                                                                                       | Product viewport and performance limits must be validated on A-core targets                                                                                       |
| JS runtime            | `pocket-mod` embeds QuickJS and mounts named surfaces                                                                                                                                                  | Lifecycle, error reporting, watchdog policy, and Tokio handoff in Consortium                                                                                      |
| GPU                   | `pocket-ui-wgpu` and headless/windowed examples                                                                                                                                                        | A lean 2D-only dependency boundary and target GPU/driver validation                                                                                               |
| Display               | winit window and wgpu surface                                                                                                                                                                          | Fullscreen Wayland kiosk behavior; direct DRM/KMS presentation is not implemented                                                                                 |
| Input                 | Backend-neutral `consortium-hmi-input` event model + `InputSource` trait, folded to button/analog by the pocket `PadState`; winit shell translates key/pointer/touch; Vita shows packed touch delivery | Concrete `libinput`/`evdev` and IPC `InputSource` backends; delivering pointer/touch to the guest (packed-touch entry); focus, calibration, and hot-plug behavior |
| Build                 | Bun compiler produces deterministic JS/pak artifacts                                                                                                                                                   | Linux target profile, builder dispatch, staging layout, cross-build inputs, and BitBake integration                                                               |
| App services          | Portable registry plus Pocket effect/Promise adapter                                                                                                                                                   | Typed errors, cancellation, backpressure, and production IPC/TEE handlers                                                                                         |
| Testing               | Headless wgpu/software rendering, tapes, goldens                                                                                                                                                       | Consortium fixtures that record/replay IPC and assert both commands and pixels                                                                                    |
| Browser compatibility | Web/WASM development host                                                                                                                                                                              | Browser-only libraries and arbitrary HTML/CSS cannot be reused unchanged                                                                                          |

There is also a dependency-shape issue worth fixing upstream. The current
`pocket-ui-wgpu` crate uses the GPU bootstrap from the broader `pocket3d`
crate, whose manifest also brings 3D-oriented dependencies. A primary 2D HMI
should either extract a small shared wgpu substrate or let the Consortium host
construct the wgpu device and pass it into the UI renderer.

## Proposed Consortium architecture

### 1. Make the application bridge backend-neutral (prototype complete)

The command handler registry and JSON invocation protocol have been extracted
from the WebKit backend. The backend-neutral layer owns:

- registration of synchronous and asynchronous handlers;
- invocation IDs, typed serialization boundaries, and structured errors;
- `Send` response futures without holding the registry lock across user work.

Cog is now one adapter over this router. This preserves the existing
`window.__BRIDGE__.invoke()` behavior while removing Cog, GLib, and WebKit
types from the service API.

Backpressure, cancellation, and task ownership remain adapter-level production
work.

### 2. Add a PocketJS host adapter (headless prototype complete)

`consortium-hmi-pocket` builds a `PocketApp` shell from `pocket-mod`,
`pocketjs-core`, and `pocket-ui-wgpu`. It currently:

1. load the planned JS and pak artifacts;
2. mount the PocketJS `ui` surface;
3. mount a separate `consortium` service surface;
4. evaluate the bundle only after both surfaces exist;
5. owns the single-threaded guest and frame pump;
6. drain completed service calls at a frame boundary;
7. ticks the UI core and can render into a headless wgpu target.

Consortium owns the product viewport configuration. A Pocket HMI declares its
logical pixel dimensions in the project manifest; `csti build` injects them
into the compiled application and `PocketApp::from_env()` applies them when it
creates the retained UI surface:

```toml
[hmi]
engine = "pocket"
backend = "wayland"
source = "submodules/pocketjs"
build = "submodules/pocketjs/dist"
app = "hero-main"

[hmi.logical]
width = 800
height = 480
```

Omitting `[hmi.logical]` preserves the `480` × `272` compatibility default.

A production display/input event loop and presentation shell are still
pending.

The guest SDK should connect `runEffect(kind, payload, callback)` to this
surface. An enqueue operation sends `{id, kind, payload}` to Rust; the UI
thread later delivers `{id, result}` or `{id, error}` before a guest frame.
This keeps external state in PocketJS's deterministic effect trace and avoids
Promise timing as an invisible input.

### 3. Introduce a real PocketJS Linux target

Do not ship a production HMI by compiling against the PSP profile or by
disabling the runtime contract. Add a first-class target or a supported custom
target registration mechanism with:

- a target name and append-only host ABI;
- product logical/physical viewport and raster density;
- truthful capabilities for buttons, cursor, touch, and baked glyphs;
- a native host that publishes matching `__host` and `__hostAbi` values;
- compiler, host, and headless contract tests.

Linux panels are not all the same size. The target design must decide whether
display geometry is a Consortium-generated product profile or a constrained
set of PocketJS profiles. It should not hard-code the PSP's 480×272 logical
viewport merely because the desktop demo does so.

### 4. Make build and staging engine-specific

Extend `HmiEngine` with `pocketjs` and keep `cog` as an explicit fallback.
The pipeline should dispatch on the engine instead of treating every HMI as a
web directory:

```text
pocketjs -> compile/validate plan -> app.js + app.pak -> dist/hmi/
cog      -> frontend build        -> web directory   -> dist/www/
```

The application build then receives explicit PocketJS artifact paths and
contract metadata. Embedding the artifacts in the executable is attractive
for integrity and atomic updates; staging them beside the executable is
friendlier to iteration and BitBake packaging. The first implementation can
stage files, then add an opt-in embed mode after measuring size and startup.

### 5. Bring up display and input in this order

1. Headless host for CI and command/effect tests.
2. Fullscreen Wayland host using the existing winit/wgpu path.
3. Keyboard and button navigation, then touch and pointer delivery in logical
   coordinates.
4. Cross-compilation and GPU validation in the i.MX9 and STM32MP2 Linux SDKs.
5. Direct DRM/KMS only after the Wayland path is stable; the current PocketJS
   repository does not provide a direct KMS presenter.

This order makes PocketJS the primary design without making the initial work
depend on board graphics bring-up. Cog remains available when a product needs
browser APIs or while a target lacks the required native display backend.

## Suggested implementation slices

Each slice should be independently testable and small enough to upstream
PocketJS changes where they belong.

1. **Linux contract spike:** add a manifest-backed desktop/Linux profile,
   publish `__hostAbi`, support product viewport input, and run a compiled demo
   through the headless wgpu host.
2. **Bridge extraction:** turn the current WebKit handler map into a
   backend-neutral command router while keeping all Cog tests passing.
3. **Pocket service prototype:** mount `consortium.*`, invoke one synchronous
   and one Tokio-backed asynchronous command, and deliver results at frame
   boundaries.
4. **Builder artifacts:** add `engine = "pocketjs"`, compile JS/pak, stage
   them under `dist/hmi`, and inject their deployed paths into the application.
5. **Wayland kiosk:** add fullscreen presentation, resize policy, clean
   shutdown, keyboard/button input, and a systemd smoke test.
6. **Touch and system integration:** map panel input, expose IPC/TEE handlers,
   and add recorded-effect/pixel-golden tests.
7. **Target hardening:** validate Yocto/OpenSTLinux builds, startup time,
   steady-state memory, frame time, font coverage, GPU recovery, and watchdog
   behavior before making PocketJS the default engine.

## Licensing and update policy

PocketJS is MIT licensed. Its vendored Inter fonts are under the SIL Open Font
License. Both are compatible with Consortium's dependency policy, subject to
preserving their notices in source and binary distributions.

Keep the integration pinned through the git submodule while the native host
contracts evolve. Prefer sending changes to PocketJS's target registry,
`pocket-mod`, and `pocket-ui-wgpu` upstream rather than carrying generated or
protocol patches in Consortium. When updating the pin, rerun the host ABI,
artifact, headless replay, and target cross-build checks before accepting it.
