# consortium-runtime-app

The A-core Linux runtime: remoteproc control, UIO-backed IPC, host-side debug
consoles, and the `#[main]` entry macro.

This is the Linux half of a Consortium system. Where
[`consortium-runtime-mcu`](../consortium-runtime-mcu) gives firmware its entry point
and shared-memory glue, this crate gives the application the same — plus the pieces
only Linux has: `/sys/class/remoteproc` for booting a coprocessor, `/dev/uioN` for
reaching shared memory and doorbells from userspace, and a `tracing` subscriber to put
the resulting logs somewhere.

## Modules

| Module                               | Role                                         |
| ------------------------------------ | -------------------------------------------- |
| `irq`                                | The portable UIO interrupt dispatch loop.    |
| `uio`                                | `/dev/uioN` open, mmap, device lifecycle.    |
| `remoteproc`                         | Load firmware, start/stop a coprocessor.     |
| `dbg_console`                        | Drain and decode coprocessor CDBG rings.     |
| `ipc`, `ipc_transport`, `dbg`, `tee` | Feature-gated re-exports for generated code. |

## Why `irq` is not under the Linux gate

Almost everything here is `cfg(target_os = "linux")`. `irq` is `cfg(unix)`, and that is
deliberate: the `[fd readiness → user-space fan-out]` slice of the UIO protocol
contains no Linux-specific API, so gating it at `unix` lets its socketpair-backed tests
run on a macOS developer machine. The real `/dev/uioN` mapping and device lifecycle
stay Linux-only in `uio`.

This is a general pattern worth copying: the part of a Linux-shaped protocol that is
just fd plumbing is usually testable everywhere, if it is separated out.

## The UIO interrupt handshake

Once `/dev/uioN` is open, **the kernel arms the device interrupt and user space owns
the mask/re-arm handshake** from then on. `irq_loop` runs one iteration per M→A
interrupt, in an order that is the whole design:

1. **wait** for the fd to become readable;
2. **drain** the 4-byte interrupt count so it is not readable again;
3. **fan out** to every registered doorbell service;
4. **re-arm** the interrupt with a 4-byte write.

Two orderings carry the correctness:

- **Step 3 before step 4.** A masking doorbell (IPCC, MU) masks its asserted lines
  during the fan-out, so a level-sensitive line that is still asserted does not
  immediately re-fire the instant it is unmasked — which would spin the loop.
- **Re-arm is the last act, never a first enable.** The kernel already armed the line
  at open; a redundant enable can unbalance a UIO driver's enable/disable accounting
  and leave the interrupt wedged off. So this loop never writes to the fd before its
  first wait.

Bind a doorbell according to which it is:

| Doorbell kind      | Bind with       | Because                        |
| ------------------ | --------------- | ------------------------------ |
| plain (GPIO, HSEM) | `bind_notifier` | edge-driven; nothing to mask   |
| masking (IPCC, MU) | `bind_service`  | must mask asserted lines first |

Windows are mapped by kind too: `mmap` for register and doorbell windows,
`mmap_zeroed` for data windows. The zeroing variant exists because the AP owns first
touch of a shared carveout — it is `no-map` reserved memory that survives a warm
reboot, so a previous run's descriptor and slot bytes must not read as valid.

The loop holds no per-channel state. It does not know which line fired; wakeups ride
entirely on each doorbell's shared `tokio::sync::Notify`, which is the same object its
`wait()` future subscribes to.

## Entry macro

```rust,ignore
#[consortium_runtime_app::main]
async fn main(context: Context) -> Result<(), Box<dyn std::error::Error>> {
    context.ipc_shm.telemetry.send(&Command::Start).await?;
    Ok(())
}

// Optional: report a bring-up failure yourself.
#[consortium_runtime_app::fail]
fn on_fail(err: ConsortiumInitError) {
    eprintln!("init failed: {err}");
}
```

The macro stacks `tokio::main` — resolved through the **user** crate's own dependency,
so the application still needs `tokio` with `macros` — then:

1. **installs a `tracing` subscriber before the generated `init()` runs.** This matters:
   without it, the UIO and IPC bring-up traces and the `#[fail]` handler's diagnostics
   would be dropped for want of a subscriber, which is exactly when you want them.
   Installation is idempotent (`try_init`), so an application that sets up its own
   subscriber does not panic. `RUST_LOG` sets the filter, defaulting to `info`.
2. calls the generated `init()` and binds its result as `context`.

Without a `#[fail]` handler, a bring-up failure logs and exits non-zero. The dispatch
is a **glob-import shadow**: `#[main]` glob-imports a fallback from `__rt`, and a
module-scope `#[fail]` item shadows it. So `#[main]`, `#[fail]`, `init`, and `Context`
must all share the module the generated file is `include!`d into.

## remoteproc

`RemoteProc` is a thin typed wrapper over `/sys/class/remoteproc/remoteprocN` — write
the firmware filename, write `start`, read back the state.

```rust,ignore
let rproc = RemoteProc::new(0);
rproc.set_firmware("consortium-mcu.elf")?;
rproc.start()?;
```

Points worth knowing:

- **Requires privilege** — these are root-owned sysfs files.
- **`set_firmware` takes a filename, not a path**; the kernel resolves it against its
  own search path (typically `/lib/firmware`).
- **The firmware must be the post-processed one.** remoteproc expects the vector table
  in a section named `.isr_vectors`, so `csti build` renames `.vector_table` for it.
- **State is read, not watched.** There is no notification when a coprocessor crashes;
  pair `state()` with the debug ring's panic flag to tell a crash from a quiet
  coprocessor.
- **Bring-up need not be raced.** The shared-memory descriptor magic is a readiness
  gate, so a coprocessor that boots late finds the descriptor already published rather
  than missing a doorbell.

## Debug consoles

`[dbg] mode` picks how the AP consumes coprocessor rings:

| Mode        | Function | Behaviour                                                                                            |
| ----------- | -------- | ---------------------------------------------------------------------------------------------------- |
| `auto`      | `spawn`  | One task per ring: drain, decode against the firmware ELF, forward each frame to `tracing`. Turnkey. |
| `interface` | `open`   | Return a handle per ring; the integrator drives reading and decoding.                                |

A debug ring has **no doorbell of its own**, so the auto task polls on a fixed interval
rather than waiting on an interrupt.

## Features

Defaults are `dbg`, `ipc-shm`, and `macros`.

| Feature             | Effect                                                                                                                             |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `ipc`               | Re-export `consortium-ipc`.                                                                                                        |
| `ipc-shm`           | UIO-backed shared-memory IPC; implies `ipc`.                                                                                       |
| `dbg`               | Debug-ring decoding and `dbg_console`.                                                                                             |
| `macros`            | The `#[main]` / `#[fail]` entry macros.                                                                                            |
| `tee`               | Re-export `consortium-tee`.                                                                                                        |
| `hmi`, `ort`, `ffi` | Re-export the HMI bridge, ONNX Runtime, and C ABI crates.                                                                          |
| chip features       | `mimx93`/`mimx95`, `stm32mp21x`/`23x`/`25x` — forwarded to the shared-memory transport so it can report doorbell window addresses. |

`stm32mp2xx` is a compatibility alias for `stm32mp25x`; new code should name a chip
family directly so STM32MP21 can select IPCC without HSEM.

## Testing

```bash
cargo test -p consortium-runtime-app
cargo clippy -p consortium-runtime-app --no-default-features \
    --features mimx9x,stm32mp25x --all-targets -- -D warnings
```

The `irq` module's tests use socketpairs in place of `/dev/uioN`, so the interrupt
protocol is covered on any Unix host. Everything touching a real device, remoteproc, or
OP-TEE needs a provisioned Linux target.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
