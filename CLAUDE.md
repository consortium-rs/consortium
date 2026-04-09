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
cargo test -p consortium-ipc-transport-mem
cargo test -p consortium-ipc-transport-uio
cargo test -p consortium-ipc-doorbell-hsem

# Run a single test by name
cargo test -p consortium-ipc <test_name>

# Lint
cargo clippy

# Format
cargo fmt
```

**Platform notes:**
- `consortium-ipc-transport-uio`, `consortium-tee`, and `consortium-runtime-app` only build on Linux.
- `consortium-ipc-transport-mem` and `consortium-runtime-mcu` must compile for bare-metal (Cortex-M).
- Tests for Linux-only crates must be run on a Linux host.

## Architecture

This is a Rust workspace providing an async abstraction for inter-processor communication on Heterogeneous Multi-Processing (HMP) SoCs, primarily targeting the STM32MP2 (Cortex-A + Cortex-M) and i.MX 95 (Cortex-A + Cortex-M).

### Crates

**Core IPC (production-ready):**
- **`consortium-ipc`** — `no_std`-compatible core traits and types. Feature-gated: `std`, `alloc`, `futures`. Exports `Channel<D, T, Tr, C>`, `Doorbell`, `Transport`, `Region`, `Codec`.
- **`consortium-ipc-transport-mem`** — Bare-metal shared-memory transport for M-core (`no_std`). Implements `Transport` via volatile slot reads/writes. Re-exports HSEM doorbell via `consortium-ipc-doorbell-hsem`.
- **`consortium-ipc-transport-uio`** — Linux UIO transport (`/dev/uioN`). Linux-only, async via Tokio. `UioDevice::open()` mmaps shared memory and spawns background interrupt handler. Re-exports HSEM doorbell via `consortium-ipc-doorbell-hsem`.
- **`consortium-ipc-doorbell-hsem`** — Hardware Semaphore (HSEM) doorbell for STM32MP2/STM32MP25. Supports both no_std (M-core with `AtomicWaker` + HSEM_IRQHandler) and std (A-core with `tokio::sync::Notify`). Implements `Doorbell` trait and `HsemChanValidate`.
- **`consortium-ipc-doorbell-mu`** — Message Unit (MU) doorbell for i.MX 9x series. Supports both no_std and std paths with two independent IRQ handlers (MU1/MU2). Implements `Doorbell` trait and `MuChanValidate`.

**Configuration & Build (partial):**
- **`consortium-common`** — Shared configuration types: `Config`, `Chip` enum, `CortexMcuCore` enum, chip-to-core mappings. Enums for STM32MP2x and i.MX 9x families.
- **`consortium-builder`** — SDK-aware cross-compilation helpers (stub). Planned: `ApplicationBuilder` (Cortex-A35/OpenSTLinux) and `FirmwareBuilder` (Cortex-M MCUs).
- **`consortium-cli`** — Debug CLI tool (stub).

**Runtime (minimal implementation, expanding):**
- **`consortium-runtime-mcu`** — Real-time runtime library for M-core firmware (stub). Must provide: bootstrap, async executor, MemTransport integration.
- **`consortium-runtime-app`** — Linux-side application harness. Exports `RemoteProc` sysfs wrapper with state readers and controls (`start`, `stop`, `set_firmware`, etc.). Linux-only.

**Trusted Execution Environment (complete):**
- **`consortium-tee`** — OP-TEE integration. Provides error types (`TeeError`, `TeeErrorOrigin`, `TeeResult`). Linux-only.
- **`consortium-tee-proc-macros`** — Proc-macro crate for TEE. Provides `tee_service!` (generates `Command` enum + `invoke_command` dispatcher) and `#[tee_command]` (generates CA `call_*` wrappers and TA `*_dispatched` handlers with advanced type support).

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

Channel addressing uses `Chan(u8)`; platforms implement `ChanValidate` to enforce valid channel IDs.

### Memory regions

`Region` trait describes a contiguous memory area with optional cache management:
- `SramRegion` — cache-coherent; flush/invalidate are noops; `is_coherent()` returns `true`
- `DdrRegion` — non-coherent; issues AArch64 `dc cvac`/`dc ivac` + `dsb sy`; compiles to noops on non-AArch64
- `SerialRegion` — no backing memory; `as_ptr()` returns null; `len()` reports max frame size

### Transport implementations

**`consortium-ipc-transport-mem` (bare-metal `MemTransport<D: Doorbell, R: Region>`):**
- Generic over platform-specific `Doorbell` (HSEM via `consortium-ipc-doorbell-hsem`, MU via `consortium-ipc-doorbell-mu`, or custom) and `Region` (SRAM, DDR, Serial)
- Unsafe constructor takes raw pointers to tx/rx slots in shared DMA memory
- `SendFut` completes eagerly: writes payload then rings doorbell synchronously
- `RecvFut` polls slot; if empty, awaits the `Doorbell::wait()` future
- Slot protocol: `[len: u32][flags: u32][payload: [u8]]`; `FLAG_VALID (0x01)` / `FLAG_CONSUMED (0x02)`

**`consortium-ipc-transport-uio` (`UioDevice` / `UioChannel`):**
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

`consortium-ipc-transport-mem` inherits `no_std` from `consortium-ipc`. `consortium-ipc-transport-uio` requires `std` + Tokio.

### Configuration types (`consortium-common`)

- **`Config`** — top-level config with `general: GeneralConfig` and `project: ProjectConfig`. Method: `validate_rt_cores()`.
- **`GeneralConfig`** — holds `chip: Chip` selection.
- **`ProjectConfig`** — holds application name and `firmware: HashMap<String, FirmwareEntry>`.
- **`FirmwareEntry`** — `core: CortexMcuCore`, `index: usize`, `path`, optional `flash_address`.
- **`Chip`** enum — `Stm32mp21x`, `Stm32mp23x`, `Stm32mp25x`, `Imx93`, `Imx93w`, `Imx937`, `Imx94`, `Imx95`, `Imx952`. Methods: `has_rt_core()`, `has_rt_core_name()`, `remoteproc_name()`, `programmable_rt_cores()`.
- **`CortexMcuCore`** enum — `CortexM0p`, `CortexM33`, `CortexM7`, `CortexM4`, `CortexM4f`. Methods: `as_key()`, `abbr()`, `from_abbr()`, `to_rust_target()`.

### TEE support (`consortium-tee` / `consortium-tee-proc-macros`)

**Error types:**
- `TeeErrorOrigin` — `NotTrustedOs`, `TrustedOs`, `TrustedApplication`, `Api`, `Unknown`
- `TeeError` — `Context`, `Session`, `Invoke`, `ParamMismatch`, `BufferTooSmall`, `TooManyParams`, `JoinError`, `Busy`. Methods: `is_transient()`, `ta_error_code()`.
- `TeeResult<T>` — alias for `Result<T, TeeError>`

**Proc macros (`consortium-tee-proc-macros`):**
- `tee_service!` — takes optional `context ContextStruct` + `commands { func_a, func_b }` block; generates a `#[repr(u64)] Command` enum (function names → CamelCase variants, `#[default] Unknown` catch-all) and `invoke_command(ctx, cmd: u32, params)` dispatcher that does `Command::from(cmd)` inside, matching the `#[ta_invoke_command]` hook signature
- `#[tee_command]` — wraps a function to generate a `_dispatched` variant (TA side) and a `call_` variant (CA side, `#[cfg(feature = "ca")]`); treats first `&mut SomePath` argument as context (forwarded to inner fn, not a TEE slot)
  - Input types: `bool`, `u32`, `&[u8]`, `&mut [u8]` — TA unpacks via `unsafe { params.N.as_value()?.a() }` / `unsafe { params.N.as_memref()? }.buffer()`; all `unsafe` blocks carry SAFETY comments
  - Return types (output slot, appended after inputs): `u32`, `u64`, `bool`, `Vec<u8>`
  - TA flushes output via `let mut __p_out = unsafe { params.N.as_value()? }; __p_out.set_a(...)`
  - CA packs via `ParamValue` / `ParamTmpRef`, wraps in `optee_teec::Operation::new(0, ...)`, calls `session.invoke_command(Command::Variant as u32, &mut _operation)`
  - `Vec<u8>` return: CA pre-allocates 4096-byte `ParamTmpRef::new_output` buffer; TA writes into it; CA reads back `buffer()[..size()]`

### Build system (`consortium-builder`)

**Status:** Stub (19 LOC). Planned features:

`ApplicationBuilder` cross-compiles for `aarch64-unknown-linux-gnu` using the ST OpenSTLinux SDK. It reads standard ST environment variables (`SDKTARGETSYSROOT`, `OECORE_NATIVE_SYSROOT`, `OECORE_TUNE_CCARGS`) and supports overrides (`OSTL_RUSTC`, `OSTL_CARGO`, `OSTL_SDK_SYSROOT`, `OSTL_SDK_EXTRA_FLAGS`). It sanitizes `PATH` to prevent the ST SDK toolchain from shadowing rustup.

`FirmwareBuilder` maps `CortexMcuCore` variants to Rust targets (via `to_rust_target()`), reads binary name from `[[bin]]` or `[package]`, verifies ELF magic after build, and supports a `release` flag:
- `CortexM0p` → `thumbv6m-none-eabi`
- `CortexM33` → `thumbv8m.main-none-eabi`
- `CortexM4` / `CortexM4f` / `CortexM7` → `thumbv7em-none-eabi`

---

## Hardware Doorbell Implementations

### ✅ Completed: HSEM (Hardware Semaphore)

**Status:** Fully implemented in `consortium-ipc-doorbell-hsem`

**Features:**
- **M-core (bare-metal, no_std):** Zero-size type using `AtomicWaker` for async waiting. Rings HSEM directly and registers HSEM IRQ handler. Uses lock ID `M_CORE_LOCKID`.
- **A-core (Linux, std):** `HsemDoorbell<PROC>` holds `Arc<tokio::sync::Notify>` for wake coordination. Exposes `.notifier()` method for external IRQ loop integration. Uses lock ID `A_CORE_LOCKID` (MASTERID 2).
- **Re-exported from:** Both `consortium-ipc-transport-mem` and `consortium-ipc-transport-uio` for convenience; `consortium-runtime-mcu` re-exports as `hsem` for backward compatibility.
- **Validates channels** via `HsemChanValidate` (max 64 channels per core).

**Platform support:** STM32MP2x and STM32MP25x SoCs.

### 🚧 Refactoring: MU (Messaging Unit)

**Status:** Refactoring in progress. The Messaging Unit has GSR/GCR/GREI registers that can implement doorbells more straightforwardly without relying on the destructive read behavior of RRn.

**Features:**
- **M-core (bare-metal, no_std):** Zero-size type using `AtomicWaker` for async waiting. Rings MU TX registers and registers MU1/MU2 IRQ handlers. Two separate handler functions for two MU instances.
- **A-core (Linux, std):** `MuDoorbell` holds base addresses and `Arc<tokio::sync::Notify>` for wake coordination. Exposes `.notifier()` method for external IRQ loop integration.
- **Validates channels** via `MuChanValidate` (max 8 channels across 2 MU instances; channels 0–1 reserved).
- **Multi-instance support:** Designed for up to 2 MU instances (MU1, MU2) with flexible base address configuration.
- **No const generic** for side/processor: Determined by `std` feature flag and base address configuration.

**Platform support:** i.MX 93, i.MX 8M Plus, i.MX 95, etc. (i.MX 94/95 have a different MU design, but they only differs in register layout, so the crate is designed to support both with conditional logic.)

**Key differences from HSEM:**
- **Signal mechanism:** Writes TX register (data transfer) instead of semaphore lock
- **Interrupt clear:** Reading RRn clears RSR (destructive) instead of dedicated ICR
- **IRQ handlers:** Two independent handlers (MU1/MU2) instead of one per processor
- **Channel encoding:** Channels 0–7 map across 2 instances with 4 TX/RX pairs each

**Platform support:** i.MX 9x series (i.MX 93, i.MX 8M Plus, i.MX 95, etc.)

To add a new doorbell:
1. Create a new crate `consortium-ipc-doorbell-<name>` (or add module to existing crate)
2. Implement the `Doorbell` trait (`ring()`, `wait()`, `pending()`)
3. Handle platform-specific register access and interrupt routing
4. Optionally implement `ChanValidate` for channel validation
5. Test with real hardware or simulation
- Unless Technical Reference Manual and register map are available in context, you shouldn't implement a doorbell from scratch.

### 2. M-Core Runtime (`consortium-runtime-mcu`)

Currently a stub. Must provide:
- **Bootstrap code:** Stack setup, BSS initialization, relocation
- **Async executor:** Cooperative executor or RTIC integration for `await` support
- **MemTransport integration:** Initialize shared memory slots, create channels
- **Interrupt handlers:** Register and dispatch M→A and A→M interrupts
- **Example firmware:** Minimal app that sends/receives messages via IPC

### 3. A-Core Runtime (`consortium-runtime-app`)

Partially implemented (RemoteProc sysfs wrapper only). Must add:
- **Firmware loading:** Map firmware ELF into memory via remoteproc
- **Lifecycle management:** Start/stop firmware, handle state transitions
- **UioDevice integration:** Initialize shared memory descriptor, open UIO device
- **Example application:** Minimal app that loads firmware and exchanges messages
- **Error recovery:** Handle firmware crashes, timeouts, channel failures

### 4. Full Compilation Pipeline

The system requires an 9-step build pipeline (see implementation-status.md):

1. **Config → Device Tree + PAC:** Rust config code generates device tree and peripheral access crate
2. **TF-M/NS-M compilation:** Compile Trusted/Non-Secure M-core firmware, bundle per remoteproc
3. **TF-A/OP-TEE compilation:** Compile ARM Trusted Firmware and OP-TEE, optionally sign
4. **HMI Web UI:** Compile Vite/React web interface, bundle static assets
5. **Application binary:** Compile Cortex-A Linux application
6. **Quantize models:** If applicable, quantize ML models for embedded inference
7. **Configuration generation:** Output systemd services, supervisor configs, nginx configs
8. **Installer script:** Attach setup and deployment scripts
9. **Package:** Bundle everything into `.tar.gz` for deployment

Status: Not started. Requires orchestration in `consortium-builder` or separate build tool.

---

## Getting Started: Platform Ports

### For STM32MP2x Series (HSEM Doorbell)

The HSEM doorbell is ready to use. To create a firmware application:

1. **Create M-core firmware** in `consortium-runtime-mcu`
   - Use `HsemDoorbell` from the re-export (`hsem::HsemDoorbell`)
   - Initialize shared memory slots using `MemTransport::new(doorbell, region)`
   - Create `Channel<Tx, YourMessage, MemTransport<HsemDoorbell, SramRegion>, PostcardCodec>`
   - Implement bootstrap and async executor integration

2. **Create A-core application** in `consortium-runtime-app`
   - Open UIO device: `UioDevice::open(uio_num)`
   - Get channel: `device.channel::<YourMessage>(Chan(0))`
   - Send/receive via `channel.send(&msg)` and `channel.recv()`
   - Handle `UioDevice::with_notify()` to integrate HSEM interrupt notifications

3. **Test integration:**
   - Build firmware for M-core target
   - Build application for A-core (Linux)
   - Load firmware via remoteproc
   - Exchange messages bidirectionally
   - Verify error handling and timeouts

### For NXP i.MX 9x Series (MU Doorbell)

The MU doorbell is ready to use. To create a firmware application:

1. **Create M-core firmware** in `consortium-runtime-mcu`
   - Use `MuDoorbell` from the crate (`consortium-ipc-doorbell-mu::MuDoorbell`)
   - Construct with `MuDoorbell::new(MU1_BASE, MU2_BASE)` or platform-specific base addresses
   - Initialize shared memory slots using `MemTransport::new(doorbell, region)`
   - Create `Channel<Tx, YourMessage, MemTransport<MuDoorbell, SramRegion>, PostcardCodec>`
   - Register `MU1_IRQHandler` and `MU2_IRQHandler` in interrupt vector table
   - Implement bootstrap and async executor integration

2. **Create A-core application** in `consortium-runtime-app`
   - Open UIO device: `UioDevice::open(uio_num)`
   - Get channel: `device.channel::<YourMessage>(Chan(2))` (use channels 2–7; 0–1 reserved)
   - Send/receive via `channel.send(&msg)` and `channel.recv()`
   - Integrate the MU doorbell's notifier: `doorbell.notifier()` → pass to UIO IRQ loop

3. **Test integration:**
   - Build firmware for M-core target (e.g., Cortex-M7)
   - Build application for A-core (Linux, aarch64)
   - Load firmware via remoteproc
   - Exchange messages bidirectionally via MU signaling
   - Verify error handling and timeouts

**Note:** MU base addresses are platform-specific. For i.MX 93:
- MU1 typically at `0x4422_0000` (A-side) and `0x4423_0000` (B-side)
- MU2 typically at `0x4243_0000` (A-side) and similar offset for B-side
- Consult your SoC's reference manual for exact addresses.

See `documents/implementation-status.md` for detailed roadmap and effort estimates.
