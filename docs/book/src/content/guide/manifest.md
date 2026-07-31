# Project Manifest

`Consortium.toml` is the source of truth for the complete system. Paths are resolved
relative to the manifest, while hardware names and address ranges are checked against
the selected chip database.

## A minimal shared-memory system

This i.MX95 fragment follows the current melt-pot reference layout:

```toml
[profile]
chip        = "mimx9596xvt"
devkit      = "imx95-19x19-evk"
shared      = "../shared"
deploy_root = "/opt/consortium"

[endpoints]
linux = "./app"
cm7   = "./mcu"

[[ipc.channel]]
name          = "sensor"
doorbell      = "mu7"
pattern       = "memory"
primary       = "linux"
secondary     = "cm7"
party.linux   = { chan = 6, type = "Command" }
party.cm7     = { chan = 7, type = "SensorReading" }
config.memory = { base = 0x00480000, length = 0x4000 }
mtu           = 128
```

The `type` under each `party` is the message **sent by that party**. Generated Linux
code therefore exposes `sensor` as a transceiver that sends `Command` and receives
`SensorReading`; the CM7 sees the inverse. Its `chan` is the doorbell line used for that
party's outgoing direction.

`primary` and `secondary` select the shared-memory protocol roles. Current systems use
the Linux/A-core as primary and the firmware core as secondary.

## Sections

### `[profile]`

| Field           | Required | Meaning                                                                                    |
| --------------- | -------- | ------------------------------------------------------------------------------------------ |
| `chip`          | yes      | Full part number resolved through the chip database                                        |
| `shared`        | yes      | Crate containing IPC and other cross-boundary types                                        |
| `deploy_root`   | yes      | On-target installation prefix used when generating paths; the builder does not write there |
| `devkit`        | no       | Full board DTS file stem; falls back to the generic SoC tree                               |
| `sysroot`       | no       | Linux target sysroot; overrides SDK environment variables                                  |
| `kernel_recipe` | no       | Yocto kernel recipe name used to generate a `.bbappend`                                    |
| `time_driver`   | no       | Generate the firmware Embassy time driver; defaults to `true`                              |

The generated time driver uses the reference timer for the family: TIM2 on STM32MP2 or
LPIT1 on i.MX9. Set `time_driver = false` when firmware owns another time base.

### `[endpoints]`

This table maps runtime endpoint names to crate roots. Names must be visible on the
selected chip, and every party used by an IPC channel must have a matching endpoint.
The conventional application name is `linux`; controller names include `cm33` and
`cm7` according to the chip.

### `[[ipc.channel]]`

Each entry describes one bidirectional typed channel. The current manifest pattern is
`memory`; the UART transport is available as a crate API but is not lowered from this
table yet.

| Field                  | Meaning                                                      |
| ---------------------- | ------------------------------------------------------------ |
| `name`                 | Rust field name under `context.ipc_shm`                      |
| `doorbell`             | Chip database doorbell, such as `ipcc1` or `mu7`             |
| `primary`, `secondary` | Endpoints participating in the descriptor handshake          |
| `party.<name>`         | This party's outgoing doorbell channel and Rust message type |
| `config.memory`        | Physical shared region containing the descriptor and slots   |
| `mtu`                  | Maximum encoded payload size in bytes                        |
| `codec`                | `postcard` by default; `prost` and `rkyv` are also supported |

Message types must exist in the `[profile].shared` crate and satisfy the selected
codec. The region must be large enough for the descriptor and both directional slots.

### `[dbg]` and `[dbg.<core>]`

The application processor always hosts debug decoding, so only coprocessor rings are
declared:

```toml
[dbg]
mode = "auto"

[dbg.cm7]
memory = { base = 0x00484000, length = 0x2000 }
mtu    = 64
```

`auto` generates background readers that forward decoded `defmt` frames to `tracing`.
`interface` exposes the readers for application-managed polling and decoding.

### `[dts]`

```toml
[dts]
mode      = "merge"
linux     = "/path/to/linux"
configfs  = false
```

`merge` is the default and emits a full device tree with reserved memory, UIO nodes,
and remoteproc fixups. `overlay` emits a `.dtso`; it cannot remove existing remoteproc
mailbox properties, so use merge mode for those fixups. `linux` points to a kernel tree
whose `dt-bindings` headers are needed to compile the generated source.

### `[peripheral.<name>]`

```toml
[peripheral.lpi2c1]
owner     = "cm7"
secure    = false
clock     = 24000000
frequency = 100000
pin.sda   = "PB0"
pin.scl   = "PB1"
```

Controller-owned peripherals become typed fields under `context.peripherals`. The
current generated singleton path supports non-secure peripherals; `secure = true` is
rejected. Required clocks, frequencies, pins, and interrupt metadata depend on the
driver and chip database.

### Optional `[tee]` and `[hmi]`

`[tee]` contains the OP-TEE trusted application's UUID. `[hmi]` selects `cog` or
`pocket`, its display backend, source directory, and build output. Pocket additionally
requires an `app` entry name; it supports `wayland` and `headless` today, not `drm`.
The logical viewport defaults to 480 × 272 and can be set under `[hmi.logical]`.

## Validation

Manifest parsing does more than syntax checking. A session validates, among other
constraints:

- chip-visible processors, peripherals, and doorbells;
- doorbell channel bounds and duplicate use;
- shared and debug region bounds, overlap, and MPU alignment;
- endpoint presence and peripheral ownership; and
- HMI engine/backend combinations.

Treat validation failures as contract errors. Fix the manifest or chip data instead of
bypassing the generated types with raw addresses.
