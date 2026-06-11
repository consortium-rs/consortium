use consortium_ipc_proc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    callback: fn(u32) -> u32,
}

fn main() {}
