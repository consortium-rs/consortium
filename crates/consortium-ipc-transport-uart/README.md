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

The IO bounds are chosen by the `std` feature, and this is the crate's one real
wrinkle:

| Build    | IO traits                                 | Supplied by                              |
| -------- | ----------------------------------------- | ---------------------------------------- |
| no `std` | `embedded_io_async::Read` / `Write` (0.7) | `consortium-ipc-transport-uart-embedded` |
| `std`    | Tokio `AsyncRead` / `AsyncWrite`          | `consortium-ipc-transport-uart-unix`     |

The split is not cosmetic. Under `std`, transport futures must be `Send` (see
`consortium_ipc::MaybeSend`, which Tokio's multi-threaded executor requires), and
`embedded_io_async`'s unnameable `async fn` futures cannot guarantee that
generically. So the two families are **separate impl blocks over the same types**
rather than one generic implementation — which is why protocol changes need testing
under both.

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

| Feature | Effect                                                           |
| ------- | ---------------------------------------------------------------- |
| `std`   | `alloc`, Tokio IO bounds, `Send` futures, `tracing` diagnostics. |
| `alloc` | Link `alloc` through `consortium-ipc`.                           |
| `defmt` | Route diagnostics to `defmt`; derive `defmt::Format` on `Error`. |

## Testing

```bash
cargo test -p consortium-ipc-transport-uart                        # Tokio family
cargo test -p consortium-ipc-transport-uart --no-default-features  # embedded-io family
cargo test -p consortium-ipc-transport-uart-unix                   # pseudo-terminals
```

Run all three when touching the protocol — the two feature paths are distinct impl
blocks, so a change to one is not covered by the other's tests.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
