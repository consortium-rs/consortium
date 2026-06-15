use consortium_codec::TrustedArchive;
use rkyv::{Archive, Serialize};

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(C)]
struct Sensor {
    id: u32,
    value: f32,
    flags: u8,
}

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(align(16))]
struct AlignedBuffer {
    data: [u32; 4],
}

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(u8)]
enum Status {
    Idle,
    Running,
    Error(u32),
}

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(C)]
struct Wrapper<T> {
    value: T,
    tag: u32,
}

#[derive(Archive, Serialize, TrustedArchive)]
#[repr(C, align(16))]
struct AlignedC {
    x: u32,
    y: u32,
}

fn assert_trusted<T: consortium_codec::TrustedArchive>() {}

fn main() {
    assert_trusted::<Sensor>();
    assert_trusted::<AlignedBuffer>();
    assert_trusted::<Status>();
    assert_trusted::<AlignedC>();
    assert_trusted::<Wrapper<u32>>();
    assert_trusted::<Wrapper<[u64; 2]>>();
    // Nested struct: confirms the where T: TrustedArchive bound is sufficient
    // for a Wrapper whose field is itself a TrustedArchive type.
    assert_trusted::<Wrapper<Sensor>>();
}
