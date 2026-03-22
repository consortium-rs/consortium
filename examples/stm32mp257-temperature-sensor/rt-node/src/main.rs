#![no_std]
#![no_main]

use consortium_rt::TxChannel;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());
    let dts = embassy_stm32::Dts::new(p.DTS);

    let channel = TxChannel::new::<TemperatureReading>(0);

    loop {
        let temp = dts.read_temperature();
        let time_since_boot_ms = embassy_time::Instant::now().as_millis();
        let reading = TemperatureReading {
            celsius: temp,
            time_since_boot_ms,
        };
        channel
            .send(reading)
            .await
            .map_err(|e| {
                defmt::error!("Failed to send temperature reading: {:?}", e);
            })
            .ok();
        embassy_time::Timer::after(core::time::Duration::from_secs(1)).await;
    }
}
