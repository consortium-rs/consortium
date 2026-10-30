# Consortium

Consortium is a Rust framework and build system for heterogeneous SoCs: systems where
Linux-capable application cores work beside real-time Cortex-M, Cortex-R, or RISC-V
cores. It turns the contracts between those processors into explicit, checked inputs
instead of scattered board-specific glue.

A Consortium project starts with three things:

- a `Consortium.toml` manifest describing processors, memory, IPC, peripherals, and
  optional TEE, debug, and HMI services;
- a shared Rust crate containing the types that cross processor boundaries; and
- one endpoint crate for each participating runtime.

From those inputs, `csti build` validates the complete system, generates endpoint
initialization code, builds its artifacts, and assembles a local `dist/` tree for the
target's packaging flow.

```text
Consortium.toml + shared types + endpoint crates
                        |
                 validate and lower
                        |
       generated resources, DTS, UIO, and build inputs
                        |
             Linux app + firmware + optional services
                        |
                  local dist/ tree
```

## What Consortium provides

- **Typed IPC.** Allocation-free channels combine a transport, codec, direction, and
  message type. Shared-memory, UART, and FDCAN transports use the same core contracts.
- **Cross-core configuration.** Chip databases validate memory placement, doorbell
  channels, visible cores, peripheral ownership, and generated Linux integration.
- **Generated bring-up.** A generated `init(Config)` returns the endpoint's owned
  `Peripherals` and `Channels`, replacing hand-written transport, debug-ring, timer,
  interrupt-vector, and peripheral setup.
- **Target integration.** The builder emits remoteproc-ready firmware, device-tree and
  kernel inputs, a systemd unit, and a BitBake-ready staging tree.
- **Optional services.** OP-TEE contracts, shared-memory `defmt` logging, and Cog or
  Pocket HMI packaging integrate with the same project model.

## Project status

Consortium is early-stage and its APIs are evolving. The i.MX 9 and STM32MP2 families
are the active reference platforms. Prefer this user guide, current crate sources, and
the melt-pot examples over older development notes when they disagree.

New readers should continue with [Getting Started](guide/getting-started.md). Readers
evaluating the design can begin with [System Architecture](guide/architecture.md).
