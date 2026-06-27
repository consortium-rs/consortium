# AGENTS.md - AI Assistant Guide for Consortium

Consortium is a Rust workspace for heterogeneous multiprocessing systems, especially
SoCs that pair a Linux-capable A-core with Cortex-M, Cortex-R, or RISC-V real-time
cores. The repository is early-stage and moving quickly. Prefer reading the current
crate sources and manifests over relying on older docs, examples, or design notes.

The current implementation centers on explicit cross-core contracts:

- `consortium-ipc` defines typed channels, transport traits, side markers, doorbells,
  and `IpcSafe`.
- `consortium-codec` defines codec families used by IPC and TEE parameter conversion.
- `consortium-shared-memory` owns low-level memory-region abstractions.
- `consortium-ipc-transport-memory` owns the descriptor/slot shared-memory transport.
- Doorbell crates bind hardware signaling to the transport layer.
- `consortium-cfg-common` and `consortium-cfg` own the current `Consortium.toml`
  schema, chip databases, validation, and DTS/UIO-oriented transpilers.
- `consortium-tee` handles the trusted execution environment contracts.
- `consortium-hmi` provides a Cog/WPE WebKit-based human-computer interface.

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

# Run host-side lint bundle
just lint

# Apply Rust, frontend, Python, and TOML fixers/formatters
just fix

# Serve documentation locally
mdbook serve docs/
```

Use `cargo test -p <crate>` for focused validation while iterating. Full-workspace
commands can hit platform-sensitive crates or code that is mid-migration.

The `justfile` is the command index, but verify recipes against current crates before
copying them into new docs. Some recipes still mention crates or type names that are not
present in the current workspace, such as `consortium-common`, `consortium-runtime-prt`,
`SharedMemConfig`, or `HmiConfig`.

The current target-oriented recipes include A-core Linux (`aarch64-unknown-linux-gnu`,
`armv7-unknown-linux-gnueabihf`), M-core bare-metal (`thumbv8m.main-none-eabihf`,
`thumbv7em-none-eabihf`), and RISC-V (`riscv32imc-unknown-none-elf`,
`riscv32imac-unknown-none-elf`, `riscv32imafc-unknown-none-elf`) checks/builds.
A-core, TEE, HMI, UIO, Cog, WebKit, and Yocto/OpenSTLinux flows are Linux/SDK oriented.

## Workspace Shape

| Area            | Crates                                                                                                                       |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Core IPC        | `consortium-ipc`, `consortium-ipc-macros`, `consortium-ipc-macros-impl`                                                      |
| Codecs          | `consortium-codec`, `consortium-codec-macros`, `consortium-codec-macros-impl`                                                |
| Shared memory   | `consortium-shared-memory`, `consortium-ipc-transport-memory`                                                                |
| Doorbells       | `consortium-ipc-doorbell-hsem`, `consortium-ipc-doorbell-ipcc`, `consortium-ipc-doorbell-mu`, `consortium-ipc-doorbell-gpio` |
| Configuration   | `consortium-cfg-common`, `consortium-cfg`, `consortium-dts`                                                                  |
| Macro helpers   | `consortium-macros-helpers`                                                                                                  |
| Runtimes/build  | `consortium-runtime-app`, `consortium-runtime-mcu`, `consortium-runtime-spl`, `consortium-builder`, `consortium-cli`         |
| Debug           | `consortium-dbg-defmt`                                                                                                       |
| TEE             | `consortium-tee`, `consortium-tee-macros`, `consortium-tee-macros-impl`, `consortium-tee-test`                               |
| HMI/Cog/ORT/FFI | `consortium-hmi`, `cogcore`, `cogcore-sys`, `consortium-ort`, `consortium-ffi`                                               |
| Examples        | `examples/melt-pot/*` for i.MX95 and STM32MP25 app/MCU/TA/shared message crates                                              |

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

`Transceiver<TyTx, TyRx, Tr, C>` is public and wraps paired `Channel<Tx, ...>` and
`Channel<Rx, ...>` values. If a transport needs separate ownership for Tx/Rx, use the
transport's split halves before constructing channels.

`IpcSafe` rejects address-space-local constructs such as pointers, references, function
pointers, `usize`, and `isize`. Add reusable `syn`/AST helpers to
`consortium-macros-helpers`; keep domain-specific derive logic in the owning
`*-macros-impl` crate.

## Codecs

`consortium-codec` is `no_std` by default and has no default codec feature. Available
families are feature-gated:

- `postcard` exposes `PostcardCodec`.
- `prost` exposes `ProstCodec`.
- `rkyv` exposes `RkyvCodec` and re-exports `TrustedArchive`.

`CodecFor<T>` owns the associated `Decoded<'buf>` type, so channel and TEE code should
avoid assuming decode always produces an owned `T`.

## Shared Memory IPC

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

## Doorbells

Doorbells provide wakeup/signaling and should keep MMIO safety invariants documented.
Use typed register abstractions and validate channel bounds through the local validation
type where available.

- `consortium-ipc-doorbell-hsem` targets STM32 HSEM. Features are `stm32mp23`,
  `stm32mp25`, `stm32h7x5`, `stm32h7x7`; default is `stm32mp25`; `std` enables Tokio
  support.
- `consortium-ipc-doorbell-ipcc` targets STM32 IPCC. Features are `stm32mp21x`,
  `stm32mp23x`, `stm32mp25x`; default is `stm32mp25x`; `std` enables Tokio support.
- `consortium-ipc-doorbell-mu` targets NXP i.MX MU. Features are `imx8mp`, `imx93`,
  `imx95`; default is `imx95`; channels `0..4` are GI lines and `4..8` are RF lines.
- `consortium-ipc-doorbell-gpio` is a single-channel GPIO doorbell. The `no_std` path
  uses a software pending flag and IRQ-side `wake`; the `std` path exposes a Tokio
  `Notify` that external interrupt code must trigger.

The `std` doorbell paths are host-testable with fake MMIO and Tokio `Notify`. Runtime
UIO integrations should bind each doorbell notifier to the Linux UIO IRQ fanout before
receiving.

## Configuration

The authoritative current schema is in `consortium-cfg-common/src/config/`, not in the
older melt-pot example manifests. Use `crates/consortium-cfg/Consortium.example.*.toml`
and `consortium-cfg/src/session.rs` tests as the best references.

Current top-level schema:

- `[profile] chip = "..."`
- `[endpoints]` maps runtime endpoint names to artifact roots.
- `[dbg.defmt]` contains `memory = { base, length }` and `mtu`.
- `[[ipc.channel]]` contains `doorbell`, `pattern`, `party.<name>`, `config.memory`,
  `mtu`, and optional `codec`.
- `[tee]` contains the OP-TEE UUID.
- `[hmi]` contains `engine`, `backend`, optional `debug`, and `build`.
- `[peripheral.<name>]` contains `owner`, `secure`, and optional pin declarations.

`Session::from_toml` parses the manifest, resolves `[profile] chip` through embedded
YAML chip-family databases, and validates doorbell availability, visible parties,
channel bounds, duplicate doorbell channels, SRAM bounds, memory overlap, and peripheral
owners. Embedded database prefixes currently cover STM32MP21/23/25 and MIMX93/95, with
`imx93`/`imx95` accepted as aliases for `mimx93`/`mimx95`.

`consortium-cfg` currently includes DTS/UIO-oriented components and transpilers such as
shared-memory regions to `generic-uio` nodes and Type-A doorbell interrupt enablement.
Keep config changes centered on the schema types first, then validation, then
transpilers.

## Build-Time Configuration

The intended builder flow injects compile-time environment variables such as
`CONSORTIUM_IPC_*`, `CONSORTIUM_DBG_*`, `CONSORTIUM_HMI_*`, and `CONSORTIUM_TEE_UUID`.
Consumers read them with `option_env!`/`env!` and parse values in `const` contexts with
`consortium-cfg-common::env_value`.

This layer is currently mid-migration. `consortium-dbg-defmt::config` uses the current
`CONSORTIUM_DBG_BASE`/`CONSORTIUM_DBG_LENGTH` pattern. Some builder and example code
still refers to older flat `[ipc]`/`[dbg]` tables or removed type names such as
`SharedMemConfig`. Before extending builder behavior, reconcile it with the current
`consortium-cfg-common` schema (`MemoryRegion`, `SharedMemory`, `Hmi`, `Dbg`, `Ipc`).

## Runtime Layers

`consortium-runtime-app` is Linux-only. It exposes remoteproc support, optional UIO-backed
IPC (`ipc-shm`), optional debug (`dbg`), and optional TEE (`tee`). `UioDevice` opens
`/dev/uioN`, maps the shared region, and runs a Tokio IRQ task that wakes registered
doorbell notifiers.

`consortium-runtime-mcu` is `no_std`, re-exports debug and shared-memory IPC pieces
behind features, and provides small helpers such as `static_buf!` for scratch buffers.
Its default feature is `ipc-shm`.

`consortium-runtime-spl` currently exposes small domain/peripheral/role traits for
partitioning-oriented code.

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

`consortium-dbg-defmt` implements defmt-over-shared-memory with `CDBG` descriptors. Its
default features are `host` and `logger`; `decode` adds `defmt-decoder` and implies
`host`. Debug configuration comes from `CONSORTIUM_DBG_*`; plain `cargo build` leaves
the config unconfigured and the logger no-op unless the binary calls the const guard.

Preserve descriptor discipline when changing debug transport behavior. Host-side tests
cover descriptor initialization, ring read/write behavior, decoding fixtures, and
overflow/error flags.

## HMI and Cog/WPE

`consortium-hmi` is Linux-only and currently exposes `KioskApp` and `Bridge`.
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
current workspace. Treat `docs/src/dev/**` as useful design history, but verify it
against current source before making it normative.

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
