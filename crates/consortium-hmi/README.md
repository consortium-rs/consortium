# consortium-hmi

Backend-neutral command bridge for Consortium HMI runtimes.

A UI needs to call into the application: arm a motor, read a sensor, change a setpoint.
This crate is the registry and dispatch for those calls — and nothing else. It contains
**no Cog, no GLib, no WebKit, no PocketJS, and no Tokio runtime state**, so product
services can be written and tested without a browser or native UI runtime present.

Rendering backends are adapter crates:

| Adapter                                            | Runtime                        |
| -------------------------------------------------- | ------------------------------ |
| [`consortium-hmi-webkit`](../consortium-hmi-webkit) | Cog / WPE WebKit (Linux)        |
| [`consortium-hmi-pocket`](../consortium-hmi-pocket) | PocketJS in QuickJS (native)     |

Input events travel the other direction and live in
[`consortium-hmi-input`](../consortium-hmi-input).

## Usage

```rust,ignore
use consortium_hmi::Bridge;

let mut bridge = Bridge::new();

// A synchronous handler.
bridge.handle("ping", |_payload| r#""pong""#.to_string());

// An async one — the call site is identical from the adapter's point of view.
bridge.handle_async("read_sensor", |payload| async move {
    let reading = sensor.read().await;
    serde_json::to_string(&reading).unwrap()
});

// The adapter feeds envelopes in and polls the returned future.
let dispatch = bridge.dispatch(message)?;
let json = dispatch.response.await;
send_back_to_guest(dispatch.id, json);
```

## Why it returns a future instead of a response

`Bridge::dispatch` does not poll. It **cannot**: a Cog/WebKit adapter runs on a GLib main
loop, a PocketJS adapter delivers responses at frame boundaries, and neither owns a Tokio
runtime this crate could assume.

So the bridge produces a `ResponseFuture` and the **adapter** decides where to poll it and
how to get the JSON back into its guest runtime. That single decision is what keeps this
crate dependency-free and lets one set of product services serve both backends unchanged.

## Design notes

- **Sync and async handlers are indistinguishable downstream.** Both are erased into one
  internal handler type and both produce a `ResponseFuture` (the sync arm wrapped in an
  immediately-ready future). A handler can become async without the adapter changing.
- **Correlation ids belong to the guest.** `Dispatch::id` is echoed from the invocation
  envelope, not generated here — the guest runtime owns request/response matching, because
  it is the side with concurrent outstanding calls.
- **Payloads stay JSON strings end to end.** The bridge never deserializes a payload; a
  handler does, with whatever type it wants. That keeps `serde` schema decisions out of
  the transport.
- **A malformed envelope still parses.** `id` and `cmd` are both `#[serde(default)]`, so a
  partial envelope becomes a dispatch that fails on *lookup* rather than failing to parse
  — which keeps the failure reportable back to the guest with an id where one was
  supplied.

## Envelope format

```json
{ "id": 42, "cmd": "read_sensor", "payload": { "channel": 3 } }
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
