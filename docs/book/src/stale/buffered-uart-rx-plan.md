# Embedded UART IPC: buffered HAL RX + real `uart-embedded` glue

## Context

The UART IPC first step (commit 03d694e) landed the COBS/CRC frame protocol, the
`UartTransport` family (no-std path generic over `embedded_io_async` **0.7**
`Read`/`Write`), and the unix backend. `consortium-ipc-transport-uart-embedded` is still
a doc-only re-export stub, per Step 4 of
`docs/book/src/dev/ipc/probably-use-embedded-io-async-may-twinkly-orbit.md`.

Meanwhile the chip HALs (`feat(hal): dma and uart`) gained embassy-style async UART
drivers — `Uart`/`UartRx`/`UartTx` + `UartState::on_interrupt` with
`embassy_sync::waitqueue::AtomicWaker` — but they implement **embedded-io-async 0.6.1**
traits, so they cannot satisfy the transport's 0.7 bounds today (both versions coexist in
Cargo.lock). Their async RX is also _unbuffered_: the FIFO is only drained when the
future is polled, so bytes can be lost between IRQ wake and poll while the executor is
busy deframing.

Goal (user-confirmed): make the UART transport work end-to-end from firmware by

1. bumping the HALs to embedded-io(-async) 0.7, and
2. adding an embassy-`BufferedUart`-style **buffered RX** driver to both chip HALs
   (IRQ drains the FIFO into a ring; async `Read` pops the ring), keeping the existing
   **direct interrupt-driven async TX** (no TX ring), and
3. turning `consortium-ipc-transport-uart-embedded` into real glue.

Reference implementation: `embassy-stm32/src/usart/buffered.rs` (fetched copy at
`/private/tmp/claude-501/-Users-ethanwu-Developer-consortium/51031b88-5240-454d-961a-1b02f6d2c085/scratchpad/embassy_buffered.rs`;
or `git submodule update --init submodules/embassy` — the submodule is registered but not
checked out). Ring primitive: `embassy_hal_internal::atomic_ring_buffer::RingBuffer`
(crates.io 0.5.0, MIT/Apache-2.0, no_std — the exact type embassy's BufferedUart uses).
Also skim embassy-imxrt's LPUART driver (github.com/OpenDevicePartnership/embassy-imxrt)
for LPUART-specific flag handling if needed.

## Step 1 — Bump HALs to embedded-io(-async) 0.7

`crates/consortium-hal-imx9/Cargo.toml` and `crates/consortium-hal-stm32mp2/Cargo.toml`:
`embedded-io = 0.7`, `embedded-io-async = 0.7` (lock resolves 0.7.1 / 0.7.0).

0.6→0.7 delta is small: `BufRead: Read` (we don't impl `BufRead`), `Write::flush` lost
its default impl (both `uart.rs` files already implement it), doc-level semantics
clarifications. Expect no code changes; `cargo check -p consortium-hal-imx9 -p
consortium-hal-stm32mp2` confirms. After this, HAL `UartRx`/`UartTx` plug directly into
`UartTransportRx`/`UartTransportTx`.

Check no other workspace crate pins embedded-io 0.6 (grep found only the two HALs).

## Step 2 — Buffered RX driver in both chip HALs

Append to `crates/consortium-hal-imx9/src/uart.rs` (LPUART) and
`crates/consortium-hal-stm32mp2/src/uart.rs` (USART); the two implementations mirror each
other the way the existing unbuffered drivers do. Add `embassy-hal-internal = { version
= "0.5", optional = true }` to the `uart` feature's deps.

New API, following the existing `UartState` convention (library owns `on_interrupt`, the
app owns `#[interrupt]` and forwards):

```rust
pub struct BufferedUartState {
    inner: UartState,          // base AtomicPtr + rx_waker + tx_waker (reused by UartTx)
    rx_ring: RingBuffer,       // embassy_hal_internal::atomic_ring_buffer::RingBuffer
    // latched line-error/overflow flags (AtomicU8 or AtomicBool set), surfaced as warnings
}
impl BufferedUartState {
    pub const fn new() -> Self;
    /// IRQ entry: drain RX FIFO into the ring, wake rx_waker if bytes arrived;
    /// then the same TIE/TCIE handling as UartState::on_interrupt (TX wakes).
    pub fn on_interrupt(&self);
}

pub struct BufferedUart { rx: BufferedUartRx, tx: UartTx }
impl BufferedUart {
    /// Same contract as Uart::new plus `rx_buf: &'static mut [u8]` ring storage
    /// (rx_ring.init(ptr, len)); enables the receiver interrupt permanently
    /// (RIE / RXFNEIE stays on — that is the point of buffered mode).
    pub unsafe fn new(base: *mut (), clock_hz: u32, config: Config,
                      state: &'static BufferedUartState,
                      rx_buf: &'static mut [u8]) -> Self;
    pub fn split(self) -> (UartTx, BufferedUartRx);
}

pub struct BufferedUartRx { regs, state: &'static BufferedUartState }
```

Details, following embassy's buffered.rs:

- `on_interrupt` RX path: `while RDRF/RXFNE && ring writer has space { push DATA/RDR }`;
  ring-full drops bytes (latch an overflow flag — the transport's CRC layer discards the
  damaged frame, so `read` never returns an error for it; emit via the latched-flag
  warning instead, embassy-style). Wake `rx_waker` whenever ≥1 byte was pushed (skip
  embassy's half-full/eager heuristics — unnecessary at our baud rates).
- Line errors (PE/FE/NF/ORE): clear the flag, latch it, continue — embassy warns and
  keeps going; do not fail the read (corrupt bytes are caught by frame CRC).
- Refactor the TX half of the existing `UartState::on_interrupt` (the TIE/TCIE blocks)
  into a shared helper so `BufferedUartState::on_interrupt` reuses it; `UartTx` keeps
  working against `state.inner`, giving `BufferedUart` its direct async TX for free.
- `BufferedUartRx::read` (mirror embassy `BufferedUartRx::read`,
  scratchpad copy lines 616–647): `poll_fn` that pops
  `reader().pop_slice()` into `buf`; `Poll::Ready(n)` if any bytes, else register
  `rx_waker` + `Pending`. Implement `embedded_io::ErrorType` (`Error = Error`),
  `embedded_io_async::Read`, `embedded_io::ReadReady` (`!ring.is_empty()`), and a
  blocking `embedded_io::Read` (spin on pop).
- unsafe blocks: document the MMIO base invariant (same wording as `Uart::new`) and the
  RingBuffer single-reader/single-writer invariant (reader only in `BufferedUartRx`,
  writer only in `on_interrupt`). Workspace lints deny undocumented unsafe.

Host unit tests (same fake-MMIO style as the doorbell crates / existing `sbr_value`
tests): construct the register block over an aligned heap buffer, set RDRF/RXFNE + DATA,
call `state.on_interrupt()`, assert ring contents and waker wake; assert no push when
the ready flag is clear. (Note: a fake status register never self-clears, so the drain
loop terminates via the ring-capacity bound — fine for the test.)

## Step 3 — `consortium-ipc-transport-uart-embedded` becomes real glue

`crates/consortium-ipc-transport-uart-embedded/`:

- Add optional chip-gated HAL re-exports, mirroring the doorbell-crate feature pattern
  (non-default chips must use `default-features = false` on the HAL dep):

  ```toml
  [features]
  defmt      = ["consortium-ipc-transport-uart/defmt", ...]
  mimx93     = ["dep:consortium-hal-imx9", "consortium-hal-imx9/mimx93"]
  mimx95     = ["dep:consortium-hal-imx9", "consortium-hal-imx9/mimx95"]
  stm32mp21x = ["dep:consortium-hal-stm32mp2", "consortium-hal-stm32mp2/stm32mp21x"]
  stm32mp23x = [...] ; stm32mp25x = [...]
  ```

  HAL deps declared `default-features = false, features = ["uart"]`; re-export as
  `pub mod imx9 { pub use consortium_hal_imx9::uart::*; }` and `pub mod stm32mp2 { ... }`
  behind `cfg(feature = ...)`.

- Rewrite the crate docs into the real end-to-end recipe: `static STATE:
BufferedUartState`, `static_buf!` ring + transport scratch (`rx_buf_len`/`tx_buf_len`),
  `BufferedUart::new` → `split()` → `UartTransport::new(chan, rx, tx, ...)` →
  `Channel<Tx/Rx>`, plus the `#[interrupt] fn LPUARTn/USARTn() { STATE.on_interrupt() }`
  forwarding block (HAL `rt` feature), matching the melt-pot mcu style
  (`examples/melt-pot/stm32mp25/mcu/src/main.rs:143`).
- Keep the existing generic re-exports (`UartTransport`, buffer sizing fns,
  `embedded_io_async`).

Optional small addition (do it, it's tiny and matches `ipc-shm`): a
`consortium-runtime-mcu` feature `ipc-uart = ["dep:consortium-ipc-transport-uart"]`
(dep with `default-features = false`) re-exporting the transport, in
`crates/consortium-runtime-mcu/Cargo.toml` + `src/lib.rs`.

## Step 4 — Recipes and docs

- `justfile`: add to `check-ipc-m8` / `check-ipc-m7` (and the matching `build-ipc-*`):
  `cargo check -p consortium-ipc-transport-uart --no-default-features`,
  `cargo check -p consortium-ipc-transport-uart-embedded --features mimx95` (and/or
  `stm32mp25x`) for both thumb targets.
  Add to `test-ipc-host`: `cargo test -p consortium-ipc-transport-uart`,
  `cargo test -p consortium-ipc-transport-uart --no-default-features --test loopback_embedded`,
  `cargo test -p consortium-ipc-transport-uart-unix`.
- `AGENTS.md` UART IPC paragraph: replace "`consortium-ipc-transport-uart-embedded` is
  currently a firmware re-export; an IRQ-fed PAC-level adapter is planned but not
  implemented" with the buffered-HAL-driver + glue description; note the HAL
  embedded-io 0.7 bump in the HAL/PAC section if wording there mentions versions.
- Update the Step-4 section of
  `docs/book/src/dev/ipc/probably-use-embedded-io-async-may-twinkly-orbit.md` only if the
  user wants the historical plan annotated (repo treats these as history — default: leave).

## Files touched

- `crates/consortium-hal-imx9/{Cargo.toml,src/uart.rs}`
- `crates/consortium-hal-stm32mp2/{Cargo.toml,src/uart.rs}`
- `crates/consortium-ipc-transport-uart-embedded/{Cargo.toml,src/lib.rs}`
- `crates/consortium-runtime-mcu/{Cargo.toml,src/lib.rs}` (ipc-uart re-export)
- `justfile`, `AGENTS.md`

Not touched: `consortium-ipc` (RPITIT traits final), the core transport crate
(`consortium-ipc-transport-uart` already has the embedded impl family), frame protocol,
unix backend, Consortium.toml schema/builder (still out of scope per the original plan).

## Verification

```bash
# HAL bump + buffered driver
cargo test -p consortium-hal-imx9
cargo test -p consortium-hal-stm32mp2
cargo check -p consortium-hal-imx9 --target thumbv8m.main-none-eabihf
cargo check -p consortium-hal-stm32mp2 --target thumbv8m.main-none-eabihf

# transport still green
cargo test -p consortium-ipc-transport-uart
cargo test -p consortium-ipc-transport-uart --no-default-features --test loopback_embedded
cargo test -p consortium-ipc-transport-uart-unix

# glue compiles for firmware targets, both chips
cargo check -p consortium-ipc-transport-uart-embedded --features mimx95     --target thumbv7em-none-eabihf
cargo check -p consortium-ipc-transport-uart-embedded --features stm32mp25x --target thumbv8m.main-none-eabihf
cargo check -p consortium-runtime-mcu --no-default-features --features ipc-uart --target thumbv8m.main-none-eabihf

just test ipc host
just lint && just fix
```

Single `cargo tree -p consortium-ipc-transport-uart-embedded --features mimx95 -i embedded-io-async`
should show only 0.7 in that graph (no 0.6/0.7 trait split on the firmware path).
