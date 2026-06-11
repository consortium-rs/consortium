use consortium_ipc_proc_macros::IpcSafe;

#[derive(IpcSafe)]
struct Bad<'a> {
    value: &'a u32,
}

fn main() {}
