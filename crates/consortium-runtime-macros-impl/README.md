# consortium-runtime-macros-impl

Implementation of the Consortium runtime entry macros.

> **Implementation detail.** Depend on `consortium-runtime-mcu` or
> `consortium-runtime-app`, which re-export these under target-neutral names. This
> crate's API is not stable.

## Why the split

`consortium-runtime-macros` must be a `proc-macro = true` crate, which can export only
macros and whose code runs inside the compiler. Holding the logic in a plain library
makes expansions ordinary `proc_macro2::TokenStream` values, so they can be asserted with
unit tests and `insta` snapshots.

## Contents

| Module  | Macros                  | Generates                                      |
| ------- | ----------------------- | ---------------------------------------------- |
| `entry` | `mcu_main`, `app_main`   | Executor attribute, `init()` call, `Context` binding. |
| `fail`  | `mcu_fail`, `app_fail`   | An init-failure handler that shadows the fallback. |

Two families, one per runtime target, differing only where they must:

- **`mcu_main`** stacks `embassy_executor::main`; **`app_main`** stacks `tokio::main`.
  Both resolve the executor through the **user** crate's own dependency rather than
  through this one — a proc-macro cannot supply a runtime, and the user's executor
  configuration (thread mode, platform) has to be theirs.
- **`app_main`** additionally installs a `tracing` subscriber *before* the generated
  `init()` runs, so bring-up diagnostics are not dropped for want of a subscriber.
- The `fail` tails differ: firmware parks with `cortex_m::asm::wfi()` because it cannot
  proceed without a `Context`; an application calls `std::process::exit(1)` so a
  supervisor can restart it.

## The glob-import shadow

This is the mechanism worth understanding, because it explains a constraint the macros
impose on user code.

`#[main]`'s expansion calls `__consortium_init_fail(err)` when the generated `init()`
returns `Err`, and glob-imports a fallback for that name from the runtime crate's `__rt`
module. Applying `#[fail]` emits a **module-scope** `fn __consortium_init_fail` that
shadows the glob-imported one — a stable Rust guarantee: a local item shadows a glob
import. With no `#[fail]`, the fallback is used unchanged.

The payoff is that the user handler receives the *concrete* generated init-error type
with no trait object, no registration call, and no runtime dispatch. The cost is the
constraint:

> **`#[main]`, `#[fail]`, `init`, and `Context` must all live in the same module** — the
> one the generated `consortium.gen.rs` is `include!`d into. Shadowing does not reach
> across modules.

A `#[fail]` handler may return `()`, in which case the diverging tail runs afterwards, or
`!`, in which case it never returns.

## Testing

```bash
cargo test -p consortium-runtime-macros-impl   # unit + insta snapshots
```

Review intentional expansion changes with `cargo insta review`. The melt-pot example
`app`/`mcu` crates are the end-to-end check that the expansions actually compile against
generated code.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
