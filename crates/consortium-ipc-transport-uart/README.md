# consortium-ipc-transport-uart

COBS-framed, CRC-protected UART transport for
[`consortium-ipc`](../consortium-ipc).

This is the first **wired** Consortium link. Where the shared-memory transport
needs a separate doorbell peripheral to say "something happened", a UART's byte
arrival _is_ the notification. That makes it the transport of choice for two chips
on one board, a debug link to a host, or any pairing with no shared address space
at all.

## Wire format

```text
COBS( chan: u8 | payload[0..=MTU] | crc32: u32 LE )  0x00
```

Each layer answers a specific way a serial line goes wrong:

| Layer      | Handles                                                                                                                                                              |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **COBS**   | Removes every `0x00` from the body, reserving it as a delimiter that cannot occur inside a frame. A receiver that lost sync only has to hunt for the next zero byte. |
| **CRC32**  | Flipped bits, a dropped byte, a peer transmitting at the wrong baud rate — corruption that framing alone cannot detect.                                              |
| **`chan`** | The `Chan` contract for serial links. One transport owns one logical channel and drops frames stamped with anybody else's byte.                                      |

COBS is chosen over a length prefix deliberately: a length field read out of sync
sends the receiver off by an arbitrary number of bytes, whereas a reserved delimiter
makes resynchronization a single-byte scan. That matters most in the case this
transport actually has to survive — a peer rebooting mid-frame.

**Frame-level corruption is not an error.** A bad COBS encoding, a CRC mismatch, a
foreign channel byte, or an over-long frame gets logged and dropped, and the
receiver keeps waiting for the next delimiter. `Error` reports only what a caller
can act on: IO failure, EOF, an oversized payload, an under-sized buffer.

## No doorbell, no side, no handshake

- **No `Doorbell` type parameter.** There is no interrupt for an application to
  forward and no wakeup to bind.
- **No `Side`.** Direction is physical — the TX and RX wires.
- **`Connect` is a documented no-op.** It is implemented anyway so generated
  bring-up code stays uniform: the same `connect(..)` call whether the link
  underneath is shared memory or a wire.

## Buffers

Like every Consortium transport this one does not allocate; the caller supplies
`&'static mut [u8]` scratch. UART needs **two** buffers, and neither is MTU-sized —
COBS expansion and the frame accumulator both add overhead. Derive the sizes rather
than guessing:

| Half              | Size              | Holds                                  |
| ----------------- | ----------------- | -------------------------------------- |
| `UartTransportTx` | `tx_buf_len(mtu)` | frame body, then the encoded frame     |
| `UartTransportRx` | `rx_buf_len(mtu)` | a read chunk, then a frame accumulator |

Both are `const fn`, so they work in array lengths. Both constructors verify the
buffer against the MTU and return `Error::BufferTooSmall` rather than trusting the
caller.

## Backend selection

The transport is written once, against this crate's `UartRead`/`UartWrite` seam.
The `std` feature only picks which IO traits that seam is blanket-implemented for:

| Build    | IO traits                                 | Supplied by                                       |
| -------- | ----------------------------------------- | ------------------------------------------------- |
| no `std` | `embedded_io_async::Read` / `Write` (0.7) | chip feature → `imx9` / `stm32mp2` `BufferedUart` |
| `std`    | Tokio `AsyncRead` / `AsyncWrite`          | `unix` feature → `unix::open` / `unix::pty_pair`  |

The seam is not cosmetic. Under `std`, transport futures must be `Send` (see
`consortium_ipc::MaybeSend`, which Tokio's multi-threaded executor requires), and
`embedded_io_async`'s unnameable `async fn` futures cannot guarantee that
generically. Carrying the `MaybeSend` bounds on `UartRead`/`UartWrite` — as the FDCAN
transport does with `CanTx`/`CanRx` — keeps **one impl family**; only two small
adapters differ. They cannot both be compiled (a type may implement both trait
families), which is why the choice is a feature. A driver that fits neither can
implement the seam directly.

## Example

```rust,ignore
use consortium_ipc_transport_uart::{UartTransport, rx_buf_len, tx_buf_len};

const MTU: usize = 256;
static mut TX: [u8; tx_buf_len(MTU)] = [0; tx_buf_len(MTU)];
static mut RX: [u8; rx_buf_len(MTU)] = [0; rx_buf_len(MTU)];

let transport = UartTransport::new(chan, reader, writer, rx_buf, tx_buf, MTU)?;
let (tx, rx) = transport.split();   // for Channel<Tx, ..> and Channel<Rx, ..>
```

`UartTransport::new` takes the RX and TX halves independently, so split HAL UART
halves or split Tokio halves fit directly.

### Linux / host: `/dev/tty*` (`unix` feature)

```rust,ignore
use consortium_ipc_transport_uart::{UartTransport, rx_buf_len, tx_buf_len, unix::open};

let (rx_wire, tx_wire) = open("/dev/ttySTM1", 115_200)?;
let transport = UartTransport::new(chan, rx_wire, tx_wire, rx_buf, tx_buf, MTU)?;
```

- **Raw 8N1, no flow control.** `open` pins the port to eight data bits, no parity,
  one stop bit, and no RTS/CTS — COBS plus CRC does the work parity and handshaking
  would otherwise do. Anything else should build a `SerialStream` directly;
  `unix::tokio_serial` is re-exported for that.
- **A Tokio runtime must already be entered**, since `open` registers with the
  current reactor.
- **Line discipline is bypassed, not managed**: termios settings are not restored on
  drop.
- **`Error::Eof` is reachable here** — a pseudo-terminal whose peer closed reports
  it, so host tests should expect it.
- `unix::pty_pair()` returns two connected pseudo-terminals, each split into
  `(rx, tx)`: a crossed UART cable with no hardware.

### Firmware: interrupt-buffered `BufferedUart` (chip features)

| Feature                                  | Re-exports                      | As         |
| ---------------------------------------- | ------------------------------- | ---------- |
| `mimx93`, `mimx95`, `mimx952`            | `consortium_hal_imx9::uart`     | `imx9`     |
| `stm32mp21x`, `stm32mp23x`, `stm32mp25x` | `consortium_hal_stm32mp2::uart` | `stm32mp2` |

Pick at most one, with `default-features = false`; the HAL rejects several at once.
With none, use any driver implementing the `embedded-io-async` traits.

A raw UART FIFO is a few bytes deep, and with COBS framing a lost byte costs the whole
frame. `BufferedUart`'s receive path is IRQ-driven, draining the FIFO into a ring on
every interrupt so bytes accumulate whether or not `recv` has been polled; size the ring
for the worst-case burst, not the average. **This crate installs no `#[interrupt]`
handlers**: forward the vector into `on_buffered_interrupt::<T>` with the same
peripheral singleton the driver took, and enable the _HAL's_ `rt` feature.

```rust,ignore
use consortium_ipc_transport_uart::stm32mp2::{BufferedUart, Config, on_buffered_interrupt};

let p = consortium_hal_stm32mp2::init();
let uart = BufferedUart::new(p.USART2, KERNEL_HZ, Config::default(), RX_RING.init([0; 512]));
let (tx_wire, rx_wire) = uart.split();
let transport = UartTransport::new(chan, rx_wire, tx_wire, rx_buf, tx_buf, MTU)?;

#[interrupt]
fn USART2() {
    on_buffered_interrupt::<consortium_hal_stm32mp2::peripherals::USART2>();
}
```

The i.MX9 flow is identical with `imx9::BufferedUart` and an `LPUARTn` vector.
`consortium-runtime-mcu` re-exports this crate behind its `ipc-uart` feature.

## Limitations

- **One logical channel per transport instance.** Multiplexing several channels over
  one physical UART is not implemented; the `chan` byte exists to make it possible,
  and today it is used to reject foreign frames.
- **No flow control and no retransmission.** A dropped frame is dropped. Layer
  acknowledgement into the message protocol if the application needs it.
- **Raw 8N1 only** in the current backends.
- **Throughput is the line rate.** For bulk traffic between cores that _do_ share
  memory, use `consortium-ipc-transport-memory` instead.

## Features

Defaults to `std`. Firmware builds want `default-features = false`.

| Feature                                    | Effect                                                              |
| ------------------------------------------ | ------------------------------------------------------------------- |
| `std`                                      | `alloc`, Tokio IO bounds, `Send` futures, `tracing` diagnostics.    |
| `alloc`                                    | Link `alloc` through `consortium-ipc`.                              |
| `defmt`                                    | Route diagnostics to `defmt`; derive `defmt::Format` on `Error`.    |
| `unix`                                     | Implies `std`; `unix::open` / `unix::pty_pair` over `tokio-serial`. |
| `mimx93` / `mimx95` / `mimx952`            | Select i.MX 9; re-export its UART driver as `imx9`.                 |
| `stm32mp21x` / `stm32mp23x` / `stm32mp25x` | Select STM32MP2; re-export its driver as `stm32mp2`.                |

## Testing

```bash
cargo test -p consortium-ipc-transport-uart                        # Tokio family
cargo test -p consortium-ipc-transport-uart --no-default-features  # embedded-io family
cargo test -p consortium-ipc-transport-uart --features unix        # pseudo-terminals (Linux)
```

The protocol is one impl, so the first two exercise the same code; still run both
when touching it or the IO adapters, since each adapter only compiles under its own
feature.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
