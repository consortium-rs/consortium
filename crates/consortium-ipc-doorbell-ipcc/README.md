# consortium-ipc-doorbell-ipcc

STM32 IPCC (inter-processor communication controller) doorbell for
[`consortium-ipc`](../consortium-ipc).

**This is the recommended STM32MP2 doorbell.** IPCC is present across
STM32MP21/23/25 and has the better-supported ST integration path.
[`consortium-ipc-doorbell-hsem`](../consortium-ipc-doorbell-hsem) is retained for
existing users but should not be chosen for new work: HSEM is absent on STM32MP21
and has shown bus-error access failures where the peripheral clock and RIF
assignment looked correct.

## What a doorbell is

`consortium-ipc`'s `Doorbell` trait is pure signaling — `ring`, `wait`, `pending`,
no payload. Shared memory carries the data; the doorbell only says "look at it".
Its one hard obligation is ordering: writes performed before `ring` must
happen-before the peer's `wait` completion, which is why the transport's
`region.flush()` sits immediately before every `ring`.

## Signaling model

IPCC exposes directional channel banks with an occupancy flag per channel. This
crate uses the simplex half of the protocol:

| Operation  | IPCC action                                                       |
| ---------- | ----------------------------------------------------------------- |
| `ring`     | set the local transmit channel occupied (`CnSCR.CHnS`)             |
| `wait`     | await the remote transmit channel becoming occupied, then clear it  |
| `pending`  | read the incoming status flag non-destructively                    |

Because occupancy is a **flag, not a queue**, `ring` returns
`IpccError::ChannelOccupied` when the previous signal has not been consumed. That
is a genuine condition surfaced to the caller — the peer has not caught up — not
something the driver retries behind your back.

Channel 0 is reserved for the system/firmware handshake; `IpccChanValidate` rejects
it along with out-of-range ids, so `Chan::new::<IpccChanValidate>(id)` is the safe
way to name a channel.

## Type-level selection

```rust,ignore
IpccDoorbell<Primary, 1>    // processor 1 on IPCC1
IpccDoorbell<Secondary, 2>  // processor 2 on the SmartRun IPCC2
```

`Side` picks the register bank (`Primary` drives `C1*` and waits on `C2TOC1SR`;
`Secondary` drives `C2*` and waits on `C1TOC2SR`). IPCC connects exactly two
processors, so `Tertiary` is rejected at compile time. `N` picks the instance.

Neither is a runtime parameter, so a wrong pairing is a build error rather than a
link that silently listens to the wrong bank.

## Construction

| Side       | Constructor                       | Obligation                            |
| ---------- | --------------------------------- | ------------------------------------- |
| Firmware   | `IpccDoorbell::new(p.IPCC1)`      | Consumes the chip HAL's peripheral singleton; a const assertion checks it against `<S, N>`. |
| Linux/UIO  | `unsafe IpccDoorbell::new_mapped(base)` | Caller guarantees `base` is the **mapped virtual** base. A physical address here is undefined behaviour. |

Even on the singleton path the window address comes from this crate's own
`registers::base` table rather than from the singleton, so the security alias the
doorbell has always used does not change underfoot.

The doorbell is `Clone`, and clones share one wake source — which is what lets
`SharedMemoryTransport::split` give a copy to each half and still have an interrupt
reach whichever half is waiting.

## Interrupts, and why IPCC needs masking

**This crate installs no `#[interrupt]` handlers.** The application owns the vector
table and forwards the non-secure `IPCC1_RX` / `IPCC2_RX` vectors. Enable the
*HAL's* `rt` feature (`consortium-hal-stm32mp2/rt`) so the `#[interrupt]` macro and
PAC vector table are available to that code.

```rust,ignore
// Firmware
#[interrupt]
fn IPCC1_RX() {
    consortium_ipc_doorbell_ipcc::notify_rx_occupied_interrupt();
}
```

On Linux the same line is **level-sensitive**, and that has a consequence worth
understanding: if the UIO IRQ loop re-arms the interrupt while a channel is still
occupied, the line fires again immediately and the loop spins. So the `std` path
binds the doorbell as a **service** rather than a plain notifier —
`notify_rx_occupied_interrupt` masks every asserted channel *before* waking, and the
woken `wait` future acknowledges and re-arms its own channel:

```rust,ignore
// Linux: bind the masking service, not the bare notifier.
let db = doorbell.clone();
uio.bind_service(std::sync::Arc::new(move || db.notify_rx_occupied_interrupt()));
```

`notifier()` remains available for integrators who mask elsewhere, but the service
is the correct default. (Contrast `consortium-ipc-doorbell-gpio`, whose edge-driven
wakeups have nothing to mask and use `bind_notifier`.)

## Features

| Feature      | Effect                                                          |
| ------------ | --------------------------------------------------------------- |
| `stm32mp25x` | Chip selection (default). IPCC1 plus the SmartRun IPCC2.          |
| `stm32mp23x` | Chip selection. IPCC1 plus the SmartRun IPCC2.                    |
| `stm32mp21x` | Chip selection. IPCC1 only.                                       |
| `std`        | Tokio `Notify` wake path and the masking service; host-testable.   |
| `defmt`      | Derive `defmt::Format` on `IpccError`.                            |
| `tracing`    | Route diagnostics to `consortium-log`'s `tracing` backend.          |

Each chip feature is forwarded to `consortium-hal-stm32mp2` so the shared PAC's
register and interrupt definitions match the target. **A dependent selecting a
non-default chip must set `default-features = false`** — otherwise the HAL's default
chip comes along too, and two chips mean an ambiguous PAC `Interrupt` enum.

## Testing

The `std` path is host-testable with fake MMIO and a Tokio `Notify`, so ring/wait
ordering, channel validation, occupancy rejection, and the masking service are all
covered without a board.

```bash
cargo test -p consortium-ipc-doorbell-ipcc
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
