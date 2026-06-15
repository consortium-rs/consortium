use consortium_tee::TeeParam;

#[derive(TeeParam)]
#[tee(buffer = 4096)]
struct Bad {
    body: u32,
}

fn main() {}
