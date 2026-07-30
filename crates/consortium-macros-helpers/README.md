# consortium-macros-helpers

Reusable `syn` predicates shared by Consortium's macro implementation crates.

## Why it exists

Three macro families keep asking the same questions of an AST:

| Consumer                       | Needs to know                                                       |
| ------------------------------ | ------------------------------------------------------------------- |
| `consortium-ipc-macros-impl`   | Is this field type a `usize`, a pointer, a reference?               |
| `consortium-codec-macros-impl` | What are the field types, and the generics inside them?             |
| `consortium-tee-macros-impl`   | Is this parameter a `&mut [u8]`, a `&mut u32`, a codec-routed path? |

Those questions have fiddly answers and no natural home in any one derive, so they live
here. The rule the workspace follows:

> **Reusable AST helpers go in this crate; domain-specific policy stays in the owning
> `*-macros-impl` crate.**

`IpcSafe` rejecting `usize` is policy. Recognizing that a `Type` _is_ a `usize` is a
helper.

## Everything here is syntactic — and that matters

A proc-macro runs **before type resolution**, so it sees tokens, not types.
`is_vec_u8` matches the _path_ `Vec<u8>`. It cannot know that

```rust,ignore
type MyBuf = Vec<u8>;
```

is the same thing, and it will happily match a user's own unrelated `Vec` type.

That is a real limitation rather than an oversight — no proc-macro can do better — and it
shapes how the derives are written. Where a syntactic check is not enough, the derives
**push the question to the compiler** by emitting a `where`-clause bound instead. That is
why `#[derive(IpcSafe)]` adds `T: IpcSafe` to a generic parameter rather than trying to
inspect it: `Wrapper<T>` then compiles and `Wrapper<usize>` does not, decided by the
type checker rather than by a token match.

## Contents

| Group               | Helpers                                                          |
| ------------------- | ---------------------------------------------------------------- |
| Field/type walking  | `collect_field_types`, `path_type_arguments`                     |
| Return types        | `get_output_type`, `is_result_returning`, `extract_result_inner` |
| Primitive shapes    | `is_unit_type`, `is_primitive`, `is_vec_u8`, `is_tuple_u32_u32`  |
| Reference shapes    | `is_slice_ref`, `mut_scalar_inner`, `is_context_ref`             |
| TEE parameter kinds | `is_tee_param_path`, `is_mut_tee_param_ref`                      |

`get_output_type` is worth calling out: it unwraps `Result<T, E>` and treats `()` as no
output, which is what lets `#[tee_command]` handle `-> T`, `-> Result<T, E>`, and `-> ()`
uniformly without three code paths.

The TEE group is TEE-flavoured by name but generic in substance: it distinguishes "a path
type routed through a codec" from "a primitive that fits in a value slot", which is the
classification `tee_command` needs when assigning parameter slots.

## Adding a helper

Add it here if a second macro crate would want it, and if it answers a question about
_syntax_. Keep the policy — which types are acceptable, what the error message says —
in the derive that owns the decision.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
