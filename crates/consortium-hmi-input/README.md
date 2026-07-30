# consortium-hmi-input

Backend-neutral, `no_std` input event model for Consortium HMI runtimes.

`no_std`, allocation-free, and `IpcSafe` — so one event definition compiles for firmware
and for the Linux host, and can travel between them.

## The problem

HMI backends and the event *sources* that drive them speak entirely different concrete
APIs:

| Source                        | API                              |
| ----------------------------- | -------------------------------- |
| Desktop window                | `winit`                           |
| Linux kiosk                   | `libinput` / `evdev`              |
| Bare-metal core, local        | `embedded-hal` touch/GPIO/encoder  |
| Bare-metal core, over IPC     | a Consortium channel carrying events |

This crate is the neutral middle: one `InputEvent` type plus one pull-based `InputSource`
contract that every source implements.

## Two decisions that carry the design

### Logical UI coordinates, not device coordinates

`InputEvent` positions are in **logical UI coordinates** — the same units the HMI app
renders at, from `[hmi.logical]` in `Consortium.toml`.

That puts device-to-logical calibration in the *source*, where the device is, rather than
in each backend. A resistive touch panel's calibration, a trackpad's acceleration, and a
mouse's raw pixels all resolve to the same coordinate space before anything downstream
sees them, so a backend never asks "which coordinate system is this event in".

Coordinates are `i16`: deliberate, because it spans any realistic panel while staying a
**fixed, `IpcSafe` width across cores**. A `usize` would be 32 bits on an M-core and 64 on
an A-core, which is exactly the class of bug `IpcSafe` exists to reject.

### Events can cross a core boundary

`InputEvent` derives `consortium_ipc::IpcSafe` *and* `serde`, so a real-time core can
serialize events with a Consortium codec and forward them to a Linux HMI host **exactly
like any other cross-core message** — no separate input transport, no special-casing.

That is what makes "the M-core reads the touch panel, the A-core draws the UI" a normal
arrangement rather than a bespoke one.

## The `InputSource` contract

```rust,ignore
pub trait InputSource {
    /// Next pending event, or `None` when drained for this poll. Must not block.
    fn poll_event(&mut self) -> Option<InputEvent>;
}
```

Pull-based rather than async or callback-driven, and that is on purpose: one trait serves
a bare-metal frame loop and a Linux host alike, with no executor assumption.

It also enforces the **single-threaded UI rule**. A display/event-loop thread drains the
source once per frame and folds the events into its backend, so the UI guest is never
called from a device callback or a worker thread — only at a frame boundary. A
callback-driven design would make that discipline the implementor's problem instead of the
trait's.

`SliceSource` replays a fixed slice of events once, for deterministic tests and scripted
headless playback, without needing `alloc`.

## `#[non_exhaustive]`

`InputEvent` is `#[non_exhaustive]`. **Match with a wildcard arm**, so new event kinds can
be added without a breaking change — which matters for a type that sits between four
different source families and two backends.

## Contents

`ButtonCode`, `InputEvent`, `PointerButton`, `ScrollAxis`, `TouchPhase`, `InputSource`,
`SliceSource`.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
