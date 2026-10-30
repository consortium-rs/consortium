extern crate self as optee_utee;
extern crate alloc;

include!("../../../support/optee_utee_mock.rs");

use consortium_tee::{TeeParam, tee_command};

#[derive(TeeParam)]
struct Config {
    value: u32,
}

enum Command {
    Configure,
}

#[tee_command]
fn configure(_config: Config) -> optee_utee::Result<()> {
    Ok(())
}

fn main() {}
