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

`consortium-uio` and `consortium-tee` only build on Linux. Tests for them must be run on a Linux host.

## Architecture

This is a Rust workspace providing an async abstraction for inter-processor communication on Heterogeneous Multi-Processing (HMP) SoCs, primarily targeting the STM32MP2 (Cortex-A + Cortex-M) and i.MX 95 (Cortex-A + Cortex-M).

### Crates

- **`consortium-ipc`** — `no_std`-compatible core traits and types. Feature-gated: `std`, `alloc`, `futures`.
- **`consortium-mem`** — Bare-metal shared-memory transport for M-core (`no_std`, no async). Implements `Transport` via volatile slot reads/writes.
- **`consortium-uio`** — Linux UIO transport (`/dev/uioN`). Linux-only, async via Tokio (features: `rt`, `sync`, `net`).
- **`consortium-app`** — Linux-side `remoteproc` sysfs wrapper. Exports `RemoteProc` (index-based constructor, `state()` reader) and `RemoteProcState` enum (Offline, Suspended, Running, Crashed, Deleted, Attached, Detached) plus `RemoteProcCoredump` enum (Enabled, Disabled, Inline).
- **`consortium-builder`** — SDK-aware cross-compilation helpers: `ApplicationBuilder` (Cortex-A35/OpenSTLinux) and `FirmwareBuilder` (Cortex-M MCUs). Async via Tokio (features: `full`).
- **`consortium-common`** — Shared configuration types: `Config`, `Chip` enum, `CortexMcuCore` enum, chip-to-core mappings.
- **`consortium-tee`** — Trusted Execution Environment support. Linux-only (links `optee-teec`). Provides error types (`TeeError`, `TeeErrorOrigin`, `TeeResult`) and parameter marshaling (`TeeParam`, `IntoTeeParam`, `FromTeeParam`, `MaterialisedParams`).
- **`consortium-tee-proc-macros`** — Proc-macro crate for TEE. Provides `#[tee_service]` (generates `Command` enum + `invoke_command` dispatcher) and `#[fee_function]` (generates CA `call_*` wrappers and TA `*_dispatched` handlers). Max 4 TEE parameters.
- **`consortium-cli`** — Debug CLI tool (stub).
- **`consortium-rt`** — Real-time runtime library (stub).

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

**Parameter marshaling:**
- `TeeParam<'a>` — enum for TEE parameter types
- `IntoTeeParam` — implemented for `&[u8]`, `Vec<u8>`, `u32`, `u64`, `bool`
- `FromTeeParam` — implemented for `Vec<u8>`, `u32`, `u64`, `bool`
- `MaterialisedParams` — raw TEEC parameter materialization

**Proc macros (`consortium-tee-proc-macros`):**
- `#[tee_service]` — takes optional `context ContextStruct` + `commands { func_a, func_b }` block; generates a `Command` enum (function names → CamelCase variants) and `invoke_command()` dispatcher
- `#[fee_function]` — wraps a function to generate a `_dispatched` variant (TA side) and a `call_` variant (CA side, feature-gated); handles up to 4 parameters; auto-packs/unpacks `bool`, `u32`, `&[u8]`, `&mut [u8]`; treats first `&mut` argument as context

### Build system (`consortium-builder`)

`ApplicationBuilder` cross-compiles for `aarch64-unknown-linux-gnu` using the ST OpenSTLinux SDK. It reads standard ST environment variables (`SDKTARGETSYSROOT`, `OECORE_NATIVE_SYSROOT`, `OECORE_TUNE_CCARGS`) and supports overrides (`OSTL_RUSTC`, `OSTL_CARGO`, `OSTL_SDK_SYSROOT`, `OSTL_SDK_EXTRA_FLAGS`). It sanitizes `PATH` to prevent the ST SDK toolchain from shadowing rustup.

`FirmwareBuilder` maps `CortexMcuCore` variants to Rust targets (via `to_rust_target()`), reads binary name from `[[bin]]` or `[package]`, verifies ELF magic after build, and supports a `release` flag:
- `CortexM0p` → `thumbv6m-none-eabi`
- `CortexM33` → `thumbv8m.main-none-eabi`
- `CortexM4` / `CortexM4f` / `CortexM7` → `thumbv7em-none-eabi`
