# Consortium Project Assistance Guide

Consortium is a Rust workspace for heterogeneous multiprocessing systems, especially
SoCs that pair a Linux-capable A-core with Cortex-M, Cortex-R, or RISC-V real-time
cores. The repository is early-stage and moving quickly. Prefer reading the current
crate sources and manifests over relying on older docs, examples, or design notes.

The current implementation centers on explicit cross-core contracts:

- `consortium-ipc` defines typed channels, transport traits, side markers, doorbells,
  and `IpcSafe`.
  - `consortium-ipc-transport-memory` owns the descriptor/slot shared-memory transport. Doorbell crates bind hardware signaling to the transport layer.
  - `consortium-ipc-transport-uart` provides COBS/CRC-framed wired IPC for embedded and Unix
    serial backends.
- `consortium-codec` defines codec families used by IPC and TEE parameter conversion.
- `consortium-cfg-common` and `consortium-cfg` own the current `Consortium.toml`
  schema, chip databases, validation, and DTS/UIO-oriented transpilers.
- `consortium-hal`, chip HAL crates, and generated PAC crates provide early
  firmware-side peripheral access for i.MX9 and STM32MP2 targets.
- `consortium-dbg` implements shared-memory `defmt` logging and an optional panic
  handler.
- `consortium-log` is the common `defmt`/`tracing` facade used by generated and
  cross-platform code.
- `consortium-tee` handles the trusted execution environment contracts.
- `consortium-hmi` owns the backend-neutral HMI command bridge;
  `consortium-hmi-webkit` adapts it to Cog/WPE WebKit, and
  `consortium-hmi-pocket` provides the native PocketJS prototype.

## Commands

```bash
# List project recipes
just

# Install/update local toolchains used by recipes
just setup

# Run host-side checks
just check

# Run focused subsystem checks
just test ipc host
just test tee host
just test hmi host
just test cfg host
just test dbg host
just test codec host

# Run host-side lint bundle
just lint

# Apply Rust, frontend, Python, and TOML fixers/formatters
just fix

# Serve documentation locally
mdbook serve docs/book/
```

Use `cargo test -p <crate>` for focused validation while iterating. Full-workspace
commands can hit platform-sensitive crates or code that is mid-migration.

The `justfile` is the command index, but verify recipes against current crates before
copying them into new docs. Some target and lint recipes still mention crates or type
names that are not present in the current workspace, such as `consortium-common`,
`consortium-runtime-prt`, `consortium-cfg-meta`, `SharedMemConfig`, or `HmiConfig`.

The current target-oriented recipes include A-core Linux (`aarch64-unknown-linux-gnu`,
`armv7-unknown-linux-gnueabihf`), M-core bare-metal (`thumbv8m.main-none-eabihf`,
`thumbv7em-none-eabihf`), and RISC-V (`riscv32imc-unknown-none-elf`,
`riscv32imac-unknown-none-elf`, `riscv32imafc-unknown-none-elf`) checks/builds.
A-core, TEE, HMI, UIO, Cog, WebKit, and Yocto/OpenSTLinux flows are Linux/SDK oriented.

## Workspace Shape

| Area            | Crates                                                                                                                                                                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Core IPC        | `consortium-ipc`, `consortium-ipc-macros`, `consortium-ipc-macros-impl`                                                                                                                                       |
| Codecs          | `consortium-codec`, `consortium-codec-macros`, `consortium-codec-macros-impl`                                                                                                                                 |
| Transports      | `consortium-shared-memory`, `consortium-ipc-transport-memory`, `consortium-ipc-transport-uart*`                                                                                                               |
| Doorbells       | `consortium-ipc-doorbell-hsem`, `consortium-ipc-doorbell-ipcc`, `consortium-ipc-doorbell-mu`, `consortium-ipc-doorbell-gpio`                                                                                  |
| Configuration   | `consortium-cfg-common`, `consortium-cfg`, `consortium-dts`, `consortium-data`                                                                                                                                |
| HAL/PAC         | `consortium-hal`, `consortium-hal-imx9`, `consortium-hal-stm32mp2`, `consortium-pac-imx9`, `consortium-pac-stm32mp2`                                                                                          |
| Macro helpers   | `consortium-macros-helpers`                                                                                                                                                                                   |
| Runtimes/build  | `consortium-runtime-app`, `consortium-runtime-mcu`, `consortium-runtime-macros`, `consortium-runtime-macros-impl`, `consortium-runtime-spl`, `consortium-runtime-sys`, `consortium-builder`, `consortium-cli` |
| Logging/debug   | `consortium-log`, `consortium-dbg`                                                                                                                                                                            |
| TEE             | `consortium-tee`, `consortium-tee-macros`, `consortium-tee-macros-impl`                                                                                                                                       |
| HMI/Cog/ORT/FFI | `consortium-hmi`, `consortium-hmi-webkit`, `consortium-hmi-pocket`, `cogcore`, `cogcore-sys`, `consortium-ort`, `consortium-ffi`                                                                              |
| Examples        | `examples/melt-pot/*` for i.MX95 and STM32MP25 app/MCU/TA/shared message crates                                                                                                                               |

The root workspace includes `crates/*` and `examples/melt-pot/shared`. Per-endpoint
example crates under `examples/melt-pot/{imx95,stm32mp25}/{app,mcu,ta}` are excluded
because the builder pipeline generates `src/consortium.gen.rs` for them and resolves
their artifacts outside plain workspace `cargo check`.

## Codecs

`consortium-codec` is `no_std` by default and has no default codec feature. Available
families are feature-gated:

- `postcard` exposes `PostcardCodec`.
- `prost` exposes `ProstCodec`.
- `rkyv` exposes `RkyvCodec` and re-exports `TrustedArchive`.

`CodecFor<T>` owns the associated `Decoded<'buf>` type, so channel and TEE code should
avoid assuming decode always produces an owned `T`.

## IPC Core

The current IPC trait stack is:

```text
Doorbell       - hardware/software signal for waking a remote processor
SendTransport  - byte send half; writes bytes and rings the doorbell
RecvTransport  - byte receive half; waits for the doorbell and reads bytes
Transport      - full-duplex transport: SendTransport + RecvTransport
CodecFor<T>    - serialization family for a message type
Channel<Tx/Rx> - typed directed endpoint over one transport half
Transceiver    - typed send/receive endpoint built from paired channels
```

`consortium-ipc` is `no_std` unless `std` is enabled. Its feature shape is small:
`alloc`, `futures`, and `std = ["alloc", "futures"]`. Keep core traits usable from
firmware-side code.

`Channel<D, T, Tr, C>` does not allocate. Callers provide a `&'static mut [u8]` scratch
buffer sized for the transport MTU. `Tx` channels require `SendTransport`; `Rx`
channels require `RecvTransport`. `ReceivedMessage` is designed to handle owned codecs
today and zero-copy codecs later.

`Transceiver<TyTx, TyRx, TrTx, TrRx, C>` is public and wraps paired `Channel<Tx, ...>`
and `Channel<Rx, ...>` values. If a transport needs separate ownership for Tx/Rx, use
the transport's split halves before constructing channels.

`IpcSafe` rejects address-space-local constructs such as pointers, references, function
pointers, `usize`, and `isize`. Add reusable `syn`/AST helpers to
`consortium-macros-helpers`; keep domain-specific derive logic in the owning
`*-macros-impl` crate.

### UART IPC

The UART family is the first wired, Type-B transport: byte arrival is the notification,
so it has no separate `Doorbell` or `Side`. `consortium-ipc-transport-uart` implements
the shared protocol as `COBS(chan | payload | CRC32-LE) + 0x00`, with one logical
`Chan` per transport and explicit RX/TX scratch buffers derived with `rx_buf_len()` and
`tx_buf_len()`.

The core crate defaults to `std`: Tokio `AsyncRead`/`AsyncWrite` and `Send` futures. Its
no-std path uses `embedded-io-async` (0.7). `consortium-ipc-transport-uart-unix` opens raw
8N1 `/dev/tty*` devices with `tokio-serial` and provides pseudo-terminal pairs for tests.
`consortium-ipc-transport-uart-embedded` is the firmware glue: with a chip feature
(`mimx93`/`mimx95` or `stm32mp21x`/`stm32mp23x`/`stm32mp25x`) it re-exports that chip
HAL's interrupt-buffered UART driver (`BufferedUart`, in `imx9`/`stm32mp2` submodules);
with no chip feature it stays chip-agnostic over any `embedded-io-async` driver. The
`BufferedUart` RX path is IRQ-buffered: the application forwards the UART interrupt into
`BufferedUartState::on_interrupt`, which drains the FIFO into an
`embassy_hal_internal::atomic_ring_buffer::RingBuffer` so no bytes are lost before the
transport polls; TX stays direct interrupt-driven. Per repo convention the crate installs
no `#[interrupt]` handlers. `consortium-runtime-mcu` re-exports the transport behind its
`ipc-uart` feature.

### Shared Memory IPC

Shared memory is split across two crates:

- `consortium-shared-memory` defines the `Region` trait and concrete `SramRegion`,
  `DdrRegion`, and `SerialRegion` types. `SramRegion` is coherent/no-op cache
  maintenance; `DdrRegion` performs AArch64 data-cache maintenance on AArch64 and
  compiles to no-ops elsewhere; `SerialRegion` is a virtual no-backing region.
- `consortium-ipc-transport-memory` defines the `CIPC` descriptor layout, channel table,
  slot header protocol, descriptor init/complete helpers, and
  `SharedMemoryTransport<S, D, R>`.

`SharedMemoryTransport` is generic over a `Side`, a `Doorbell`, and a region implementing
`RegionSharedMemoryTransportable`. Currently `SramRegion` and `DdrRegion` implement that
transport marker.

The descriptor uses absolute channel directions (`PrimaryToSecondary` and
`SecondaryToPrimary`) and resolves them relative to the `Side` type. `Primary` is the
Linux/A-core convention, and `Secondary` is the firmware/M-core convention in current
examples and tests.

`SharedMemoryTransport::split()` returns `SharedMemoryTransportTx` and
`SharedMemoryTransportRx`. Use these halves when building simultaneous
`Channel<Tx, ...>` and `Channel<Rx, ...>` endpoints from one transport.

Shared-memory tests live under `crates/consortium-ipc-transport-memory/tests/` and cover
sync descriptor layout, Tokio futures, split halves, codec-backed channels, atomic-waker
no-std-shaped futures, Embassy-style polling, and end-to-end primary/secondary exchange.
Prefer extending these heap-backed or fake-doorbell tests before requiring hardware.

### Doorbells

Doorbells provide wakeup/signaling and should keep MMIO safety invariants documented.
Use typed register abstractions and validate channel bounds through the local validation
type where available.

Each hardware doorbell depends on its related chip HAL (`consortium-hal-stm32mp2` or
`consortium-hal-imx9`) and forwards its chip-selection feature to the HAL, so the shared
PAC's register/interrupt definitions match the target. A doorbell dep that
selects a non-default chip must set `default-features = false` so the HAL's default chip
is not also pulled in (two chips = ambiguous PAC `Interrupt`).

- `consortium-ipc-doorbell-hsem` targets STM32 HSEM. Chip features are `stm32mp23x` and
  `stm32mp25x` (both map to the PAC's `stm32mp257`); default is `stm32mp25x`; `std`
  enables Tokio support.
- `consortium-ipc-doorbell-ipcc` targets STM32 IPCC. Chip features are `stm32mp21x`,
  `stm32mp23x`, `stm32mp25x`; default is `stm32mp25x`; `std` enables Tokio support.
- `consortium-ipc-doorbell-mu` targets NXP i.MX MU. Chip features are `mimx93` and
  `mimx95`; default is `mimx95`; channels `0..4` are GI lines and `4..8` are RF lines.
- `consortium-ipc-doorbell-gpio` is a single-channel GPIO doorbell. The `no_std` path
  uses a software pending flag and IRQ-side `wake`; the `std` path exposes a Tokio
  `Notify` that external interrupt code must trigger.

No hardware doorbell installs `#[interrupt]` handlers. The application owns the vector
table and forwards each relevant IRQ into the crate's exported waker: HSEM calls
`hsem::notify()`, IPCC RX lines call `ipcc::notify_rx_occupied_interrupt()`, and each MU
line calls `mu::notify::<N>()` with its instance number. Enable the _HAL's_ `rt` feature
(`consortium-hal-*/rt`) so the `#[interrupt]` macro and PAC vector table are available to
that application code; the doorbell crates themselves no longer expose an `rt` feature.
The relevant vectors are HSEM (non-secure), IPCC1_RX/IPCC2_RX (non-secure), and the MU
lines a core drives (i.MX93 `MU1_B`/`MU2_B`; i.MX95 CM7 `MU5_A`/`MU7_B`/`MU8_B`). See the
melt-pot `mcu/src/main.rs` files for worked examples.

The `std` doorbell paths are host-testable with fake MMIO and Tokio `Notify`. Runtime
UIO integrations should bind each doorbell notifier to the Linux UIO IRQ fanout before
receiving.

## HAL and PAC

`consortium-hal` is a small `no_std` trait crate for generated chip metadata and drivers:
`Peripheral`, `ClockControl`, `InterruptSource`, `MonotonicTimer`, and `nvic::Irq`.

`consortium-pac-imx9` and `consortium-pac-stm32mp2` are generated `no_std` PAC crates.
Their chip features are `mimx93`/`mimx95` (default `mimx95`) and `stm32mp21`/`stm32mp257`
(default `stm32mp257`), respectively. PAC generation uses the `just pac-*` recipes, vendor
submodules under `submodules/`, parser scripts under `scripts/pac/`, and
generated/validated outputs under `output/` and `data/`. Avoid hand-editing generated PAC
output unless the recipe or patch workflow calls for it.

`consortium-hal-imx9` and `consortium-hal-stm32mp2` re-export the shared HAL and their
PAC crates (with `default-features = false`), re-export `interrupts`/`Interrupt` and the
`rt`-gated `#[interrupt]` macro, and expose feature-gated DMA, GPIO, I2C, timer-driver,
and UART modules. They also own the chip-selection features forwarded to the PAC:
`mimx93`/`mimx95` (hal-imx9, default `mimx95`) and `stm32mp21x`/`stm32mp23x`/`stm32mp25x`
(hal-stm32mp2, default `stm32mp25x`, where `stm32mp23x`/`stm32mp25x` both map to the PAC's
`stm32mp257`). Keep these crates firmware-friendly: `no_std`, narrow unsafe blocks, and
chip-specific details inside the chip HAL rather than the shared HAL traits.

## Configuration

The authoritative current schema is in `consortium-cfg-common/src/config/`, not in the
older melt-pot example manifests. Use `crates/consortium-cfg/Consortium.example.*.toml`
and `consortium-cfg/src/session.rs` tests as the best references.

Current top-level schema:

- `[profile]` contains required `chip`, `shared`, and `deploy_root`; optional `devkit`
  selects a bundled board DTS by full file stem, `sysroot` selects the Linux target
  sysroot, `kernel_recipe` enables generation of a kernel `.bbappend`, and
  `time_driver` (default `true`) toggles the sealed controller-core Embassy time driver,
  timer vector, and `defmt::timestamp!`.
- `[dts]` controls Linux device-tree output: `mode = "merge"` (default) emits a full
  tree, while `overlay` emits a `.dtso`; `base` overrides the base tree and `linux`
  points at a kernel source tree for GPL-licensed dt-bindings used during compilation.
  `configfs` opts into `CONFIG_OF_CONFIGFS` for overlay deployment. Overlay mode cannot
  delete remoteproc mailbox properties, so use merge mode for those fixups.
- `[endpoints]` maps runtime endpoint names to artifact roots.
- `[dbg.<core>]` declares a defmt logger on coprocessor `<core>` (e.g. `[dbg.cm33]`)
  and contains `memory = { base, length }` and `mtu`. The AP is always the host that
  decodes the logs, so there is no `[dbg.ap]`; a coprocessor with no table has no debug.
  An optional top-level `[dbg] mode` (`auto`/`interface`, default `auto`) selects how
  the AP consumes the rings: `auto` generates a console that forwards decoded defmt
  frames to `tracing`; `interface` returns reader handles for the integrator. The
  field flattens alongside the `[dbg.<core>]` tables.
- `[[ipc.channel]]` contains `name`, `doorbell`, `pattern`, `primary`, `secondary`,
  `party.<name>`, `config.memory`, `mtu`, and optional `codec` (default `postcard`).
- `[tee]` contains the OP-TEE UUID.
- `[hmi]` contains `engine`, `backend`, optional `debug`, `source`, and `build`.
- `[peripheral.<name>]` contains `owner`, `secure`, optional `frequency`, optional
  `clock`, and optional pin declarations.

`Session::from_toml` parses the manifest, resolves `[profile] chip` through embedded
hardware databases, and validates doorbell availability, visible parties, channel
bounds, duplicate doorbell channels, region bounds/overlap/MPU alignment, debug regions,
and peripheral owners. `consortium-data` now carries generated per-core chip maps,
peripheral descriptions, memory databases, and a large vendor DTS/include database for
STM32MP2 and i.MX9. Treat these as generated/imported data; change their generation
workflow rather than casually editing large database files.

`consortium-cfg` includes lowering, portable/Linux codegen, debug-console and peripheral
bring-up codegen, UIO/Kconfig generation, vendor-tree DTS merge/overlay rendering,
remoteproc fixups, and Type-A doorbell interrupt enablement. Keep config changes centered
on schema types first, then validation and lowering, then generated code and build
artifacts. Update snapshot tests under
`crates/consortium-cfg/tests/` when generated output intentionally changes.

### Build and Packaging Pipeline

The user-facing CLI binary is `csti`; `csti build` runs the end-to-end
`Consortium.toml` pipeline. It compiles the Linux application, controller firmware,
optional TA and HMI; post-processes firmware for Linux remoteproc; generates endpoint
modules, DTS/Kconfig/boot integration, and a systemd unit; then assembles a local
`dist/` staging tree. It does not deploy directly to a live target.

Firmware post-processing renames `.vector_table` to the remoteproc-expected
`.isr_vectors` section. Outputs include the deployable ELF and debug/original artifacts.
The staged tree can contain `app/`, `firmware/`, `ta/`, `www/`, `dts/linux/`, `kernel/`,
`boot/`, and `systemd/`, plus a generated BitBake recipe. CLI flags support staging the
tree into a recipe `files/` directory and invoking or dry-running BitBake. Keep the
builder's planning, compilation, staging, and external BitBake hand-off boundaries clear.

### Build-Time Configuration

The builder flow injects compile-time environment variables such as `CONSORTIUM_IPC_*`,
`CONSORTIUM_DBG_*`, `CONSORTIUM_HMI_*`, and `CONSORTIUM_TEE_UUID`. Consumers read them
with `option_env!`/`env!`. The builder emits numeric env-var values as decimal strings;
firmware-side consumers that need const parsing can use `const-str` and should not depend
on the std-oriented schema crates.

Current IPC build-time variables are channel-local metadata such as
`CONSORTIUM_IPC_LENGTH`, `CONSORTIUM_IPC_UIO`, and `CONSORTIUM_IPC_MTU`; the shared
memory base comes from generated code and UIO mapping, not `CONSORTIUM_IPC_BASE`.
`consortium-dbg::config` reads `CONSORTIUM_DBG_BASE` and `CONSORTIUM_DBG_LENGTH`.
`consortium-hmi-webkit` reads `CONSORTIUM_HMI_DIST` and optional
`CONSORTIUM_HMI_DEBUG_URL`. The HMI build command is selected by
`CONSORTIUM_HMI_BUILD_CMD`, then `vp`, then `npm`.

The build pipeline writes `src/consortium.gen.rs` into generated endpoint crates. The
example app/MCU crates use `include!("consortium.gen.rs")`; do not expect plain cargo
commands to work for those crates before the pipeline has generated that file.

### Runtime Layers

`consortium-runtime-app` is Linux-only. It exposes remoteproc support, optional UIO-backed
IPC (`ipc-shm`), optional debug (`dbg`), and optional TEE (`tee`). `UioDevice` opens
`/dev/uioN`, maps the shared region, and runs a Tokio IRQ task that wakes registered
doorbell notifiers.

`consortium-runtime-mcu` is `no_std`, re-exports debug and shared-memory IPC pieces
behind features, and provides small helpers such as `static_buf!` for scratch buffers.
Its default features are `ipc-shm` and `macros`. It also exposes `rsc_table`: raw bindings come from
the generated `consortium-runtime-sys` crate, while `SharedResourceTable` and
`RSC_TABLE` emit an RPMsg-vdev-plus-trace table in the firmware `.resource_table`
section. Keep packed-layout access and Linux remoteproc ABI requirements explicit.

### Entry Macros and `Context`

`consortium-runtime-macros` (facade) + `consortium-runtime-macros-impl` provide the
runtime entry-point attribute macros, re-exported by the runtime crates behind their
`macros` feature (default on). Firmware writes `#[consortium_runtime_mcu::main] async fn
main(context: Context, spawner: Spawner)`; applications write
`#[consortium_runtime_app::main] async fn main(context: Context)`. The macro stacks the
executor attribute (`embassy_executor::main` / `tokio::main`, resolved through the _user_
crate's own dep — so firmware still needs `embassy-executor` with `executor-thread` +
`platform-cortex-m`, and apps still need `tokio` with `macros`), calls the generated
`init()`, and binds its result as `context`.

`Context` is the aggregate the generated `consortium.gen.rs` exposes for the whole
manifest — connected transceivers at `context.ipc_shm.<name>`, plus `context.peripherals`
on controller cores that own `[peripheral.*]`. On both sides `init()` is
`async fn init() -> Result<Context, ConsortiumInitError>`; the two-phase
`IpcMemoryEndpoints::new`/`connect` API stays available for non-macro callers. On failure
the macro glue calls `__consortium_init_fail`; define an optional
`#[consortium_runtime_{mcu,app}::fail] fn on_fail(err: ConsortiumInitError)` to report it
(its module-scope item shadows the runtime crate's glob-imported fallback, which otherwise
hard-fails: MCU parks with `wfi`, the app exits non-zero). `#[main]`, `#[fail]`, `init`,
and `Context` must share the module the generated file is `include!`d into.

On controller cores the generated `init()` also seals what firmware used to hand-write:
it resets the CDBG ring (`ConsortiumLogger::init`) when a `[dbg.<core>]` logger is present,
and brings up the chip's Embassy time driver via `timer_driver::bringup` (STM32MP2 TIM2 /
i.MX9 LPIT1), emitting the matching timer vector and `defmt::timestamp!` into the generated
module. Disable this with `[profile] time_driver = false` when firmware owns its own time
base (these defaults are reference-board values). See `consortium-cfg`'s
`components/time.rs` and the melt-pot `mcu/src/main.rs` files.

`consortium-runtime-spl` currently exposes small domain/peripheral/role traits for
partitioning-oriented code.

`consortium-builder` owns the pipeline for compiling application, firmware, TA, and HMI
artifacts from a `Consortium.toml` session. It bridges the current config schema into
generated endpoint modules and compile-time environment variables; prefer extending
`consortium-cfg` lowering first, then the builder pipeline.

## TEE

`consortium-tee` is `no_std` with `alloc`. The default feature is `ca`, but CA error
exports are gated to Linux non-test builds. The `ta` feature is host-agnostic and is used
by host-side macro/parameter tests.

`TeeParam<C = NoCodec>` converts Rust values to and from `TeeParamSlot`. Primitive
values map to TEE value slots and implement `TeeParam<C>` for any `C`. Serialized values
map to memref slots and require a `CodecFor<T>` implementation. `#[derive(TeeParam)]`
generates serialized support; `#[tee(max_size = N)]` overrides the default serialized
buffer bound.

When changing TEE macros, update trybuild coverage and insta snapshots in the macro
implementation crate before relying on the proc-macro facade.

## Debug Logging

`consortium-log` is a `no_std` macro facade with optional `defmt` and `tracing`
backends. Both may be enabled, or neither (no-op). Use the portable format-string plus
positional-arguments subset in shared code; backend-specific `defmt` hints and
`tracing` fields are only safe when that backend is exclusively enabled. Arguments must
not have side effects because disabled log calls do not evaluate them.

`consortium-dbg` implements defmt-over-shared-memory with `CDBG` descriptors. Its
default features are `host` and `logger`; `decode` adds `defmt-decoder` and implies
`host`. Debug configuration comes from `CONSORTIUM_DBG_*`; plain `cargo build` leaves
the config unconfigured and the logger no-op unless the binary calls the const guard.

Preserve descriptor discipline when changing debug transport behavior. Host-side tests
cover descriptor initialization, ring read/write behavior, decoding fixtures, and
overflow/error flags.

The optional panic handler emits an error packet, latches panic status in the debug
descriptor, then waits with the target architecture's WFI instruction.

## HMI

`consortium-hmi` is the portable application-service registry and response-future
contract. It contains no Cog, GLib, WebKit, PocketJS, or Tokio runtime state.
`consortium-hmi-webkit` is Linux-only and exposes `KioskApp` plus the shared `Bridge`;
in debug builds it can load `CONSORTIUM_HMI_DEBUG_URL`, otherwise it serves the compiled
dist path from `CONSORTIUM_HMI_DIST`. `consortium-hmi-pocket` is the native prototype:
it hosts a PocketJS JS/pak pair, mounts the bridge into QuickJS, delivers responses at
frame boundaries, and exposes headless wgpu rendering. It does not yet provide the
production Wayland/DRM/input shell or builder integration.
`cogcore-sys` owns raw bindgen output and vendored Cog/WPE/WebKit headers; `cogcore`
wraps a small safe subset and keeps WebKit/WPE types mostly opaque.

Cog/WPE work needs Linux Cog, GLib/GIO, WPE, and WebKit development packages. On hosts
without those packages, prefer formatting, source review, and narrow tests that do not
link Cog/WebKit. Use the `cogcore.yml` workflow or a properly provisioned Linux machine
for binding generation and wrapper validation.

When extending `cogcore`, prefer `glib`/`gio` types for public GLib/GIO interop, keep
raw ownership and refcounting inside the safe wrapper layer, and document each unsafe
block with the pointer lifetime or ownership invariant.

## Testing Guidance

- Use `cargo test -p <crate>` for focused host-side validation.
- Use `just test ipc host` for the current host IPC bundle.
- Use `cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test`
  for the no-std-shaped shared-memory future path.
- UART protocol changes should run `cargo test -p consortium-ipc-transport-uart`,
  `cargo test -p consortium-ipc-transport-uart --no-default-features`, and
  `cargo test -p consortium-ipc-transport-uart-unix` for pseudo-terminal integration.
- Use trybuild and insta snapshots for proc-macro behavior.
- Use fake MMIO, heap-backed regions, and mock doorbells for protocol logic.
- Keep hardware, SDK, UIO, remoteproc, OP-TEE runtime, and Cog/WebKit checks separate
  from default host checks.
- Do not assume Linux-only crates or no-std target checks work on every developer
  machine.

## Style and Tooling

- Rust edition is 2024 across the workspace.
- Workspace lints deny undocumented unsafe blocks and unsafe-operation shortcuts.
- Keep unsafe blocks narrow and explain the hardware, memory, aliasing, or ABI invariant.
- `cargo fmt` handles Rust formatting.
- Taplo uses two-space TOML indentation and a 100-column width.
- `typos.toml` configures spelling checks.
- `.ls-lint.yml` defines crate, test, doc, script, board, and simulator naming rules.

When updating docs, use crate names, features, APIs, and schema fields present in the
current workspace. The mdBook lives under `docs/book/`. Treat `docs/book/src/dev/**` and
the dated `docs/book/src/log/**` entries as design/debugging history; introductions are
useful conceptual context, but verify all of them against current source before making
them normative.

## Dependency Policy

Use credible crates for common functionality, especially in tests. Before adding a
dependency, ensure the crate is compatible with Apache-2.0-friendly licensing and with
the relevant `no_std`/`alloc`/`std` target constraints.

<!--VITE PLUS START-->

# Using Vite+, the Unified Toolchain for the Web

This project is using Vite+, a unified toolchain built on top of Vite, Rolldown, Vitest, tsdown, Oxlint, Oxfmt, and Vite Task. Vite+ wraps runtime management, package management, and frontend tooling in a single global CLI called `vp`. Vite+ is distinct from Vite, and it invokes Vite through `vp dev` and `vp build`. Run `vp help` to print a list of commands and `vp <command> --help` for information about a specific command.

Docs are local at `node_modules/vite-plus/docs` or online at https://viteplus.dev/guide/.

## Review Checklist

- [ ] Run `vp install` after pulling remote changes and before getting started.
- [ ] Run `vp check` and `vp test` to format, lint, type check and test changes.
- [ ] Check if there are `vite.config.ts` tasks or `package.json` scripts necessary for validation, run via `vp run <script>`.
- [ ] If setup, runtime, or package-manager behavior looks wrong, run `vp env doctor` and include its output when asking for help.

<!--VITE PLUS END-->
