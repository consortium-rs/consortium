# Consortium Project Assistance Guide

Consortium is a Rust workspace for heterogeneous multiprocessing systems, especially
SoCs that pair a Linux-capable A-core with Cortex-M, Cortex-R, or RISC-V real-time
cores. The repository is early-stage and moving quickly. Prefer reading the current
crate sources and manifests over relying on older docs, examples, or design notes.

The current implementation centers on explicit cross-core contracts:

- `consortium-ipc` defines typed channels, transport traits, side markers, doorbells,
  and `IpcSafe`.
  - `consortium-ipc-transport-memory` owns the descriptor/slot shared-memory transport. Doorbell crates bind hardware signaling to the transport layer.
  - `consortium-ipc-transport-uart` provides COBS/CRC-framed wired IPC for embedded and Unix
    serial backends.
  - `consortium-ipc-transport-fdcan` provides segmented wired IPC over CAN frames, with
    chip glue in `consortium-ipc-transport-fdcan-embedded`.
- `consortium-codec` defines codec families used by IPC and TEE parameter conversion.
- `consortium-cfg-common` and `consortium-cfg` own the current `Consortium.toml`
  schema, chip databases, validation, and DTS/UIO-oriented transpilers.
- `consortium-hal`, chip HAL crates, and generated PAC crates provide early
  firmware-side peripheral access for i.MX9 and STM32MP2 targets.
- `consortium-dbg` implements shared-memory `defmt` logging and an optional panic
  handler.
- `consortium-log` is the common `defmt`/`tracing` facade used by generated and
  cross-platform code.
- `consortium-tee` handles the trusted execution environment contracts.
- `consortium-hmi` owns the backend-neutral HMI command bridge;
  `consortium-hmi-webkit` adapts it to Cog/WPE WebKit, and
  `consortium-hmi-pocket` provides the native PocketJS prototype.
  `consortium-hmi-input` is the shared `no_std`, `IpcSafe` input-event model.

## Commands

```bash
# List project recipes
just

# Install/update local toolchains used by recipes
just setup

# Run host-side checks
just check

# Run focused subsystem checks
just test ipc host
just test tee host
just test hmi host
just test cfg host
just test dbg host
just test codec host

# Run host-side lint bundle
just lint

# Apply Rust, frontend, Python, and TOML fixers/formatters
just fix

# Serve documentation locally
mdbook serve docs/book/
```

Use `cargo test -p <crate>` for focused validation while iterating. Full-workspace
commands can hit platform-sensitive crates or code that is mid-migration.

## Host Execution Policy

Run non-Rust-build work on the host by default. This includes Git and GitHub
operations, Python scripts, `release.py`, `publish.py`, `uv`/`uvx`, Vite+,
manifest and lockfile updates, formatting, and non-Rust linters.

- On macOS, route Rust builds, tests, Clippy, and publish verification through
  the Linux container recipes provided by the `justfile` (for example,
  `just linux '...'` or `just linux-full '...'`). Run everything else directly
  on the macOS host unless a task explicitly requires Linux.
- On Linux, use the standard host environment and normal `just`/`cargo`
  commands.
- On Windows, run Rust build transactions inside WSL. Host-capable tooling may
  run natively.

The `justfile` is the command index, but verify recipes against current crates before
copying them into new docs.

The current target-oriented recipes include A-core Linux (`aarch64-unknown-linux-gnu`,
`armv7-unknown-linux-gnueabihf`), M-core bare-metal (`thumbv8m.main-none-eabihf`,
`thumbv7em-none-eabihf`), and RISC-V (`riscv32imc-unknown-none-elf`,
`riscv32imac-unknown-none-elf`, `riscv32imafc-unknown-none-elf`) checks/builds.
A-core, TEE, HMI, UIO, Cog, WebKit, and Yocto/OpenSTLinux flows are Linux/SDK oriented.

## Workspace Shape

| Area            | Crates                                                                                                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Core IPC        | `consortium-ipc`, `consortium-ipc-macros`, `consortium-ipc-macros-impl`                                                                                                                                                           |
| Codecs          | `consortium-codec`, `consortium-codec-macros`, `consortium-codec-macros-impl`                                                                                                                                                     |
| Transports      | `consortium-shared-memory`, `consortium-ipc-transport-memory`, `consortium-ipc-transport-uart*`, `consortium-ipc-transport-fdcan*`                                                                                                |
| Doorbells       | `consortium-ipc-transport-memory::bell` (IPCC, MU, HSEM), `consortium-ipc-doorbell-gpio`                                                                                                                                          |
| Configuration   | `consortium-cfg-common`, `consortium-cfg`, `consortium-dts`, `consortium-data`                                                                                                                                                    |
| HAL/PAC         | `consortium-hal`, `consortium-hal-imx9`, `consortium-hal-stm32mp2`, `consortium-pac-imx9`, `consortium-pac-stm32mp2`                                                                                                              |
| Macro helpers   | `consortium-macros-helpers`                                                                                                                                                                                                       |
| Runtimes/build  | `consortium-runtime-app`, `consortium-runtime-mcu`, `consortium-runtime-macros`, `consortium-runtime-macros-impl`, `consortium-runtime-spl`, `consortium-runtime-sys`, `consortium-builder`, `csti` (in `crates/consortium-cli/`) |
| Logging/debug   | `consortium-log`, `consortium-log-macros`, `consortium-dbg`                                                                                                                                                                       |
| TEE             | `consortium-tee`, `consortium-tee-macros`, `consortium-tee-macros-impl`                                                                                                                                                           |
| HMI/Cog/ORT/FFI | `consortium-hmi`, `consortium-hmi-input`, `consortium-hmi-webkit`, `consortium-hmi-pocket`, `cogcore`, `cogcore-sys`, `consortium-ort`, `consortium-ffi`                                                                          |
| Examples        | `examples/melt-pot/*` for i.MX93, i.MX95, and STM32MP25 app/MCU/TA/shared message crates                                                                                                                                          |

The root workspace includes `crates/*` and `examples/melt-pot/shared`. Per-endpoint
example crates under `examples/melt-pot/{imx95,stm32mp25}/{app,mcu,ta}` are excluded
because the builder pipeline generates `src/consortium.gen.rs` for them and resolves
their artifacts outside plain workspace `cargo check`.

## Codecs

`consortium-codec` is `no_std` by default and has no default codec feature. Available
families are feature-gated:

- `postcard` exposes `PostcardCodec`.
- `prost` exposes `ProstCodec`.
- `rkyv` exposes `RkyvCodec` and re-exports `TrustedArchive`.

`CodecFor<T>` owns the associated `Decoded<'buf>` type, so channel and TEE code should
avoid assuming decode always produces an owned `T`.

## IPC Core

The current IPC trait stack is:

```text
Doorbell       - hardware/software signal for waking a remote processor
SendTransport  - byte send half; writes bytes and rings the doorbell
RecvTransport  - byte receive half; waits for the doorbell and reads bytes
Transport      - full-duplex transport: SendTransport + RecvTransport
Connect        - readiness rendezvous with the remote end, run before traffic
CodecFor<T>    - serialization family for a message type
Channel<Tx/Rx> - typed directed endpoint over one transport half
Transceiver    - typed send/receive endpoint built from paired channels

AckSendTransport - optional: a send that resolves once the peer has taken the message
```

`consortium-ipc` is `no_std` unless `std` is enabled. Its feature shape is small:
`alloc`, `futures`, and `std = ["alloc", "futures"]`. Keep core traits usable from
firmware-side code.

`Channel<D, T, Tr, C>` does not allocate. Callers provide a `&'static mut [u8]` scratch
buffer sized for the transport MTU. `Tx` channels require `SendTransport`; `Rx`
channels require `RecvTransport`. `ReceivedMessage` is designed to handle owned codecs
today and zero-copy codecs later.

`Connect` is the standard bring-up entry point for transports that must agree with a
peer before carrying traffic. It is deliberately narrow: it means "wait until the far
side is ready", so it is async, it has no timeout (callers own the deadline), and it is
called once before any send/recv and before `split()`. Its associated `Params` carries
only what is negotiated; anything identifying the link belongs in the constructor.
`SharedMemoryTransport` implements it per `Side` (the primary publishes the descriptor
and waits for the ack, the secondary validates and acks), so generated bring-up code
emits one call with no side branch; the UART transport implements it as a documented
no-op. Local bring-up is _not_ `Connect`: peripherals keep constructor + config +
typestate, and their ordering stays in `consortium-cfg`'s generated `init()`; doorbells
keep `bind_notifier`/`bind_service`.

`Transceiver<TyTx, TyRx, TrTx, TrRx, C>` is public and wraps paired `Channel<Tx, ...>`
and `Channel<Rx, ...>` values. If a transport needs separate ownership for Tx/Rx, use
the transport's split halves before constructing channels.

`IpcSafe` rejects address-space-local constructs such as pointers, references, function
pointers, `usize`, and `isize`. Add reusable `syn`/AST helpers to
`consortium-macros-helpers`; keep domain-specific derive logic in the owning
`*-macros-impl` crate.

### UART IPC

The UART family is the first wired, Type-B transport: byte arrival is the notification,
so it has no separate `Doorbell` or `Side`. `consortium-ipc-transport-uart` implements
the shared protocol as `COBS(chan | payload | CRC32-LE) + 0x00`, with one logical
`Chan` per transport and explicit RX/TX scratch buffers derived with `rx_buf_len()` and
`tx_buf_len()`.

The core crate defaults to `std`: Tokio `AsyncRead`/`AsyncWrite` and `Send` futures. Its
no-std path uses `embedded-io-async` (0.7). `consortium-ipc-transport-uart-unix` opens raw
8N1 `/dev/tty*` devices with `tokio-serial` and provides pseudo-terminal pairs for tests.
`consortium-ipc-transport-uart-embedded` is the firmware glue: with a chip feature
(`mimx93`/`mimx95` or `stm32mp21x`/`stm32mp23x`/`stm32mp25x`) it re-exports that chip
HAL's interrupt-buffered UART driver (`BufferedUart`, in `imx9`/`stm32mp2` submodules);
with no chip feature it stays chip-agnostic over any `embedded-io-async` driver. The
`BufferedUart` RX path is IRQ-buffered: the application forwards the UART interrupt into
`BufferedUartState::on_interrupt`, which drains the FIFO into an
`embassy_hal_internal::atomic_ring_buffer::RingBuffer` so no bytes are lost before the
transport polls; TX stays direct interrupt-driven. Per repo convention the crate installs
no `#[interrupt]` handlers. `consortium-runtime-mcu` re-exports the transport behind its
`ipc-uart` feature.

### FDCAN IPC

The FDCAN family is the second wired, Type-B transport, and differs from UART in three
ways worth knowing before touching it.

First, **there is no `embedded-io-async` seam**: `embedded-can` 0.4 is blocking/`nb` only,
and a bus carries frames rather than bytes. `consortium-ipc-transport-fdcan` therefore
declares its own `CanTx`/`CanRx`/`CanErrorType` traits, with `CanFrame` blanket-implemented
for every `embedded_can::Frame` so both chip HAL frame types work unadapted.

Second, **there is one impl family, not two**. UART needs parallel `std`/`no_std` impls only
because `embedded_io_async`'s futures cannot be proven `Send` generically; because this
crate owns its seam, the `MaybeSend` bounds live on `CanTx`/`CanRx` and one set of transport
impls serves both. `tests/loopback.rs` runs unchanged under `--no-default-features`, which
is the evidence for that claim — keep it that way.

Third, **there is nothing to frame or checksum**: the controller already delimits frames and
CRCs them, so the framing layer only segments and reassembles. One header byte per frame,
`[kind:2 | low:6]`, where `low` is a payload length on a `Single` and a wrapping sequence
number on a `Continue`; a `First` frame spends two more bytes on a `u16` total length. The
`Single` length is explicit rather than implied by the DLC because CAN FD quantizes the DLC
above 8 bytes, so a short FD frame arrives padded. For the same reason the transmit path
pads each segment up to a representable length (`padded_len`) before calling
`CanFrame::build` — a no-op on classic CAN, required on FD, and invisible to the receiver.

`Chan` does not go on the wire. Each half binds a CAN identifier — the transmit half stamps
it, the receive half drops anything else — and the two directions must use different ones,
since both halves see every frame on the bus. `FdcanTransport::new` rejects equality with
`Error::SameIdentifier`; a controller in loopback/self-reception mode echoes every frame
back, so a shared identifier would self-reassemble. Filtering is in software because neither HAL
programs hardware acceptance filters yet.

Corruption is not an error, as in UART: a foreign identifier, remote frame, reserved kind,
sequence gap, orphan continuation, or over-MTU length is logged, drops the partial message,
and resynchronizes on the next `Single`/`First`. `Error` has no `Eof` — a CAN bus has no peer
to close it.

Only the receive half takes a buffer (`rx_buf_len`); frames are fixed-size, so the transmit
path stages each one on the stack. There is no `tx_buf_len`.

`consortium-ipc-transport-fdcan-embedded` holds the chip glue. Because both the traits and
the HAL halves are foreign to it, the orphan rule forces local newtypes: `imx9::CanBusTx`/
`CanBusRx` and `stm32mp2::CanBusTx`/`CanBusRx`, reached through each module's `split()`. Per
repo convention it installs no `#[interrupt]` handlers — forward the vector to the HAL's
`on_interrupt::<T>()`. `consortium-runtime-mcu` re-exports the transport behind `ipc-fdcan`.

Both chip HALs' `Can` now split into `CanTx`/`CanRx` halves over separate wakers (a single
`AtomicWaker` would let one half clobber the other's registration). STM32MP2 receives into
an eight-element FIFO 0, so partition `CAN_SRAM` by at least `can::REGION_BYTES` per
instance; i.MX9 still has one receive mailbox, so a burst can overrun it.

### Shared Memory IPC

Shared memory is split across two crates:

- `consortium-shared-memory` defines the `Region` trait and concrete `UncachedRegion`,
  `CachedRegion`, and `SerialRegion` types. `UncachedRegion` represents a
  non-cacheable mapping; `CachedRegion` performs AArch64 or Cortex-M7 data-cache
  maintenance; `SerialRegion` is a virtual no-backing region.
- `consortium-ipc-transport-memory` defines the `CIPC` descriptor layout, channel table,
  slot header protocol, descriptor init/complete helpers, and
  `SharedMemoryTransport<S, D, R>`.

`SharedMemoryTransport` is generic over a `Side`, a `Doorbell`, and a region implementing
`RegionSharedMemoryTransportable`. Currently `UncachedRegion` and `CachedRegion` implement that
transport marker.

The descriptor uses absolute channel directions (`PrimaryToSecondary` and
`SecondaryToPrimary`) and resolves them relative to the `Side` type. `Primary` is the
Linux/A-core convention, and `Secondary` is the firmware/M-core convention in current
examples and tests. Slot starts and strides are 64-byte aligned so independently
written directions never share a maintained cache line; manifest validation ensures
each window contains the aligned descriptor and bidirectional slot pair. Generated CM7
initialization invalidates and enables the data cache when any channel sets
`config.cache = true`.

`SharedMemoryTransport::split()` returns `SharedMemoryTransportTx` and
`SharedMemoryTransportRx`. Use these halves when building simultaneous
`Channel<Tx, ...>` and `Channel<Rx, ...>` endpoints from one transport.

**Acknowledged sends are opt-in and additive.** `SendTransport::send` is fire-and-forget
on every channel and always will be; a channel built with `new_acked` (from
`config.ack = true`) also implements `consortium-ipc`'s separate `AckSendTransport`, whose
`send_acked` resolves only once the peer has drained _that_ slot and reports
`Error::PeerIntegrityError` when the peer took the message and rejected its CRC.

Two flags carry this, at two levels, and conflating them is the mistake to avoid. The
descriptor channel entry's `FLAG_CHAN_ACK_CAPABLE` is a _capability_ — "both ends wired
acknowledgement lines" — agreed once in the handshake so a mismatched pair fails at
`connect` with `DescriptorError::AckMismatch` rather than hanging. Whether an individual
message wants acknowledging is `FLAG_ACK_REQUESTED` in that message's _slot_ header, set
by `send_acked` and never by `send`, so a capable channel costs nothing extra for
fire-and-forget traffic. The receiver rings for exactly the messages that asked —
including on the integrity-error path, since a corrupt frame still frees the slot and a
parked sender must be woken either way.

Acknowledgement takes **its own pair of doorbell lines**, one per core, rather than reusing
the data lines. Every line carries exactly one waiter: the hardware doorbells hold a single
`AtomicWaker` per line, arm and disarm it unrefcounted, and _consume_ the status on the
first poll, so one edge reaches one waiter no matter what. The receive half is already
parked on the data line. Reclaiming those two channels would mean reworking per-line waker
fan-out, arm/disarm refcounting, and status acknowledgement in the `bell` protocol —
deliberately not attempted.

Three behaviours worth knowing: two peers both awaiting an acknowledgement with neither
draining the other direction deadlock (the acknowledgement comes from the peer's receive
path, so it must be running); a cancelled `send_acked` loses the confirmation, not the
message; and a plain `send` in flight makes `send_acked` return `Error::NoChannel` rather
than park, because an unacknowledged message frees its slot silently and there would be no
edge to wait for.

Shared-memory tests live under `crates/consortium-ipc-transport-memory/tests/` and cover
sync descriptor layout, Tokio futures, split halves, codec-backed channels, atomic-waker
no-std-shaped futures, Embassy-style polling, and end-to-end primary/secondary exchange.
Prefer extending these heap-backed or fake-doorbell tests before requiring hardware.

### Doorbells

Doorbells provide wakeup/signaling and should keep MMIO safety invariants documented.
There are two kinds:

- **Type-A bells** — IPCC, MU, and (frozen) HSEM — live in
  `consortium-ipc-transport-memory::bell`, behind the transport's chip features.
- **`consortium-ipc-doorbell-gpio`** is a single-channel GPIO doorbell. The `no_std`
  path uses a software pending flag and IRQ-side `wake`; the `std` path exposes a Tokio
  `Notify` that external interrupt code must trigger, bound with `bind_notifier`.

**A Type-A bell is one protocol over a per-block register model.** A block implements
only `unsafe trait BellHw`: `CHANNELS`, `RESERVED`, `PHYS_BASE`, `line(id)`, `ring`,
`raw_pending`, `ack`, `arm`/`disarm`, `asserted`/`disarm_all`, and a per-line `waker`
table, where status bit `n` is channel `n`. Everything else is written once in
`bell/mod.rs`: `BellA<H>` implements `Doorbell` and `ChanValidate` (one bounds check),
`LineWait<H>` is the one wait future, and `service_interrupt::<H>()` (no_std) /
`BellA::service_interrupt()` (std) is the one ISR core, which masks every asserted line
**before** waking. Fences, critical sections, waker registration, and the `std` `Notify`
re-subscription belong to the generic — a `BellHw` impl never does them. The protocol,
and why each step of its poll order is load-bearing, is documented on `bell/mod.rs` and
tested once against an in-memory `FakeBell` in `bell/tests.rs`; a block's own tests only
check its register mapping. Read both before changing any block.

- `bell::ipcc` — STM32 IPCC (`stm32mp21x`/`23x`/`25x`); `IpccDoorbell<S, N>`. `ring`
  reports `BellError::Busy` while the previous signal is unconsumed. The recommended
  STM32MP2 doorbell, and the first validated on hardware.
- `bell::mu` — NXP i.MX MU (`mimx93`/`mimx95`, mutually exclusive); `MuDoorbell<S, N>`.
  Channels `0..4` are GI lines and `4..8` RF lines (`MuLine`); neither reports `Busy` — a
  pending GI is idempotent and an RF ring whose `TRn` is still full is skipped.
  `MuDoorbell::new` also opens the window's i.MX 93 LPCG clock gate. Validated on i.MX 93
  hardware (September 2026, before the `bell` port).
- `bell::hsem` — STM32 HSEM (`stm32mp23x`/`25x`); `HsemDoorbell<S, 1>`. Frozen: prefer
  IPCC. `ring` is a 1-step read-lock then unlock, and reports `Busy` when another core
  holds the semaphore. `S` is the interrupt bank (`Tertiary` is C3). Not validated on
  hardware.

The `bell` ports kept each block's register sequences but changed how they are driven
(one generic wait/ISR, and `disarm_all` masks with one read-modify-write per register),
so earlier bring-ups need a re-run on the current code before they count for it.

The chip features pull in `consortium-hal-stm32mp2`/`consortium-hal-imx9` with
`default-features = false` and forward the chip, so the PAC's register/interrupt
definitions match the target (two chips of one family = ambiguous PAC `Interrupt`).

**A bell's `Side` is its register window or bank, not the transport's descriptor role.**
For an i.MX MU, `Primary` means MUA and `Secondary` means MUB — which half of the block
this core drives — while `SharedMemoryTransport`'s `Primary`/`Secondary` is the
shared-memory protocol's role, chosen by the manifest's `primary`/`secondary`. On both
i.MX 93 and i.MX 95 the A-core is on MUA and the M-core on MUB, so they coincide for the
documented `primary = "linux"`, but a manifest that swaps the roles keeps the windows.
`impl_channel!` takes the two as independent type parameters and `consortium-cfg` lowers
them separately: the window comes from each party's `window:` field in the chip database
(`Transceiver::doorbell_window`), the role from the manifest. Conflating them fails
`MuDoorbell::new`'s `Instance<Side = S>` bound. The i.MX 93 CM33's chip map exposes both
`MU1_MUA`/`MU1_MUB` (and MU2's), so the database, not the dump, picks MUB. A
single-window block (HSEM, IPCC) has no `window_length` and its marker keeps meaning the
core bank, so it still follows the role.

`Doorbell::wait` is RPITIT (`-> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend`),
like `SendTransport`/`RecvTransport`. `LineWait` is a named future because its `Drop`
disarms its line on cancel (a `select!` timeout, a dropped `recv`) — **disarm only**,
never acknowledge, or an unconsumed notification is lost. The trade is that the future
type is unnameable through the trait, so a receive loop holds it as a local across an
`.await`. `recv_bytes` in the transport is the worked example, and its ordering is
load-bearing — create the wait future, poll it **once** through a `poll_fn` to register
the waker, re-scan the slots, and only then park on it. Awaiting it directly registers and
parks in one step, which reopens the lost-wakeup race the `RacyDoorbell` test covers (the
fan-out `notify_waiters()` stores no permit).

No doorbell installs `#[interrupt]` handlers, and none exposes an `rt` feature.
`consortium-cfg` **generates** each Type-A vector into `consortium.gen.rs`'s
`__consortium_vectors` module from the manifest, alongside the time-driver vector, as
`bell::service_interrupt::<bell::<block>::<Block><S, N>>()` — the window is
`BellHw::PHYS_BASE`, so no MMIO base appears in generated code — and binds the AP side
with `uio.bind_service(move || db.service_interrupt())`. Enable the _HAL's_ `rt` feature
(`consortium-hal-*/rt`) so the `#[interrupt]` macro and PAC vector table are available to
that code. A core the chip PAC's shipped interrupt table does not describe (STM32MP2's
CM0+) has no `Interrupt` variant to name and keeps writing its vectors by hand. The
relevant vectors are HSEM and IPCC1_RX/IPCC2_RX (non-secure), and the MU lines a core
drives (i.MX93 CM33 `MU1_B`/`MU2_B`; i.MX95 CM7 `MU5_A`/`MU7_B`/`MU8_B`).

## HAL and PAC

`consortium-hal` is a small `no_std` trait crate for generated chip metadata and drivers:
`Peripheral`, `ClockControl`, `InterruptSource`, `MonotonicTimer`, and `nvic::Irq`.

`consortium-pac-imx9` and `consortium-pac-stm32mp2` are generated `no_std` PAC crates.
Their chip features are `mimx93`/`mimx95` (default `mimx95`) and `stm32mp21`/`stm32mp257`
(default `stm32mp257`), respectively. `just pac-regen` regenerates every PAC output end to
end — chip dumps and IR under `crates/consortium-data/database/{chip,peri}/`, the PACs'
`interrupts/` and `peripherals/`, and the HALs' `src/peripherals/<chip>.rs` — from the
vendor submodules, with the pinned chiptool (`just setup-chiptool`). `just pac-verify`
fails if the committed files differ from a fresh run. Corrections the parsers cannot
make are patches, each headed with its reason: `scripts/pac/patches/<submodule>/` for
vendor headers, `scripts/pac/patches/ir/` for the IR and chip dumps (applied before the
interrupt/singleton generators), and `scripts/pac/patches/generated/` for the Rust.
Never hand-edit the outputs; add or update a patch and re-run.

`consortium-hal-imx9` and `consortium-hal-stm32mp2` re-export the shared HAL and their
PAC crates (with `default-features = false`), re-export `interrupts`/`Interrupt` and the
`rt`-gated `#[interrupt]` macro, and expose feature-gated ADC, CAN, DMA, GPIO, I2C, I3C,
PWM, SPI, timer, UART, and watchdog modules plus the Embassy time driver (all on by
default alongside the default chip; STM32MP2 adds LPTIM). They also
own the chip-selection features forwarded to the PAC:
`mimx93`/`mimx95` (hal-imx9, default `mimx95`) and `stm32mp21x`/`stm32mp23x`/`stm32mp25x`
(hal-stm32mp2, default `stm32mp25x`, where `stm32mp23x`/`stm32mp25x` both map to the PAC's
`stm32mp257`; MP23 is the same die and reference manual, RM0457, but mounts its own
generated singleton map without the instances its datasheet rules out, such as I2C3–6,
FDCAN3, LPDMA and IPCC2). Keep these crates firmware-friendly: `no_std`, narrow unsafe blocks, and
chip-specific details inside the chip HAL rather than the shared HAL traits.

### Peripheral Singletons

Drivers take ownership of a peripheral, not its address. `<hal>::init()` hands out
the chip's singletons once as a `Peripherals` struct, and each driver's constructor
consumes the one it drives:

```rust
let p = consortium_hal_imx9::init();
let i2c = consortium_hal_imx9::i2c::I2c::new(p.LPI2C1, 24_000_000, 400_000);
let heartbeat = consortium_hal_imx9::gpio::Gpio::new(p.GPIO2).pin(0);
```

No driver takes an MMIO base and no constructor is `unsafe`. `peripherals::<NAME>::steal()`
is the escape hatch for bring-up outside `init()`; it carries the exclusivity obligation
the safe path discharges.

`src/peripherals/<chip>.rs` in each chip HAL is generated by
`scripts/pac/gen_peripherals.py` from the same `consortium-data` chip dumps that feed
`gen_interrupts.py`, and mounted by `lib.rs` as the private `chip` module. It declares
the singletons (`consortium_hal::peripherals!`) and invokes each driver's
`impl_<driver>!` macro with that instance's base and NVIC line; the driver module owns the
macro, so the generated file is data only. `consortium_hal::peripherals!` is
`embassy_hal_internal::peripherals!` minus the per-type `defmt::Format` derive (one
`.defmt.*` link section per marker overruns the Mach-O section limit on `defmt`-enabled
macOS host builds) and minus the `no_mangle` once-guard (two chip HALs can land in one
binary, as they do in the `consortium-cfg` UI tests). Per-instance interrupt state lives behind
`Instance::state()`, so applications forward a vector with
`<hal>::i2c::on_interrupt::<peripherals::LPI2C1>()` rather than naming a `static`. i.MX9
eDMA channels have no dump entry, so the generator synthesizes `<CONTROLLER>_CH<n>`
singletons from the layout table in `dma.rs`; STM32MP2 dumps list channels directly.
Re-run the generator (not hand edits) after changing a chip dump.

The Type-A bells depend on the chip HAL, so they bind the HAL's MU/HSEM/IPCC singletons
themselves. `<Doorbell>::new(p.MU7_MUB)` is the firmware path — a const assertion checks
the singleton against the doorbell's `<Side, N>` type parameters, and the window address
still comes from the block's own `registers::base` table (`BellHw::PHYS_BASE`) so its
security alias does not change. Linux/UIO windows are `mmap`s with no singleton, so the AP side keeps
`unsafe fn new_mapped(base)`.

Generated `init(Config)` calls `<hal>::init()` once and moves fields out of it: to the
time driver, to IPC doorbells, or into the generated endpoint-local `Peripherals`
allowlist. The full HAL bundle never escapes generated init. A controller core with no
bundled chip map has no singletons to name and keeps address-based doorbell construction.
`[peripheral.*]` entries with `secure = true` are rejected because the singletons carry
only the non-secure alias.

## Configuration

The authoritative current schema is in `consortium-cfg-common/src/config/`, not in the
older melt-pot example manifests. Use `crates/consortium-cfg/Consortium.example.*.toml`
and `consortium-cfg/src/session.rs` tests as the best references.

Current top-level schema:

- `[profile]` contains required `chip`, `shared`, and `deploy_root`; optional `devkit`
  selects a board DTS by full file stem in the `csti build --linux` checkout,
  `sysroot` selects the Linux target
  sysroot, `kernel_recipe` enables generation of a kernel `.bbappend`, and
  `time_driver` (default `true`) toggles the sealed controller-core Embassy time driver,
  timer vector, and `defmt::timestamp!`. `workspace` (`nested`, the default, or
  `unified`) says where endpoint crates put `target/`: `nested` means each crate has its
  own, and `unified` means every endpoint is a member of one Cargo workspace rooted
  beside `Consortium.toml`, which must then contain a root `Cargo.toml` with a
  `[workspace]` table (`resolve_workspace` checks this). The melt-pot examples use
  `nested`.
- `[dts]` controls Linux device-tree output: `mode = "merge"` (default) emits a full
  tree, while `overlay` emits a `.dtso`; `base` overrides the base tree.
  `csti build --linux <DIR>` supplies all BSP DTS/DTSI sources and dt-bindings.
  `configfs` opts into `CONFIG_OF_CONFIGFS` for overlay deployment. Overlay mode cannot
  delete remoteproc mailbox properties, so use merge mode for those fixups.
- `[endpoints]` maps runtime endpoint names to artifact roots.
- `[dbg.<core>]` declares a defmt logger on coprocessor `<core>` (e.g. `[dbg.cm33]`)
  and contains `memory = { base, length }` and `mtu`. The AP is always the host that
  decodes the logs, so there is no `[dbg.ap]`; a coprocessor with no table has no debug.
  An optional top-level `[dbg] mode` (`auto`/`interface`, default `auto`) selects how
  the AP consumes the rings: `auto` generates a console that forwards decoded defmt
  frames to `tracing`; `interface` returns reader handles for the integrator. The
  field flattens alongside the `[dbg.<core>]` tables.
- `[[ipc.channel]]` contains `name`, `doorbell`, `pattern`, `primary`, `secondary`,
  `party.<name>`, `config.memory`, optional controller-side `config.cache` (default
  `false`), optional `config.ack` (default `false`), `mtu`, and optional `codec`
  (default `postcard`). `config.ack` makes `send_acked` available on the channel and
  requires each party to declare an `ack_chan` beside its `chan`; those extra doorbell
  channels go through the same bounds and duplicate checks as the data lines.
- `[tee]` contains the OP-TEE UUID.
- `[hmi]` contains `engine` (`cog` or `pocket`), `backend`, optional `debug`,
  `source`, `build`, and optional `app`. `app` names the PocketJS entry to build
  (the mounted `-main` entry, e.g. `hero-main`) and is required when
  `engine = "pocket"`; `Hmi::validate` also rejects `pocket` + `backend = "drm"`
  (no DRM shell yet). `[hmi.logical]` contains `width` and `height` for the
  logical UI viewport and defaults to `480` × `272`; the builder passes these
  dimensions to the Pocket runtime.
- `[peripheral.<name>]` contains `owner`, `secure`, optional `frequency`, optional
  `clock`, and optional pin declarations.

`Session::from_toml` parses the manifest, resolves `[profile] chip` through embedded
hardware databases, and validates doorbell availability, visible parties, channel
bounds, duplicate doorbell channels, region bounds/overlap/MPU alignment, debug regions,
and peripheral owners. `consortium-data` carries generated per-core chip maps,
peripheral descriptions, and memory databases. Linux DTS sources are never bundled.

`consortium-cfg` includes lowering, portable/Linux codegen, debug-console and peripheral
bring-up codegen, UIO/Kconfig generation, vendor-tree DTS merge/overlay rendering,
remoteproc fixups, and Type-A doorbell interrupt enablement. Keep config changes centered
on schema types first, then validation and lowering, then generated code and build
artifacts. Update snapshot tests under
`crates/consortium-cfg/tests/` when generated output intentionally changes.

### Build and Packaging Pipeline

The user-facing CLI binary is `csti`; `csti build` runs the end-to-end
`Consortium.toml` pipeline. It compiles the Linux application, controller firmware,
optional TA and HMI; post-processes firmware for Linux remoteproc; generates endpoint
modules, DTS/Kconfig/boot integration, and a systemd unit; then assembles a local
`dist/` staging tree. It does not deploy directly to a live target.

`csti fetch` is the pipeline's network half: it resolves the same Cargo projects the
build plan does (firmware per remote core, TA, application) and runs `rustup target add`
per distinct triple, `cargo fetch --target <triple>` per project, and the HMI frontend's
package install (`vp`/`npm install`, or `bun install` for Pocket; override with
`CONSORTIUM_HMI_FETCH_CMD`). `csti build --offline` then passes `--offline` to every
cargo invocation. Unlike `pipeline::plan`, `pipeline::fetch` needs no codegen and no
OP-TEE toolchain. `CargoOptions { release, offline }` is the shared carrier for cargo
flags across the three Rust stages.

`csti check` (`pipeline::check`) is the pipeline's `cargo check`: it validates the
manifest, writes each endpoint's `consortium.gen.rs`/`memory.x`, and runs `cargo check`
per project with the same targets and injected env as a build — no link, objcopy,
signing, HMI build, `dist/`, or `dtc`. `--linux` is optional (the DTS is only rendered
when given), and the TA is skipped rather than failing when `TA_DEV_KIT_DIR` is unset.
Because the AP module `include_bytes!`es each `[dbg.<core>]` firmware's
`*_original.elf`, an unbuilt one is replaced by an empty `<name>_check.elf` stand-in
beside the cargo output until the next `csti build` regenerates the module. Host provisioning — the OP-TEE dev kit, a Yocto
sysroot, kernel sources for `dtc`, BitBake's downloads — stays outside `fetch`.

Firmware post-processing renames `.vector_table` to the remoteproc-expected
`.isr_vectors` section. Outputs include the deployable ELF and debug/original artifacts.
The staged tree can contain `app/`, `firmware/`, `ta/`, `www/`, `dts/linux/`, `kernel/`,
`boot/`, and `systemd/`, plus a generated BitBake recipe. CLI flags support staging the
tree into a recipe `files/` directory and invoking or dry-running BitBake. Keep the
builder's planning, compilation, staging, and external BitBake hand-off boundaries clear.

### Build-Time Configuration

The builder flow injects compile-time environment variables such as `CONSORTIUM_IPC_*`,
`CONSORTIUM_DBG_*`, `CONSORTIUM_HMI_*`, and `CONSORTIUM_TEE_UUID`. Consumers read them
with `option_env!`/`env!`. The builder emits numeric env-var values as decimal strings;
firmware-side consumers that need const parsing can use `const-str` and should not depend
on the std-oriented schema crates.

Every endpoint `cargo` invocation (application, firmware, TA) also receives two
build-host directories: `CSTI_MANIFEST_PATH` (the absolute directory holding
`Consortium.toml`) and `CSTI_MCU_PATH` (the absolute firmware crate root, set only when
the manifest declares exactly one). The generated AP module `include_bytes!`es each
`[dbg.<core>]` ELF as `concat!(env!("CSTI_MCU_PATH"), "/target/...")` for a `nested`
workspace, or relative to `CSTI_MANIFEST_PATH` for a `unified` one, so the committed
`consortium.gen.rs` carries no build-host path. A nested manifest with several firmware
crates has no single `CSTI_MCU_PATH` and falls back to absolute paths.

Current IPC build-time variables are channel-local metadata such as
`CONSORTIUM_IPC_LENGTH`, `CONSORTIUM_IPC_UIO`, and `CONSORTIUM_IPC_MTU`; the shared
memory base comes from generated code and UIO mapping, not `CONSORTIUM_IPC_BASE`.
`consortium-dbg::config` reads `CONSORTIUM_DBG_BASE` and `CONSORTIUM_DBG_LENGTH`.
`consortium-hmi-webkit` (engine `cog`) reads `CONSORTIUM_HMI_DIST` and optional
`CONSORTIUM_HMI_DEBUG_URL`; the Cog build command is selected by
`CONSORTIUM_HMI_BUILD_CMD`, then `vp`, then `npm`. `consortium-hmi-pocket` (engine
`pocket`) reads `CONSORTIUM_HMI_POCKET_BUNDLE` and `CONSORTIUM_HMI_POCKET_PAK`
(on-target `{deploy_root}/hmi/<app>.{js,pak}`), plus
`CONSORTIUM_HMI_LOGICAL_WIDTH` and `CONSORTIUM_HMI_LOGICAL_HEIGHT`, via
`PocketApp::from_env`; its build
runs `bun tools/build.ts <app> --outdir=<build>` (override with
`CONSORTIUM_HMI_BUILD_CMD`), and the builder stages the bundle+pak into `dist/hmi/`.

The build pipeline writes `src/consortium.gen.rs` into generated endpoint crates. The
example app/MCU crates use `include!("consortium.gen.rs")`; do not expect plain cargo
commands to work for those crates before the pipeline has generated that file.

### Runtime Layers

`consortium-runtime-app` is Linux-oriented, but its UIO interrupt dispatch is split out
into a portable `irq` module gated `cfg(unix)` (not `target_os = "linux"`), so the
`[fd readiness -> user-space fan-out]` slice is socketpair-testable on macOS as well as
Linux; the real `/dev/uioN` mapping and device lifecycle stay in the Linux-only `uio`
module. The crate exposes remoteproc support, optional UIO-backed IPC (`ipc-shm`),
optional debug (`dbg`), and optional TEE (`tee`). `UioDevice::open(uio_id, enable_irq)`
opens the device, maps regions (`mmap` for register/doorbell windows, `mmap_zeroed` for
zero-initialized data windows), and runs `irq::irq_loop`: wait for the fd to become
readable, drain the 4-byte interrupt count, re-arm the interrupt with a 4-byte write, then
fan the wakeup out to every registered doorbell service. Bind a doorbell with
`bind_notifier(Arc<Notify>)` (a non-masking doorbell such as GPIO, wakeup is just
`notify_waiters()`) or `bind_service(Arc<dyn Fn()>)` (every Type-A bell registers a service
that masks its asserted lines _before_ waking, so a level-sensitive line does not
immediately re-fire the UIO IRQ when the loop re-arms).

`consortium-runtime-mcu` is `no_std`, re-exports debug and shared-memory IPC pieces
behind features, and provides small helpers such as `static_buf!` for scratch buffers.
Its default features are `ipc-shm` and `macros`. It also exposes `rsc_table`: raw bindings come from
the generated `consortium-runtime-sys` crate, while `SharedResourceTable` and
`RSC_TABLE` emit an RPMsg-vdev-plus-trace table in the firmware `.resource_table`
section. Keep packed-layout access and Linux remoteproc ABI requirements explicit.

### Entry Macros and Generated Resources

`consortium-runtime-macros` (facade) + `consortium-runtime-macros-impl` provide the
runtime entry-point attribute macros, re-exported by the runtime crates behind their
`macros` feature (default on). Firmware writes `#[consortium_runtime_mcu::main] async fn
main(spawner: Spawner)`; applications write `#[consortium_runtime_app::main] async fn
main()`. Either may return `()` or `Result<(), E: core::fmt::Debug>`. The macro only
stacks the executor attribute (`embassy_executor::main` / `tokio::main`, resolved through
the user crate's own dependency) and dispatches the outcome through `MainOutcome`. It
does not call generated initialization. An error is logged, then MCU parks with `wfi` or
the application exits non-zero. There is no `#[fail]` macro or name-shadow dispatch.

Generated `consortium.gen.rs` exposes Embassy-shaped resources. `Config` is
`#[non_exhaustive]`, has public boolean fields, and has a hand-written `Default` enabling
every manifest-selected step. MCU `init(Config) -> (Peripherals, Channels)` is
infallible; AP `init(Config) -> Result<(Peripherals, Channels), ConsortiumInitError>` is
fallible because UIO opening and debug-console construction can fail. MCU knobs are
`logger`, `mpu`, `nvic`, and `time_driver`; AP knobs are `tracing`, `uio_irqs`, and
`dbg_console`. Cache enablement has no knob: a cached manifest window always enables the
required cache policy.

`Peripherals` is generated locally rather than re-exporting the HAL's full bundle. On
firmware it contains exactly the uppercase singleton fields declared in
`[peripheral.*]`; doorbells and the time-driver timer are consumed internally and never
exposed. Manifest clock/frequency values become constants such as
`LPI2C1_CLOCK_HZ`/`LPI2C1_FREQUENCY`, and users construct drivers themselves. On AP it
contains platform owners (`uio` and the optional debug console), with the debug field
declared first for safe drop order. Neither `Peripherals` nor `Channels` implements
`Drop`, because partial moves such as `p.GPIO2` and `c.SENSOR` are intentional.

`Channels` contains one `Peri<'static, channels::NAME>` per uppercase endpoint. Generated
init installs the unsplit transport and two take-once scratch buffers into that
instance's `EndpointState`, then returns the singleton bundle. Connect is per endpoint:
`Transceiver::connect(c.SENSOR).await?` is the concise spelling, while
`Endpoint::new(c.SENSOR).connect().await?` supports deferred or spawned bring-up. Both
guarantee `Connect::connect` precedes `SharedMemoryTransport::split()`. A peer that never
appears therefore stalls only the endpoint being connected. The AP transport halves
retain `Arc<UioDevice>` owners for every mapping they use.

Controller init still resets a configured CDBG ring, configures MPU/NVIC, enables CM7
data cache for cached windows, and brings up the Embassy time driver (STM32MP2 TIM2 /
i.MX9 LPIT1), emitting the matching timer vector and `defmt::timestamp!`. See
`consortium-cfg`'s `components/time.rs` and the melt-pot MCU examples.

`consortium-runtime-spl` currently exposes small domain/peripheral/role traits for
partitioning-oriented code.

`consortium-builder` owns the pipeline for compiling application, firmware, TA, and HMI
artifacts from a `Consortium.toml` session. It bridges the current config schema into
generated endpoint modules and compile-time environment variables; prefer extending
`consortium-cfg` lowering first, then the builder pipeline.

## TEE

`consortium-tee` is `no_std` with `alloc`. The default feature is `ca`, but CA error
exports are gated to Linux non-test builds. The `ta` feature is host-agnostic and is used
by host-side macro/parameter tests.

`TeeParam<C = NoCodec>` converts Rust values to and from `TeeParamSlot`. Primitive
values map to TEE value slots and implement `TeeParam<C>` for any `C`. Serialized values
map to memref slots and require a `CodecFor<T>` implementation. `#[derive(TeeParam)]`
generates serialized support; `#[tee(max_size = N)]` overrides the default serialized
buffer bound.

When changing TEE macros, update trybuild coverage and insta snapshots in the macro
implementation crate before relying on the proc-macro facade.

## Debug Logging

`consortium-log` is a `no_std` macro facade with optional `defmt` and `tracing`
backends. Both may be enabled, or neither (no-op). Use the portable format-string plus
positional-arguments subset in shared code; backend-specific `defmt` hints and
`tracing` fields are only safe when that backend is exclusively enabled. Arguments must
not have side effects because disabled log calls do not evaluate them.

`consortium-dbg` implements defmt-over-shared-memory with `CDBG` descriptors. Its
default features are `host` and `logger`; `decode` adds `defmt-decoder` and implies
`host`. Debug configuration comes from `CONSORTIUM_DBG_*`; plain `cargo build` leaves
the config unconfigured and the logger no-op unless the binary calls the const guard.

Preserve descriptor discipline when changing debug transport behavior. Host-side tests
cover descriptor initialization, ring read/write behavior, decoding fixtures, and
overflow/error flags.

The optional panic handler emits an error packet, latches panic status in the debug
descriptor, then waits with the target architecture's WFI instruction.

## HMI

`consortium-hmi` is the portable application-service registry and response-future
contract. It contains no Cog, GLib, WebKit, PocketJS, or Tokio runtime state.
`consortium-hmi-webkit` is Linux-only and exposes `KioskApp` plus the shared `Bridge`;
in debug builds it can load `CONSORTIUM_HMI_DEBUG_URL`, otherwise it serves the compiled
dist path from `CONSORTIUM_HMI_DIST`. `consortium-hmi-pocket` is the native prototype:
it hosts a PocketJS JS/pak pair, mounts the bridge into QuickJS, delivers responses at
frame boundaries, and exposes headless wgpu rendering. Its optional `shell` feature adds
a thin `winit` window host (`run`/`ShellConfig`, Wayland/X11 on Linux and Cocoa on macOS)
that drives `PocketApp::frame` and blits the logical-size UI to the surface; the `kiosk`
example runs a built `<app>-main.{js,pak}` in it, and `PocketApp::from_env` loads the
pair from the builder-injected env vars. The `csti build` pipeline supports
`engine = "pocket"` end to end (bun build, `<app>.js`/`.pak` verification, `dist/hmi/`
staging, env injection); it does not yet provide the production DRM/KMS + seat/`libinput`
shell.
`cogcore-sys` owns raw bindgen output and vendored Cog/WPE/WebKit headers; `cogcore`
wraps a small safe subset and keeps WebKit/WPE types mostly opaque.

Cog/WPE work needs Linux Cog, GLib/GIO, WPE, and WebKit development packages. On hosts
without those packages, prefer formatting, source review, and narrow tests that do not
link Cog/WebKit. Use the `cogcore.yml` workflow or a properly provisioned Linux machine
for binding generation and wrapper validation.

When extending `cogcore`, prefer `glib`/`gio` types for public GLib/GIO interop, keep
raw ownership and refcounting inside the safe wrapper layer, and document each unsafe
block with the pointer lifetime or ownership invariant.

## Testing Guidance

- Use `cargo test -p <crate>` for focused host-side validation.
- Use `just test ipc host` for the current host IPC bundle.
- Use `cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test`
  for the no-std-shaped shared-memory future path.
- UART protocol changes should run `cargo test -p consortium-ipc-transport-uart`,
  `cargo test -p consortium-ipc-transport-uart --no-default-features`, and
  `cargo test -p consortium-ipc-transport-uart-unix` for pseudo-terminal integration.
- FDCAN protocol changes should run `cargo test -p consortium-ipc-transport-fdcan` and
  the same with `--no-default-features`. Both must pass: the second is what proves the
  one-impl-family claim, since it runs the identical `loopback` test as `no_std`.
- Use trybuild and insta snapshots for proc-macro behavior.
- Snapshot changes are accepted automatically outside CI and reviewed in git:
  `.cargo/config.toml` sets `INSTA_UPDATE=always`, and `consortium-cfg`'s
  `tests/conf.rs` rewrites `artifacts.snap` whenever `CI` is unset
  (`UPDATE_SNAPSHOTS=0` forces it to fail instead). CI sets `INSTA_UPDATE=no` and
  `CI`, so a mismatch fails there.
- Use fake MMIO, heap-backed regions, and mock doorbells for protocol logic.
- Keep hardware, SDK, UIO, remoteproc, OP-TEE runtime, and Cog/WebKit checks separate
  from default host checks.
- Do not assume Linux-only crates or no-std target checks work on every developer
  machine.

## Linux Builds on macOS (Apple `container`)

The `*-a64` / `*-a32` recipes refuse to run on a non-Linux host, and the Linux-only
crates (`consortium-runtime-app`, `cogcore*`, `consortium-hmi-webkit`) cannot be
compiled on macOS at all. On Apple Silicon, Apple `container` boots a _native_
arm64 Linux VM, so `aarch64-unknown-linux-gnu` is a **native** build there and the
melt-pot manifests' `[profile] sysroot = "/"` resolves to the image root — no Yocto
SDK and no cross linker are involved for the 64-bit A-core.

`Dockerfile.linux` defines two stages, driven by the `just linux-*` recipes:

- `base` (`just linux-image`) — Rust toolchains, `uv`, Vite+, `dtc`,
  `arm-none-eabi-objcopy`, and `just`. Enough for the `*-a64` / `*-a32`
  recipes and repository tooling. The Rust toolchain is the channel pinned in
  `rust-toolchain.toml` (staged into the build context by the recipe), so the
  `--rm` containers never re-download it; rebuild the image after a toolchain bump.
- `full` (`just linux-image-full`) — adds WPE WebKit/Cog from the
  consortium-rs/wpe-kiosk-stack-prebuilt `.deb` and an OP-TEE 4.10.0 TA dev kit.
  Enough for a full `csti build` of melt-pot. The `.deb` (~45 MB) is fetched on
  demand by `just setup-wpe-deb` into the gitignored `assets/`; it is not part of
  `just setup`, since most hosts never build this image. Point
  `CONSORTIUM_WPE_DEB` at an existing copy to skip the download.

```bash
just linux-setup                        # start services + builder with a routable DNS
just linux-image                        # or: just linux-image-full
just linux just check-ipc-a64           # run any repo recipe inside the container
just linux-full 'cargo build -p cogcore'
```

Things that are easy to get wrong here:

- **DNS.** `--dns` is not cosmetic. On a host whose VPN hands out fake-ip DNS (the
  198.18.0.0/15 pool), the VM inherits an address it cannot route and every network
  call _hangs silently_ rather than failing. The recipes pass `--dns` for this;
  override with `CONSORTIUM_LINUX_DNS`.
- **Do not set a global `CARGO_TARGET_DIR`.** The melt-pot example crates are
  excluded from the workspace so they build into per-crate `target/` dirs, and the
  build pipeline looks for their ELFs at exactly those paths. The recipes instead
  mount a named volume over `/workspace/target`, which also keeps Linux artifacts
  out of the macOS `./target` tree — sharing one target dir between host and
  container makes every switch a full rebuild.
- **Ubuntu 24.04, not Debian bookworm.** The prebuilt WPE stack needs GLIBC_2.38
  and libicu74.
- **The HMI frontend still builds on the host.** `website/` uses pnpm `catalog:`
  deps and `vp`, and `node_modules/` lives in the mounted repo, so a Linux install
  would clobber the macOS one. Build the frontend on the host, then set
  `CONSORTIUM_HMI_BUILD_CMD=/bin/true` so the container consumes the existing dist.
  `CONSORTIUM_HMI_FETCH_CMD=/bin/true` does the same for `csti fetch`'s install step.
- **The kernel tree is mounted at `/root/linux`.** `linux-run`/`linux-shell` mount
  host `~/Developer/linux` (override with `CONSORTIUM_LINUX_KERNEL`), shallow-cloning
  torvalds/linux there first if it is missing, so `csti build` inside the container
  takes `--linux /root/linux`. Build endpoints with `csti build`, not a raw
  `cargo build` in the endpoint crate: only the pipeline regenerates
  `consortium.gen.rs`/`memory.x`, injects the build env, and post-processes firmware.
- Running the pipeline rewrites the committed `src/consortium.gen.rs`. Its debug-ELF
  `include_bytes!` paths are relative to `CSTI_MCU_PATH`, so they no longer differ
  between host and container, but review other regenerated output before committing.

## Style and Tooling

- Rust edition is 2024 across the workspace.
- Workspace lints deny undocumented unsafe blocks and unsafe-operation shortcuts.
- Keep unsafe blocks narrow and explain the hardware, memory, aliasing, or ABI invariant.
- `cargo fmt` handles Rust formatting.
- Taplo uses two-space TOML indentation and a 100-column width.
- `typos.toml` configures spelling checks.
- `.ls-lint.yml` defines crate, test, doc, script, board, and simulator naming rules.

When updating docs, use crate names, features, APIs, and schema fields present in the
current workspace. The mdBook lives under `docs/book/`. Treat `docs/book/src/dev/**` and
the dated `docs/book/src/log/**` entries as design/debugging history; introductions are
useful conceptual context, but verify all of them against current source before making
them normative.

## Dependency Policy

Use credible crates for common functionality, especially in tests. Before adding a
dependency, ensure the crate is compatible with Apache-2.0-friendly licensing and with
the relevant `no_std`/`alloc`/`std` target constraints.

## Publishing

`docs/PUBLISHING.md` is the release runbook. The constraints worth knowing before
touching a manifest:

- The CLI publishes as **`csti`**, not `consortium-cli` — that name, and the bare
  `consortium` name, are taken on crates.io by unrelated projects. Its sources stay
  in `crates/consortium-cli/`, so cargo invocations use `-p csti`.
- Every intra-workspace **dependency and build-dependency carries both `path` and
  `version`**, which is what lets cargo rewrite them to registry requirements.
  **Dev-dependencies are deliberately path-only**: cargo strips those from the
  published manifest, which keeps test-only edges (such as `consortium-cfg` →
  `consortium-runtime-app`) out of the publish graph. Do not add versions to them.
- `consortium-hmi-pocket` is `publish = false`. Its `pocket-mod`, `pocket-ui-wgpu`,
  and `pocket3d` deps come from an upstream **git `rev`**, not from
  `submodules/pocketjs`, because PocketJS is not on crates.io yet and cargo refuses
  to publish a package with a git dependency. The `rev` is the commit the submodule
  is checked out at; bump both together. The submodule is still needed for the
  bun/JS side. `melt-pot-shared` is `publish = false` as an example crate.
- MSRV is `rust-version = "1.87"` in `[workspace.package]`, enforced by clippy's
  `incompatible_msrv` on every `just lint`. Raise it rather than `#[allow]`-ing.
- `cogcore-sys` and `consortium-runtime-sys` redistribute vendor material and carry
  compound SPDX expressions instead of inheriting `Apache-2.0`.
- `just publish-dry` requires Linux — `cogcore-sys` cannot build on macOS at all.
  Use `just publish-dry-host` for the partial macOS run.

<!--VITE PLUS START-->

# Using Vite+, the Unified Toolchain for the Web

This project is using Vite+, a unified toolchain built on top of Vite, Rolldown, Vitest, tsdown, Oxlint, Oxfmt, and Vite Task. Vite+ wraps runtime management, package management, and frontend tooling in a single global CLI called `vp`. Vite+ is distinct from Vite, and it invokes Vite through `vp dev` and `vp build`. Run `vp help` to print a list of commands and `vp <command> --help` for information about a specific command.

Docs are local at `node_modules/vite-plus/docs` or online at https://viteplus.dev/guide/.

## Review Checklist

- [ ] Run `vp install` after pulling remote changes and before getting started.
- [ ] Run `vp check` and `vp test` to format, lint, type check and test changes.
- [ ] Check if there are `vite.config.ts` tasks or `package.json` scripts necessary for validation, run via `vp run <script>`.
- [ ] If setup, runtime, or package-manager behavior looks wrong, run `vp env doctor` and include its output when asking for help.

<!--VITE PLUS END-->
