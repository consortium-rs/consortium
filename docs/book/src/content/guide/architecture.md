# System Architecture

Consortium separates the system's **contracts** from its **mechanisms**. The manifest
and shared Rust types define what must agree across processors; generated code chooses
and initializes the mechanisms that satisfy those contracts on a particular chip.

## From description to running endpoints

```text
                         build time

  Consortium.toml ──> parse + validate ──> lowered system model
         +                                      |
  shared Rust types                             +── generated endpoint modules
         +                                      +── compile-time configuration
  endpoint crates                               +── DTS, UIO, Kconfig, boot files
                                                        |
                                                        v
                         run time

                Linux Context <── typed IPC ──> firmware Context
                     |                                |
              UIO / remoteproc                  HAL / interrupts
```

Validation happens before compilation. A channel with an unavailable doorbell, an
out-of-range channel number, an overlapping memory region, or an invisible endpoint is
rejected at the system boundary rather than discovered during board bring-up.

## Layers

| Layer         | Main crates                                                                           | Responsibility                                                    |
| ------------- | ------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Contracts     | `consortium-codec`, `consortium-ipc`, `consortium-tee`, `consortium-hmi`              | Portable types and traits shared by runtimes                      |
| Mechanisms    | `consortium-ipc-transport-*`, `consortium-ipc-doorbell-*`, `consortium-shared-memory` | Byte movement, signaling, framing, and cache visibility           |
| Hardware      | `consortium-hal-*`, `consortium-pac-*`                                                | Peripheral ownership and chip-specific register access            |
| Configuration | `consortium-cfg-common`, `consortium-cfg`, `consortium-data`                          | Schema, chip databases, validation, lowering, and code generation |
| Runtime       | `consortium-runtime-app`, `consortium-runtime-mcu`                                    | Linux and firmware integration around generated code              |
| Build         | `consortium-builder`, `csti`                                                          | Compile, post-process, stage, and hand off artifacts              |

The core IPC and codec crates are `no_std` by default. Linux-only concerns such as UIO,
remoteproc, and host-side debug decoding stay in `consortium-runtime-app`; firmware
keeps a small runtime surface suitable for Embassy and bare-metal targets.

## The IPC composition model

Typed IPC is built from small independent pieces:

```text
Doorbell                      signal the peer; carries no payload
SendTransport / RecvTransport move bytes in one direction
Transport                     full-duplex marker
Connect                       one-time peer readiness rendezvous
CodecFor<T>                   convert one message type to and from bytes
Channel<Tx/Rx, ...>           typed, directed endpoint
Transceiver                   paired send and receive channels
```

For shared memory, the transport owns a `CIPC` descriptor and one slot per direction.
The primary publishes the descriptor; the secondary validates its version and layout,
then acknowledges it. Generated `init()` calls `Connect::connect` once before traffic
and before splitting the transport into its Tx and Rx halves.

The UART transport uses the same byte-transport traits but needs no separate doorbell:
byte arrival is the notification. Its wire format is a COBS-delimited frame containing
a logical channel, payload, and little-endian CRC32.

## Boundary safety

`#[derive(IpcSafe)]` rejects address-space-local values such as pointers, references,
function pointers, `usize`, and `isize`. Serialization requirements remain codec
specific: postcard messages derive serde traits, prost messages implement `Message`,
and rkyv messages use the archive traits generated for that family.

This check is structural, not a replacement for protocol design. Stable integer widths,
explicit units, bounded payloads, and versioning still belong in the shared message
crate.

## Ownership rules

Consortium makes ownership concrete at every layer:

- a manifest assigns each peripheral and IPC endpoint to a visible processor;
- a chip HAL's `init()` hands out each peripheral singleton once;
- a generated `Context` owns the live transports, UIO mappings, and peripherals for
  one endpoint; and
- channels own their fixed scratch buffers, so normal send and receive paths do not
  allocate.

The application still owns interrupt handlers on firmware. Doorbell and HAL crates
export wake functions, but do not install vectors behind the application's back.
