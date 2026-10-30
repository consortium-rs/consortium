use consortium_tee::{tee_command, tee_service};

/// Runs inside OP-TEE. The macro also emits `call_check_temperature`
/// for the Linux side, so both halves share one signature.
#[tee_command]
pub fn check_temperature(temperature_mdc: i32) -> bool {
    (-20_000..=85_000).contains(&temperature_mdc)
}

tee_service! {
    commands { check_temperature }
}
