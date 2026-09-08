# Consortium examples

## Utilities

- [`utils/defmt-host`](utils/defmt-host/) maps a CDBG UIO ring, drains all defmt
  messages currently available, and exits without polling.

## melt-pot

A full-stack demo: a real-time coprocessor streams simulated temperature
readings over shared-memory IPC to a Linux application, which runs an OP-TEE
threshold check and serves the verdict to a Cog/WebKit HMI. It ships for three
platforms, each using a single coprocessor:

| Platform             | AP (Linux) | Coprocessor firmware | IPC doorbell | IPC region          |
| -------------------- | ---------- | -------------------- | ------------ | ------------------- |
| `melt-pot/stm32mp25` | CA35       | CM33                 | HSEM         | SRAM2 (0x0006_0000) |
| `melt-pot/imx95`     | CA55       | CM7                  | MU7          | OCRAM (0x0048_0000) |
| `melt-pot/imx93`     | CA55       | CM33                 | MU2          | OCRAM (0x0048_0000) |

The STM32MP257 CM0+ and the i.MX95 M33 system-manager cores are intentionally
not used.

`melt-pot/imx93` is the i.MX93 counterpart of `melt-pot/imx95`, and the pair is
the worked example of two silicon differences the manifest does _not_ spell out:

- Its CM33 has no L1 data cache, so `config.cache` is rejected on its channels.
- Its MU windows are **mirrored**. The CM33 owns side A and Linux side B, the
  reverse of i.MX95, so the generated bring-up builds `MuDoorbell<Primary, 2>`
  from `MU2_MUA` and installs `#[interrupt] fn MU2_A()` even though the channel
  declares `secondary = "cm33"`. The register window is lowered from the chip
  database, separately from the primary/secondary descriptor role.

It signals over MU2 rather than MU1 because the board DTS already wires MU1 to
the CM33 remoteproc node as its RPMsg mailbox. Its firmware runs from TCM rather
than a DDR carveout — see `[remoteproc.cm33]` in the mimx93 chip database.

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
