# consortium-ipc

Core inter-processor communication primitives for Consortium: the trait stack that
every transport, doorbell, and codec plugs into, plus the typed endpoints that
application and firmware code actually calls.

This is the *abstraction* crate. It contains no hardware access, no allocation, and
no runtime. Concrete links live in sibling crates
(`consortium-ipc-transport-memory`, `consortium-ipc-transport-uart*`), concrete
signaling lives in the `consortium-ipc-doorbell-*` crates, and concrete
serialization lives in `consortium-codec`.

## Architecture

Six traits compose into two user-facing types. Read the stack bottom-up: bytes and
signals at the bottom, typed messages at the top.

```text
                    ┌──────────────────────────────┐
  application  ───► │ Transceiver<TyTx, TyRx, ...> │  paired typed endpoint
                    └──────────────┬───────────────┘
                    ┌──────────────┴───────────────┐
                    │ Channel<D, T, Tr, C>         │  one typed direction
                    └───────┬──────────────┬───────┘
              CodecFor<T> ──┘              └── SendTransport / RecvTransport
              (encode/decode)                   (bytes in/out)
                                                     │
                                                Doorbell
                                                (wake the peer)
```

| Item                     | Responsibility                                                             |
| ------------------------ | -------------------------------------------------------------------------- |
| [`Doorbell`]             | Data-free wake signal to a remote core; `ring` / `wait` / `pending`.        |
| [`SendTransport`]        | Byte send half. Writes bytes, rings the doorbell, reports `max_send_size`.  |
| [`RecvTransport`]        | Byte receive half. Awaits the doorbell, reads bytes, reports `max_recv_size`. |
| [`Transport`]            | Marker for full-duplex links: `SendTransport + RecvTransport`.              |
| [`Connect`]              | One-shot readiness rendezvous with the peer, run before any traffic.        |
| `CodecFor<T>`            | Serialization family for a message type (from `consortium-codec`).          |
| [`Channel<D, T, Tr, C>`] | Typed, single-direction endpoint over one transport half.                   |
| [`Transceiver`]          | Typed bidirectional endpoint pairing a `Tx` and an `Rx` channel.            |

Supporting types:

- [`Chan`] — a validated channel identifier. Its numeric meaning is
  transport-specific (an HSEM gate, an MU register index, a UART frame header
  byte), so validation is delegated to a per-platform [`ChanValidate`] impl.
- [`Side`] ([`Primary`] / [`Secondary`] / [`Tertiary`]) — which register view of a
  multi-ported peripheral a driver drives. In this workspace `Primary` is the
  Linux/A-core convention and `Secondary` the firmware convention.
- [`Tx`] / [`Rx`] — direction markers, sealed via the [`Direction`] trait.
- [`IpcSafe`] — marker for types whose bytes mean the same thing on both cores.
- [`MaybeSend`] — `Send` under `std`, vacuous under `no_std`, so one trait
  definition serves Tokio and Embassy.

### No allocation, caller-owned buffers

`Channel` never allocates. The caller hands it a `&'static mut [u8]` scratch buffer
sized for the transport MTU at construction. On firmware that is typically a
`static mut` array (see `consortium_runtime_mcu::static_buf!`); on Linux it is a
leaked `Box`. This is what lets the same `Channel` type serve both sides of a link.

### Bring-up: `Connect`

`Connect` is deliberately narrow. It means exactly "wait until the far side is
ready", which is why it is async, carries no timeout (the caller owns the
deadline), and is called once — after construction, before any `send`/`recv`, and
before splitting a full-duplex transport into halves.

Side asymmetry lives in the *type*, not in a runtime branch: `SharedMemoryTransport`
implements `Connect` per `Side` (the primary publishes the descriptor and waits for
the ack; the secondary validates and acks), so generated bring-up code emits one
`connect(..)` call with no `if side == ..`. The UART transport implements it as a
documented no-op, because byte arrival is its own notification.

Local bring-up is *not* `Connect`. Peripherals keep constructor + config +
typestate, with their ordering emitted into `consortium-cfg`'s generated `init()`;
doorbells keep `bind_notifier` / `bind_service`.

## Usage

```rust,ignore
use consortium_codec::PostcardCodec;
use consortium_ipc::{Chan, Channel, Rx, Transceiver, Tx};

// 1. Build the transport (shared memory, UART, ...) and hand it its doorbell.
let mut transport = /* SharedMemoryTransport::<Secondary, _, _>::new(..) */;

// 2. Rendezvous with the peer, once, before any traffic.
transport.connect(params).await?;

// 3. Split into halves and wrap each in a typed channel.
let (tx_half, rx_half) = transport.split();
let tx = Channel::<Tx, Command, _, PostcardCodec>::new(Chan::new::<V>(0)?, tx_half, tx_buf);
let rx = Channel::<Rx, Telemetry, _, PostcardCodec>::new(Chan::new::<V>(0)?, rx_half, rx_buf);

// 4. Talk.
let mut link = Transceiver::new(tx, rx);
link.send(&Command::Start).await?;
let reply = link.recv().await?;
```

`recv` returns a [`ReceivedMessage`], not a bare `T`. Owned codecs such as
`PostcardCodec` set `Decoded<'buf> = T` and `Deref` straight through; zero-copy
codecs set `Decoded<'buf> = &'buf T::Archived` and the `'_` lifetime already
prevents a second `recv` while the handle is live. Writing against
`ReceivedMessage` from the start means switching codecs does not change call sites.

## Message types and `IpcSafe`

`IpcSafe` rejects address-space-local constructs — raw pointers, references,
function pointers, `usize`, and `isize` — because their values are meaningless on
the other side of the link. Use the derive, which walks the whole field type tree
(so `Vec<usize>` and `Option<*const u8>` are rejected too):

```rust,ignore
use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct MotorCommand {
    motor_id: u8,
    target_rpm: i32,
    torque_limit: f32,
}
```

A hand-written `impl` is `unsafe`, so out-of-derive implementors must acknowledge
the ABI contract explicitly.

## Implementing a transport

1. Implement `TransportError` to name your error type.
2. Implement `SendTransport` and/or `RecvTransport`. Both return
   `impl Future<..> + MaybeSend` (RPITIT), so a plain `async fn` works — including
   one that awaits an unnameable future such as `embedded_io_async::Write::write`.
   Implement `Transport` as a marker if the link is full-duplex.
3. Implement `Connect` if a peer must agree before traffic flows; use
   `Params = ()` when there is nothing to negotiate.
4. Report honest `max_send_size` / `max_recv_size` — callers size their scratch
   buffers from these.
5. If the link needs a channel-id namespace, implement `ChanValidate` for a
   platform marker type so `Chan::new::<P>(id)` rejects out-of-range ids.

Doorbell implementors additionally owe the ordering contract on
[`Doorbell::ring`]: writes performed before `ring` must happen-before the peer's
`wait` completion or `pending` observation. For software pending bits that is a
release/acquire pair; for hardware doorbells the signal must not be observable
before the shared-memory writes it publishes.

## Features

| Feature   | Effect                                                                |
| --------- | --------------------------------------------------------------------- |
| *(none)*  | `no_std`, no allocation. The default firmware shape.                  |
| `alloc`   | Links `alloc` for downstream use.                                     |
| `futures` | Opts in to futures-oriented helpers.                                  |
| `tracing` | Routes internal logging through `consortium-log`'s `tracing` backend.  |
| `defmt`   | Routes internal logging through `defmt` and derives `defmt::Format`.   |
| `std`     | `alloc + futures + tracing`, and makes `MaybeSend` require `Send`.     |

Enabling `std` is what makes channel futures `Send`, which Tokio's multi-threaded
executor requires. Leave it off for Embassy.

## Testing

```bash
cargo test -p consortium-ipc                     # host, no_std shape
cargo test -p consortium-ipc --features std      # Send futures
just test ipc host                               # full host IPC bundle
```

`trybuild` cases cover `IpcSafe` derive rejections.

[`Chan`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Chan.html
[`ChanValidate`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.ChanValidate.html
[`Channel<D, T, Tr, C>`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Channel.html
[`Connect`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Connect.html
[`Direction`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Direction.html
[`Doorbell`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Doorbell.html
[`Doorbell::ring`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Doorbell.html#tymethod.ring
[`IpcSafe`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.IpcSafe.html
[`MaybeSend`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.MaybeSend.html
[`Primary`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Primary.html
[`ReceivedMessage`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.ReceivedMessage.html
[`RecvTransport`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.RecvTransport.html
[`Rx`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Rx.html
[`SendTransport`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.SendTransport.html
[`Secondary`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Secondary.html
[`Side`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Side.html
[`Tertiary`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Tertiary.html
[`Transceiver`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Transceiver.html
[`Transport`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Transport.html
[`Tx`]: https://docs.rs/consortium-ipc/latest/consortium_ipc/struct.Tx.html

## License

Apache-2.0. See [LICENSE](../../LICENSE).
