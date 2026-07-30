# consortium-runtime-macros

Proc-macro facade for the Consortium runtime entry points.

## Why this crate exists

A crate with `proc-macro = true` can export nothing but macros, and its code runs in the
compiler rather than in the target program — a poor home for logic. So each macro family
in this workspace is split in two:

| Crate                            | Role                                                |
| -------------------------------- | --------------------------------------------------- |
| `consortium-runtime-macros`      | `proc-macro` facade. Converts `TokenStream`, forwards. |
| `consortium-runtime-macros-impl` | Plain library holding the expansion logic and tests.   |

## Contents

Four macros, two per runtime target. The runtime crates re-export them under
target-neutral names, which is why firmware and applications both write `#[main]`:

| This crate  | Re-exported as                       |
| ----------- | ------------------------------------ |
| `mcu_main`  | `#[consortium_runtime_mcu::main]`     |
| `mcu_fail`  | `#[consortium_runtime_mcu::fail]`     |
| `app_main`  | `#[consortium_runtime_app::main]`     |
| `app_fail`  | `#[consortium_runtime_app::fail]`     |

## Usage

Do not depend on this crate directly. Depend on the runtime for your target — the
`macros` feature is on by default in both:

```rust,ignore
// Firmware
#[consortium_runtime_mcu::main]
async fn main(context: Context, spawner: Spawner) { /* ... */ }

// Application
#[consortium_runtime_app::main]
async fn main(context: Context) -> Result<(), Box<dyn std::error::Error>> { /* ... */ }
```

Each `#[main]` stacks the appropriate executor attribute — `embassy_executor::main` or
`tokio::main` — resolved through the **user** crate's own dependency. So firmware still
needs `embassy-executor` with `executor-thread` and `platform-cortex-m`, and an
application still needs `tokio` with `macros`. Then it calls the generated `init()` and
binds its result as `context`.

`#[fail]` marks an optional init-failure handler. Without one, firmware parks with `wfi`
and an application exits non-zero.

**`#[main]`, `#[fail]`, `init`, and `Context` must share the module the generated
`consortium.gen.rs` is `include!`d into** — the handler dispatch is a glob-import
shadow, which only works within one module. See the
[impl crate's README](../consortium-runtime-macros-impl/README.md) for why.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
