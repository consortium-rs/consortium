# consortium-ipc-transport-uart-embedded

Firmware-side glue for the [UART IPC transport](../consortium-ipc-transport-uart):
binds the chip HALs' interrupt-buffered UART drivers to the `embedded-io-async`
transport family.

The protocol — COBS framing, CRC32, channel byte, buffer sizing — lives in
`consortium-ipc-transport-uart` and is generic over `embedded_io_async::Read` /
`Write`. This crate supplies the concrete driver and re-exports both, so firmware
needs one dependency instead of three.

```text
consortium-ipc-transport-uart        protocol, generic over IO
  ├── ...-uart-unix                  /dev/tty* via tokio-serial   ← Linux/host
  └── ...-uart-embedded (this crate) chip HAL BufferedUart        ← firmware
```

## Chip selection

| Feature                                 | Re-exports                          | As         |
| --------------------------------------- | ----------------------------------- | ---------- |
| `mimx93`, `mimx95`                      | `consortium_hal_imx9::uart`         | `imx9`     |
| `stm32mp21x`, `stm32mp23x`, `stm32mp25x`| `consortium_hal_stm32mp2::uart`     | `stm32mp2` |
| *(none)*                                | nothing chip-specific               | —          |

Pick exactly one; the HAL rejects several at once, because two chip selections mean
an ambiguous PAC `Interrupt` enum. Each feature also forwards the chip selection to
the HAL so the shared PAC's register and interrupt definitions match the target.

**With no chip feature the crate is chip-agnostic**: use any driver implementing the
`embedded-io-async` traits — Embassy's, a vendor HAL's, your own — with the
re-exported transport types. Nothing here is required to run the UART transport on
firmware; the crate exists to save the wiring for the two supported families.

## The interrupt-buffered receive path

A raw UART FIFO is a few bytes deep. At 115 200 baud a byte lands every ~87 µs, so
an async task that is not scheduled promptly loses data — and with COBS framing a
lost byte costs the whole frame, not one field.

`BufferedUart` closes that gap: its receive path is IRQ-driven, draining the FIFO
into an `embassy_hal_internal::atomic_ring_buffer::RingBuffer` on every interrupt, so
bytes accumulate whether or not the transport has polled `recv` yet. Transmit stays
direct interrupt-driven.

**Per repository convention this crate installs no `#[interrupt]` handlers.** The
application owns the vector table and forwards the UART interrupt into
`on_buffered_interrupt::<T>`, parameterized with the same peripheral singleton the
driver took. This is the same wake pattern the doorbell crates use — the crate
exports the entry point, the application decides which vector reaches it. Enable the
*HAL's* `rt` feature so the `#[interrupt]` macro and PAC vector table are available
to that code.

## Example (STM32MP2, `stm32mp25x` + the HAL's `rt`)

```rust,ignore
use consortium_ipc::Chan;
use consortium_ipc_transport_uart_embedded::{
    UartTransport, rx_buf_len, tx_buf_len,
    stm32mp2::{BufferedUart, Config, on_buffered_interrupt},
};
use consortium_runtime_mcu::static_buf;

const MTU: usize = 256;

// Ring storage for the IRQ handler, plus the transport scratch buffers.
// `static_buf!(NAME, LEN)` declares a `StaticCell<[u8; LEN]>`; `.init(..)`
// yields the `&'static mut [u8]` each constructor wants.
static_buf!(RX_RING, 512);
static_buf!(RX_SCRATCH, rx_buf_len(MTU));
static_buf!(TX_SCRATCH, tx_buf_len(MTU));

// The peripheral singleton carries the register base and the driver's interrupt
// state; the platform must leave USART2 clocked and pinned out.
let p = consortium_hal_stm32mp2::init();
let uart = BufferedUart::new(p.USART2, KERNEL_HZ, Config::default(), RX_RING.init([0; 512]));
let (tx_wire, rx_wire) = uart.split();

let transport = UartTransport::new(
    Chan::new(0)?,
    rx_wire,
    tx_wire,
    RX_SCRATCH.init([0; rx_buf_len(MTU)]),
    TX_SCRATCH.init([0; tx_buf_len(MTU)]),
    MTU,
)?;
let (tx, rx) = transport.split();

#[interrupt]
fn USART2() {
    on_buffered_interrupt::<consortium_hal_stm32mp2::peripherals::USART2>();
}
```

The i.MX9 flow is identical with `imx9::BufferedUart` and an `LPUARTn` vector. See
the melt-pot `mcu/src/main.rs` files for full worked examples of the vector-table
forwarding convention.

## Considerations and limitations

- **The ring must be sized for the burst, not the average.** Size it from the
  worst-case latency between interrupt and `recv` poll, times the line rate. A ring
  that overflows drops bytes exactly like an unbuffered FIFO would.
- **Clocks and pinmux are not this crate's job.** The peripheral must already be
  clocked and pinned out; on a generated firmware build `consortium-cfg` emits that
  ordering into `init()` from `[peripheral.*]`.
- **`consortium-runtime-mcu` re-exports this crate** behind its `ipc-uart` feature,
  which is the usual way firmware reaches it.
- **No `std`, no allocation** — buffers are caller-provided `&'static mut [u8]`.

## Features

| Feature                                  | Effect                                          |
| ---------------------------------------- | ----------------------------------------------- |
| `mimx93` / `mimx95`                      | Select i.MX 9; re-export its UART driver as `imx9`. |
| `stm32mp21x` / `stm32mp23x` / `stm32mp25x` | Select STM32MP2; re-export its driver as `stm32mp2`. |
| `defmt`                                  | Forward `defmt` to the transport and chip HAL.    |

No feature is on by default, which keeps the chip-agnostic build the default.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
