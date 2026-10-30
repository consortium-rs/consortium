# Inter-Processor Communication

Inter-processor communication (IPC) lets processors with separate runtimes cooperate
without pretending they share one address space. A Linux application and a real-time
firmware image may observe the same SRAM, but they do not share Rust references, heaps,
executors, or failure handling.

Consortium models that boundary as typed messages over an explicit transport.

## The conventional path

Linux systems often use RPMsg: VirtIO vrings carry buffers in shared memory and a
mailbox interrupt notifies the remote processor. RPMsg is a good fit when Linux kernel
integration and OpenAMP interoperability are the primary requirements.

Consortium offers a smaller userspace-oriented path for systems that want the
cross-core contract expressed directly in Rust. Linux maps validated memory and
doorbell windows through UIO; firmware accesses the corresponding physical regions and
hardware interrupt lines.

## The Consortium model

```text
message type
    |
CodecFor<T>       typed value <-> bytes
    |
Channel<Tx/Rx>    direction, channel id, and fixed scratch buffer
    |
Transport         byte movement and MTU
    |
Doorbell          peer notification when the transport needs one
```

A `Transceiver` pairs one Tx channel and one Rx channel. Application code normally sees
only its generated transceivers and calls `send().await` or `recv().await`; the manifest
selects the concrete codec, transport, doorbell, and memory region.

`Channel` does not allocate. It encodes or decodes through a fixed scratch buffer sized
to the transport MTU. This keeps the same API usable under Tokio on Linux and in a
`no_std` firmware image.

## Shared-memory IPC

The shared-memory transport owns a `CIPC` descriptor followed by directional message
slots. Directions are absolute—primary to secondary and secondary to primary—then
resolved into local Tx and Rx halves for each processor.

Before traffic begins, both processors run a readiness rendezvous through `Connect`:

1. the primary publishes the descriptor and proposed layout;
2. the secondary validates the magic, version, channel count, and MTU; and
3. the secondary acknowledges the descriptor.

Each endpoint performs this handshake once, when the application connects it
(`Transceiver::connect(c.SENSOR).await?`), before the transport is split into its Tx and
Rx halves. `Connect` intentionally has no timeout; callers own their deadline policy.

Sends are fire-and-forget. A channel declared with `config.ack = true` also offers
`send_acked`, which resolves once the peer has drained that message's slot; it uses its
own pair of doorbell lines, declared per party as `ack_chan`.

A doorbell carries no payload. It publishes the fact that transport state changed and
provides the ordering edge that makes earlier shared-memory writes visible to the peer.
Current hardware bindings cover STM32 HSEM/IPCC, i.MX MU, and GPIO signaling.

## Memory visibility

`consortium-shared-memory` makes cache behavior part of the region type:

- `UncachedRegion` models a non-cacheable mapping and uses no-op maintenance; and
- `CachedRegion` performs AArch64 or Cortex-M7 data-cache maintenance.

On Linux, shared windows stay uncached `/dev/uioN` mappings. On firmware,
`config.cache = true` retains the controller's cacheable default and selects
explicit maintenance; generated Cortex-M7 bring-up invalidates and enables the
data cache before connecting the endpoints. The default programs a non-cacheable
MPU carveout. Cached transport slots use cache-line-rounded strides so the two
directions never share a line. The same manifest also drives reserved-memory and
UIO device-tree output.

## UART IPC

UART is a transport-coupled notification path: receiving bytes is itself the wakeup, so
there is no separate `Doorbell` or processor `Side`. Frames use:

```text
COBS(logical channel | payload | CRC32 little-endian) + 0x00
```

The host implementation uses Tokio serial I/O and provides pseudo-terminal test pairs.
The firmware adapter works with `embedded-io-async`; chip features add interrupt-buffered
UART drivers for i.MX9 and STM32MP2.

## FDCAN IPC

CAN is transport-coupled in the same way: a frame arriving is the wakeup, so again there
is no `Doorbell` and no `Side`. What differs from UART is that the controller already
delimits and CRCs every frame, so there is nothing to frame or checksum — but it cannot
carry a message longer than one frame's payload. The transport supplies the missing piece,
segmentation and reassembly, with one header byte per frame:

```text
byte 0: [ kind:2 | low:6 ]
  00 Single   : low = payload length; the whole message follows
  01 First    : low = 0; bytes 1..3 = total length u16 LE, then the head
  10 Continue : low = sequence number, 1,2,3,… mod 64; then the body
```

The logical channel does not go on the wire. Each half binds a CAN identifier instead —
the transmit half stamps it into every frame, the receive half drops anything else — and
the two directions use different identifiers, since both halves see all bus traffic.

A lost frame costs one message rather than the endpoint: the receiver reports the sequence
gap, discards the partial message, and resynchronizes on the next one.

Today the transport rides classic 8-byte frames. The seam is written for CAN FD's 64-byte
frames, so enabling them is a chip-HAL change rather than a protocol one.

## Safe message boundaries

`#[derive(IpcSafe)]` recursively rejects values that cannot be meaningful on another
processor: raw pointers, references, function pointers, `usize`, and `isize`. It is
designed to catch address-space and word-size mistakes at compile time.

The codec supplies the serialization contract. Postcard and prost produce owned
decoded values; rkyv can return a validated view borrowing the receive buffer. For this
reason `recv()` returns `ReceivedMessage` instead of assuming every codec creates an
owned `T`.

See [System Architecture](../guide/architecture.md) for the crate layout and
[Generated Runtime](../guide/runtime.md) for endpoint code.
