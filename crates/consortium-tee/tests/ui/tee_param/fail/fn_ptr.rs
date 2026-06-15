use consortium_tee::TeeParam;

#[derive(TeeParam)]
struct Bad {
    cb: fn(u32) -> u32,
}

fn main() {}
