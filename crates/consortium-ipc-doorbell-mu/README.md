# consortium-ipc-doorbell-mu

NXP i.MX MU (Messaging Unit) doorbell for [`consortium-ipc`](../consortium-ipc).

This is the i.MX 9 doorbell — the peripheral an i.MX95 CA55 and CM7 use to signal
each other across a shared-memory window.

## What a doorbell is

`consortium-ipc`'s `Doorbell` trait is pure signaling — `ring`, `wait`, `pending`,
no payload. Shared memory carries the data; the doorbell only says "look at it".
Its one hard obligation is ordering: writes performed before `ring` must
happen-before the peer's `wait` completion, which is why the transport's
`region.flush()` sits immediately before every `ring`.

## Two line kinds in one channel space

An MU instance offers two independent signaling mechanisms. This crate packs both
into the single flat channel space the `Doorbell` trait expects:

| `Chan` range | Line kind                      | Registers                          |
| ------------ | ------------------------------ | ---------------------------------- |
| `0..4`       | general-purpose interrupt (GI)  | `GCR` / `GSR` / `GIER`              |
| `4..8`       | receive-full (RF)               | transmit/receive regs + `RSR` / `RCR` |

GI lines are pure interrupts. RF lines work by writing a token word to a transmit
register, which sets the peer's receive-full flag — a doorbell that happens to carry
four bytes this crate does not use. Both signal equally well; which to pick is a
board-integration decision, so both are exposed rather than one being hidden behind
the other.

Each instance's side-B (MUB) window sits one register-window length above its side-A
(MUA) base, so a single base table plus a length describes both processor views.

## Type-level selection

```rust,ignore
MuDoorbell<Primary, 7>    // CA55 view of MU7 — the i.MX95 CA55 <-> CM7 default route
MuDoorbell<Secondary, 7>  // CM7 view of the same instance
```

`Side` picks the MUA/MUB window; `N` picks the instance. Neither is a runtime
parameter, so a wrong pairing is a build error rather than a link that silently
drives the wrong window.

## Construction

| Side       | Constructor                          | Obligation                          |
| ---------- | ------------------------------------ | ----------------------------------- |
| Firmware   | `MuDoorbell::new(p.MU7_MUB)`         | Consumes the chip HAL's peripheral singleton. A const assertion checks the singleton's side against `S`, so `MuDoorbell<Secondary, 7>` cannot be built from the MUA singleton. |
| Linux/UIO  | `unsafe MuDoorbell::new_mapped(base)`| Caller guarantees `base` is the **mapped virtual** base of instance `N` for side `S`. A physical address here is undefined behaviour. |

Even on the singleton path the window address comes from this crate's own
`registers::base` table rather than from the singleton, so the security alias the
doorbell has always used does not change underfoot.

The doorbell is `Clone`, and clones share one wake source — which is what lets
`SharedMemoryTransport::split` give a copy to each half and still have an interrupt
reach whichever half is waiting.

## Interrupts, and why MU needs masking

**This crate installs no `#[interrupt]` handlers.** The application owns the vector
table and forwards each MU vector a core drives into `notify::<N>()`, parameterized
with that instance number. The relevant vectors are i.MX93 `MU1_B` / `MU2_B` and
i.MX95 CM7 `MU5_A` / `MU7_B` / `MU8_B`. Enable the *HAL's* `rt` feature
(`consortium-hal-imx9/rt`) so the `#[interrupt]` macro and PAC vector table are
available to that code.

```rust,ignore
// Firmware
#[interrupt]
fn MU7_B() {
    consortium_ipc_doorbell_mu::notify::<7>();
}
```

On Linux the MU line is **level-sensitive**, and that has a consequence worth
understanding: if the UIO IRQ loop re-arms the interrupt while a status flag is still
set, the line fires again immediately and the loop spins. So the `std` path binds the
doorbell as a **service** rather than a plain notifier — `notify_gi_interrupt`
disables every asserted GI and RF line *before* waking. Disabling only masks the
interrupt and leaves `GSR`/`RSR` set, so the woken `wait` future still observes its
flag, acknowledges it, and re-arms:

```rust,ignore
// Linux: bind the masking service, not the bare notifier.
let db = doorbell.clone();
uio.bind_service(std::sync::Arc::new(move || db.notify_gi_interrupt()));
```

`notifier()` remains available for integrators who mask elsewhere, but the service is
the correct default. (Contrast `consortium-ipc-doorbell-gpio`, whose edge-driven
wakeups have nothing to mask and use `bind_notifier`.)

## Features

| Feature  | Effect                                                            |
| -------- | ----------------------------------------------------------------- |
| `mimx95` | Chip selection (default). Eight MU instances.                      |
| `mimx93` | Chip selection. Two MU instances.                                  |
| `std`    | Tokio `Notify` wake path and the masking service; host-testable.    |
| `defmt`  | Derive `defmt::Format` on `MuError`.                               |

Enabling both chip features is a **compile error** — the instance count and base
table differ. Each is forwarded to `consortium-hal-imx9` so the shared PAC's register
and interrupt definitions match the target, so **a dependent selecting the non-default
chip must set `default-features = false`**; otherwise the HAL's default chip comes
along too and two chips mean an ambiguous PAC `Interrupt` enum.

## Testing

The `std` path is host-testable with fake MMIO and a Tokio `Notify`, so ring/wait
ordering, GI and RF channel mapping, channel validation, and the masking service are
all covered without a board.

```bash
cargo test -p consortium-ipc-doorbell-mu
cargo test -p consortium-ipc-doorbell-mu --no-default-features --features mimx93
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
