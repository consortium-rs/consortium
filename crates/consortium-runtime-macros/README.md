# consortium-runtime-macros

Proc-macro facade for Consortium runtime entry points.

- `mcu_main` is re-exported as `consortium_runtime_mcu::main` and accepts
  `async fn main(spawner: Spawner)`.
- `app_main` is re-exported as `consortium_runtime_app::main` and accepts
  `async fn main()`.

Both accept either `()` or `Result<(), E>` where `E: core::fmt::Debug`. The macro keeps
the user's signature and body in an inner async function, stacks the appropriate
Embassy/Tokio executor on a unit-returning outer function, and reports the outcome
through the runtime's `MainOutcome` trait. Generated `init(Config)` remains explicit in
the user's body.
