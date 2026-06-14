use consortium_codec_macros::TrustedArchive;
use rkyv::{Archive, Serialize};

#[derive(Archive, Serialize, TrustedArchive)]
struct NakedStruct {
    x: u32,
}

fn main() {}
