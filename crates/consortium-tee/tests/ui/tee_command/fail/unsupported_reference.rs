extern crate self as optee_utee;

use consortium_tee::tee_command;

pub type Result<T> = core::result::Result<T, Error>;

pub struct Error;

impl Error {
    pub fn new(_kind: ErrorKind) -> Self {
        Self
    }
}

pub enum ErrorKind {
    BadParameters,
}

pub struct Parameters(pub Param, pub Param, pub Param, pub Param);

pub struct Param;

enum Command {
    Unsupported,
}

#[tee_command]
fn unsupported(_value: &u32) -> optee_utee::Result<()> {
    Ok(())
}

fn main() {}
