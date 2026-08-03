# Consortium

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![host tests](https://github.com/consortium-rs/consortium/actions/workflows/host-tests.yml/badge.svg)](https://github.com/consortium-rs/consortium/actions/workflows/host-tests.yml)
[![linters](https://github.com/consortium-rs/consortium/actions/workflows/linters.yml/badge.svg)](https://github.com/consortium-rs/consortium/actions/workflows/linters.yml)
[![PAC](https://github.com/consortium-rs/consortium/actions/workflows/pac.yml/badge.svg)](https://github.com/consortium-rs/consortium/actions/workflows/pac.yml)
[![website](https://img.shields.io/badge/website-csti.dev-0b7285)](https://csti.dev)
[![book](https://img.shields.io/badge/book-book.csti.dev-0b7285)](https://book.csti.dev)

<!-- Not wired up yet:
[![codecov](https://codecov.io/gh/consortium-rs/consortium/branch/main/graph/badge.svg)](https://codecov.io/gh/consortium-rs/consortium)
[![CodSpeed](https://img.shields.io/badge/codspeed-benchmarks-1f2937)](https://codspeed.io/consortium-rs/consortium)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](https://discord.gg/)
-->

A Rust workspace for heterogeneous multiprocessing (HMP/AMP) systems — SoCs such as
STM32MP2 and i.MX 9 that pair a Linux-capable A-core with Cortex-M, Cortex-R, or RISC-V
real-time cores. Consortium describes the whole system in one `Consortium.toml` manifest
and generates the cross-core glue from it: typed async IPC endpoints, device-tree and
UIO plumbing, firmware bring-up, and a staged deployment tree.

> [!NOTE]
> Early-stage and moving quickly. The API may change significantly between releases.
> Prefer the crate sources and [the book](https://book.csti.dev) over older design notes.

## Objective

Cross-core integration on these SoCs is normally assembled by hand: a device tree edited
in one place, a remoteproc or RPMsg channel in another, a shared-memory layout agreed by
comment, message structs duplicated on both sides and kept in sync by discipline. Every
one of those seams is a place where the two cores can silently disagree.

Consortium's goal is to turn those seams into **explicit contracts that are checked once
and then generated**:

- One manifest owns the memory map, channels, doorbells, peripherals, and endpoints.
  Validation rejects overlapping regions, unavailable doorbells, and unowned peripherals
  before anything is built.
- Message types are ordinary Rust types shared by both cores, with `IpcSafe` rejecting
  anything whose meaning is address-space-local (pointers, references, `usize`).
- Bring-up that firmware used to hand-write — logger reset, time driver, transport
  connect, peripheral singletons — is emitted into a generated `init()`.
- The same contracts work from `no_std` firmware and from a Linux application, so the
  real-time side is not a second-class citizen of the toolchain.

## Components

- **Core** — `consortium-runtime-app` (Linux, UIO/remoteproc), `consortium-runtime-mcu`
  (`no_std`), the entry-point macros and `Context` aggregate, `consortium-codec`
  (postcard/prost/rkyv), the `consortium-log` facade, `consortium-dbg`
  defmt-over-shared-memory, and the `csti` CLI plus `consortium-builder` pipeline that
  compiles, generates, and stages a whole system from `Consortium.toml`.
- **IPC** — `consortium-ipc` typed channels, transceivers, transport traits, and the
  `Connect` rendezvous, carried over a shared-memory transport
  (`consortium-shared-memory`, `consortium-ipc-transport-memory`) or a COBS/CRC-framed
  UART transport, with hardware doorbells for STM32 HSEM and IPCC, i.MX MU, and GPIO.
- **TEE** — `consortium-tee` OP-TEE contracts: `TeeParam` conversion between Rust values
  and TEE value/memref slots, codec-backed serialized parameters, and derive macros
  shared by the client-application and trusted-application sides.
- **HMI** — `consortium-hmi`, a backend-neutral command bridge between the UI and the
  application, adapted to Cog/WPE WebKit by `consortium-hmi-webkit` (over the `cogcore`
  bindings) or to the native PocketJS runtime by `consortium-hmi-pocket`.
- **PAC** — `consortium-pac-imx9` and `consortium-pac-stm32mp2`, generated `no_std`
  peripheral access crates produced from vendor material by the `just pac-*` recipes and
  validated in CI.
- **HAL** — `consortium-hal` traits plus the `consortium-hal-imx9` and
  `consortium-hal-stm32mp2` chip HALs: peripheral singletons handed out once by `init()`,
  and CAN, DMA, GPIO, I2C, SPI, timer, and interrupt-buffered UART drivers that take
  ownership of a peripheral rather than an MMIO address.
- **Cfg** — `consortium-cfg-common` and `consortium-cfg` own the `Consortium.toml`
  schema, the chip/memory databases in `consortium-data`, validation, lowering, and the
  DTS/UIO/Kconfig/remoteproc transpilers that turn a manifest into buildable output.
- **ORT** _(stub)_ — `consortium-ort`, reserved for ONNX Runtime inference on the A-core
  alongside real-time IPC work. The dependency is pinned and proven to build; the crate
  exports nothing yet.
- **FFI** _(stub)_ — `consortium-ffi`, reserved for an `extern "C"` surface so a non-Rust
  host can drive a Consortium runtime. Exports nothing yet.

## Getting started

```bash
just setup
```

```bash
just check
```

```bash
just test ipc host
```

Running `just` with no arguments lists the recipes. The end-to-end flow is `csti build`,
which runs the `Consortium.toml` pipeline and assembles a `dist/` staging tree; see
`examples/melt-pot/` for worked i.MX95 and STM32MP25 systems, and
[the book](https://book.csti.dev) for the manifest reference.

On macOS, route Rust builds and tests through the Linux container recipes (for example
`just linux 'just check-ipc-a64'`); Linux-only crates such as `consortium-runtime-app`
and `cogcore` cannot compile natively there.

## Contributing

Contributions are welcome — see [CONTRIBUTING.md](CONTRIBUTING.md) for development
setup, the host-versus-container execution policy, review expectations, and the policy
on AI coding assistants (allowed and encouraged, with human review required and an
`Assisted-by:` tag; only humans sign off).

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). To report a
vulnerability, follow [SECURITY.md](SECURITY.md) rather than opening a public issue.

<!-- ## Sponsor

A sponsorship page is coming soon.
-->

## License

Consortium is licensed under the [Apache License, Version 2.0](LICENSE).

Three crates redistribute vendor material and carry compound SPDX expressions instead:

| Crate                    | SPDX                                  |
| ------------------------ | ------------------------------------- |
| `consortium-data`        | `Apache-2.0 AND BSD-3-Clause AND MIT` |
| `consortium-runtime-sys` | `Apache-2.0 AND BSD-3-Clause`         |
| `cogcore-sys`            | `Apache-2.0 AND MIT`                  |

Vendor device trees under `crates/consortium-data/database/dts/` must be dual-licensed;
GPL-only files cannot ship (see that directory's `README.md`). Unless you state
otherwise, any contribution you intentionally submit for inclusion shall be licensed as
above, with no additional terms.
