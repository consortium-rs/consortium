use consortium_ipc::IpcSafe;

#[derive(IpcSafe)]
struct Header {
    id: u32,
    flags: u16,
    enabled: bool,
}

#[derive(IpcSafe)]
enum Message<T> {
    Empty,
    Header(Header),
    Payload {
        data: [u8; 8],
        meta: (u16, u32),
        value: T,
    },
}

fn assert_ipc_safe<T: consortium_ipc::IpcSafe>() {}

fn main() {
    assert_ipc_safe::<Header>();
    assert_ipc_safe::<Message<u32>>();
}
