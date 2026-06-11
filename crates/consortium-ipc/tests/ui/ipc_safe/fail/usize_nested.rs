use consortium_ipc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    value: Option<usize>,
}

fn main() {}
