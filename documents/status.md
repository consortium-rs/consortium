# Consortium — Implementation Status

This document records what has been implemented and what remains to be done.

---

## Done

### `consortium-ipc` — Core trait hierarchy ✅

All traits and types that make up the public API surface are implemented.

- **`Doorbell`** trait — pure signalling primitive (`ring`, `wait`, `pending`)
- **`Region`** trait — contiguous memory area with cache management (`flush`, `invalidate`, `is_coherent`)
  - `SramRegion` — always-coherent; flush/invalidate are no-ops
  - `DdrRegion` — issues AArch64 `dc cvac` / `dc ivac` + `dsb sy`; compiles to no-ops on non-AArch64
  - `SerialRegion` — virtual, no backing memory; used for UART/SWD trace channels
- **`Transport`** trait — byte mover; owns a `Doorbell`; async `send` / `recv` via GAT futures
- **`Codec`** trait — serialise/deserialise messages; default `PostcardCodec` via postcard + serde
- **`Channel<D, T, Tr, C>`** — typed, directed, allocation-free endpoint
  - `ReceivedMessage<'a, T, C>` for zero-copy codec support
  - `ChannelError<C, Tr>` distinguishing codec vs transport errors
  - `Chan(u8)` addressing + `ChanValidate` trait

### `consortium-mem` — Bare-metal shared-memory transport ✅

Used on the M-core (Cortex-M, no OS, no allocator).

- `MemTransport<D, R>` over raw pointers to shared DMA memory
- Slot protocol: `[len: u32][flags: u32][payload: [u8]]`; `FLAG_VALID (0x01)` / `FLAG_CONSUMED (0x02)`
- `SendFut` completes synchronously (write + doorbell ring in one poll)
- `RecvFut` polls slot; awaits `Doorbell::wait()` when empty

### `consortium-uio` — Linux UIO transport ✅

Used on the A-core (Cortex-A, Linux, Tokio).

- `UioDevice::open(n)` — mmaps `/dev/uioN` resource 0, parses binary descriptor (magic `0x434F4E53`, version 1); fails unless state is `DescState::Ready`
- Background Tokio task: waits on `AsyncFd::readable()` for M→A IRQs, broadcasts via `tokio::sync::Notify`
- `UioChannel` implements `Transport`
- Descriptor lifecycle: `KernelInit (0) → Proposed (1) → MReady (2) → Ready (3)` → `Suspended (4)` / `Error (5)`

### `consortium-common` — Shared configuration types ✅

- `Config`, `GeneralConfig`, `ProjectConfig`, `FirmwareEntry`
- `Chip` enum: `Stm32mp21x`, `Stm32mp23x`, `Stm32mp25x`, `Imx93`, `Imx93w`, `Imx937`, `Imx94`, `Imx95`, `Imx952`
  - `has_rt_core()`, `has_rt_core_name()`, `remoteproc_name()`, `programmable_rt_cores()`, `app_cpu()`, `app_compile_target()`
- `CortexMcuCore` enum: `CortexM0p`, `CortexM33`, `CortexM7`, `CortexM4`, `CortexM4f`
  - `as_key()`, `abbr()`, `from_abbr()`, `to_rust_target()`
- `CortexAppCore` enum: `CortexA35`, `CortexA55`
  - `to_rust_target()`
- `Config::validate_rt_cores()` — checks every firmware entry against the declared chip

### `consortium-builder` — Cross-compilation helpers ✅

- `ApplicationBuilder` — cross-compiles for `aarch64-unknown-linux-gnu` using the ST OpenSTLinux SDK
  - Reads `SDKTARGETSYSROOT`, `OECORE_NATIVE_SYSROOT`, `OECORE_TUNE_CCARGS`
  - Overrides: `OSTL_RUSTC`, `OSTL_CARGO`, `OSTL_SDK_SYSROOT`, `OSTL_SDK_EXTRA_FLAGS`
  - Sanitises `PATH` to prevent the ST SDK toolchain from shadowing rustup
- `FirmwareBuilder` — cross-compiles for Cortex-M targets
  - Maps `CortexMcuCore` → Rust target triple via `to_rust_target()`
  - Reads binary name from `[[bin]]` or `[package]` in `Cargo.toml`
  - Verifies ELF magic after build
  - Supports `release` flag

### `consortium-tee-proc-macros` — TEE proc-macro crate ✅

- `tee_service!` — generates `#[repr(u64)] Command` enum (function names → CamelCase) and `invoke_command` dispatcher matching the `#[ta_invoke_command]` hook signature
- `#[tee_command]` — generates `_dispatched` (TA side) and `call_*` (CA side, `#[cfg(feature = "ca")]`) variants
  - Supported input types: `bool`, `u32`, `&[u8]`, `&mut [u8]`
  - Supported output types: `u32`, `u64`, `bool`, `Vec<u8>`
  - All `unsafe` blocks carry `// SAFETY:` comments
  - Max 4 TEE parameter slots enforced at compile time
  - Optional `serde` feature for serialised types

### `consortium-app` — remoteproc sysfs wrapper (partial) ⚠️

- `RemoteProc` struct (index-based constructor)
- `RemoteProcState` enum: `Offline`, `Suspended`, `Running`, `Crashed`, `Deleted`, `Attached`, `Detached`
- `RemoteProcCoredump` enum: `Enabled`, `Disabled`, `Inline`
- Methods: `state()`, `start()`, `stop()`, `set_firmware()`, `set_coredump()`, `set_recovery()`

**Known issues in the current code:**
- `From<u8>` and `From<String>` for `RemoteProcState`, and `From<String>` for `RemoteProcCoredump`, call `panic!()` on unrecognised input — these should use `TryFrom` with a proper error type
- All public methods return `std::io::Result` which conflates I/O errors with invalid-state errors; a dedicated error type would be cleaner
- `impl ToString` is deprecated in favour of `impl Display`
- `lib.rs` contains boilerplate `add()` / `it_works` that was never removed

### `consortium-tee` — TEE error types (partial) ⚠️

- `TeeErrorOrigin` enum: `NotTrustedOs`, `TrustedOs`, `TrustedApplication`, `Api`, `Unknown`
- `TeeError` enum: `Context`, `Session`, `Invoke`, `ParamMismatch`, `BufferTooSmall`, `TooManyParams`, `JoinError`, `Busy`
  - `is_transient()` and `ta_error_code()` methods
- `TeeResult<T>` type alias

**What is missing:** session management, context pooling, CA/TA lifecycle — only the error surface is defined.

---

## Not Yet Done

### `consortium-rt` — Real-time runtime library ❌

Currently a placeholder (`add(left, right)` boilerplate).

What it should provide for bare-metal Cortex-M targets:

1. **Minimal async executor** — a `block_on(future)` spin-loop that polls a single top-level future to completion, suitable for targets with no OS scheduler. Needs a no-op `Waker` implementation backed by a `RawWakerVTable` so that futures satisfy the `Context` requirement.

2. **Interrupt-driven waker integration** — a mechanism for hardware ISRs (HSEM, MU) to wake a sleeping task rather than busy-spinning. This requires storing a `Waker` handle that the ISR can call `wake()` on, and a matching `Doorbell` implementation that `wait()`s on that stored waker.

3. **Multi-task cooperative scheduler (longer term)** — a simple round-robin executor over a fixed-size task list, enabling, for example, a firmware that simultaneously handles an IPC receive loop and a sensor polling loop without an RTOS.

### `consortium-cli` — Debug / management CLI ❌

Currently a placeholder (`println!("Hello, world!")`).

What it should provide:

1. **Remote processor lifecycle commands** — wrap `consortium-app::RemoteProc` to expose `status [--index N]`, `start --index N`, `stop --index N` as subcommands.

2. **Firmware commands** — `firmware --index N --path <path>` to write a firmware path into sysfs; `coredump --index N --mode <enabled|disabled|inline>`.

3. **Recovery control** — `recovery --index N --enable / --disable`.

4. **Configuration validation** — parse a `consortium.toml` (via `consortium-common::Config`) and validate it against the detected chip: `validate --config path/to/consortium.toml`.

5. **Build integration (longer term)** — `build --config path/to/consortium.toml` that delegates to `consortium-builder` to cross-compile both application and firmware in one step.

### `consortium-tee` — TEE session management ❌

Only error types are defined. What is still needed:

1. **`TeeContext`** — wraps `optee_teec::Context`; manages library initialisation and teardown.

2. **`TeeSession`** — wraps `optee_teec::Session`; opens a session to a TA UUID, calls `invoke_command`, closes on drop.

3. **`SessionPool` (optional)** — reusable session pool to amortise the cost of repeated open/close cycles for high-frequency TA calls.

4. **CA-side integration with `#[tee_command]`** — the proc-macro already generates `call_*` wrappers (behind `#[cfg(feature = "ca")]`); the session management API needs to match what those generated wrappers expect.

---

## Known Issues / TODOs in Existing Code

| Location | Item |
|----------|------|
| `consortium-uio/src/lib.rs:256` | TODO: write channel ID to MU TX data register (UIO map1) so the M-core can skip scanning all slots — currently the M-core must scan every slot to find the waking channel |
| `consortium-app/src/remoteproc.rs:30,45,77` | `panic!()` on invalid enum conversions — should be `TryFrom` + error type |
| `consortium-common/src/config/chip_map.rs:48` | `panic!("Heterogeneous CPU targets are not supported yet")` — `Chip::app_compile_target()` panics when a chip has cores with different Rust targets; should return `Result` |
| `consortium-app/src/lib.rs` | Boilerplate `add()` / `it_works` test never cleaned up |
| `consortium-rt/src/lib.rs` | Boilerplate `add()` / `it_works` test — entire crate is a placeholder |
| `consortium-cli/src/main.rs` | `println!("Hello, world!")` — entire crate is a placeholder |
