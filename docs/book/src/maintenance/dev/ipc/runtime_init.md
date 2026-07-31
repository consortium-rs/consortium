# Runtime Initialization Guide

This document describes how to initialize and use the `consortium-runtime-mcu` and
`consortium-runtime-app` crates for STM32MP2 (IPCC preferred on MP21/23/25; the
stale HSEM path exists only on MP23/25) and i.MX 9x (MU) platforms.

## Architecture Overview

The consortium runtime system requires coordination between M-core and A-core:

```
M-core                           A-core
------                           ------
1. Enable IRQs                   1. (optional) Start firmware
2. Create doorbell               2. Poll descriptor until Ready
3. Write descriptor + set MReady 3. Open UIO device
4. Construct Transport/Channel   4. Construct Transport/Channel
5. Run executor + tasks          5. Run Tokio + tasks
```

## M-Core (`consortium-runtime-mcu`)

### Setup: Interrupt and Descriptor Initialization

The snippet below uses the stale HSEM path. Prefer IPCC for new STM32MP2
firmware; retain HSEM only for existing STM32MP23/25 experiments.

```rust
use consortium_runtime_mcu::{consortium_ipc_transport_memory::descriptor, irq};
use consortium_ipc_transport_memory::transport::SharedMemoryTransport;
use consortium_ipc_doorbell_hsem::HsemDoorbell;

// 1. Enable the HSEM IRQ (must be called before doorbell.wait())
unsafe { irq::enable_doorbell_interrupt() };

// 2. Create the doorbell (zero-cost)
let doorbell = HsemDoorbell::<1>::new();

// 3. Write the descriptor header to shared memory
// SAFETY: region_base must point to the shared DMA-coherent region
unsafe {
    let channels = [
        descriptor::ChannelLayout {
            offset: 0x100,
            size: 0x208,    // 8-byte header + 512 bytes payload
            direction: descriptor::ChannelDirection::AToM,
        },
    ];
    descriptor::descriptor_init(
        region_base,
        1,                      // num_channels
        512,                    // transfer size
        &channels,
    );

    // Signal M-core readiness to A-core
    descriptor::set_state(region_base, descriptor::DescState::MReady);
}

// 4. Construct transport and channel
let region = SramRegion::new(region_base, region_size);
let transport = unsafe {
    SharedMemoryTransport::new(
        chan,
        doorbell,
        region,
        region_base,
    )
};

// 5. Declare and use static buffers
static_buf!(RX_BUF, 512);
let rx_buf = RX_BUF.init([0u8; 512]);
let rx_ch = Channel::new(chan, transport, rx_buf);

// 6. Run your async executor (Embassy, RTIC2, custom)
// Example with Embassy:
#[embassy_executor::task]
async fn ipc_receiver(ch: Channel<Rx, MyMessage, _>) {
    loop {
        match ch.recv().await {
            Ok(msg) => {
                // Process message
            }
            Err(e) => {
                // Handle error
            }
        }
    }
}
```

### Key Components

- **`descriptor::descriptor_init()`** — Initializes the shared descriptor header
- **`descriptor::set_state(base, state)`** — Transitions descriptor state (e.g., `MReady → Ready`)
- **`irq::enable_doorbell_interrupt()`** — Enables the currently selected HSEM/MU doorbell interrupt when the IRQ number is verified in runtime code. Prefer IPCC on STM32MP2; IPCC IRQ numbers/routing remain board/PAC-provided.
- **`slot_pair_ptrs(base, idx, size)`** — Computes raw slot pointers for low-level setup
- **`static_buf!(NAME, SIZE)`** — Declares properly aligned static buffers

### Features

- **`stm32mp21x`** — Enables IPCC doorbell support
- **`stm32mp23x`** / **`stm32mp25x`** — Enables IPCC and stale HSEM doorbell support; prefer IPCC
- **`imx93`** / **`imx95`** — Enables MU doorbell support (i.MX variants)
- **`imx9x`** — Umbrella feature (requires one of the above)

On i.MX95, the current CA55 <-> CM7 IPC default is MU7 with flat channel IDs 24 and
25 (`IMX95_CA55_CM7_TX_CHAN` / `IMX95_CA55_CM7_RX_CHAN`). MU1-4 connect CA55
<-> CM33, MU5 connects CM7 <-> CM33, MU6 connects NONE <-> CM33, and MU7-8
connect CA55 <-> CM7.

STM32MP21/23/25 expose IPCC1 at `0x4049_0000..=0x4049_03FF`. STM32MP23/25 also
expose the SmartRun IPCC2 block at `0x4625_0000..=0x4625_03FF`; STM32MP21 does
not have IPCC2.

## A-Core (`consortium-runtime-app`)

### Setup: Firmware Lifecycle and UIO Device

```rust
use consortium_runtime_app::{
    mmap, remoteproc::{RemoteProc, RemoteProcState}, uio::UioDevice,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. (Optional) Start the M-core firmware
    let proc = RemoteProc::new(0);
    proc.set_firmware("firmware.elf")?;
    proc.start()?;

    // Wait for firmware to boot and transition the descriptor to Ready
    proc.wait_state(RemoteProcState::Running, Duration::from_secs(5)).await?;

    // 2. Poll descriptor until M-core signals Ready(3)
    mmap::wait_for_ready(0, Duration::from_secs(10)).await?;

    // 3. Open the UIO device
    let dev = UioDevice::open(0, 2, 512)?;

    // 4. Get a channel and construct the IPC Channel
    static RX_BUF: static_cell::StaticCell<[u8; 512]> =
        static_cell::StaticCell::new([0u8; 512]);

    let chan = Chan::new::<HsemChanValidate>(2)?;
    let transport = dev.channel_transport(chan, doorbell, region)?;
    let rx_buf = RX_BUF.init([0u8; 512]);
    let rx: Channel<Rx, MyMessage, _> = Channel::new(chan, transport, rx_buf);

    // 5. Spawn receiver task
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => println!("Received: {:?}", msg),
                Err(e) => eprintln!("Recv error: {}", e),
            }
        }
    });

    // Keep main task alive
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

### UIO Transport Construction

- **`UioDevice::open(uio_num, num_channels, transfer_size)`** — Opens UIO, mmaps resource 0,
  parses the descriptor, and starts the IRQ listener task.
- **`UioDevice::channel_transport(chan, doorbell, region)`** — Creates a
  `SharedMemoryTransport` for one logical channel.

### State Polling

- **`mmap::wait_for_ready(uio_num, timeout)`** — Polls descriptor until `Ready(3)`
  - Returns early if descriptor enters `Error` or `Suspended` state
  - Mmaps UIO resource 0 and polls at 50ms intervals

- **`RemoteProc::wait_state(target, timeout)`** — Polls remoteproc sysfs state
  - Example: `proc.wait_state(RemoteProcState::Running, Duration::from_secs(5))`

## Shared Memory Layout

The descriptor occupies the first 0x20 + 16×N bytes of the shared region:

```
Offset  Size  Field
0x00    4     magic           (0x434F_4E53 = "CONS")
0x04    1     version         (1)
0x05    1     num_channels
0x06    2     reserved
0x08    4     proposed_mtu    (kernel/A-core → M-core)
0x0C    4     accepted_mtu    (M-core writes back)
0x10    4     effective_mtu   (final agreed MTU)
0x14    4     state           (DescState enum)
0x18    8     reserved
0x20    16×N  channel table   (tx_offset, tx_size, rx_offset, rx_size per channel)
```

Slot layout (at each offset):

```
0x00    4     length (payload bytes)
0x04    4     flags (0x01 = VALID, 0x02 = CONSUMED)
0x08    N     payload (up to effective_mtu bytes)
```

## Stale Example: STM32MP25 with HSEM

Prefer IPCC for new STM32MP2 integrations. This HSEM example remains as a
reference for existing STM32MP23/25 experiments only.

### M-Core Firmware (`consortium-runtime-mcu`)

```rust
#![no_std]

use consortium_runtime_mcu::{consortium_ipc_transport_memory::descriptor, irq};
use consortium_ipc_transport_memory::transport::SharedMemoryTransport;
use consortium_ipc_doorbell_hsem::HsemDoorbell;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    irq::enable_doorbell_interrupt();

    let doorbell = HsemDoorbell::<1>::new();
    let region = SramRegion::new(SHARED_MEMORY_BASE, SHARED_MEMORY_SIZE);

    unsafe {
        let channels = [descriptor::ChannelLayout {
            offset: 0x100,
            size: 0x208,
            direction: descriptor::ChannelDirection::AToM,
        }];
        descriptor::descriptor_init(
            SHARED_MEMORY_BASE,
            1,
            512,
            &channels,
        );
        descriptor::set_state(SHARED_MEMORY_BASE, descriptor::DescState::MReady);
    }

    let transport = unsafe {
        SharedMemoryTransport::new(
            Chan::new(2).unwrap(),
            doorbell,
            region,
            SHARED_MEMORY_BASE,
        )
    };

    static_buf!(BUF, 512);
    let buf = BUF.init([0u8; 512]);
    let ch = Channel::new(Chan::new(2).unwrap(), transport, buf);

    spawner.spawn(receiver(ch)).unwrap();
}

#[embassy_executor::task]
async fn receiver(ch: Channel<Rx, u32, _>) {
    loop {
        match ch.recv().await {
            Ok(msg) => {
                // Echo the message back
            }
            Err(e) => {
                // Log error
            }
        }
    }
}
```

### A-Core Application (`consortium-runtime-app`)

```rust
use consortium_runtime_app::{mmap, remoteproc::*, uio::UioDevice};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proc = RemoteProc::new(0);
    proc.set_firmware("firmware.elf")?;
    proc.start()?;
    proc.wait_state(RemoteProcState::Running, Duration::from_secs(5)).await?;

    mmap::wait_for_ready(0, Duration::from_secs(10)).await?;

    let dev = UioDevice::open(0, 2, 512)?;
    let ch = Chan::new::<HsemChanValidate>(2)?;

    println!("IPC ready!");
    Ok(())
}
```

## Error Handling

### Common Errors

- **`DescState::Error` or `Suspended`** — M-core did not complete initialization
- **`WaitError::Timeout`** — Firmware did not boot or descriptor not written within timeout
- **`UioDevice::open` returns "descriptor not ready"** — `wait_for_ready` was not called or timed out before opening

### Debugging Tips

1. Check `/sys/class/remoteproc/remoteproc0/state` to verify firmware is running
2. Check `/sys/class/uio/uio0/maps/map0/` to verify memory mapping
3. Verify M-core writes descriptor magic at offset 0x00 (should be `0x434F_4E53`)
4. Check descriptor state at offset 0x14 (expected: 3 = Ready)

---

See the plan file at `.claude/plans/rustling-beaming-adleman.md` for architectural rationale and design decisions.
