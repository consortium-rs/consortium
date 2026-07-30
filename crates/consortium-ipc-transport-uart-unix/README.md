# consortium-ipc-transport-uart-unix

`/dev/tty*` backend for the
[UART IPC transport](../consortium-ipc-transport-uart), plus pseudo-terminal pairs
for host testing.

This crate is deliberately tiny. The protocol — COBS framing, CRC32, channel byte,
buffer sizing — lives in `consortium-ipc-transport-uart`. All this adds is a
correctly configured Tokio serial stream, split into the `(rx, tx)` halves that
transport wants.

```text
consortium-ipc-transport-uart        protocol, generic over IO
  ├── ...-uart-unix   (this crate)   /dev/tty* via tokio-serial   ← Linux/host
  └── ...-uart-embedded              chip HAL BufferedUart        ← firmware
```

## Usage

```rust,no_run
use consortium_ipc_transport_uart::{UartTransport, rx_buf_len, tx_buf_len};
use consortium_ipc_transport_uart_unix::open;

# fn scratch(n: usize) -> &'static mut [u8] { Box::leak(vec![0; n].into_boxed_slice()) }
# async fn demo(chan: consortium_ipc::Chan) -> Result<(), Box<dyn std::error::Error>> {
const MTU: usize = 256;

let (rx_wire, tx_wire) = open("/dev/ttySTM1", 115_200)?;
let transport = UartTransport::new(
    chan,
    rx_wire,
    tx_wire,
    scratch(rx_buf_len(MTU)),
    scratch(tx_buf_len(MTU)),
    MTU,
)?;
let (tx, rx) = transport.split();
# Ok(()) }
```

The transport's `Error`, `UartTransport`, and buffer-sizing helpers are re-exported,
so one dependency is usually enough.

## Considerations

- **Raw 8N1, no flow control.** `open` pins the port to eight data bits, no parity,
  one stop bit, and no RTS/CTS. That is what the framing layer assumes: COBS
  delimiting plus a CRC does the work parity and handshaking would otherwise do, and
  a fixed configuration removes one way for the two ends to disagree. A link needing
  anything else should build a `SerialStream` directly — `tokio_serial` is
  re-exported for that.
- **A Tokio runtime must already be entered.** `open` registers the stream with the
  current reactor, so calling it outside a runtime fails.
- **Line discipline is bypassed, not managed.** `open_native_async` puts the device
  in raw mode; this crate does not restore prior termios settings on drop.
- **`Error::Eof` is reachable here.** A real UART never reports end-of-stream, but a
  pseudo-terminal whose peer closed does — host tests should expect it.
- **Errors name the device.** `OpenError::Open` carries the path, because with
  several `/dev/tty*` candidates on a board the underlying `tokio_serial` error
  alone does not say which one was tried.

## Testing

`pty_pair()` returns two connected pseudo-terminals, each already split into
`(rx, tx)` halves: bytes written to one side's `tx` arrive on the other's `rx`, a
crossed UART cable with no hardware involved. That is how this crate's integration
tests exercise the `std` transport family end to end — real framing, real CRC, real
resynchronization, no board.

```bash
cargo test -p consortium-ipc-transport-uart-unix
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
