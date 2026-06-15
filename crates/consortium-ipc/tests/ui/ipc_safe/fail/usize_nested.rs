use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    value: Option<usize>,
}

fn main() {}
