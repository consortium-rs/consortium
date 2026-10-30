# consortium-ipc-doorbell-gpio

Single-channel GPIO doorbell for [`consortium-ipc`](../consortium-ipc).

Signals a remote processor by pulsing a general-purpose output pin. Maximum
flexibility, minimum help: everything a dedicated IPC peripheral would give you for
free is the integrator's here.

## When to reach for this

GPIO is the fallback, not the default. Prefer a real IPC peripheral —
IPCC (`consortium-ipc-transport-memory::bell::ipcc`) on STM32MP2, MU
(`bell::mu`) on i.MX 9 — whenever
the two cores share one.

GPIO earns its place when they do not:

- two separate chips on one board, with no shared IPC block at all;
- a core whose IPC peripheral is assigned to another security domain;
- a bring-up spike where a spare pin is faster to wire than a RIF assignment.

## What a doorbell is

`consortium-ipc`'s `Doorbell` trait is pure signaling — `ring`, `wait`, `pending`,
no payload. Shared memory carries the data; the doorbell only says "look at it".
Its one hard obligation is ordering: writes performed before `ring` must
happen-before the peer's `wait` completion. This crate discharges that with a
release fence before the pulse and an acquire on the pending flag.

## Signaling and waiting

`ring()` pulses the pin high then low to raise a rising-edge interrupt on the far
side. Receiving is a `GpioSignal` — a pending flag and one `AtomicWaker` — that
`wait()` completes on. `std` and `no_std` run the same code; only the pin guard
differs (`Mutex` on `std`, `RefCell` otherwise).

```rust,ignore
static SIGNAL: GpioSignal = GpioSignal::new();
let doorbell = GpioDoorbell::new(pin, &SIGNAL);

// Bare metal: the application's GPIO IRQ handler drives the wake.
#[interrupt]
fn EXTI0() {
    SIGNAL.wake();
}

// Linux: whatever watches the line drives it, e.g. a UIO IRQ service.
uio.bind_service(Arc::new(|| SIGNAL.wake()));
```

The pending flag keeps a wake that lands before the waiter first polls, so a
handshake that rings before its peer is listening still completes. GPIO wakeups are
edge-driven, so unlike the Type-A bells' services there is nothing to mask before
waking.

## Considerations and limitations

Read these before choosing GPIO; several are load-bearing.

- **Rising-edge signaling, so a pulse can be missed.** `ring` sets the pin high then
  immediately low. If the peer's edge interrupt is masked across both transitions,
  the signal is _gone_ — there is no status flag to re-read, unlike IPCC's occupancy
  bit or MU's `GSR`. Pair it with a transport whose payload state is
  self-describing: the shared-memory transport's `FLAG_VALID` turns a missed doorbell
  into a latency problem rather than a lost message.
- **Single channel per instance.** Unlike HSEM (16) or MU (8), the `ch` argument to
  `ring()` and `wait()` is _ignored_ — every signal uses the same pin. Multi-channel
  designs instantiate several `GpioDoorbell<P>` with different pins.
- **One waiter per signal.** A `GpioSignal` holds a single waker, like every Type-A
  bell line. Give each doorbell its own `static` signal; doorbells sharing one share
  their wakes.
- **No pin ownership beyond `embedded-hal`.** The doorbell is generic over any
  `OutputPin`, so pinmux, drive strength, pull configuration, and — on the receiving
  side — edge-triggered interrupt setup are all the integrator's. This crate only
  pulses and only waits.
- **`ring` takes `&self` but `OutputPin::set_high` needs `&mut self`,** hence the
  `RefCell` / `Mutex` around the pin. On `no_std` that means `ring` must not be
  called re-entrantly from an interrupt while a `ring` is in progress, or the
  `RefCell` panics.

## Features

| Feature | Effect                                                                |
| ------- | --------------------------------------------------------------------- |
| `std`   | Default. Tokio `Notify` wake path plus `linux-embedded-hal` for pins. |

Build with `default-features = false` for the bare-metal software-pending path.

## Testing

```bash
cargo test -p consortium-ipc-doorbell-gpio
cargo test -p consortium-ipc-doorbell-gpio --no-default-features
```

Both paths are testable with a mock `OutputPin`; run each, since they are separate
`cfg` branches rather than one implementation.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
