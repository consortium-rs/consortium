# consortium-runtime-app

The Linux application runtime: remoteproc control, UIO mapping and IRQ fan-out,
host-side debug consoles, tracing setup, and Tokio entry glue.

```rust,ignore
#[consortium_runtime_app::main]
async fn main() -> Result<(), ConsortiumError> {
    let (p, c) = init(Config::default())?;
    let mut sensor =
        consortium_ipc_transport_memory::Transceiver::connect(c.SENSOR).await?;
    let message = sensor.recv().await?;
    Ok(())
}
```

`p` retains platform resources, with the debug-console field ordered before UIO owners
so its tasks stop before mappings unmap. Connected transports also retain their own UIO
owner clones. The entry macro accepts `()` or `Result<(), E: Debug>`; on error it
initializes tracing, logs, and exits non-zero.
