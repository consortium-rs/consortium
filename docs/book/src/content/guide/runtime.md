# Generated Runtime

Every endpoint includes a generated `consortium.gen.rs`. It defines the endpoint's
`Context`, error types, and `async fn init()`, all specialized from the same manifest.
The runtime entry macros call that function before user code begins.

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

#[derive(Clone, Copy, Debug, Deserialize, IpcSafe, Serialize)]
pub struct Command {
    pub set_point_mdc: i32,
}
```

Fixed-width integers are intentional. References, pointers, function pointers,
`usize`, and `isize` are not meaningful across different address spaces or word sizes
and are rejected by `IpcSafe`.

## Linux application entry

Keep the generated include and entry functions in the same module:

```rust,ignore
use shared::SensorReading;

include!("consortium.gen.rs");

#[consortium_runtime_app::fail]
fn on_init_failure(error: ConsortiumInitError) {
    eprintln!("Consortium initialization failed: {error:?}");
}

#[consortium_runtime_app::main]
async fn main(mut context: Context) {
    loop {
        let message = context.ipc_shm.sensor.recv().await.expect("IPC receive");
        let reading: SensorReading = message.into_inner();
        tracing::info!(sequence = reading.sequence, "sensor update");
    }
}
```

The macro installs a Tokio entry point and tracing subscriber, awaits generated
initialization, then calls the function with a ready `Context`. The application crate
still needs its own `tokio` dependency with the `macros` feature because the generated
attribute resolves through the user crate.

## Firmware entry

Firmware uses the matching Embassy entry macro:

```rust,ignore
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use shared::SensorReading;

include!("consortium.gen.rs");

#[consortium_runtime_mcu::main]
async fn main(context: Context, _spawner: Spawner) {
    let mut sensor = context.ipc_shm.sensor;
    let mut sequence = 0;

    loop {
        let reading = SensorReading {
            temperature_mdc: 25_000,
            sequence,
        };
        sensor.send(&reading).await.expect("IPC send");
        sequence = sequence.wrapping_add(1);
    }
}
```

The firmware crate must depend on `embassy-executor` with the target's executor
features. Without a custom `#[consortium_runtime_mcu::fail]` handler, initialization
failure parks the core with `wfi`; the Linux fallback logs and exits non-zero.

## What `init()` owns

Depending on the endpoint and manifest, generated initialization:

- takes the chip HAL's peripheral singletons once;
- constructs UIO mappings or physical shared-memory regions;
- binds Linux interrupt fan-out services;
- performs every IPC readiness handshake, splits transports, and creates typed
  transceivers;
- resets a configured firmware debug ring and starts its host decoder;
- brings up the generated Embassy time driver; and
- returns controller-owned peripheral drivers under `context.peripherals`.

The `Context` must stay alive while those services are used. In particular, a Linux
context owns its UIO devices and automatic debug-console tasks.

## Interrupt handlers remain application-owned

Hardware crates do not install interrupt vectors. Firmware enables the chip HAL's `rt`
feature, defines the relevant handler, and forwards it to the exported wake path:

- HSEM: `hsem::notify()`;
- IPCC RX: `ipcc::notify_rx_occupied_interrupt()`;
- MU: `mu::notify::<N>()`; and
- buffered UART: the HAL's per-instance interrupt hook.

The melt-pot firmware `main.rs` files are the current worked examples. Generated code
does install the selected time-driver vector when `[profile] time_driver = true`.

## Receiving and codecs

`recv()` returns `ReceivedMessage`, not necessarily an owned message. Postcard and
prost decode to owned values, while rkyv can borrow an archived value from the receive
buffer. Use the wrapper by reference where possible, or call `into_inner()` only when
the selected codec's decoded type is appropriate.

For custom bring-up, the generated two-phase `IpcMemoryEndpoints::new` and `connect`
API remains available. Callers must connect once before any traffic and before
splitting a transport; timeout policy belongs to the caller.
