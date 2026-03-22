# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p consortium-core
cargo test -p consortium-uio

# Run a single test by name
cargo test -p consortium-core <test_name>

# Lint
cargo clippy

# Format
cargo fmt
```

`consortium-uio` only builds on Linux (`#![cfg(target_os = "linux")]`). Tests for it must be run on a Linux host.

## Architecture

This is a Rust workspace providing an async abstraction for inter-processor communication on Heterogeneous Multi-Processing (HMP) SoCs, primarily targeting the STM32MP2 (Cortex-A + Cortex-M) and i.MX 95 (Cortex-A + Cortex-M).

### Crates

- **`consortium-core`** — `no_std`-compatible core traits and types. Feature-gated: `std`, `alloc`, `futures`.
- **`consortium-uio`** — Linux UIO transport (`/dev/uioN`). Linux-only, async via Tokio.
- **`consortium-cli`** — A debug CLI tool running on developing environment host instead of the HMP SoC.

### Core trait hierarchy

```
Doorbell  — pure signaling primitive (ring/wait/pending)
Transport — byte mover; owns a Doorbell internally; send/recv return async futures
Codec     — serialize/deserialize messages (default: PostcardCodec via postcard)
Channel<D, T, Tr, C> — typed, directed endpoint (D = Tx|Rx); wraps Transport + Codec
```

`Channel` does not allocate. Callers must provide a `&'static mut [u8]` scratch buffer sized to `transport.max_message_size()`.

### Memory regions

`Region` trait describes a contiguous memory area with optional cache management:
- `SramRegion` — cache-coherent (flush/invalidate are noops)
- `DdrRegion` — non-coherent; issues AArch64 `dc cvac`/`dc ivac` instructions, compiles to noops on non-AArch64
- `SerialRegion` — no backing memory; `len()` reports max frame size

### UIO transport (`consortium-uio`)

`UioDevice::open(uio_num)` mmaps resource 0 of `/dev/uioN`, parses a binary descriptor from its start, and spawns a background Tokio task that waits for M→A IRQs (via a second fd) and broadcasts wakeups via `tokio::sync::Notify`. Call `UioDevice::channel(chan)` to get a `UioChannel` which implements `Transport`.

The shared-memory descriptor layout (magic `0x434F_4E53` / "CONS", version 1) must be in `DescState::Ready` before `open` will succeed. Each channel has a tx slot (A→M) and rx slot (M→A) described by byte offsets into the mmap'd region. Slots use volatile reads/writes; the slot module handles the consumed/written flag protocol.
