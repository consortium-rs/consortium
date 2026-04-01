# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p consortium-ipc
cargo test -p consortium-mem
cargo test -p consortium-uio

# Run a single test by name
cargo test -p consortium-ipc <test_name>

# Lint
cargo clippy

# Format
cargo fmt
```

`consortium-uio` only builds on Linux (`#![cfg(target_os = "linux")]`). Tests for it must be run on a Linux host.

## Architecture

This is a Rust workspace providing an async abstraction for inter-processor communication on Heterogeneous Multi-Processing (HMP) SoCs, primarily targeting the STM32MP2 (Cortex-A + Cortex-M) and i.MX 95 (Cortex-A + Cortex-M).

### Crates

- **`consortium-ipc`** — `no_std`-compatible core traits and types. Feature-gated: `std`, `alloc`, `futures`.
- **`consortium-mem`** — Bare-metal shared-memory transport for M-core (`no_std`, no async). Implements `Transport` via volatile slot reads/writes.
- **`consortium-uio`** — Linux UIO transport (`/dev/uioN`). Linux-only, async via Tokio.
- **`consortium-app`** — Linux-side `remoteproc` sysfs wrapper (`RemoteProc`, state management, firmware loading).
- **`consortium-builder`** — SDK-aware cross-compilation helpers: `ApplicationBuilder` (Cortex-A35/OpenSTLinux) and `FirmwareBuilder` (Cortex-M MCUs). Async via Tokio.
- **`consortium-common`** — Shared configuration types: `Config`, `Chip` enum (STM32MP2x, i.MX 93/94/95), `CortexMcuCore` enum, chip-to-core mappings.
- **`consortium-cli`** — Debug CLI tool (stub).
- **`consortium-rt`** — Real-time runtime library (stub).
- **`consortium-tee`** — Trusted Execution Environment support (stub).

### Core trait hierarchy

```
Doorbell  — pure signaling primitive (ring/wait/pending)
Region    — contiguous memory area with optional cache management (flush/invalidate/is_coherent)
Transport — byte mover; owns a Doorbell internally; send/recv return async futures (GATs)
Codec     — serialize/deserialize messages (default: PostcardCodec via postcard)
Channel<D, T, Tr, C> — typed, directed endpoint (D = Tx|Rx); wraps Transport + Codec
```

`Channel` does not allocate. Callers must provide a `&'static mut [u8]` scratch buffer sized to `transport.max_message_size()`.

`ReceivedMessage<'a, T, C>` wraps the decoded value with a lifetime tied to the scratch buffer, enabling zero-copy codecs. `ChannelError<C, Tr>` distinguishes codec errors from transport errors.

### Memory regions

`Region` trait describes a contiguous memory area with optional cache management:
- `SramRegion` — cache-coherent; flush/invalidate are noops; `is_coherent()` returns `true`
- `DdrRegion` — non-coherent; issues AArch64 `dc cvac`/`dc ivac` + `dsb sy`; compiles to noops on non-AArch64
- `SerialRegion` — no backing memory; `as_ptr()` returns null; `len()` reports max frame size

### Transport implementations

**`consortium-mem` (bare-metal `MemTransport<D, R>`):**
- Unsafe constructor takes raw pointers to tx/rx slots in shared DMA memory
- `SendFut` completes eagerly: writes payload then rings doorbell synchronously
- `RecvFut` polls slot; if empty, awaits the `Doorbell::wait()` future
- Slot protocol: `[len: u32][flags: u32][payload: [u8]]`; `FLAG_VALID (0x01)` / `FLAG_CONSUMED (0x02)`

**`consortium-uio` (`UioDevice` / `UioChannel`):**
- `UioDevice::open(uio_num)` mmaps resource 0 of `/dev/uioN`, parses a binary descriptor from its start, and spawns a background Tokio task that waits for M→A IRQs (via a second fd) and broadcasts wakeups via `tokio::sync::Notify`
- Call `UioDevice::channel(chan)` to get a `UioChannel` which implements `Transport`
- The shared-memory descriptor layout (magic `0x434F_4E53` / "CONS", version 1) must be in `DescState::Ready` before `open` will succeed
- Each channel has a tx slot (A→M) and rx slot (M→A) described by byte offsets into the mmap'd region
- Slots use volatile reads/writes; the slot module handles the consumed/written flag protocol

### `DescState` lifecycle

```
KernelInit (0) → Proposed (1) → MReady (2) → Ready (3)
                                                ↓
                                           Suspended (4) / Error (5)
```

### `no_std` / feature gates

`consortium-ipc` is `no_std` by default. Features:
- `std` — enables `Send` bound on futures via the `MaybeSend` trait
- `alloc` — enables allocator support in serde/postcard
- `futures` — placeholder

`consortium-mem` inherits `no_std` from `consortium-ipc`. `consortium-uio` requires `std` + Tokio.

### Build system (`consortium-builder`)

`ApplicationBuilder` cross-compiles for `aarch64-unknown-linux-gnu` using the ST OpenSTLinux SDK. It reads standard ST environment variables (`SDKTARGETSYSROOT`, `OECORE_NATIVE_SYSROOT`, `OECORE_TUNE_CCARGS`) and supports overrides (`OSTL_RUSTC`, `OSTL_CARGO`, `OSTL_SDK_SYSROOT`, `OSTL_SDK_EXTRA_FLAGS`). It sanitizes `PATH` to prevent the ST SDK toolchain from shadowing rustup.

`FirmwareBuilder` maps `CortexMcuCore` variants to Rust targets:
- `CortexM0p` → `thumbv6m-none-eabi`
- `CortexM33` → `thumbv8m.main-none-eabi`
- `CortexM4` / `CortexM4f` / `CortexM7` → `thumbv7em-none-eabi`
