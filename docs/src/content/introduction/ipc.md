# Inter-Processor Communication (IPC)

Inter-Processor Communication (IPC) is a core task in HAMPU development. Unlike MPU + MCU architectures that rely on serial links such as UART or SPI, HAMPU systems commonly use shared memory as the primary IPC transport between the CA and CM domains.

## Traditional Path: RPMsg

A conventional Linux-oriented AMP IPC stack is often built around RPMsg. In this model, the CA side communicates through a Linux RPMsg interface, such as `rpmsg_char`, `rpmsg_tty`, or a custom RPMsg kernel driver. The RPMsg framework packages messages into VirtIO vrings located in a shared memory region, while a mailbox interrupt or IPI is used to notify the remote processor that new data is available. On the CM side, an OpenAMP, RPMsg-Lite, RTOS, or bare-metal endpoint consumes the message and delivers it to the CM application.

Conceptually, the traditional RPMsg path can be summarized as follows:

```text
CA Linux user app / kernel driver
        |
        |  /dev/rpmsg*, rpmsg_char, rpmsg_tty, or custom rpmsg driver
        v
Linux RPMsg framework
        |
VirtIO / vring
        |
Shared memory region
        |
Mailbox interrupt / IPI
        |
CM OpenAMP / RPMsg-Lite / RTOS / bare-metal endpoint
        |
CM application
```

In this architecture, RPMsg provides the logical messaging abstraction, VirtIO/vring manages shared-memory buffer exchange, shared memory carries the actual payload, and the mailbox or IPI acts as a lightweight notification mechanism rather than a data transport.

## An Alternative Simpler Approach

In Consortium, we provide an alternative and simpler approach focused on simplicity and auditability. The design abstracts communication patterns first, then integrates them with concrete communication components, especially shared-memory IPC.

### Generic Abstraction on IPC

An IPC transaction generally requires several components:

- a mechanism that prompts the receiver,
- a medium that carries the transmission content, and
- a pattern that converts messages into a communication-friendly format, namely serialization and deserialization.

Inspired by RPC, Consortium defines the following abstractions around these essential components:

2. `Chan`. Represents the hardware channel ID and is validated before use.

3. `Doorbell`, paired with `Chan`. A `Doorbell` is the mechanism that prompts the receiver. Consortium separates doorbells into three types:

   - Type-A (`BellA`): Explicit on-chip inter-processor doorbell. This includes HAMPU hardware mailbox peripherals used to fire interrupts between processors, such as HSEM (Hardware Semaphore), MU (Messaging Unit), and IPCC (Inter-processor Communication Controller).
   - Type-B (`BellB`): Implicit transport-coupled notification. This refers to notification behavior inherent to transport activity, such as UART, SPI, and similar communication interfaces.
   - Type-C (`BellC`): External or board-level notification, especially GPIO-based interrupts.

   A doorbell has a simple role: `ring()` triggers an interrupt to the peer core, `wait()` asynchronously waits for an interrupt from the peer core, and `pending()` performs a non-blocking check for an arrived interrupt. Each operation is paired with a specific `Chan`, which identifies the doorbell channel.

4. `Transport`. A `Transport` may be half-duplex, full-duplex, split-directional, or shared bidirectional. Like `Doorbell`, Consortium defines three transport categories:

   - Shared Memory. On-chip RAM-based transport, commonly used in HAMPU systems. In addition to the generic transport abstraction, Consortium introduces `Descriptor` and `Slot`.
   - Wired. Transport over a specific wired protocol, such as UART, SPI, CAN, or EtherCAT.
   - Wireless. Transport over a specific wireless protocol, such as Wi-Fi or Bluetooth Low Energy.

   A transport provides `send()` and `recv()` operations for data transmission and reception.

5. `Codec`. `Codec` is a project-wide abstraction used across Consortium, including IPC, TEE, and HMI. It defines how typed messages are serialized and deserialized. Refer to the standalone `Codec` section for more information.

6. `Channel`. In Consortium, IPC is composed of typed channels. All IPC transactions are strongly typed. A channel uses type generics to define its direction (`Tx` or `Rx`), the selected `Transport`, the selected `Codec`, and the message type.

7. `Transceiver`. A `Transceiver` is commonly used during initialization to expose the initialized IPC endpoint to the user. After initialization, the user can directly use `.[ipc name]` to send and receive messages, or split the endpoint into separate transmitter and receiver channels.

### Shared Memory Communication Fit

#### Abstraction Extensions

Based on the generic IPC abstraction, Consortium minimally extends the model to support shared-memory communication. The following abstractions are introduced:

1. `Region`: a memory region described by its base pointer, length, coherency policy, and cache maintenance actions such as flush and invalidate.

2. `Descriptor`: a shared handshake region that describes the section using fields such as magic, version, and protocol-specific handshake conventions.

3. `Slot`: a message payload region with flags and a defined format, either fixed-length or variable-length, optionally protected by CRC.

#### Memory Access

The Cortex-A side often runs a full operating system, which usually relies on virtual memory. In this scenario, Consortium adopts `mmap` for direct access to the shared physical memory region.

On Linux, Consortium provides a UIO-based access paradigm. Through UIO, the CA side can map the physical memory region into userspace and listen for interrupts through the UIO interrupt interface.

#### Cache Coherency

Consortium provides two options for maintaining cache coherency across different deployment scenarios:

1. Declare the shared memory region as non-cacheable.

   - On Cortex-A, the shared region is declared as `reserved-memory` and marked `no-map`.
   - On Cortex-M, such as M7 and M33, the MPU is configured to declare the shared memory region as outer-shareable and non-cacheable.

   In this case, the `flush()` and `invalidate()` actions defined by `Region` may be implemented as no-ops and eliminated by the compiler.

2. Maintain cache explicitly for cached mappings.

   In this mode, `Region` allows custom `flush()` and `invalidate()` actions to be defined. When `send()` and `recv()` are called by the transport, the corresponding cache maintenance actions are invoked automatically and paired with the required memory fence or barrier operations.
