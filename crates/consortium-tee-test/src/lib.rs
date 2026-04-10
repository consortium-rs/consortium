#![cfg_attr(feature = "ta", no_std)]

use consortium_tee::{tee_command, tee_service};

#[tee_command]
fn hello_world(a: u32) -> u32 {
    a
}

tee_service! {
    commands { hello_world }
}
