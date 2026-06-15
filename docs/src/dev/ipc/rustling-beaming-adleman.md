# Plan: Runtime Initialization for `runtime-mcu` and `runtime-app`

## Context

Both runtime crates are still mostly thin re-export/init surfaces — no full bootstrap,
channel setup, or executor wiring. The core IPC, transport, and doorbell crates are
complete; the runtimes need to wire them together into usable startup sequences.

The goal is to define what _initialization code_ each crate should expose, and in what order those steps must occur.

---

## Startup Sequence (both sides must agree)

```
M-core                           A-core
------                           ------
1. Init NVIC / enable IRQs       1. RemoteProc::start() [optional]
2. Construct doorbell            2. Poll descriptor until MReady
3. Construct transport           3. UioDevice::open() (needs Ready state)
4. Write DescState::MReady       4. dev.channel_transport(chan, ...) → SharedMemoryTransport
5. Wait for A-core ack           5. Channel::new(chan, uio_ch, buf)
6. Write DescState::Ready        6. Spawn Tokio tasks
7. Construct Channels
8. Run executor + spawn tasks
```

---

## `runtime-mcu` — What to Initialize

### 1. NVIC / Interrupts

- For HSEM: enable HSEM1 IRQ in NVIC. The `HSEM_IRQHandler` is already `#[no_mangle]` in `doorbell-hsem`.
- For IPCC: enable the board/PAC-selected RX occupied interrupt line and call
  `notify_rx_occupied_interrupt()` from that ISR. STM32MP21 uses IPCC only; STM32MP23/25
  can use HSEM or IPCC.
- For MU: enable the IRQ for the MU instance selected by the SoC route. On current i.MX95
  CA55 <-> CM7 defaults, use MU7 (runtime helper IRQ 207 on CM7) and channels 24/25.
  `doorbell-mu` provides chip-gated `MU1_IRQHandler` through `MU8_IRQHandler` where
  applicable.
- Runtime exposes verified generic doorbell IRQ helpers only where IRQ numbers are known;
  IPCC IRQ routing should remain board/PAC-provided until the interrupt table is wired in.
- These are one-liners but must be called before any `doorbell.wait()`.

### 2. Doorbell Construction

- HSEM: `HsemDoorbell::<1>::new()` — zero-cost, no args.
- MU: `MuDoorbell::default()` or `MuDoorbell::new(bases)` — needs `[usize; N]` base
  addresses, where `N` is selected by `imx8mp`, `imx93`, or `imx95`. i.MX95 CA55
  <-> CM7 code should default to `IMX95_CA55_CM7_TX_CHAN`/`IMX95_CA55_CM7_RX_CHAN`
  (flat channel IDs 24/25 on MU7).
- Runtime should re-export these with correct feature gating.

### 3. Shared Memory Descriptor (new)

- M-core must write the descriptor header (magic `0x434F_4E53`, version 1, channel slots) at offset 0 of the shared SRAM region before A-core can open the UIO device.
- Expose descriptor initialization around the existing
  `consortium-ipc-transport-memory::descriptor::descriptor_init()` and
  `set_state()` helpers.
- M-core writes `KernelInit → Proposed → MReady` after transport is set up; then waits for A-core to set `Ready`.

### 4. `SharedMemoryTransport` Init Helper

- Provide a safe(r) wrapper around `SharedMemoryTransport::new()` that validates the
  descriptor/channel setup:
  ```rust
  pub fn shared_memory_transport<D, R>(base: *mut u8, chan: Chan, doorbell: D, region: R) -> SharedMemoryTransport<D, R>
  ```
- Slot stride = `8 + mtu` bytes. Tx slot at `base + slot_idx * stride`, Rx slot at the mirror offset negotiated via the descriptor.

### 5. Static Scratch Buffers

- User must provide `&'static mut [u8]` — runtime cannot allocate in `no_std`.
- Provide a macro `static_buf!(NAME, SIZE)` that declares a properly aligned static buffer. This is a common pattern in Embassy; we can replicate it.

### 6. No Executor — Bring Your Own

- Do not embed an executor. Users will use Embassy, RTIC2, or a bare-metal spin loop.
- Document that `runtime-mcu` provides IPC init; the executor is the user's responsibility.

---

## `runtime-app` — What to Initialize

### 1. Keep Runtime Re-exports Focused

- `runtime-app` now re-exports `consortium_ipc_transport_memory` as `ipc_transport`
  and owns the Linux UIO wrapper under `runtime_app::uio`.

### 2. Firmware Lifecycle (existing, wire it up)

- `RemoteProc` is already complete. Expose it at the top-level re-export.
- Optionally add `RemoteProc::wait_state(target, timeout)` async helper that polls sysfs in a Tokio interval loop.

### 3. Descriptor Polling (new)

- Before `UioDevice::open()`, the descriptor must be in `Ready(3)`.
- Expose `async fn wait_for_ready(uio_num: usize, timeout: Duration) -> Result<(), Error>` that mmaps resource 0 without full init, polls `DescState` byte, and returns when `Ready`.

### 4. `UioDevice` / Shared Transport Init

- Re-export `UioDevice`.
- `UioDevice::open(uio_num, num_channels, transfer_size)` →
  `dev.channel_transport(chan, doorbell, region)` → `Channel::new(chan, transport, buf)`.
- Provide a convenience function:
  ```rust
  pub async fn open_channel<T>(uio_num: usize, chan: Chan, buf: &'static mut [u8]) -> Result<Channel<Rx, T, SharedMemoryTransport<_, _>>, Error>
  ```

### 5. Tokio Runtime

- `runtime-app` is already `tokio`-based (UioDevice spawns tasks internally).
- Do not embed `#[tokio::main]` — callers bring their own runtime. Document this.
- No wrapping needed; just correct re-exports.

### 6. Doorbell / Notify Wiring (STM32 HSEM path only)

- `UioDevice` has its own `Arc<Notify>` for the IRQ loop (already wired).
- `HsemDoorbell::<2>::new(vaddr)` has its own `Arc<Notify>` — these are separate and currently not bridged.
- For the UIO path the separate HSEM `Notify` is unused; `UioDevice`'s internal loop handles wakeup.
- For the standalone (non-UIO) HSEM A-core path: expose `HsemDoorbell::notifier()` in the re-export and document that callers must call `notify.notify_waiters()` from their IRQ handler.
- No bridging code needed now; document the two paths clearly.

---

## Critical Files to Modify

| File                                                   | Change                                                                                                           |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `crates/consortium-runtime-mcu/src/lib.rs`             | Add doorbell IRQ helpers where verified, descriptor write helpers, `mem_transport_for_slot`, `static_buf!` macro |
| `crates/consortium-runtime-app/src/lib.rs`             | Fix re-export name, add `wait_for_ready`, `open_channel` convenience fn                                          |
| `crates/consortium-runtime-app/src/remoteproc.rs`      | Add `wait_state` async helper                                                                                    |
| New: `crates/consortium-runtime-mcu/src/descriptor.rs` | Descriptor write + state helpers for M-core side                                                                 |
| New: `crates/consortium-runtime-app/src/descriptor.rs` | Descriptor poll helpers for A-core side                                                                          |

Existing re-usable code:

- `consortium-ipc-transport-memory/src/descriptor.rs` — already parses the descriptor format; M-core init should use the same layout constants
- `consortium-ipc-transport-memory/src/lib.rs` — shared-memory transport constructor to wrap
- `consortium-runtime-app/src/remoteproc.rs` — `RemoteProc` (complete, no changes needed)

---

## Design Decisions (confirmed)

1. **Descriptor ownership**: M-core writes the full descriptor (magic `0x434F_4E53`, version 1, channel count, slot offsets). A-core polls until `Ready(3)`. Runtime needs a full `descriptor::write()` API on the M-core side.

2. **Executor**: Embassy. Use `embassy_executor::task` entry points and `static_cell::StaticCell` (or Embassy's `StaticCell`) for static scratch buffers. Add `embassy-executor` and `static-cell` to `runtime-mcu` dependencies.

3. **MU chip features**: Split `imx9x` into `imx93` and `imx95` sub-features, with `imx9x` as an umbrella that enables the `mu` doorbell crate. Feature flags: `imx9x = []`, `imx93 = ["imx9x", "consortium-ipc-doorbell-mu/imx93"]`, `imx95 = ["imx9x", "consortium-ipc-doorbell-mu/imx95"]`.

---

## Verification

After implementation:

1. `cargo build -p consortium-runtime-mcu --features stm32mp25x --target thumbv7em-none-eabi` — must compile clean
2. `cargo build -p consortium-runtime-app` (Linux host) — must compile clean (fixes broken re-export)
3. Write an integration doc/example showing M-core init → `MReady` → A-core `wait_for_ready` → `UioDevice::open` → message exchange
