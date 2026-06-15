use consortium_tee::TeeParam;

#[derive(TeeParam)]
struct Bad {
    erased: dyn Send,
}

fn main() {}
