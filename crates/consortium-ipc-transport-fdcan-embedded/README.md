# consortium-ipc-transport-fdcan-embedded

Firmware-side glue for
[`consortium-ipc-transport-fdcan`](../consortium-ipc-transport-fdcan).

Re-exports the transport and, when a chip feature is selected, that chip HAL's
CAN driver plus the adapters that let its split halves drive the transport.

With **no** chip feature the crate is chip-agnostic: the transport works with any
type implementing `CanTx`/`CanRx`, so an Embassy or vendor driver needs only
those two impls. That is why `default = []`.

## Features

| Feature                                  | Effect                                        |
| ---------------------------------------- | --------------------------------------------- |
| none (default)                           | Chip-agnostic; bring your own `CanTx`/`CanRx` |
| `mimx93`, `mimx95`                       | i.MX9 FlexCAN via the `imx9` module           |
| `stm32mp21x`, `stm32mp23x`, `stm32mp25x` | STM32MP2 FDCAN via the `stm32mp2` module      |
| `defmt`                                  | Routes the transport's logging to `defmt`     |

Choose exactly one chip feature; the HAL rejects several at once. Each chip
feature pulls in the matching HAL with its `can` driver and forwards the chip
selection, so the shared PAC's register and interrupt definitions match the
target.

## Bring-up

```rust,ignore
use consortium_ipc_transport_fdcan::{FdcanTransport, rx_buf_len};
use consortium_ipc_transport_fdcan::embedded_can::{Id, StandardId};
use consortium_ipc_transport_fdcan_embedded::stm32mp2;

let (tx, rx) = stm32mp2::split(can);
let mut transport = FdcanTransport::new(
    chan,
    Id::Standard(StandardId::new(0x101).unwrap()), // we transmit here
    Id::Standard(StandardId::new(0x102).unwrap()), // the peer transmits here
    tx,
    rx,
    RX_BUF.init([0; rx_buf_len(MTU)]),
    MTU,
)?;
transport.connect(()).await?;
let (tx, rx) = transport.split();
```

The two directions must use different identifiers — both halves see every frame
on the bus, and an endpoint must not reassemble its own traffic.

Clocks, pin muxing and the bit rate are `consortium-cfg`'s job via
`[peripheral.*]`; this crate only adapts an already-configured controller.

## Interrupts

This crate installs no `#[interrupt]` handlers, per the repository-wide
convention: the crate exports the entry point, the application decides which
vector reaches it.

```rust,ignore
use consortium_hal_stm32mp2::peripherals;
use consortium_ipc_transport_fdcan_embedded::stm32mp2::on_interrupt;

#[interrupt]
fn FDCAN1_IT0() {
    on_interrupt::<peripherals::FDCAN1>();
}
```

The STM32MP2 driver enables interrupt line 0 (`FDCANx_IT0`); i.MX9 raises one
line per instance. Without that handler the async paths never wake — the
blocking `embedded_can::nb::Can` paths still work.

## Receive depth

A segmented message is a burst of back-to-back frames, so keep a task parked in
`recv` rather than polling between other work:

- **STM32MP2** receives into an eight-element FIFO 0. All instances share one
  `CAN_SRAM` window, so leave at least `REGION_BYTES` between the
  `msg_ram_offset` values handed to different instances.
- **i.MX9** receives into a single mailbox, so a frame arriving while the
  previous one is unread is reported as `Error::Overrun` and lost. Matching
  STM32MP2's slack needs FlexCAN's RX FIFO, which the driver does not use yet.

Either way a lost frame costs one message, not the endpoint: the transport
reports the sequence gap, drops the partial message, and resynchronizes on the
next one.

## Verification

The crate has no tests of its own; it is verified by cross-compiling:

```bash
cargo check -p consortium-ipc-transport-fdcan-embedded --target thumbv8m.main-none-eabihf --features stm32mp25x
cargo check -p consortium-ipc-transport-fdcan-embedded --target thumbv7em-none-eabihf --features mimx95
```

Both run as part of `just check-ipc-m8` and `just check-ipc-m7`.

## License

Apache-2.0. See [LICENSE](LICENSE).
