use consortium_ipc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    ptr: *const u8,
}

fn main() {}
