# Inter-process communication (IPC)

Consortium IPC provides a unified API for sending and receiving messages between the different components of CPUs. It abstracts away the underlying communication mechanisms, allowing developers to focus on the logic of their applications rather than the details of inter-process communication.

The abstraction of the Consortium IPC is as follows:

```
doorbell -> transport -> codec -> channel
```

- **Doorbell**: The doorbell is the hardware mechanism that triggers the communication between the different components. It can be implemented using various hardware mechanisms, such as semaphores, message units, or shared memory. The Consortium IPC provides a unified API for interacting with the doorbell, allowing developers to easily trigger communication between the components.
- **Transport**: The transport layer is responsible for managing the communication channel between the different components. It handles the synchronization and coordination of messages, ensuring that messages are delivered reliably and in order. The Consortium IPC provides a unified API for interacting with the transport layer, allowing developers to easily manage the communication channel between the components.
- **Codec**: The codec layer is responsible for serializing and deserializing messages between the different components. It ensures that messages are properly formatted and can be correctly interpreted by the receiving component. The Consortium IPC provides a unified API for interacting with the codec layer, allowing developers to easily serialize and deserialize messages between the components.
- **Channel**: The channel layer is the abstraction that represents the communication channel between the different components. It provides a simple and consistent API for sending and receiving messages, abstracting away the complexities of the underlying communication mechanisms. The Consortium IPC provides a unified API for interacting with the channel layer, allowing developers to easily send and receive messages between the different components.

The final effect of the Consortium IPC is that developers can send and receive messages between the different components using a simple and consistent API, without worrying about the details of inter-process communication. This allows developers to focus on building innovative applications rather than dealing with the complexities of inter-component communication.

## IPC Layer Stack

The Consortium IPC is organized as a series of abstraction layers:

```
┌──────────────────────────────────────────────────────────────┐
│  Application Layer                                           │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Channel<Tx/Rx, MessageType, Transport, Codec>          │ │
│  │  .send(msg).await? / .recv().await?                     │ │
│  │  Type-safe, allocation-free, async                      │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│  Codec Layer                                                 │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Codec Trait: T <─→ [u8]                                │ │
│  │  Default: PostcardCodec (serde-based, compact binary)   │ │
│  │  Future: Zero-copy codecs (rkyv)                        │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│  Transport Layer                                             │
│  ├─ SharedMemoryTransport (M-core / shared memory)           │
│  │  Generic over Doorbell + Region                          │
│  │  Manages slot protocol, async polling                    │
│  └─ runtime-app UIO path (Linux A-core)                      │
│     mmap-backed, Tokio interrupt task                       │
└──────────────────────────────────────────────────────────────┘
                           │
┌────────────┬─────────────┴──────────────┬───────────┐
│            │                            │           │
▼            ▼                            ▼           ▼
Doorbell   Slot Protocol              Region        Channel
 │           │                          │            ID
 │           ▼                          ▼            │
 │   [len:u32|flags:u32|data]   Cache coherency   Chan(u8)
 │                              Management
 ▼
Hardware Signaling
├─ HSEM (STM32MP23/25)
├─ IPCC (STM32MP21/23/25)
├─ MU (i.MX)
└─ GPIO/Notify (testing)
```

## Developer Experience

### M-Core (Bare-Metal Firmware)

```rust
// Static buffer for IPC (sized to max message MTU)
static mut TX_BUF: [u8; 256] = [0u8; 256];
static mut HSEM_DOORBELL: HsemDoorbell = /* ... */;
static mut REGION: DdrRegion = /* ... */;

#[main]
async fn main() {
    // 1. Create transport (owning doorbell, managing slots)
    let mut transport = SharedMemoryTransport::new(
        chan,
        doorbell,
        region,
        unsafe { SHARED_REGION_BASE },
    );

    // 2. Create typed channel for a specific message
    let mut tx: Channel<Tx, SensorData, _, PostcardCodec> =
        Channel::new(chan, transport, unsafe { &mut TX_BUF });

    // 3. Send data (async, awaitable)
    loop {
        let data = SensorData {
            temperature: 26.9,
            timestamp_ms: system_time_ms(),
        };
        tx.send(&data).await?;  // blocks until A-core reads
        delay_ms(1000);
    }
}
```

### A-Core (Linux + Tokio)

```rust
use consortium_runtime_app::{mmap, remoteproc::RemoteProc, uio::UioDevice};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load and start firmware
    let remoteproc = RemoteProc::new(0);
    remoteproc.set_firmware("firmware.elf")?;
    remoteproc.start()?;

    // 2. Wait for the descriptor, then open UIO
    mmap::wait_for_ready(0, Duration::from_secs(10)).await?;
    let uio = UioDevice::open(0, 2, 512)?;

    // 3. Create typed channel matching M-core
    let mut rx: Channel<Rx, SensorData, _, PostcardCodec> =
        Channel::new(chan, uio.channel_transport(chan, doorbell, region)?, unsafe { &mut RX_BUF });

    // 4. Receive messages in loop (async, awaitable)
    loop {
        let msg = rx.recv().await?;
        println!("Temp: {}°C at {}ms", msg.temperature, msg.timestamp_ms);
    }

    // Cleanup
    remoteproc.stop()?;
    Ok(())
}
```
