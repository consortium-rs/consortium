# consortium-ipc-transport-fdcan

Segmenting FDCAN IPC transport for [Consortium](https://github.com/consortium-rs/consortium).

This is the workspace's second _wired_, Type-B transport, after
`consortium-ipc-transport-uart`. Type-B means the link itself is the
notification: a frame arriving is the wakeup, so there is no `Doorbell` and no
`Side` — unlike shared memory, where both are needed to coordinate two cores
over one window.

## Wire format

A CAN controller already delimits every frame and protects it with a CRC, so
there is nothing here to frame or checksum — the UART transport's COBS and CRC32
layers have no counterpart. What CAN lacks is a way to carry a message longer
than one frame, so that is all this crate adds:

```text
byte 0: [ kind:2 | low:6 ]
  00 Single   : low = payload length; the whole message follows
  01 First    : low = 0; bytes 1..3 = total length u16 LE, then the head
  10 Continue : low = sequence number, 1,2,3,… mod 64; then the body
  11 reserved
```

The 6-bit field does double duty, and both readings fit exactly. On a `Single`
it is the payload length — at most 63 even for FD — so the length never has to
be inferred from the DLC. That matters because CAN FD quantizes the DLC above 8
bytes (…, 32, 48, 64), so a 30-byte frame arrives padded to 32 and an implied
length would be wrong. On a `Continue` the same field is a wrapping sequence
number that catches a dropped frame; the message's real end comes from the
`First` frame's total, so padding on the final frame is ignored.

## Sizing

One payload byte per frame goes to the header, and a `First` frame spends two
more on the total length:

| Frame kind | Classic (8 B) | FD (64 B) |
| ---------- | ------------- | --------- |
| `Single`   | 7 B           | 63 B      |
| `First`    | 5 B           | 61 B      |
| `Continue` | 7 B           | 63 B      |

`frames_for(mtu, frame_payload)` reports what a full-MTU message costs in
frames, which is the number to reach for when budgeting bus time. A 128-byte
message is 19 classic frames — about 2.5 ms at 500 kbit/s — or 3 FD frames.

Segments are padded up to a length the medium can describe (`padded_len`). On
classic CAN that is a no-op, since every length `0..=8` is exact. On FD the DLC
jumps 8, 12, 16, 20, 24, 32, 48, 64, so a 10-byte segment goes out as 12. The
receiver ignores the difference — a `Single` carries its own length and a
`Continue` is bounded by the declared total.

Only the receive half takes a buffer, sized by `rx_buf_len`. CAN frames are
fixed-size, so the transmit path stages each one in a stack array and slices the
caller's payload directly; there is no `tx_buf_len` to match the UART crate's.

## Identifiers

`Chan` does not go on the wire. CAN frames already carry an identifier, so each
half is bound to one: the transmit half stamps it into every frame, and the
receive half drops anything else. The two directions **must** use different
identifiers, since both halves see every frame on the bus and an endpoint must
not reassemble its own traffic. `FdcanTransport::new` rejects equality with
`Error::SameIdentifier` rather than leaving it to be discovered at runtime — a
controller running with loopback or self-reception, as bring-up self-tests use,
echoes every frame straight back. Halves on genuinely separate buses may share
an identifier; build them with `FdcanTransportTx::new`/`FdcanTransportRx::new`,
which do not pair the two directions.

Filtering is currently done in software, because neither chip HAL programs
hardware acceptance filters yet. Frames for other identifiers therefore cost a
wakeup; a hardware filter is the natural optimization and needs no change here.

## Backends

| Crate                                     | Controller                                          |
| ----------------------------------------- | --------------------------------------------------- |
| `consortium-ipc-transport-fdcan-embedded` | i.MX9 FlexCAN and STM32MP2 FDCAN, via the chip HALs |
| this crate, no backend                    | Any type implementing `CanTx`/`CanRx`               |

`CanFrame` is blanket-implemented for every `embedded_can::Frame`, so a driver
whose frame type already implements that trait needs no adapter.

## Limitations

- **Classic 8-byte frames.** The seam is FD-ready — a frame type reporting
  `MAX_PAYLOAD = 64` needs no change here — but neither chip HAL drives FD yet.
- **No flow control.** A receiver that is not already parked in `recv` can miss
  frames from a burst; the segmentation layer reports the resulting sequence gap
  and drops that message, then resynchronizes on the next one. The STM32MP2
  driver's eight-element receive FIFO gives some slack; the i.MX9 driver's
  single receive mailbox gives none.
- **One `Chan` per transport.** No multiplexing of several logical channels onto
  one identifier.
- **Sequence wrap.** Losing exactly a multiple of 64 consecutive frames is
  invisible to the counter. The codec's own decode is the backstop.

## Features

| Feature | Effect                                                                 |
| ------- | ---------------------------------------------------------------------- |
| `std`   | Default. Widens `MaybeSend` to `Send` and routes logging to `tracing`. |
| `alloc` | Implied by `std`; enables `consortium-ipc`'s allocating paths.         |
| `defmt` | Routes logging to `defmt` and derives `Format` on `Error`.             |

Unlike the UART transport, `std` does not select a second set of impls: the
`CanTx`/`CanRx` seam belongs to this crate, so its futures are declared
`MaybeSend` once and one impl family serves both firmware and host.

## Testing

```bash
cargo test -p consortium-ipc-transport-fdcan
cargo test -p consortium-ipc-transport-fdcan --no-default-features
```

Both run the same sources, including the same `loopback` integration test —
which is the evidence for the single-impl-family claim above. The `std` run
additionally proves the futures are `Send`, since `MaybeSend` requires it there.

## License

Apache-2.0. See [LICENSE](LICENSE).
