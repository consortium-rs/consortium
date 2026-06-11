use consortium_tee::TeeParam;

#[derive(TeeParam)]
struct Bad<'a> {
    value: &'a u32,
}

fn main() {}
