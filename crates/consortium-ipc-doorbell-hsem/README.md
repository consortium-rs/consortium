# consortium-ipc-doorbell-hsem

STM32 HSEM (hardware semaphore) doorbell for
[`consortium-ipc`](../consortium-ipc).

> **Prefer [`consortium-ipc-doorbell-ipcc`](../consortium-ipc-doorbell-ipcc) for new
> STM32MP2 work.** This crate is retained for experiments and existing
> STM32MP23/25 users. IPCC is present across STM32MP21/23/25 and has the better
> supported ST integration path; HSEM is absent on STM32MP21, and field testing has
> shown surprising access failures where the peripheral clock and RIF assignment
> looked correct but reads still produced a bus error.

## What a doorbell is

`consortium-ipc`'s `Doorbell` trait is pure signaling — `ring`, `wait`, `pending`,
no payload. Shared memory carries the data; the doorbell only says "look at it".
Its one hard obligation is ordering: writes performed before `ring` must
happen-before the peer's `wait` completion, which is why the transport's
`region.flush()` sits immediately before every `ring`.

## How a semaphore becomes a doorbell

HSEM is a lock peripheral, not a mailbox, so the signaling is a side effect of the
locking protocol:

| Operation | HSEM action                                                                |
| --------- | -------------------------------------------------------------------------- |
| `ring`    | 1-step read-lock, then immediately unlock — the **unlock** raises the remote core's HSEM interrupt |
| `wait`    | arm this core's `CnIER`, resolve when the semaphore appears in `CnMISR`, then clear it |
| `pending` | read `CnMISR` non-destructively                                             |

The channel space is the 16 hardware semaphores.

## Type-level selection

```rust,ignore
HsemDoorbell<Primary, 1>    // core C1
HsemDoorbell<Secondary, 1>  // core C2
HsemDoorbell<Tertiary, 1>   // core C3
```

`Side` picks the per-core register bank. HSEM is **single-instance**, so `N` is always
`1`; the const is kept for shape consistency with the IPCC and MU drivers.

Unlike IPCC, HSEM does connect three cores, so `Tertiary` is valid here.

## Construction

| Side       | Constructor                            | Obligation                        |
| ---------- | -------------------------------------- | --------------------------------- |
| Firmware   | `HsemDoorbell::new(p.HSEM)`            | Consumes the chip HAL's `HSEM` peripheral singleton; a const assertion checks it against `<S, N>`. |
| Linux/UIO  | `unsafe HsemDoorbell::new_mapped(base)`| Caller guarantees `base` is the **mapped virtual** base. A physical address here is undefined behaviour. |

Even on the singleton path the window address comes from this crate's own
`registers::base` table rather than from the singleton, so the security alias the
doorbell has always used does not change underfoot.

The doorbell is `Clone`, and clones share one wake source — which is what lets
`SharedMemoryTransport::split` give a copy to each half and still have an interrupt
reach whichever half is waiting.

## Interrupts are the application's

**This crate installs no `#[interrupt]` handlers.** The application owns the vector
table and forwards the non-secure `HSEM` vector. Enable the *HAL's* `rt` feature
(`consortium-hal-stm32mp2/rt`) so the `#[interrupt]` macro and PAC vector table are
available to that code.

```rust,ignore
// Firmware
#[interrupt]
fn HSEM() {
    consortium_ipc_doorbell_hsem::notify();
}
```

On `std` the wake source is a Tokio `Notify`. `wait` enables the line's `CnIER` while
pending and disarms it on completion, so userspace owns the mask and re-arming the UIO
IRQ does not immediately re-fire a level-sensitive line. Bind it through
`consortium-runtime-app`'s UIO fan-out:

```rust,ignore
uio.bind_notifier(doorbell.notifier());
```

## Features

| Feature      | Effect                                                            |
| ------------ | ----------------------------------------------------------------- |
| `stm32mp25x` | Chip selection (default). Forwarded to `consortium-hal-stm32mp2`.    |
| `stm32mp23x` | Chip selection; shares the STM32MP25 HSEM layout.                   |
| `std`        | Tokio `Notify` wake path; host-testable with fake MMIO.              |
| `defmt`      | Derive `defmt::Format` on `HsemError`.                              |
| `tracing`    | Route diagnostics to `consortium-log`'s `tracing` backend.            |

Each chip feature is forwarded to `consortium-hal-stm32mp2` so the shared PAC's
register and interrupt definitions match the target. **A dependent selecting a
non-default chip must set `default-features = false`** — otherwise the HAL's default
chip comes along too, and two chips mean an ambiguous PAC `Interrupt` enum.

## Testing

The `std` path is host-testable with fake MMIO and a Tokio `Notify`, so the
lock/unlock ring sequence, `CnIER` arming, and channel validation are covered without
a board.

```bash
cargo test -p consortium-ipc-doorbell-hsem
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
