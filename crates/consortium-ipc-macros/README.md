# consortium-ipc-macros

Proc-macro facade for [`consortium-ipc`](../consortium-ipc).

## Why this crate exists

A crate with `proc-macro = true` can export nothing but macros, and its code runs
in the compiler rather than in the target program. That makes it a poor place to
put logic. So the workspace splits every macro family in two:

| Crate                        | Role                                                      |
| ---------------------------- | --------------------------------------------------------- |
| `consortium-ipc-macros`      | `proc-macro` facade. Converts `TokenStream` and forwards. |
| `consortium-ipc-macros-impl` | Plain library holding the real expansion logic.           |

Everything here is one line per macro. The benefit lands in the impl crate, whose
expansions can be exercised with ordinary unit tests and `insta` snapshots rather
than only through `trybuild`.

## Contents

- `#[derive(IpcSafe)]` — assert that a struct or enum is safe to transmit as a
  cross-core IPC message, rejecting address-space-local field types anywhere in
  the field type tree.

## Usage

Do not depend on this crate directly. `consortium-ipc` re-exports the derive next
to the trait it implements, so one dependency is enough:

```rust,ignore
use consortium_ipc::IpcSafe; // both the trait and the derive

#[derive(IpcSafe)]
struct MotorCommand {
    motor_id: u8,
    target_rpm: i32,
}
```

See the [`consortium-ipc` README](../consortium-ipc/README.md) for what `IpcSafe`
guarantees and why.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
