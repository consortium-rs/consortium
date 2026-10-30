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
| `melt-pot/stm32mp25` | CA35       | CM33                 | IPCC1        | SRAM2 (0x0006_4000) |
| `melt-pot/imx95`     | CA55       | CM7                  | MU7          | OCRAM (0x0048_0000) |
| `melt-pot/imx93`     | CA55       | CM33                 | MU1          | OCRAM (0x0048_0000) |

The STM32MP257 CM0+ and the i.MX95 M33 system-manager cores are intentionally
not used.

`melt-pot/imx93` is the i.MX93 counterpart of `melt-pot/imx95`. Two silicon
differences show up only through the chip database, not the manifest:

- Its CM33 has no L1 data cache, so `config.cache` is rejected on its channels.
- Its firmware runs from TCM rather than a DDR carveout — see `[remoteproc.cm33]`
  in the mimx93 chip database.

On both i.MX parts Linux drives the MU's A window and the M-core its B window, so
the generated CM33 bring-up builds the MU1 doorbell from `MU1_MUB` and installs
the `MU1_B` vector. The board DTS wires MU1 to the CM33 remoteproc node as its
RPMsg mailbox; the generated DTS deletes those `mboxes` so the kernel never
drives it alongside the IPC channel.

### Layout

```
melt-pot/
  shared/                 # IPC message types (SensorReading, Command)
  <platform>/
    Consortium.toml       # the manifest the build pipeline consumes
    app/                  # CA Linux application (HMI + TEE client)
    mcu/                  # coprocessor firmware
    ta/                   # OP-TEE trusted application
    website/              # Cog/WPE HMI frontend
```

### Building

The per-endpoint crates (`app`, `mcu`, `ta`) are built by the `csti` pipeline,
not plain `cargo build`. The pipeline reads `Consortium.toml`, resolves the chip
database, and writes a generated `src/consortium.gen.rs` into each crate (IPC
endpoints, interrupt vectors, MPU/NVIC and time-driver bring-up on the firmware
side; UIO bring-up, IPC transceivers, and the defmt console on the app side) that
the crate `include!`s, and injects the build-time environment that code reads.
The generated modules are committed, with build-host paths expressed relative to
`CSTI_MCU_PATH` so they do not differ between machines. The per-endpoint crates
are excluded from the root workspace and build into their own per-crate
`target/`; only `shared/` is a workspace member.

```sh
csti build --linux <KERNEL_TREE> --manifest examples/melt-pot/stm32mp25/Consortium.toml
csti build --linux <KERNEL_TREE> --manifest examples/melt-pot/imx95/Consortium.toml
csti build --linux <KERNEL_TREE> --manifest examples/melt-pot/imx93/Consortium.toml
```

`csti check` type-checks the same endpoints without linking or staging. On macOS,
run either inside the Linux container (`just linux-full '...'`, kernel tree at
`/root/linux`).

Building the HMI requires the Cog/WPE/WebKit development packages to be present
and version-matched with the vendored `cogcore` bindings.
