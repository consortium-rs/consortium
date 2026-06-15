#![allow(deprecated)]
extern crate alloc;

use consortium_tee::TeeParam;

#[derive(TeeParam)]
#[tee(max_size = 4096)]
struct LargePayload {
    id: u32,
    tag: u16,
}

fn main() {}
