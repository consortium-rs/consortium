use consortium_tee::TeeParam;

#[derive(TeeParam)]
struct Bad {
    ptr: *mut u8,
}

fn main() {}
