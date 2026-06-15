use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct Bad {
    callback: fn(u32) -> u32,
}

fn main() {}
