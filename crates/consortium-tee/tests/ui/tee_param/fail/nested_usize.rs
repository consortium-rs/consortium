use consortium_tee::TeeParam;

#[derive(TeeParam)]
struct Bad {
    value: Option<usize>,
}

fn main() {}
