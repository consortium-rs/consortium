# Consortium IPC: Vision & Design

## The Developer Experience

The entire system exists to make this work:

```rust
// M-core (bare-metal, no_std)
tx.send(TemperatureSensorData { temp: 26.9, time_elapsed: 6 }).await?;

// A-core (Linux, Tokio)
let data = rx.recv().await?;
println!("temp={}, elapsed={}ms", data.temp, data.time_elapsed);
```

No manual serialization. No raw pointer juggling at the call site. No explicit cache flushes. No doorbell management. Just typed, async send and receive across the processor boundary.

The complexity lives in the layers below. The application developer sees none of it.

---

## The Layer Stack

```
┌─────────────────────────────────────────────────┐
│  Application                                    │
│  tx.send(SensorData { ... }).await?             │
│  rx.recv().await?                               │
└──────────────┬──────────────────────────────────┘
               │  Channel<Tx/Rx, T, Tr, C>
               │  typed, directed, allocation-free
               ▼
┌─────────────────────────────────────────────────┐
│  Codec                                          │
│  T → [u8]  (send)                               │
│  [u8] → T  (recv)                               │
│  default: PostcardCodec (postcard + serde)      │
└──────────────┬──────────────────────────────────┘
               │  raw bytes
               ▼
┌─────────────────────────────────────────────────┐
│  Transport                                      │
│  send(chan, &[u8])   recv(chan, &mut [u8])      │
│  owns Doorbell; drives slot protocol            │
│  ┌──────────────────┬────────────────────────┐  │
│  │  SharedMemory    │  runtime-app UIO       │  │
│  │  transport       │  Linux / Tokio         │  │
│  └──────────────────┴────────────────────────┘  │
└───────┬──────────────────────────┬──────────────┘
        │                          │
        ▼                          ▼
┌───────────────┐        ┌─────────────────────────┐
│  Slot (SHMEM) │        │  Doorbell               │
│  len/flags/   │        │  ring() / wait()        │
│  payload      │        │  HSEM, IPCC, MU, etc.   │
└───────────────┘        └─────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────┐
│  Region                                         │
│  as_ptr() / flush() / invalidate()              │
│  SramRegion | DdrRegion | SerialRegion          │
└─────────────────────────────────────────────────┘
```

---

## Layer 1: Channel — The Public API

`Channel<D, T, Tr, C>` is what application code touches. It is a typed, directed endpoint over a transport.

```rust
// Tx side (M-core)
let mut tx: Channel<Tx, TemperatureSensorData, SharedMemoryTransport<_, _>> = /* ... */;
tx.send(&TemperatureSensorData { temp: 26.9, time_elapsed: 6 }).await?;

// Rx side (A-core)
let mut rx: Channel<Rx, TemperatureSensorData, SharedMemoryTransport<_, _>> = /* ... */;
let msg = rx.recv().await?;
```

**Type parameters:**

| Parameter | Role                    | Example                                                  |
| --------- | ----------------------- | -------------------------------------------------------- |
| `D`       | Direction: `Tx` or `Rx` | Compile-time guard—can't call `recv()` on a `Tx` channel |
| `T`       | Message type            | `TemperatureSensorData`                                  |
| `Tr`      | Transport               | `SharedMemoryTransport<HsemDoorbell, DdrRegion>`         |
| `C`       | Codec                   | `PostcardCodec` (default)                                |

**Zero-allocation contract:**

`Channel` never calls the allocator. Callers provide a `&'static mut [u8]` scratch buffer sized to `transport.max_message_size()`. The buffer is reused across every send and recv.

```rust
static mut BUF: [u8; 256] = [0u8; 256];
let tx = Channel::<Tx, SensorData, _>::new(chan, transport, unsafe { &mut BUF });
```

**`ReceivedMessage<'a, T, C>`:**

`recv()` returns `ReceivedMessage` rather than `T` directly. The lifetime `'a` ties the decoded value to the scratch buffer. For `PostcardCodec`, `T` is copied off the wire and `'a` is unused but structurally present. For future zero-copy codecs (e.g., rkyv), `Decoded<'a, T> = &'a T::Archived`—the returned reference directly into the buffer, and the borrow checker prevents the buffer from being reused until the handle is dropped.

**Error type:**

```rust
enum ChannelError<C: Codec, Tr: Transport> {
    Codec(C::Error),        // encode/decode failed
    Transport(Tr::Error),   // underlying send/recv failed
}
```

The `?` operator propagates either variant.

---

## Layer 2: Codec — Type ↔ Bytes

The `Codec` trait decouples serialization format from transport.

```rust
pub trait Codec {
    type Decoded<'buf, T: 'buf>;
    type Error: Debug;

    fn encode<T: Serialize>(msg: &T, buf: &mut [u8]) -> Result<usize, Self::Error>;
    fn decode<'buf, T: DeserializeOwned>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf, T>, Self::Error>;
}
```

**Default: `PostcardCodec`**

Uses the [postcard](https://docs.rs/postcard) crate. Compact binary, `no_std` compatible, zero-copy-optional. A `TemperatureSensorData { temp: 26.9, time_elapsed: 6 }` encodes to ~10 bytes.

**Future: zero-copy codec**

A codec backed by [rkyv](https://docs.rs/rkyv) would set `Decoded<'buf, T> = &'buf T::Archived`. The borrow lives in the scratch buffer. No copy happens on receive—the decoded reference points directly into shared memory. This is opt-in; the channel's type signature changes and the borrow checker enforces it automatically.

---

## Layer 3: Transport — Byte Mover

`Transport` moves raw byte slices across the processor boundary. It knows about channels (addressed by `Chan(u8)`) and owns a `Doorbell` internally.

```rust
pub trait Transport {
    type Error;
    type SendFut<'a>: Future<Output = Result<(), Self::Error>>;
    type RecvFut<'a>: Future<Output = Result<usize, Self::Error>>;

    fn send<'a>(&'a mut self, ch: Chan, data: &[u8]) -> Self::SendFut<'a>;
    fn recv<'a>(&'a mut self, ch: Chan, buf: &'a mut [u8]) -> Self::RecvFut<'a>;
    fn max_message_size(&self) -> usize;
}
```

Transport has two implementations for the two sides of an HMP SoC.

### SharedMemoryTransport (M-core / UIO-backed paths)

Used on Cortex-M. No OS, no allocator, no async runtime beyond cooperative polling.

`SharedMemoryTransport<D: Doorbell, R: Region>` uses descriptor-backed slots in
DMA-accessible shared memory. The doorbell and region types are injected at
construction so the same transport compiles for HSEM, IPCC, MU, GPIO, or any
platform-specific mechanism.

**Send path:**

1. Write payload into tx slot via volatile stores.
2. `region.flush()` — issues `dc cvac` + `dsb sy` for each cache line if DDR; noop if SRAM.
3. Write `len` then `FLAG_VALID` into slot header (volatile, ordered).
4. `doorbell.ring(ch)` — notifies A-core.
5. Return immediately. `SendFut` resolves on the first poll.

**Receive path:**

1. `region.invalidate()` — issues `dc ivac` + `dsb sy` to discard stale cache lines.
2. Check `FLAG_VALID` on rx slot (volatile read).
3. If present: copy payload, write `FLAG_CONSUMED`, return.
4. If not: poll `doorbell.wait(ch)` to register waker; return `Pending`.
5. When M→A IRQ fires, waker reschedules task; next poll retries from step 1.

### Runtime-App UIO Path (A-core, Linux/Tokio)

Used on Cortex-A running Linux. The transport is backed by a UIO (`/dev/uioN`) kernel driver that mmaps shared memory and delivers M-core interrupts as readable file events.

`UioDevice::open(uio_num, num_channels, transfer_size)`:

1. Opens `/dev/uio0`, mmaps resource 0 (shared SRAM/DDR region).
2. Parses a binary descriptor from offset 0 (magic `0x434F4E53`, version 1). Fails if state ≠ `Ready`.
3. Spawns a background Tokio task: waits on `AsyncFd::readable()` for M→A IRQs, then broadcasts `irq_notify.notify_waiters()`.

`uio.channel_transport(chan, doorbell, region)` returns a `SharedMemoryTransport`
backed by the parsed descriptor and the UIO mmap.

**Send path (A→M):**

1. Check if tx slot is still consumed (previous message was processed).
2. If not: subscribe to `irq_notify`, await it, retry.
3. If consumed: write payload into slot, set `FLAG_VALID`. (Kernel/hardware delivers the interrupt; UIO's MU TX register write triggers M-core IRQ.)

**Receive path (M→A):**

1. Try-read rx slot (same `FLAG_VALID` check as `SharedMemoryTransport`).
2. If data present: copy payload, mark `FLAG_CONSUMED`, return.
3. If not: subscribe to `irq_notify`, await it.
4. Background task fires on each M→A interrupt, waking all waiting receivers.

---

## Layer 4: Slot Protocol — The Shared Contract

The slot is the fundamental unit of shared memory between M-core and A-core.

```
Offset  Size  Field    Description
0x00    4     len      Payload byte count (volatile u32)
0x04    4     flags    FLAG_VALID (0x01) | FLAG_CONSUMED (0x02) (volatile u32)
0x08    mtu   payload  Raw message bytes
```

**State machine:**

```
Free ─────────────(sender writes)──────────> Pending
                  len = N
                  copy payload
                  flags = FLAG_VALID

Pending ──────────(receiver reads)─────────> Consumed
                  check FLAG_VALID
                  copy payload
                  flags = FLAG_CONSUMED

Consumed ─────────(sender recycles)────────> Free
                  (checks FLAG_CONSUMED
                   before next write)
```

**Why volatile?** Both cores access this memory. The compiler must not cache reads or coalesce writes. Volatile loads/stores prevent that at the language level; cache operations at the hardware level (see Region).

**Write ordering:** `len` is written before `flags`. A receiver that observes `FLAG_VALID` is guaranteed to see the correct `len`.

---

## Layer 5: Doorbell — Pure Signaling

The `Doorbell` trait abstracts the interrupt-signaling primitive. It carries no data—it only tells the other core "something is ready."

```rust
pub trait Doorbell {
    type Error;
    type WaitFut<'a>: Future<Output = Result<Chan, Self::Error>>;

    fn ring(&self, ch: Chan) -> Result<(), Self::Error>;  // synchronous notify
    fn wait<'a>(&self, ch: Chan) -> Self::WaitFut<'a>;   // async wait
    fn pending(&self, ch: Chan) -> bool;                  // non-blocking check
}
```

`Chan(u8)` maps to whatever the platform uses: an HSEM semaphore ID, an IPCC channel, an MU transmit register index, a GPIO line number, a serial byte, or a Tokio `Notify` slot in unit tests.

**SharedMemoryTransport owns one Doorbell.** Application code never calls `ring()` or `wait()` directly—Transport manages that internally.

---

## Layer 6: Region — Memory Backing & Cache

The `Region` trait describes a contiguous piece of memory, including its cache coherency properties.

```rust
pub trait Region {
    unsafe fn as_ptr(&self) -> *mut u8;
    fn len(&self) -> usize;
    fn flush(&self);       // clean dirty lines to PoC
    fn invalidate(&self);  // discard stale lines
    fn is_coherent(&self) -> bool;
}
```

| Type           | Backing        | flush / invalidate                     | is_coherent | Use case                  |
| -------------- | -------------- | -------------------------------------- | ----------- | ------------------------- |
| `SramRegion`   | On-chip SRAM   | Noops                                  | `true`      | M-core-local buffers      |
| `DdrRegion`    | DDR DRAM       | AArch64 `dc cvac`/`dc ivac` + `dsb sy` | `false`     | Cross-core shared buffers |
| `SerialRegion` | None (virtual) | Noops                                  | `true`      | UART/SWD trace channels   |

When `SharedMemoryTransport` is monomorphized over `SramRegion`, the cache operations compile away entirely. When monomorphized over `DdrRegion`, they emit the correct data cache maintenance for that region implementation.

---

## Descriptor Lifecycle (UIO)

Before any application can open a `UioDevice`, the shared-memory descriptor must have been negotiated between the kernel and M-core.

```
KernelInit (0)
    │  kernel writes magic, version, proposed MTU
    ▼
Proposed (1)
    │  M-core reads proposed MTU, writes accepted MTU
    ▼
MReady (2)
    │  kernel locks in effective MTU, writes channel table
    ▼
Ready (3)  ← UioDevice::open() requires this state
    │
    ├──> Suspended (4)   firmware update / sleep
    └──> Error (5)       fatal condition
```

The `effective_mtu` in the descriptor drives `Transport::max_message_size()`, which in turn determines the minimum scratch buffer size callers must provide to `Channel`.

---

## Platform Targets

| Core                    | Transport               | Doorbell                          | Region                     | Async runtime                |
| ----------------------- | ----------------------- | --------------------------------- | -------------------------- | ---------------------------- |
| Cortex-M (STM32MP21)    | `SharedMemoryTransport` | IPCC                              | `SramRegion` / `DdrRegion` | cooperative executor or RTIC |
| Cortex-M (STM32MP23/25) | `SharedMemoryTransport` | HSEM or IPCC                      | `SramRegion` / `DdrRegion` | cooperative executor or RTIC |
| Cortex-M (i.MX 95)      | `SharedMemoryTransport` | MU                                | `SramRegion` / `DdrRegion` | cooperative executor         |
| Cortex-A (Linux)        | runtime-app UIO path    | Linux UIO + `tokio::sync::Notify` | mmap'd via UIO driver      | Tokio                        |

The same `Channel<D, T, Tr, C>` API compiles for all rows. Only the concrete types in `Tr` differ.

---

## `no_std` Compatibility

`consortium-ipc` (traits and `Channel`) is `no_std` by default.

| Feature  | Effect                                             |
| -------- | -------------------------------------------------- |
| _(none)_ | Pure `no_std`; `MaybeSend` has no bounds           |
| `std`    | Adds `Send` bound on futures via `MaybeSend: Send` |
| `alloc`  | Enables allocator support in serde/postcard        |

`consortium-mem` inherits `no_std`. `consortium-uio` requires `std` + Tokio and is Linux-only.

---

## Summary

The vision is a zero-cost, zero-allocation, fully typed async IPC layer where the application developer writes:

```rust
tx.send(TemperatureSensorData { temp: 26.9, time_elapsed: 6 }).await?;
let data = rx.recv().await?;
```

...and the system handles serialization (Codec), byte movement (Transport), shared memory slot management (slot protocol), cache coherency (Region), and inter-core signaling (Doorbell)—invisibly, correctly, and without ever calling `malloc`.
