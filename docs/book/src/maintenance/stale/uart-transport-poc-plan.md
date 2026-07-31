# UART IPC Transport PoC (`consortium-ipc-transport-uart*`)

## Context

Consortium's IPC today has one transport: shared memory (`consortium-ipc-transport-memory`). The docs ([IPC introduction](../../content/introduction/ipc.md)) already classify UART as a **Wired transport** with a **Type-B doorbell** (notification implicit in transport activity), and `Doorbell`'s docs reserve the `Chan` meaning "Serial: logical channel byte in frame header". This plan adds UART as the second transport candidate, as a proof of concept:

- embedded side speaks `embedded-io-async`,
- _nix side speaks `/dev/tty_`,
- split across `consortium-ipc-transport-uart` (unified entry + frame protocol), `consortium-ipc-transport-uart-embedded`, `consortium-ipc-transport-uart-unix`.

**Key decision (user-approved):** refactor `SendTransport`/`RecvTransport` in `consortium-ipc` from GAT named futures to **RPITIT** (`-> impl Future<...> + MaybeSend`, stable since 1.75). Rationale: `embedded_io_async::Read/Write` futures are unnameable, so they cannot satisfy `type SendFut<'a>` on stable without boxing (alloc on MCU) or nightly TAIT. Grep confirmed nothing outside the trait/impl files names `SendFut`/`RecvFut`; `Doorbell` (and its `WaitFut` GAT) stays **unchanged**.

**PoC scope (user-approved):** frame protocol + core transport + full unix backend with host tests; embedded crate created as a thin/stub layer with a `thumbv8m` compile check only. Config schema (`Consortium.toml`), builder env vars, multi-channel demux, DMA, and the IRQ-fed RX ring are explicitly out of scope for this step.

## Step 1 — RPITIT refactor in `consortium-ipc`

[`crates/consortium-ipc/src/transport.rs`](https://github.com/consortium-rs/consortium/blob/main/crates/consortium-ipc/src/transport.rs):

```rust
pub trait SendTransport: TransportError {
    fn send(&mut self, data: &[u8])
        -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
    fn max_send_size(&self) -> usize;
}

pub trait RecvTransport: TransportError {
    fn recv(&mut self, buf: &mut [u8])
        -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend;
    fn max_recv_size(&self) -> usize;
}
```

(Edition 2024 RPITIT captures all in-scope lifetimes automatically, so the old explicit `'a` plumbing disappears.)

Update call sites / impls:

- [`crates/consortium-ipc/src/channel.rs`](https://github.com/consortium-rs/consortium/blob/main/crates/consortium-ipc/src/channel.rs) — `send`/`recv` bodies already just `.await`; only the `MockTransport` test impls change (drop `type SendFut/RecvFut`, return the same `ready(..)` values behind `-> impl Future` or convert to `async fn`).
- [`crates/consortium-ipc/src/transceiver.rs`](https://github.com/consortium-rs/consortium/blob/main/crates/consortium-ipc/src/transceiver.rs) — delegates through `Channel`; adjust bounds only if it repeats the GAT names.
- [`crates/consortium-ipc-transport-memory/src/transport.rs`](https://github.com/consortium-rs/consortium/blob/main/crates/consortium-ipc-transport-memory/src/transport.rs) — the six `SendTransport`/`RecvTransport` impl blocks (combined + Tx/Rx halves) drop the `type SendFut/RecvFut` associated types and return the existing hand-rolled `SendFut`/`RecvFut` structs from `-> impl Future + MaybeSend` methods. The future structs themselves are untouched.
- Transport-memory tests (`tests/async_atomic_waker/`, `tests/async_embassy/`, etc.) implement `Doorbell`, not the transport traits — expected to pass unchanged; fix any stragglers found by `cargo test`.

## Step 2 — `consortium-ipc-transport-uart` (unified entry, owns the wire protocol)

New crate under `crates/` (auto-picked-up by the workspace `crates/*` glob). `no_std` unless `std`; feature shape mirrors transport-memory:

```toml
[features]
default  = ["std"]
alloc    = ["consortium-ipc/alloc"]
defmt    = ["consortium-ipc/defmt", "consortium-log/defmt", "dep:defmt"]
embedded = ["dep:consortium-ipc-transport-uart-embedded"]
unix     = ["dep:consortium-ipc-transport-uart-unix", "std"]
std      = ["alloc", "consortium-log/tracing"]
```

Deps: `consortium-ipc` (default-features = false), `consortium-log`, `embedded-io-async = 0.7` (no_std), `cobs` (default-features = false), `crc32fast` (already in workspace, default-features = false), `thiserror`.

### Wire frame (`src/frame.rs`)

COBS-delimited for self-resynchronization after line noise:

```
COBS( chan: u8 | payload[0..=MTU] | crc32: u32 LE ) 0x00
```

- `chan` is the logical channel byte per the `Doorbell` doc contract; PoC accepts only the constructed `Chan` and drops mismatches (single channel per port, like the GPIO doorbell is single-channel).
- CRC32 via `crc32fast` over `chan | payload`.
- `pub const fn max_frame_len(mtu: usize) -> usize` sizing helper (COBS overhead = ⌈len/254⌉ + 1, plus delimiter) so callers/`static_buf!` can size scratch buffers.
- Pure functions `encode_frame(chan, payload, out) -> usize` and an incremental `FrameDeframer` state machine (`push_byte`/`push_slice` → `Option<frame>`), fully unit-testable without IO.

### Transport (`src/transport.rs`)

No `Side`, no `Doorbell` — byte arrival is the Type-B doorbell:

```rust
pub struct UartTransportTx<W: embedded_io_async::Write> {
    chan: Chan, io: W, scratch: &'static mut [u8], mtu: usize,
}
pub struct UartTransportRx<R: embedded_io_async::Read> {
    chan: Chan, io: R, deframer: FrameDeframer, acc: &'static mut [u8], mtu: usize,
}
pub struct UartTransport<R: Read, W: Write> { tx: UartTransportTx<W>, rx: UartTransportRx<R> }
```

- Caller-provided `&'static mut [u8]` scratch/accumulation buffers, sized via `max_frame_len(mtu)` — same no-alloc discipline as `Channel` (`consortium-runtime-mcu::static_buf!` works for these).
- `impl SendTransport`: `async fn`-style body — `encode_frame` into scratch, `io.write_all(..).await`, `io.flush().await`. `max_send_size() == mtu`.
- `impl RecvTransport`: loop `io.read(chunk).await`, feed the deframer; on complete frame verify CRC + chan, copy payload into the caller's buffer, return length. Corrupt/oversize/foreign-chan frames are dropped (counted via `consortium-log` warn) and the loop continues — `recv` only resolves with a valid frame.
- Constructed from **separate R/W halves** so HAL split UART halves and tokio split halves both fit; `UartTransport::split()` returns the Tx/Rx structs for simultaneous `Channel<Tx,..>`/`Channel<Rx,..>` use, mirroring `SharedMemoryTransport::split()`.
- Error enum via `thiserror` (`Io(E)`, `FrameTooLarge`, `BufferTooSmall`), `defmt::Format` behind the feature, matching doorbell-crate error style.

## Step 3 — `consortium-ipc-transport-uart-unix`

Thin Linux/macOS std crate:

- `tokio-serial` (v5, MIT — meets dependency policy) opens `/dev/tty*` with raw mode + baud; `embedded-io-adapters` (`tokio-1` feature) wraps the tokio halves as `embedded_io_async::Read`/`Write`.
- Public surface: `open(path, baud) -> Result<(UartRx, UartTx)>` where the returned types are `FromTokio<ReadHalf<SerialStream>>` newtypes/aliases feeding straight into `UartTransport::new`.
- `SerialStream::pair()` (pty pair) is exposed for tests behind `#[cfg(unix)]`.

## Step 4 — `consortium-ipc-transport-uart-embedded` (stub for PoC)

Because the core is already generic over `embedded-io-async` (which Embassy/modern HAL UARTs implement directly), this crate starts minimal:

- `no_std` crate that re-exports `embedded_io_async` and the core transport types, plus doc comments describing the intended pattern (app owns `#[interrupt]`, per repo convention).
- Planned-but-deferred (documented in the crate docs, not built now): IRQ-fed RX ring implementing `embedded_io_async::Read` for PAC-level UARTs without an async HAL.
- Serves as the compile-check anchor for `thumbv8m.main-none-eabihf`.

## Step 5 — Tests

In `crates/consortium-ipc-transport-uart/tests/`:

- `frame.rs` — encode/decode roundtrip, byte-at-a-time feeding, resync after leading garbage, CRC corruption dropped, oversize frame dropped, `max_frame_len` bounds.
- `loopback.rs` (std) — in-memory duplex fake implementing `embedded_io_async::Read/Write` (Tokio mpsc-backed, lives in `tests/common/`); transport roundtrip, split halves, fragmented delivery, and a codec-backed `Channel<Tx/Rx>` postcard roundtrip mirroring the transport-memory channel tests.

In `crates/consortium-ipc-transport-uart-unix/tests/`:

- `pty_e2e.rs` — `SerialStream::pair()` pty loopback: typed `Transceiver`-style exchange both directions over a real tty fd (runs on macOS and Linux).

## Verification

```bash
cargo test -p consortium-ipc
cargo test -p consortium-ipc-transport-memory            # refactor regression
cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test
cargo test -p consortium-ipc-transport-uart
cargo test -p consortium-ipc-transport-uart-unix
just test ipc host
cargo check -p consortium-ipc-transport-uart --no-default-features --target thumbv8m.main-none-eabihf
cargo check -p consortium-ipc-transport-uart-embedded --target thumbv8m.main-none-eabihf
just lint && just fix
```

End-to-end demo (the PoC deliverable): a small `examples`-style test or doc snippet driving two `UartTransport`s over the pty pair exchanging postcard-encoded typed messages.

## Out of scope (follow-ons, noted for later)

- `Consortium.toml` schema (`transport = "uart"`, `config.uart = { device, baud }`) → `consortium-cfg-common` types first, then validation/lowering/builder env vars (`CONSORTIUM_IPC_UART_*`).
- Multi-channel demux over one port (needs a dispatcher task); PoC is one channel per port.
- Embedded IRQ ring + DMA TX; AGENTS.md / ipc.md doc updates once the API settles.
