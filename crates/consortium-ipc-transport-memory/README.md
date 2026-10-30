# consortium-ipc-transport-memory

Shared-memory transport for Consortium IPC: the `CIPC` descriptor, its channel
table, the slot protocol, and `SharedMemoryTransport`.

This is the primary Consortium link — the one an A-core and an M-core on the same
SoC use. It implements [`consortium-ipc`](../consortium-ipc)'s `SendTransport`,
`RecvTransport`, and `Connect` over a window of memory both cores can address,
using a `Doorbell` for wakeups because shared memory on its own cannot notify
anybody.

The pieces it does **not** own:

| Concern                                     | Crate                       |
| ------------------------------------------- | --------------------------- |
| Trait stack, typed channels                 | `consortium-ipc`            |
| `Region`, cache semantics, header magic     | `consortium-shared-memory`  |
| Hardware wakeups (HSEM, GPIO)               | `consortium-ipc-doorbell-*` |
| Window addresses, validation, bring-up code | `consortium-cfg`            |

IPCC and MU are the exception: as Type-A doorbells they live here, under `bell`.

## Type-A doorbells (`bell`)

IPCC, MU, and HSEM differ in which registers they poke, not in how a wait must be
driven, and that protocol is easy to get subtly wrong. `bell` writes it once:

```text
BellA<H>      Doorbell impl: fence + ring, raw-status pending, LineWait
LineWait<H>   the one wait future, with its Drop-time disarm
service       ISR core: read asserted lines, mask them, then wake
H: BellHw     per block: ring, raw status, ack, arm/disarm, waker table
```

A block implements only the `unsafe trait BellHw` register model; fences, critical
sections, waker registration, and the `std` `Notify` re-subscription belong to the
generic. The blocks are `bell::ipcc` (`IpccDoorbell<S, N> = BellA<Ipcc<S, N>>`) and
`bell::mu` (`MuDoorbell<S, N> = BellA<Mu<S, N>>`, GI and RF lines). Firmware vectors
forward to `bell::service_interrupt::<H>()` with the block's register model,
which `consortium-cfg` generates; on Linux, bind `BellA::service_interrupt` with
`UioDevice::bind_service`.

## Layout

One window holds everything. A fixed-size descriptor at offset 0 describes a
channel table; each entry points at a slot elsewhere in the same window.

```text
0x00  ┌────────────────────────────────┐
      │ header: "CIPC", version        │  from consortium-shared-memory
      │ num_channels, updated ids      │
      │ tx/rx_transfer_size            │
      │ state (DescState)              │
0x20  ├────────────────────────────────┤
      │ channel table: N × 12 bytes    │  offset, size, direction
      ├────────────────────────────────┤
      │ cache-line padding             │
      ├────────────────────────────────┤
      │ slot 0  [len|flags|crc32|data] │  ← one direction, aligned
      │ padding to cache-line boundary │
      │ slot 1  [len|flags|crc32|data] │  ← the other, aligned
      └────────────────────────────────┘
```

Channel directions in the table are **absolute** — `PrimaryToSecondary` and
`SecondaryToPrimary` — and each side resolves them to TX or RX through its own
`Side` type parameter. One descriptor therefore describes the link identically from
both ends, and neither end needs a runtime branch on "am I the primary". In this
workspace `Primary` is the Linux/A-core convention and `Secondary` the firmware
convention.

## Protocol

### Bring-up is `Connect`, once per `Side`

| Side        | `connect` does                                                   |
| ----------- | ---------------------------------------------------------------- |
| `Primary`   | publish the descriptor, `flush`, ring, await the ack             |
| `Secondary` | await/observe the descriptor, validate it, `flush`, ring the ack |

The header magic is the readiness gate: the primary writes it **last**, so a
secondary that sees `CIPC` knows the whole descriptor landed. That makes the
doorbell an optimization rather than a requirement — a secondary that boots late
finds the descriptor already published and skips the wait entirely, which is the
common case when Linux brings up a coprocessor through remoteproc.

Because the asymmetry lives in the type, generated bring-up code emits one
`connect(..)` call for both sides.

`DescriptorLayout` is the negotiated part, and only that: `num_channels` plus a
proposed `mtu`. Everything identifying the link — channel ids, doorbell, region,
base — is fixed at construction. `mtu` is a _proposal_, not a verdict: after
`connect` returns, both sides report the resulting **slot capacity** through
`max_send_size` / `max_recv_size`, which is what `send` enforces. A secondary
asking for more than the primary published does not enlarge the slots.

### Steady state is the slot protocol

```text
send: pick a consumed TX slot → write payload → len → crc32 → FLAG_VALID
      → region.flush()  → doorbell.ring()

recv: doorbell.wait()   → region.invalidate()
      → check FLAG_VALID → read len → copy payload → verify crc32
      → FLAG_CONSUMED
```

Two things carry the correctness of this:

- **`FLAG_VALID` is written last and read first.** A peer sees either a complete
  frame or no frame, never a half-written one — with no lock and no atomic beyond
  the flag word itself.
- **`flush` precedes every `ring`, `invalidate` follows every wake.** That is what
  makes a `CachedRegion` link work at all; on an `UncachedRegion` both compile away
  through monomorphization, so a coherent link pays nothing for the discipline.
- **Every slot starts on a 64-byte boundary and occupies a cache-line-rounded
  stride.** Opposite directions never share a maintained line, so cleaning the
  CM7-owned slot cannot write stale bytes over the AP-owned slot.

A CRC32 over the payload catches what ordering cannot: a truncated DMA, a
misconfigured MPU window, a peer writing with the wrong cache attributes. A
mismatch sets `FLAG_INTEGRITY_ERROR` in the slot and reports no message rather
than handing up corrupt bytes.

## Usage

```rust,ignore
use consortium_ipc::{Connect, Secondary};
use consortium_ipc_transport_memory::{DescriptorLayout, SharedMemoryTransport};

// SAFETY: `base` covers the descriptor, channel table, and slots, and nothing
// else aliases the window.
let mut transport = unsafe {
    SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, doorbell, region, base)
};

// Same call on both sides; Side selects the protocol role.
transport.connect(DescriptorLayout::new(2, 256)).await?;

let (tx, rx) = transport.split();
```

`split()` yields `SharedMemoryTransportTx` and `SharedMemoryTransportRx`, each
carrying one channel id and a clone of the doorbell and region handles, so a `Tx`
and an `Rx` `Channel` can be driven concurrently over one window. Split **after**
`connect` — the handshake is a property of the whole link.

In practice generated code does all of this: `csti build` emits an `init()` that
constructs the transport, connects it, splits it, and hands back connected
transceivers at `context.ipc_shm.<name>`.

## Considerations and limitations

- **Depth is one frame per direction.** A slot holds a single message, so a sender
  whose peer has not yet set `FLAG_CONSUMED` gets `Error::NoChannel` rather than
  queueing. Backpressure is the caller's to handle — this is a deliberate choice
  for a transport that must work with a fixed, statically described window.
- **At most `CHANNEL_NUM_MAX` (16) channels** per descriptor.
- **`new` is `unsafe`.** The caller guarantees the window covers the descriptor,
  channel table, and every slot, and that nothing else aliases it. On Linux the
  pointer comes from a `mmap` of `/dev/uioN`; on firmware from a linker-placed
  symbol. `consortium-cfg` validates bounds, overlap, and MPU alignment against
  `Consortium.toml` before generating either.
- **Not a security boundary.** CRC32 detects accident, not tampering. A malicious
  peer sharing the window is outside the threat model; use the TEE for that.
- **`connect` has no timeout**, per the `Connect` contract. Wrap it in
  `tokio::time::timeout` or `embassy_time::with_timeout` for bounded bring-up.
- **`Send` impls are hand-written.** The transport holds a raw base pointer, so
  `Send` is `unsafe impl`'d conditionally on the doorbell and region being `Send`;
  no shared reference to the window is ever handed out through the safe API.

## Features

Defaults to `std`. Firmware builds want `default-features = false`.

| Feature                      | Effect                                             |
| ---------------------------- | -------------------------------------------------- |
| `std`                        | `alloc`, Tokio support, UIO errors, `tracing` logs |
| `alloc`                      | link `alloc` through `consortium-ipc`              |
| `defmt`                      | route protocol tracing to `defmt`; derive `Format` |
| `doorbell-hsem`              | re-export the HSEM crate under `doorbell`          |
| `stm32mp21x` / `23x` / `25x` | select the STM32MP2 chip; enables `bell::ipcc`     |
| `mimx93` / `mimx95`          | select the i.MX 9 chip; enables `bell::mu`         |

Chip features exist so `doorbell::SharedMemoryTransportDoorbell` can report a
doorbell peripheral's window address, which generated Linux bring-up needs in order
to map it. The transport itself is generic over any `Doorbell` and needs none of
them.

## Testing

Tests run the real protocol against heap-backed regions and fake doorbells, so they
need no hardware.

```bash
cargo test -p consortium-ipc-transport-memory
cargo test -p consortium-ipc-transport-memory --no-default-features \
    --test async_atomic_waker_test    # no_std-shaped future path
cargo test -p consortium-ipc-transport-memory -F stm32mp25x --lib  # bell + IPCC
cargo test -p consortium-ipc-transport-memory -F mimx95 --lib      # bell + MU
just test ipc host
```

Coverage spans sync descriptor layout, Tokio futures, split halves, codec-backed
channels, Embassy-style polling, and an end-to-end primary/secondary exchange.
Prefer extending these before requiring hardware.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
