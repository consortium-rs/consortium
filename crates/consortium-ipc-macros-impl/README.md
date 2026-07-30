# consortium-ipc-macros-impl

Derive implementations behind [`consortium-ipc-macros`](../consortium-ipc-macros).

> **Implementation detail.** Depend on `consortium-ipc`, which re-exports the
> derives next to the traits they implement. This crate's API is not stable.

## Why the split

`consortium-ipc-macros` must be a `proc-macro = true` crate, which can export only
macros and whose code runs inside the compiler. Moving the logic into a plain
library — this crate — buys testability: expansions are ordinary
`proc_macro2::TokenStream` values, so they can be asserted with unit tests and
`insta` snapshots (`tests/snap.rs`) instead of only through `trybuild` compile
failures in the consumer.

The layering the workspace uses for every macro family:

```text
consortium-ipc            re-exports the derive next to the trait
  └── consortium-ipc-macros        proc-macro facade, one line per macro
        └── consortium-ipc-macros-impl   expansion logic (this crate)
              └── consortium-macros-helpers   reusable syn/AST utilities
```

Reusable AST work — walking a type's field tree, collecting generic parameters —
belongs one level down in `consortium-macros-helpers`. What stays here is the
IPC-specific *policy*.

## `derive_ipc_safe`

Rejects, recursively through generics, arrays, and tuples:

| Construct                 | Why it cannot cross a core boundary                          |
| ------------------------- | ------------------------------------------------------------ |
| `*const T`, `*mut T`      | Addresses are meaningless in the peer's address space.        |
| `&T`, `&mut T`            | Same, plus no lifetime survives serialization.                |
| `fn(..)`                  | Code addresses are not portable across cores.                 |
| `usize`, `isize`          | Width differs between a 32-bit M-core and a 64-bit A-core.     |

Design notes:

- **All errors are accumulated** before returning, so a struct with three bad
  fields reports three diagnostics rather than one per rebuild.
- **Errors carry spans** from the offending type, so the message points at the
  field rather than at the `derive`.
- **Generic parameters gain an `IpcSafe` bound** rather than being inspected. That
  pushes the check to the instantiation site, so `Wrapper<T>` compiles but
  `Wrapper<usize>` does not.
- The emitted `unsafe impl` carries a `SAFETY` comment recording what the derive
  verified, because the workspace denies `undocumented_unsafe_blocks`.

## Testing

```bash
cargo test -p consortium-ipc-macros-impl   # unit + insta snapshots
cargo test -p consortium-ipc               # trybuild rejection cases
```

Update snapshots with `cargo insta review` when an expansion changes on purpose.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
