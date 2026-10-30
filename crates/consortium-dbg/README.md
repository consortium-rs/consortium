# consortium-dbg

`defmt` logging over shared memory: the `CDBG` ring, its firmware-side
`global_logger`, its host-side reader and decoder, and an optional panic handler.

## The problem

A coprocessor on a Consortium SoC usually has no UART of its own to log through,
or has one already spoken for. But it does share memory with a Linux-capable core
that has a filesystem, a clock, and somewhere to put logs. So the debug path runs
the same way as everything else in this workspace: firmware writes into a shared
window, Linux reads it.

`defmt` is the right encoder for the firmware end. Format strings and type
metadata stay in the ELF and never reach the wire, so a log line costs a few bytes
and no formatting code — which is what makes logging affordable on a Cortex-M at
all. The cost is that the bytes are meaningless without the ELF they came from, so
the host side needs a decoder and the matching binary.

## Architecture

```text
 firmware (no_std)                        Linux / host
 ────────────────────                     ─────────────────────
 defmt::info!(..)
   └─ logger::ConsortiumLogger            host::HostRing
        (global_logger; rzcobs-encodes      (validate CDBG, drain frames)
         one frame, wraps it in a             └─ host::DefmtSession
         packet, appends to the ring)             (decode against the ELF)
                   │                                      │
                   └────────  CDBG shared window  ────────┘
                         descriptor + circular buffer
 panic::(handler)  ─ latches Flags::Panic in the descriptor
```

| Module       | Role                                                                  |
| ------------ | --------------------------------------------------------------------- |
| `config`     | Compile-time region base/length from `CONSORTIUM_DBG_*`.              |
| `descriptor` | The `CDBG` descriptor and the single-writer/single-reader ring rules. |
| `packet`     | Per-frame envelope inside the ring.                                   |
| `logger`     | Firmware `#[global_logger]`; encodes and appends.                     |
| `host`       | Host-side validation, draining, and `defmt` decoding.                 |
| `panic`      | Optional `#[panic_handler]` that records a crash in the ring.         |

The descriptor header (`CDBG` magic + version) comes from
`consortium-shared-memory`, the same header `consortium-ipc-transport-memory` uses
for `CIPC` — so a misconfigured base address produces a clear "wrong structure"
error rather than garbage.

## Descriptor discipline

The ring has exactly one writer (firmware) and one reader (host), and **there is no
lock**. What keeps it correct is that each side only mutates the fields it owns:

| Field                            | Owner             |
| -------------------------------- | ----------------- |
| `write_ptr`, `capacity`, `flags` | writer (firmware) |
| `read_ptr`                       | reader (host)     |

`write_ptr == read_ptr` means empty. Preserve that split when changing anything
here — it is the entire synchronization story.

## Frames, and why they are staged

The ring is a byte buffer, but a reader must recover _frame_ boundaries: an encoded
`defmt` frame is not self-delimiting, and a host attaching partway through a
firmware run starts reading at an arbitrary offset. So each frame is wrapped in a
packet header:

```text
[0..2]  magic : u16  - 0xC0DE, deliberately not ASCII
[2]     flag  : u8   - FLAG_VALID | FLAG_CONSUMED
[3]     len   : u8   - payload bytes that follow
[4..]   payload      - one rzcobs-encoded defmt frame
```

The logger stages one complete frame before appending it, for two reasons: frames
must be rzcobs-encoded before storage (so the bytes `defmt` hands over are not the
bytes to store), and `len` sits in a header that precedes the payload (so the length
must be known first).

Consequences worth knowing:

- **`len` is a `u8`, so a frame carries at most 255 payload bytes.** A frame that
  overruns the staging buffer is **dropped with an overflow flag latched**, never
  truncated — a partial frame would desynchronize the decoder for everything after
  it.
- **`MAGIC` is non-ASCII on purpose.** A plausible misconfiguration points the
  region at something text-shaped; a non-ASCII magic makes that visible instead of
  accidentally matching.
- **`static mut` staging is sound** because `defmt` guarantees the
  `acquire`/`write`/`release` span runs in a single, non-reentrant context. Every
  `unsafe` block in `logger.rs` cites that; preserve the assumption when editing.

## Configuration

Layout comes from the builder, not from firmware source. `csti build` lowers
`[dbg.<core>]` in `Consortium.toml` into `CONSORTIUM_DBG_BASE` and
`CONSORTIUM_DBG_LENGTH`, which `config` reads with `option_env!` and parses in a
`const` context. (This replaced an older scheme of `extern "C"` linker symbols and
matching `#[no_mangle]` statics the firmware had to define itself.)

Under a plain `cargo build` those variables are absent, so `config::CONFIGURED` is
`false` and the logger becomes a **no-op** rather than a build failure — which is
what lets firmware crates be `cargo check`ed without running the pipeline. Firmware
that genuinely needs logging should assert with `config::assert_configured` rather
than discover the silence at runtime.

There is no `[dbg.ap]`: the AP is always the host that decodes, never a producer. A
coprocessor with no `[dbg.<core>]` table simply has no debug ring.

## Warm resets and panics

Debug SRAM often survives a reset, so firmware that just starts writing would resume
into a stale ring with cursors pointing at a previous run's frames.
`ConsortiumLogger::init` resets the descriptor to an empty ring; generated `init()`
calls it automatically when a `[dbg.<core>]` logger is present.

The optional panic handler disables interrupts, emits the panic message as an
ordinary `defmt` frame (so the host decodes it with no extra code), latches
`Flags::Panic` in the descriptor, then halts with the target's WFI. It never blocks
waiting for the host to drain. A double panic is guarded: if logging itself faults,
the second entry skips straight to halt rather than recursing. The `Panic` flag is
how a host tells a crash from a clean end-of-stream in retained memory.

## Features

Defaults are `host` and `logger`.

| Feature         | Effect                                                                  |
| --------------- | ----------------------------------------------------------------------- |
| `logger`        | Firmware `#[global_logger]` writing into the ring.                      |
| `host`          | Host-side `HostRing` reader; implies `std`.                             |
| `decode`        | Adds `defmt-decoder`/`defmt-parser` for `DefmtSession`; implies `host`. |
| `panic-handler` | Installs `#[panic_handler]`; implies `logger`.                          |
| `arch-cortex-m` | Interrupt-disable and halt for Cortex-M.                                |
| `arch-riscv`    | Interrupt-disable and halt for RISC-V.                                  |

`panic-handler` needs **exactly one** arch feature, and the two are mutually
exclusive. Both conditions are `compile_error!`s rather than link-time surprises.

`decode` re-exports `defmt_decoder` and `defmt_parser`, so a consumer such as
`consortium-runtime-app` can name `Frame` and `Level` without its own dependency on
either.

## Testing

```bash
cargo test -p consortium-dbg              # descriptor, ring, packet
cargo test -p consortium-dbg --features decode
just test dbg host
```

Host-side tests cover descriptor initialization, ring read/write behaviour, decoding
fixtures, and the overflow/error flags — no target hardware needed.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
