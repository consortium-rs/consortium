# Generated Runtime

Every endpoint includes a generated `consortium.gen.rs`. It defines `Config`,
`Peripherals`, `Channels`, endpoint error types, and a synchronous `init(config)`
specialized from the manifest. Initialization is explicit at the application call site;
the entry macro only installs the executor and handles the entry function's outcome.

## Define shared messages

With the default postcard codec, a shared message derives serde traits and `IpcSafe`:

```rust,ignore
use consortium_ipc::IpcSafe;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, IpcSafe, Serialize)]
pub struct SensorReading {
    pub temperature_mdc: i32,
    pub sequence: u32,
}
```

Fixed-width integers are intentional. References, pointers, function pointers,
`usize`, and `isize` are rejected because they are not portable across address spaces.

## Linux application entry

```rust,ignore
use consortium_ipc_transport_memory::Transceiver;

include!("consortium.gen.rs");

#[consortium_runtime_app::main]
async fn main() -> Result<(), ConsortiumError> {
    let (p, c) = init(Config::default())?;
    let mut sensor = Transceiver::connect(c.SENSOR).await?;

    loop {
        let message = sensor.recv().await?;
        tracing::info!(sequence = message.sequence, "sensor update");
    }
}
```

The application still needs `tokio` with its `macros` feature. `p` owns platform
resources such as UIO mappings and the optional debug console. Each connected transport
also retains the mapping owners it uses, so moving or dropping unrelated `p` fields does
not dangle an endpoint.

## Firmware entry

```rust,ignore
#![no_std]
#![no_main]

use consortium_ipc_transport_memory::Transceiver;
use embassy_executor::Spawner;

include!("consortium.gen.rs");

#[consortium_runtime_mcu::main]
async fn main(_spawner: Spawner) -> Result<(), ConsortiumError> {
    let (p, c) = init(Config::default());
    let mut sensor = Transceiver::connect(c.SENSOR).await?;
    let mut heartbeat = Gpio::new(p.GPIO2).pin(0).into_output(Level::Low);

    // Use `sensor` and `heartbeat`.
    Ok(())
}
```

The firmware crate needs `embassy-executor` with the target executor features. The entry
macro accepts `()` or `Result<(), E>` where `E: core::fmt::Debug`; an error is logged and
parks firmware with `wfi`, while the Linux runtime logs and exits non-zero.

## What `init(config)` owns

Generated initialization performs synchronous, local bring-up:

- takes the chip HAL and generated channel singleton bundles once, panicking on a
  second call;
- opens UIO devices or constructs physical shared-memory regions;
- binds Linux interrupt fan-out services;
- installs each unsplit IPC transport and its two take-once scratch buffers behind the
  matching `c.NAME` token;
- resets a configured firmware debug ring and starts its host decoder when enabled;
- brings up the generated Embassy time driver when enabled; and
- returns only the peripheral singletons allowed by `[peripheral.*]`.

The MCU `Config` controls logger, MPU, NVIC, and time-driver steps. The AP `Config`
controls tracing, UIO IRQ listeners, and the debug console. Every field defaults to
`true`. Cache enablement is intentionally not optional when the manifest declares a
cached shared window.

## Per-channel connection

`Transceiver::connect(c.SENSOR).await?` is the concise path. To retain an unconnected
endpoint for later or move it into another task, use:

```rust,ignore
let endpoint = Endpoint::new(c.SENSOR);
let sensor = endpoint.connect().await?;
```

Both spellings run descriptor negotiation before `SharedMemoryTransport::split()`, then
construct the typed Tx and Rx channels. A missing peer therefore blocks only the channel
being connected. Timeout policy belongs to the caller.

## Interrupt handlers remain application-owned

Hardware crates do not install interrupt vectors. Firmware enables the chip HAL's `rt`
feature, defines the relevant handler, and forwards it to the exported wake path:

- HSEM: `hsem::notify()`;
- IPCC RX: `ipcc::notify_rx_occupied_interrupt()`;
- MU: `mu::notify::<N>()`; and
- buffered UART: the HAL's per-instance interrupt hook.

Generated code installs only the selected time-driver vector. The melt-pot firmware
`main.rs` files are the current worked examples.

## Receiving and codecs

`recv()` returns `ReceivedMessage`, not necessarily an owned message. Postcard and prost
decode to owned values, while rkyv can borrow an archived value from the receive buffer.
Use the wrapper by reference where possible, or call `into_inner()` when appropriate for
the selected codec.
