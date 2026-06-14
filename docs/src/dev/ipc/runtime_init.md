# Runtime Initialization Guide

This document describes how to initialize and use the `consortium-runtime-mcu` and
`consortium-runtime-app` crates for STM32MP2 (IPCC on MP21, HSEM or IPCC on
MP23/25) and i.MX 9x (MU) platforms.

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

```rust
use consortium_runtime_mcu::{descriptor, irq, slot_pair_ptrs};
use consortium_ipc_transport_mem::MemTransport;
use consortium_ipc_doorbell_hsem::HsemDoorbell;

// 1. Enable the HSEM IRQ (must be called before doorbell.wait())
unsafe { irq::enable_doorbell_interrupt() };

// 2. Create the doorbell (zero-cost)
let doorbell = HsemDoorbell::<1>::new();

// 3. Write the descriptor header to shared memory
// SAFETY: region_base must point to the shared DMA-coherent region
unsafe {
    let channels = [
        descriptor::ChanDesc {
            tx_offset: 0x100,
            tx_size: 0x208,    // 8-byte header + 512 bytes payload
            rx_offset: 0x400,
            rx_size: 0x208,
        },
    ];
    descriptor::write_descriptor(
        region_base,
        1,                      // num_channels
        512,                    // proposed_mtu
        512,                    // accepted_mtu
        &channels,
    );

    // Signal M-core readiness to A-core
    descriptor::set_state(region_base, descriptor::DescState::MReady);
}

// 4. Construct transport and channel
let region = SramRegion::new(region_base, region_size);
let (tx_ptr, rx_ptr) = slot_pair_ptrs(region_base, 0, 520);

let transport = unsafe {
    MemTransport::new(
        chan,
        doorbell,
        region,
        tx_ptr,
        rx_ptr,
        512, // MTU
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

- **`descriptor::write_descriptor()`** — Initializes the shared descriptor header
- **`descriptor::set_state(base, state)`** — Transitions descriptor state (e.g., `MReady → Ready`)
- **`irq::enable_doorbell_interrupt()`** — Enables the currently selected HSEM/MU doorbell interrupt when the IRQ number is verified in runtime code. IPCC IRQ numbers/routing remain board/PAC-provided.
- **`slot_pair_ptrs(base, idx, size)`** — Computes slot pointers from region base
- **`static_buf!(NAME, SIZE)`** — Declares properly aligned static buffers

### Features

- **`stm32mp21x`** — Enables IPCC doorbell support
- **`stm32mp23x`** / **`stm32mp25x`** — Enables HSEM and IPCC doorbell support
- **`imx93`** / **`imx95`** — Enables MU doorbell support (i.MX variants)
- **`imx9x`** — Umbrella feature (requires one of the above)

STM32MP21/23/25 expose IPCC1 at `0x4049_0000..=0x4049_03FF`. STM32MP23/25 also
expose the SmartRun IPCC2 block at `0x4625_0000..=0x4625_03FF`; STM32MP21 does
not have IPCC2.

## A-Core (`consortium-runtime-app`)

### Setup: Firmware Lifecycle and UIO Device

```rust
use consortium_runtime_app::{
    descriptor, remoteproc::{RemoteProc, RemoteProcState},
    open_rx_channel, open_tx_channel,
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
    descriptor::wait_for_ready(0, Duration::from_secs(10)).await?;

    // 3. Open the UIO device
    let dev = ipc_uio::UioDevice::open(0)?;

    // 4. Get a channel and construct the IPC Channel
    static RX_BUF: static_cell::StaticCell<[u8; 512]> =
        static_cell::StaticCell::new([0u8; 512]);

    let uio_ch = dev.channel(Chan::new(1)?)?;
    let rx_buf = RX_BUF.init([0u8; 512]);
    let rx: Channel<Rx, MyMessage, _> = Channel::new(chan, uio_ch, rx_buf);

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

### Convenience Functions

- **`open_rx_channel(uio_num, chan, buf)`** — Opens UIO device and creates an Rx channel
- **`open_tx_channel(uio_num, chan, buf)`** — Opens UIO device and creates a Tx channel

### State Polling

- **`descriptor::wait_for_ready(uio_num, timeout)`** — Polls descriptor until `Ready(3)`
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

## Complete Example: STM32MP25 with HSEM

### M-Core Firmware (`consortium-runtime-mcu`)

```rust
#![no_std]

use consortium_runtime_mcu::{descriptor, irq, slot_pair_ptrs};
use consortium_ipc_transport_mem::MemTransport;
use consortium_ipc_doorbell_hsem::HsemDoorbell;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    irq::enable_doorbell_interrupt();

    let doorbell = HsemDoorbell::<1>::new();
    let region = SramRegion::new(SHARED_MEMORY_BASE, SHARED_MEMORY_SIZE);

    unsafe {
        let channels = [descriptor::ChanDesc {
            tx_offset: 0x100,
            tx_size: 0x208,
            rx_offset: 0x400,
            rx_size: 0x208,
        }];
        descriptor::write_descriptor(
            SHARED_MEMORY_BASE,
            1,
            512,
            512,
            &channels,
        );
        descriptor::set_state(SHARED_MEMORY_BASE, descriptor::DescState::MReady);
    }

    let (tx_ptr, rx_ptr) = slot_pair_ptrs(SHARED_MEMORY_BASE, 0, 520);
    let transport = unsafe {
        MemTransport::new(
            Chan::new(0).unwrap(),
            doorbell,
            region,
            tx_ptr,
            rx_ptr,
            512,
        )
    };

    static_buf!(BUF, 512);
    let buf = BUF.init([0u8; 512]);
    let ch = Channel::new(Chan::new(0).unwrap(), transport, buf);

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
use consortium_runtime_app::{descriptor, remoteproc::*};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proc = RemoteProc::new(0);
    proc.set_firmware("firmware.elf")?;
    proc.start()?;
    proc.wait_state(RemoteProcState::Running, Duration::from_secs(5)).await?;

    descriptor::wait_for_ready(0, Duration::from_secs(10)).await?;

    let dev = ipc_uio::UioDevice::open(0)?;
    let ch = dev.channel(Chan::new(0)?)?;

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
