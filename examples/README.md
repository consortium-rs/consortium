# Consortium examples

## Utilities

- [`utils/defmt-host`](utils/defmt-host/) maps a CDBG UIO ring, drains all defmt
  messages currently available, and exits without polling.

## melt-pot

A full-stack demo: a real-time coprocessor streams simulated temperature
readings over shared-memory IPC to a Linux application, which runs an OP-TEE
threshold check and serves the verdict to a Cog/WebKit HMI. It ships for two
platforms, each using a single coprocessor:

| Platform             | AP (Linux) | Coprocessor firmware | IPC doorbell | IPC region          |
| -------------------- | ---------- | -------------------- | ------------ | ------------------- |
| `melt-pot/stm32mp25` | CA35       | CM33                 | HSEM         | SRAM2 (0x0006_0000) |
| `melt-pot/imx95`     | CA55       | CM7                  | MU7          | OCRAM (0x0048_0000) |

The STM32MP257 CM0+ and the i.MX95 M33 system-manager cores are intentionally
not used.

### Layout

```
melt-pot/
  Consortium.toml         # platform index
  shared/                 # IPC message types (SensorReading, Command)
  <platform>/
    Consortium.toml       # the manifest the build pipeline consumes
    app/                  # CA Linux application (HMI + TEE client)
    mcu/                  # coprocessor firmware
    ta/                   # OP-TEE trusted application
```

### Building

The per-endpoint crates (`app`, `mcu`, `ta`) are built by the `consortium`
build pipeline, not plain `cargo build`. The pipeline reads `Consortium.toml`,
resolves the chip database, and writes a generated `src/consortium.gen.rs` into
each crate (IPC endpoints, MPU/NVIC bring-up on the firmware side; UIO bring-up,
IPC transceivers, and the defmt console on the app side) that the crate
`include!`s. Those generated modules embed absolute build-host paths, so they
are git-ignored and regenerated on each build. For this reason the per-endpoint
crates are excluded from the workspace and build into their own per-crate
`target/`; only `shared/` is a workspace member.

```sh
consortium build --manifest examples/melt-pot/stm32mp25/Consortium.toml
consortium build --manifest examples/melt-pot/imx95/Consortium.toml
```

Building the HMI requires the Cog/WPE/WebKit development packages to be present
and version-matched with the vendored `cogcore` bindings.
