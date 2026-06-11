use consortium_ipc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad<'a> {
    value: &'a u32,
}

fn main() {}
