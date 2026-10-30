extern crate self as optee_utee;

include!("../../../support/optee_utee_mock.rs");

use consortium_tee::tee_command;

enum Command {
    Unsupported,
}

#[tee_command]
fn unsupported(_value: &u32) -> optee_utee::Result<()> {
    Ok(())
}

fn main() {}
