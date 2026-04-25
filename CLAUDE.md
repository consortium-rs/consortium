# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build all crates
cargo build

# Run all tests (platform-dependent; see Platform Notes below)
cargo test

# Run tests for a specific crate
cargo test -p consortium-ipc

# Lint
cargo clippy

# Format
cargo fmt

# Serve documentation locally
mdbook serve docs/
```

A `justfile` is available with cross-compilation recipes for A-core (aarch64/armv7), M-core (Cortex-M8/M7), and RISC-V targets. Run `just` to list all recipes.

**Platform notes:**
- Linux-only crates: `consortium-ipc-transport-memory` (with `std` feature), `consortium-tee`, `consortium-runtime-app`, `consortium-rpi`, `consortium-ipc-doorbell-gpio`
- Bare-metal targets: `consortium-ipc-transport-memory` (no `std`), `consortium-runtime-mcu` require Cortex-M compilation
- Run tests for Linux/bare-metal crates only on their respective platforms

## Architecture

This is a Rust workspace providing an async abstraction for inter-processor communication on Heterogeneous Multi-Processing (HMP) SoCs, primarily targeting the STM32MP2 (Cortex-A + Cortex-M) and i.MX 95 (Cortex-A + Cortex-M).

### Crate Organization Reference

| Category | Purpose |
|----------|---------|
| **Core IPC** | `consortium-ipc` — core traits and types |
| **Transport** | `consortium-ipc-transport-memory` — unified M-core + A-core transport (feature-gated) |
| **Doorbells** | `consortium-ipc-doorbell-hsem`, `consortium-ipc-doorbell-mu`, `consortium-ipc-doorbell-gpio` |
| **Codecs** | `consortium-codec-postcard` (default), `consortium-codec-prost`, `consortium-codec-rkyv` |
| **Runtimes** | `consortium-runtime-mcu`, `consortium-runtime-app` |
| **Config & Build** | `consortium-common`, `consortium-builder`, `consortium-cli` |
| **Debug** | `consortium-dbg-defmt` |
| **RPI Subsystem** | `consortium-rpi`, `consortium-rpi-meta` |
| **TEE** | `consortium-tee`, `consortium-tee-proc-macros`, `consortium-tee-test` |
| **Utilities** | `consortium-ffi`, `consortium-hmi`, `consortium-ort` |

### Core Crates

- **`consortium-ipc`** — `no_std` core traits (`Doorbell`, `Transport`, `Region`, `Codec`) and `Channel<D, T, Tr, C>` for typed messaging. Features: `std`, `alloc`, `futures`.
- **`consortium-ipc-transport-memory`** — Unified shared-memory transport for both M-core (bare-metal, no `std`) and A-core (Linux UIO + Tokio, `std` feature). Descriptor-based slot layout (`CIPC` magic header).
- **`consortium-ipc-doorbell-hsem`** — Hardware Semaphore doorbell for STM32MP2/STM32MP25 (both no_std and std).
- **`consortium-ipc-doorbell-mu`** — Message Unit doorbell for i.MX 9x series (both no_std and std).
- **`consortium-ipc-doorbell-gpio`** — GPIO-based doorbell via `embedded-hal` 1.0 (Linux-only, in progress).

**Codecs:**
- **`consortium-codec-postcard`** — Default compact binary codec (no_std-compatible).
- **`consortium-codec-prost`** — Protocol Buffers for cross-language messaging.
- **`consortium-codec-rkyv`** — Zero-copy deserialization for performance-critical paths.

**Config & Build:**
- **`consortium-common`** — Shared `Config` types, `Chip` enum, chip-to-core mappings, and `DescriptorHeader` (magic + version) used by all shared-memory descriptors.
- **`consortium-builder`** — Cross-compilation helpers (`ApplicationBuilder`, `FirmwareBuilder`, `OpteeBuilder`).
- **`consortium-cli`** — Debug CLI tool.
- **`consortium-dbg-defmt`** — Defmt logging over shared memory via a circular buffer with `CDBG` descriptor header; streams MCU logs to the A-core host.

**Runtime & RPI:**
- **`consortium-runtime-mcu`** — M-core runtime with descriptor init and IRQ integration (no_std).
- **`consortium-runtime-app`** — A-core Linux harness with UIO and remoteproc helpers.
- **`consortium-rpi`** — Resource Partition Isolation (RPI) subsystem support.
- **`consortium-rpi-meta`** — RPI metadata and STM32MP chip feature flags.

**TEE:**
- **`consortium-tee`** — OP-TEE integration and error types (Linux-only).
- **`consortium-tee-proc-macros`** — `tee_service!` and `#[tee_command]` macros for TA/CA code generation.

**Utilities:**
- **`consortium-ffi`**, **`consortium-hmi`**, **`consortium-ort`** — FFI bindings, HMI integration, and ONNX Runtime support.

### Core trait hierarchy

```
Doorbell  — signaling primitive (ring/wait/pending)
Region    — memory area with optional cache management
Transport — owns a Doorbell; send/recv async futures
Codec     — serialize/deserialize messages
Channel<D, T, Tr, C> — typed endpoint wrapping Transport + Codec (D = Tx|Rx)
```

See `docs/src/arch/ipc.md` for design details.

---

## Hardware Doorbell Implementations

| Doorbell | Platform | Status | Notes |
|----------|----------|--------|-------|
| **HSEM** | STM32MP2x, STM32MP25x | ✅ Complete | M-core: `AtomicWaker`; A-core: `tokio::sync::Notify` |
| **MU** | i.MX 8MP/93/95 | ✅ Complete | Feature-gated SoC selection; channels 2–7 (0–1 reserved) |
| **GPIO** | Linux (custom SoCs) | 🔄 In Progress | `embedded-hal` 1.0; polling/interrupt modes |

**Adding a new doorbell:**
1. Create crate `consortium-ipc-doorbell-<name>`
2. Implement `Doorbell` trait (`ring()`, `wait()`, `pending()`)
3. Optionally implement `ChanValidate` for channel bounds
4. Add unit tests; document platform support and register access safety
5. Only implement if Technical Reference Manual and register map are available

---

## Getting Started

For both STM32MP2x (HSEM) and i.MX 9x (MU) platforms:
1. Build M-core firmware using `consortium-runtime-mcu` + appropriate doorbell crate
2. Build A-core application using `consortium-runtime-app` + `consortium-ipc-transport-memory`
3. Create `Channel<D, T, Transport, Codec>` instances for bidirectional messaging
4. Load firmware via remoteproc and exchange messages

See `docs/src/dev/ipc/` for detailed implementation guides and `docs/src/dev/ipc/implementation-status.md` for roadmap.

---

## Resource Partition Isolation (RPI) Subsystem

**`consortium-rpi`** provides peripheral access control and session management for HMP SoCs. Enforces hardware-enforced resource isolation on STM32MP2x and i.MX 9x platforms.

Key features:
- Peripheral Access Controller (PAC) generation from hardware metadata
- Session lifecycle management with pin/resource validation
- Per-core resource constraints and isolation

See `docs/src/dev/rpi/` for configuration, PAC design, and session validation details. Feature-gated chip support via `consortium-rpi-meta`.

---

## Simulation

Renode-based STM32MP257 simulator in `sim/renode/stm32mp257/` with HSEM peripheral model. No i.MX simulator yet.

---

## Documentation

Full architectural and implementation guides available in `docs/src/`:
- `arch/` — Design overviews (IPC, TEE, FFI, ORT, HMI)
- `dev/ipc/` — IPC implementation guides and roadmap
- `dev/tee/` — OP-TEE SDK and proc-macro design
- `dev/rpi/` — Resource Partition Isolation subsystem
- `dev/hil/` — Hardware-in-the-loop test plans
- Build locally: `mdbook serve docs/`
