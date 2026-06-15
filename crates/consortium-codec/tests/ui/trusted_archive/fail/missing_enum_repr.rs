use consortium_codec::TrustedArchive;
use rkyv::{Archive, Serialize};

#[derive(Archive, Serialize, TrustedArchive)]
enum NakedEnum {
    A,
    B(u32),
}

fn main() {}
