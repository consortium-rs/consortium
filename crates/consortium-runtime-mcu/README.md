# consortium-runtime-mcu

The `no_std` controller-core runtime: Embassy entry glue, remoteproc resource tables,
MPU helpers, and feature-gated IPC/debug re-exports.

```rust,ignore
#[consortium_runtime_mcu::main]
async fn main(_spawner: Spawner) -> Result<(), ConsortiumError> {
    let (p, c) = init(Config::default());
    let mut sensor =
        consortium_ipc_transport_memory::Transceiver::connect(c.SENSOR).await?;
    let bus = I2c::new(p.LPI2C1, LPI2C1_CLOCK_HZ, LPI2C1_FREQUENCY);
    Ok(())
}
```

Generated `init` performs synchronous local bring-up and installs each unsplit
transport. Connecting one channel performs its descriptor handshake before splitting,
so an absent peer does not stall unrelated endpoints. An entry `Err(E: Debug)` is logged
and parks the core with `wfi`.

The firmware owns doorbell and peripheral interrupt handlers; the runtime and hardware
crates expose forwarding hooks but do not install application vectors.
