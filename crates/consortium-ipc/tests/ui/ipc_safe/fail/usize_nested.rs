use consortium_ipc_proc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    value: Option<usize>,
}

fn main() {}
