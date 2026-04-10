#![no_std]
#![no_main]

extern crate optee_utee;

use consortium_tee_test::invoke_command;
use optee_utee::{Parameters, Result, ta_invoke_command};

#[ta_invoke_command]
fn invoker(command_id: u32, params: &mut Parameters) -> Result<()> {
    invoke_command(command_id, params)
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
