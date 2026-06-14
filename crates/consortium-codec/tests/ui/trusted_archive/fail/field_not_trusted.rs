use consortium_codec_macros::TrustedArchive;
use rkyv::{Archive, Serialize};

/// A type that derives rkyv Archive but does NOT implement TrustedArchive.
#[derive(Archive, Serialize)]
#[repr(C)]
struct Untrusted {
    x: u32,
}

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(C)]
struct Container {
    inner: Untrusted,
}

fn main() {}
