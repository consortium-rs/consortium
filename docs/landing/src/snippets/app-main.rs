#[consortium_runtime_app::main]
async fn main() -> Result<(), ConsortiumError> {
    let (_p, c) = init(Config::default())?;
    let mut sensor = Transceiver::connect(c.SENSOR).await?;

    let mut ctx = Context::new()?;
    let mut session = ctx.open_session(Uuid::parse_str(TA_UUID)?)?;

    loop {
        let reading: SensorReading = sensor.recv().await?.into_inner();
        let ok = call_check_temperature(&mut session, reading.temperature_mdc)?;
        tracing::info!(seq = reading.seq, ok, "verdict from the secure world");
    }
}
