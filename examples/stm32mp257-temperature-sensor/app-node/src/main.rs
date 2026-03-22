#[tokio::main]
async fn main() {
    let channel = RxChannel::new::<TemperatureReading>(0);

    loop {
        match channel.recv().await {
            Ok(reading) => {
                tracing::info!("Received temperature reading: {:?}", reading);
            }
            Err(e) => {
                tracing::error!("Failed to receive temperature reading: {:?}", e);
            }
        }
    }
}
