# consortium-tee-macros

Proc-macro facade for [`consortium-tee`](../consortium-tee).

## Why this crate exists

A crate with `proc-macro = true` can export nothing but macros, and its code runs in
the compiler rather than in the target program — a poor home for logic. Each macro
family in this workspace is therefore split in two:

| Crate                        | Role                                                     |
| ---------------------------- | -------------------------------------------------------- |
| `consortium-tee-macros`      | `proc-macro` facade. Converts `TokenStream`, forwards.    |
| `consortium-tee-macros-impl` | Plain library holding the expansion logic and its tests.   |

## Contents

The three macros together turn one Rust function into both halves of a TEE call.

| Macro                 | Generates                                                       |
| --------------------- | --------------------------------------------------------------- |
| `#[derive(TeeParam)]` | `impl<C: CodecFor<Self>> TeeParam<C>` — makes a type carryable in a Memref slot. |
| `#[tee_command]`      | `call_<name>` (CA side) and `<name>_dispatched` (TA side) for one function. |
| `tee_service!`        | The `Command` enum and the TA's `invoke_command` dispatch table.  |

## Usage

Do not depend on this crate directly. `consortium-tee` re-exports all three alongside
the traits they generate impls for:

```rust,ignore
use consortium_tee::{TeeParam, tee_command, tee_service};

#[derive(TeeParam, serde::Serialize, serde::Deserialize)]
#[tee(max_size = 256)]
struct MotorCommand { motor_id: u8, target_rpm: i32 }

#[tee_command(codec = PostcardCodec)]
fn set_motor(cmd: MotorCommand) -> Result<(), TeeError> { /* … */ }

tee_service! {
    commands { set_motor }
}
```

Notes that bite:

- **`codec` is per call site, not per type.** `#[derive(TeeParam)]` stays generic over
  the codec, so one type serves commands using different families. Omit `codec` only
  for primitive-only commands.
- **`Unknown` is a reserved command name** — `tee_service!` uses it for the
  `#[default]` catch-all variant.
- **`#[tee(max_size = N)]`** sets the Memref buffer the CA pre-allocates; the default
  is 1024 bytes.

See the [`consortium-tee` README](../consortium-tee/README.md) for the slot model,
the parameter type table, and why the derive rejects `usize`.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
