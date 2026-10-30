#[consortium_runtime_mcu::main]
async fn main(_s: Spawner) -> Result<(), ConsortiumError> {
    let (_p, c) = init(Config::default());
    let mut sensor = Transceiver::connect(c.SENSOR).await?;

    for seq in 0u32.. {
        let temperature_mdc = read_sensor();
        let reading = SensorReading { temperature_mdc, seq };
        sensor.send(&reading).await?;
        // IPCC1 rings; the CA35 wakes and decodes it
        Timer::after_secs(1).await;
    }
    Ok(())
}
