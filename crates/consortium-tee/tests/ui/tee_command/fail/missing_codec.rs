extern crate self as optee_utee;
extern crate alloc;

use consortium_tee::{TeeParam, tee_command};

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

impl Param {
    pub unsafe fn as_memref(&mut self) -> Result<Memref> {
        Ok(Memref { buffer: &mut [] })
    }
}

pub struct Memref {
    buffer: &'static mut [u8],
}

impl Memref {
    pub fn buffer(&mut self) -> &mut [u8] {
        self.buffer
    }
}

#[derive(TeeParam)]
struct Config {
    value: u32,
}

#[tee_command]
fn configure(_config: Config) -> optee_utee::Result<()> {
    Ok(())
}

fn main() {}
