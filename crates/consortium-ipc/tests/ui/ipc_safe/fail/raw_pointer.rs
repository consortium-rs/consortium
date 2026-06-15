use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    ptr: *const u8,
}

fn main() {}
