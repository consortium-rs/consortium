#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemperatureReading {
    pub celsius: f32,
    pub time_since_boot_ms: u64,
}
