# consortium-runtime-mcu

The controller-core firmware runtime: entry macro, remoteproc resource table, MPU
setup, and feature-gated re-exports of the IPC and debug stacks.

`no_std`. This is the firmware half of a Consortium system;
[`consortium-runtime-app`](../consortium-runtime-app) is the Linux half.

## Deliberately thin

Most of what firmware needs is **generated** by `consortium-cfg` into
`consortium.gen.rs`. What remains here is the handful of things generated code cannot
supply: an entry point, the ELF section Linux remoteproc reads, cache configuration,
and one dependency edge that pulls in the transports.

| Module      | Role                                                       |
| ----------- | ---------------------------------------------------------- |
| `rsc_table` | The `.resource_table` section a Linux host parses at boot. |
| `mpu`       | Mark the shared carveout non-cacheable on a cached core.   |
| `log`       | Logging facade re-export for generated code.               |

## Entry macro

```rust,ignore
#[consortium_runtime_mcu::main]
async fn main(context: Context, spawner: Spawner) {
    let telemetry = context.ipc_shm.telemetry;
    // ...
}

// Optional: report a bring-up failure yourself.
#[consortium_runtime_mcu::fail]
fn on_fail(err: ConsortiumInitError) {
    defmt::error!("init failed: {}", err);
}
```

The macro stacks `embassy_executor::main`, resolved through the **user** crate's own
dependency — so firmware still needs `embassy-executor` with `executor-thread` and
`platform-cortex-m` — then calls the generated `init()` and binds its result as
`context`.

Without a `#[fail]` handler, a bring-up failure parks the core with `wfi`. The dispatch
is a **glob-import shadow**: `#[main]` glob-imports a fallback from `__rt`, and a
module-scope `#[fail]` item shadows it. So `#[main]`, `#[fail]`, `init`, and `Context`
must all share the module the generated file is `include!`d into.

## What the generated `init()` already does

Firmware used to hand-write all of this. It is now sealed:

1. **resets the CDBG ring** (`ConsortiumLogger::init`) when the core has a
   `[dbg.<core>]` logger — debug SRAM survives a warm reset, so without this the logger
   would resume into a stale ring with cursors pointing at a previous run's frames;
2. **brings up the chip HAL's Embassy time driver** (STM32MP2 TIM2, i.MX9 LPIT1) and
   emits the matching timer vector and `defmt::timestamp!`. These are chip-specific, so
   they cannot live in the chip-agnostic `#[main]` macro. Disable with
   `[profile] time_driver = false` when firmware owns its own time base — the defaults
   are reference-board values.
3. **constructs, `connect`s, and splits** each IPC transport into a transceiver at
   `context.ipc_shm.<name>`;
4. **hands out `context.peripherals`** on cores owning `[peripheral.*]`.

The two-phase `IpcMemoryEndpoints::new` / `connect` API stays available for non-macro
callers.

## Interrupts are still yours

Neither this crate nor the doorbell or HAL crates install `#[interrupt]` handlers. The
application owns the vector table and forwards each relevant IRQ into the exported wake
path:

| Source              | Forward to                             |
| ------------------- | -------------------------------------- |
| HSEM                | `hsem::notify()`                       |
| IPCC1_RX / IPCC2_RX | `ipcc::notify_rx_occupied_interrupt()` |
| MU line `N`         | `mu::notify::<N>()`                    |
| Buffered UART       | `uart::on_buffered_interrupt::<P>()`   |

Enable the chip **HAL's** `rt` feature (`consortium-hal-*/rt`) so the `#[interrupt]`
macro and PAC vector table are available to that code. See the melt-pot
`mcu/src/main.rs` files for worked examples.

## Scratch buffers

Every channel and buffered-UART constructor wants a `&'static mut [u8]`.
`static_buf!` declares a `StaticCell` whose `.init(..)` yields one, keeping the raw
`static mut` out of application code:

```rust,ignore
use consortium_runtime_mcu::static_buf;

static_buf!(RX_SCRATCH, 512);
let buf = RX_SCRATCH.init([0; 512]);
```

## MPU and cache coherency

On a cached core (Cortex-M7), a shared-memory window must be accessed non-cacheably, or
the two sides observe different bytes unless every descriptor poll is bracketed with
clean/invalidate. `mpu` programs one region as Normal, shareable, non-cacheable and
enables the MPU with the default map kept as a privileged background region.

The register interface differs by architecture and is selected by the active target:

| Architecture              | Block         | Attributes                            |
| ------------------------- | ------------- | ------------------------------------- |
| ARMv7-M (`thumbv7em`, M7) | `RBAR`/`RASR` | power-of-two size, inline `TEX/C/B/S` |
| ARMv8-M (`thumbv8m`, M33) | `RBAR`/`RLAR` | base/limit, attributes via `MAIR0`    |
| ARMv6-M (`thumbv6m`, M0+) | —             | no data cache; nothing to disable     |

On targets without a cache to manage — ARMv6-M, host builds, RISC-V — the module is
empty.

## Resource table

`rsc_table` re-exports the raw `.resource_table` structures generated by
[`consortium-runtime-sys`](../consortium-runtime-sys) and adds `SharedResourceTable`,
which emits an RPMsg-vdev-plus-trace table into the firmware's `.resource_table`
section.

The distinction matters: the bindgen `ResourceTable` and `FwRscVdev` types model the
kernel **parser** view, and their trailing arrays are flexible array members holding no
data — so they cannot be statically initialized. To _emit_ a table you need a concrete
struct with a fixed `offset[N]` and the entries laid out inline. That is what
`SharedResourceTable` is.

## Features

Defaults are `ipc-shm` and `macros`.

| Feature       | Effect                                                                                  |
| ------------- | --------------------------------------------------------------------------------------- |
| `ipc-shm`     | Shared-memory IPC (`ipc` is an alias).                                                  |
| `ipc-uart`    | The COBS/CRC-framed UART transport, instead or as well.                                 |
| `macros`      | The `#[main]` / `#[fail]` entry macros.                                                 |
| `dbg`         | Re-export `consortium-dbg` for the CDBG logger.                                         |
| `defmt`       | Route generated-code logging to `defmt`.                                                |
| chip features | `mimx93`/`mimx95`, `stm32mp21x`/`23x`/`25x` — forwarded to the shared-memory transport. |

**Enabling `defmt` here is not sufficient on its own.** `defmt`'s macros expand to
literal `defmt::` paths, so the firmware crate that includes the generated module must
also depend on `defmt` directly. The fallback `#[fail]` handler is silent without it,
by design — rich reporting belongs in a user `#[fail]` handler, which owns the error
type.

`stm32mp2xx` is a compatibility alias for `stm32mp25x`; new code should name a chip
family directly so STM32MP21 can select IPCC without HSEM.

## Testing

```bash
cargo test -p consortium-runtime-mcu
just check-ipc-m8    # thumbv8m.main-none-eabihf (Cortex-M33)
just check-ipc-m7    # thumbv7em-none-eabihf     (Cortex-M7)
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
