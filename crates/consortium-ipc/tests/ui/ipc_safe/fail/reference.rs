use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct Bad<'a> {
    value: &'a u32,
}

fn main() {}
