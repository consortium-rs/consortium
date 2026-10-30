#[derive(Clone, Copy, Debug, IpcSafe, Serialize, Deserialize)]
pub struct SensorReading {
    pub temperature_mdc: i32,
    pub seq: u32,
}
