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

Generated initialization performs this handshake once, before splitting the transport.
`Connect` intentionally has no timeout; callers using the lower-level API own their
deadline policy.

A doorbell carries no payload. It publishes the fact that transport state changed and
provides the ordering edge that makes earlier shared-memory writes visible to the peer.
Current hardware bindings cover STM32 HSEM/IPCC, i.MX MU, and GPIO signaling.

## Memory visibility

`consortium-shared-memory` makes cache behavior part of the region type:

- `SramRegion` models coherent on-chip SRAM and uses no-op cache maintenance;
- `DdrRegion` performs AArch64 data-cache maintenance where required; and
- `SerialRegion` is a virtual, unbacked region used for framed transports.

On Linux, shared and doorbell windows are mapped from `/dev/uioN`. On firmware, the
generated code uses validated physical addresses and configures the expected memory
attributes. The same manifest also drives reserved-memory and UIO device-tree output.

## UART IPC

UART is a transport-coupled notification path: receiving bytes is itself the wakeup, so
there is no separate `Doorbell` or processor `Side`. Frames use:

```text
COBS(logical channel | payload | CRC32 little-endian) + 0x00
```

The host implementation uses Tokio serial I/O and provides pseudo-terminal test pairs.
The firmware adapter works with `embedded-io-async`; chip features add interrupt-buffered
UART drivers for i.MX9 and STM32MP2.

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
