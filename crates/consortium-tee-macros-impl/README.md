# consortium-tee-macros-impl

Macro implementations behind [`consortium-tee-macros`](../consortium-tee-macros).

> **Implementation detail.** Depend on `consortium-tee`, which re-exports the macros
> next to the traits they implement. This crate's API is not stable.

## Why the split

`consortium-tee-macros` must be a `proc-macro = true` crate, which can export only
macros and whose code runs inside the compiler. Holding the logic in a plain library
makes expansions ordinary `proc_macro2::TokenStream` values, so they can be asserted
with unit tests and `insta` snapshots rather than only through `trybuild` failures.

That matters more here than for the other macro families in this workspace: these
macros generate **both sides of a privilege boundary**, and the generated code is not
something a reader can easily hold in their head. A snapshot diff is how a change to
slot assignment becomes reviewable.

```text
consortium-tee                    re-exports the macros next to the traits
  └── consortium-tee-macros            proc-macro facade
        └── consortium-tee-macros-impl      expansion logic (this crate)
              └── consortium-macros-helpers      reusable syn/AST predicates
```

## Contents

| Module        | Generates                                                         |
| ------------- | ----------------------------------------------------------------- |
| `tee_param`   | `impl<C: CodecFor<Self>> TeeParam<C>` for a user type.            |
| `tee_command` | `call_<name>` (CA) and `<name>_dispatched` (TA) for one function. |
| `tee_service` | The `Command` enum and `invoke_command` dispatch for a TA.        |

### `tee_param`

Validates fields against the same structural rules as `IpcSafe` — no raw pointers, no
bare references, no function pointers, no `usize`/`isize` — accumulating **all**
diagnostics before failing so a struct with three bad fields reports three errors.

The `usize` rule is load-bearing rather than pedantic: a 32-bit TA and a 64-bit CA
genuinely disagree on the width, so one `usize` field shifts every field after it.

`#[tee(max_size = N)]` sets `MAX_SIZE`, the Memref buffer the CA pre-allocates
(default 1024).

### `tee_command`

Classifies each parameter into a slot and a direction, then emits two functions plus
the original body unchanged:

- `<name>_dispatched` — TA side, `#[cfg(feature = "ta")]`. Unpacks
  `::optee_utee::Parameters` through `TeeParam`, calls the function, flushes `out` and
  `inout` values back into their slots.
- `call_<name>` — CA side, `#[cfg(feature = "ca")]`. Packs arguments, invokes the
  command, reads output slots back.

Attribute arguments are `codec = Path` and `ctx`. `ctx = name` and `ctx = 0` are
accepted for backward compatibility and emit a deprecation warning.

### `tee_service`

Builds the `Command` enum (PascalCase variants numbered from 0, plus a `#[default]
Unknown` catch-all) and the `invoke_command` that routes each variant to the matching
`<function>_dispatched`. `Unknown` is rejected as a user-supplied command name, since
it is the catch-all.

## The shared design decision

**The codec is a type parameter, never baked into an expansion.** `tee_param` emits an
impl generic over `C`; `tee_command` substitutes the `codec = …` argument at the call
site. A message type is therefore reusable across commands with different codecs, and
shareable with the IPC layer without inheriting its codec choice.

Reusable `syn` predicates — `is_primitive`, `is_slice_ref`, `is_tee_param_path`,
`collect_field_types`, and friends — live in `consortium-macros-helpers`, because
`tee_command`'s parameter classification and `tee_param`'s field validation need the
same AST questions answered. What stays here is TEE-specific policy: which Rust type
maps to which parameter slot, and in which direction.

## Testing

```bash
cargo test -p consortium-tee-macros-impl                      # unit + insta snapshots
cargo test -p consortium-tee --no-default-features --features ta
```

Review intentional expansion changes with `cargo insta review`. Update the `trybuild`
cases and snapshots here **before** relying on the proc-macro facade.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
