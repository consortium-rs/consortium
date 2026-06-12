# AGENTS.md - AI Assistant Guide for Consortium

Consortium is a Rust workspace for heterogeneous multiprocessing systems, especially
SoCs that pair Linux-capable Cortex-A cores with real-time Cortex-M or similar
microcontroller cores. The project provides shared abstractions for inter-processor
communication, trusted execution, configuration, debugging, and build/runtime glue for
platforms such as STM32MP2 and NXP i.MX 9x.

The repository is still early-stage. APIs and crate boundaries are expected to evolve,
but the current direction is clear: keep cross-core contracts explicit, keep `no_std`
support viable for firmware-side code, and use host-side tests wherever hardware is not
required.

## Commands

```bash
# Build the workspace
cargo build

# Run host-side tests
cargo test

# Run tests for one crate
cargo test -p consortium-ipc

# Lint Rust code
cargo clippy

# Format Rust code
cargo fmt

# List project recipes
just

# Run the host-side lint bundle
just lint

# Serve documentation locally
mdbook serve docs/
```

The `justfile` is the command index for routine development. It includes host checks,
TEE checks, TOML formatting/linting through Taplo, spelling checks through `typos`, file
name checks through `ls-lint`, and cross-compilation recipes for A-core, M-core, and
RISC-V targets. Some target recipes are platform-dependent and should be treated as
hardware or SDK workflows rather than generic macOS host checks.

## Platform Notes

- Linux-only or Linux-oriented crates include `consortium-tee` CA-side support,
  `consortium-runtime-app`, `consortium-ipc-transport-memory` with `std`, and the
  Linux path of GPIO or UIO-backed integrations.
- Bare-metal crates and feature paths include `consortium-runtime-mcu`,
  `consortium-ipc-transport-memory` without `std`, and no-std doorbell code.
- TEE TA-side macro and parameter code is intentionally usable without Linux. The CA
  runtime surface remains Linux-gated.
- Prefer targeted crate tests on non-Linux hosts. Full workspace tests can include
  platform-sensitive crates.

## Workspace Shape

| Area | Crates |
| ---- | ------ |
| Core IPC | `consortium-ipc`, `consortium-ipc-macros`, `consortium-ipc-macros-impl` |
| Shared transport | `consortium-ipc-transport-memory` |
| Doorbells | `consortium-ipc-doorbell-hsem`, `consortium-ipc-doorbell-mu`, `consortium-ipc-doorbell-gpio`, `consortium-ipc-doorbell-ipcc` |
| Common types and codecs | `consortium-common` |
| Configuration | `consortium-cfg`, `consortium-cfg-meta` |
| Runtimes and build | `consortium-runtime-mcu`, `consortium-runtime-app`, `consortium-builder`, `consortium-cli` |
| Debug | `consortium-dbg-defmt` |
| TEE | `consortium-tee`, `consortium-tee-macros`, `consortium-tee-macros-impl`, `consortium-tee-test` |
| Utilities | `consortium-ffi`, `consortium-hmi`, `consortium-ort` |

`CLAUDE.md` currently delegates to this file, so this guide is the canonical assistant
orientation document.

## Core Architecture

The IPC layer is organized around a small trait hierarchy:

```text
Doorbell  - signaling primitive for notifying a remote processor
Region    - memory area with optional cache maintenance hooks
Transport - byte transport that owns or uses a Doorbell
Codec     - serialization family implemented as CodecFor<T>
Channel   - typed endpoint built from transport plus codec
```

`consortium-ipc` is `no_std` by default and exposes the central traits and channel
types. `std`, `alloc`, and `futures` are feature-gated. Keep cross-core data structures
portable: the `IpcSafe` marker trait rejects address-space-local constructs such as raw
pointers, references, function pointers, and `usize`/`isize`. Use
`#[derive(IpcSafe)]` from `consortium-ipc-macros` when adding message types; the macro
implementation lives in `consortium-ipc-macros-impl` so it can be tested without going
through the proc-macro wrapper.

`consortium-common` owns common project types and the codec family. Postcard, Prost,
and rkyv support are feature-gated modules under `consortium_common::codec`.

## Shared-Memory IPC

`consortium-ipc-transport-memory` is the unified shared-memory transport crate. It uses
descriptor-backed channel layouts and slot headers over a memory region marked by the
`CIPC` descriptor magic. The same crate serves host/Linux and firmware/bare-metal
directions through features:

- `std` enables Tokio-facing support and Linux-oriented dependencies.
- `alloc` enables allocation without full `std`.
- default features currently include `std`.

The transport test suite includes a host-thread integration test that initializes a heap
backed descriptor, uses a mock host doorbell, sends `ping`, and receives `pong`. This is
the preferred style for validating shared-memory protocol logic without hardware.

## Doorbells

Doorbells provide the wakeup/signaling half of IPC. Existing implementations should be
used as patterns for any new platform:

- `consortium-ipc-doorbell-hsem` targets STM32MP2/STM32MP25 hardware semaphores.
- `consortium-ipc-doorbell-mu` targets NXP i.MX Message Unit hardware.
- `consortium-ipc-doorbell-gpio` is the GPIO-backed Linux-oriented implementation path.
- `consortium-ipc-doorbell-ipcc` exists as the IPCC crate placeholder for STM32MP21 series
which do not have HSEM peripheral.

When adding or changing a doorbell, keep register access safety documented, validate
channel bounds through the local validation type when appropriate, and maintain the
dual-path design where practical: `no_std` firmware-side waiting plus `std`/Tokio host
notification.

## Configuration

- `consortium-cfg` contains configuration, peripheral, pin, session, and validation
  logic for resource partitioning and platform setup.
- `consortium-cfg-meta` contains generated STM32MP metadata and chip-family feature
  flags such as `stm32mp21xxxx`, `stm32mp23xxxx`, and `stm32mp25xxxx`.

Use the `docs/src/dev/rpi/` documentation as the design reference for resource
partitioning concepts. Code lives in the `cfg` crates.

## TEE Model

`consortium-tee` provides the shared TEE parameter model, macro re-exports, and the
Linux-gated CA runtime/error surface. Parameter conversion is modeled through runtime
traits.

Important concepts:

- `TeeParam<C = NoCodec>` converts Rust values to and from `TeeParamSlot`.
- Primitive values map to TEE value slots and do not invoke a codec.
- Serialized values map to memref slots and require a `CodecFor<T>` implementation.
- `#[derive(TeeParam)]` generates serialized parameter support for custom types.
- `#[tee(max_size = N)]` controls the maximum serialized buffer size for derived
  parameters; the default is defined by the derive implementation.
- `tee_service!` and `#[tee_command]` are exported through `consortium-tee-macros`.
- Macro implementation logic lives in `consortium-tee-macros-impl` and is covered by
  trybuild and insta snapshot tests.

When changing TEE macros, update both compile-fail tests and token snapshots as needed.
Prefer testing macro logic in the implementation crate before relying on the proc-macro
facade.

## Debug Logging

`consortium-dbg-defmt` implements defmt-over-shared-memory using a circular descriptor
with `CDBG` magic. Host-side tests cover descriptor initialization, frame read/write,
multi-threaded producer/consumer behavior, and overflow flagging. Preserve this
descriptor discipline when changing debug transport behavior.

## Runtime and Build Layers

`consortium-runtime-mcu` is the firmware-side runtime surface. It is expected to grow
around descriptor initialization, interrupt integration, and an async execution model
for M-core firmware.

`consortium-runtime-app` is the A-core/Linux harness. It owns host-side lifecycle
concerns such as remoteproc integration and application orchestration.

`consortium-builder` is the intended home for cross-compilation and packaging helpers,
including firmware, Linux application, OP-TEE, HMI assets, model artifacts, and installer
bundle generation. Keep build orchestration code out of low-level IPC crates unless it
is genuinely part of their compile-time contract.

## Testing Guidance

- Use `cargo test -p <crate>` for focused host-side validation.
- Use trybuild tests for proc-macro acceptance and rejection behavior.
- Use insta snapshots for macro expansion output where stable generated tokens matter.
- Use heap-backed or mock-doorbell tests for shared-memory protocol behavior that does
  not require real hardware.
- Keep hardware-dependent tests separate from default host checks.
- Do not assume Linux-only crates or no-std target checks will work on every developer
  machine.

## Style and Tooling

- Rust edition is 2024 across the workspace.
- Workspace lints deny undocumented unsafe blocks and unsafe-operation shortcuts.
- Keep unsafe blocks narrow and explain the hardware, memory, or ABI invariant they rely
  on.
- `cargo fmt` handles Rust formatting.
- Taplo configuration uses two-space TOML indentation and a 100-column width.
- `typos.toml` configures spelling checks.
- `.ls-lint.yml` defines naming rules for crates, tests, docs, scripts, board files, and
  simulator files.
- Crate and file names should follow the configured lint rules rather than local taste.

## Documentation Map

- `docs/src/arch/` contains architecture overviews for IPC, TEE, FFI, ORT, and HMI.
- `docs/src/dev/ipc/` contains IPC implementation notes and roadmap material.
- `docs/src/dev/tee/` contains OP-TEE SDK and macro design material.
- `docs/src/dev/rpi/` contains resource partitioning design material; read it as the
  conceptual source for the current `cfg` crates.
- `docs/src/dev/hil/` contains hardware-in-the-loop planning.
- `sim/renode/stm32mp257/` contains the STM32MP257 Renode simulator assets.

When updating docs, use the crate names and APIs present in the workspace.
