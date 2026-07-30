# consortium-codec-macros-impl

Derive implementations behind [`consortium-codec-macros`](../consortium-codec-macros).

> **Implementation detail.** Depend on `consortium-codec` with the `rkyv` feature;
> it re-exports the derive next to the trait. This crate's API is not stable.

## Why the split

`consortium-codec-macros` must be a `proc-macro = true` crate, which can export
only macros and whose code runs inside the compiler. Holding the logic in a plain
library instead makes expansions ordinary `proc_macro2::TokenStream` values, so
they can be asserted with unit tests and `insta` snapshots rather than only
through `trybuild` failures in the consumer.

```text
consortium-codec              re-exports the derive next to the trait
  └── consortium-codec-macros        proc-macro facade
        └── consortium-codec-macros-impl   expansion logic (this crate)
              └── consortium-macros-helpers   reusable syn/AST utilities
```

## `derive_trusted_archive`

Generates the `unsafe impl TrustedArchive` that lets `RkyvCodec` decode a type
with `rkyv::access_unchecked` on builds where `bytecheck`'s validator is
unavailable (no `alloc`). Because that call is unsound on bytes that are not a
correctly aligned archive of the type, the derive checks two things at compile
time:

1. **Stable repr.** The type must pin down its layout:
   - structs and unions — `#[repr(C)]` or `#[repr(align(N))]`
   - enums — `#[repr(u8..u64)]`, `#[repr(i8..i64)]`, or `#[repr(C)]`

   A default-repr type is rejected: the compiler may reorder its fields, so its
   archived form has no layout worth trusting.

2. **Field bounds.** Every field type must itself satisfy `TrustedArchive`,
   enforced through generated `where`-clause predicates. Generic type parameters
   receive the same bound, so `Wrapper<T>` compiles and the check lands at the
   instantiation site.

The emitted `unsafe impl` carries a short invariant note for auditors, since the
workspace denies `undocumented_unsafe_blocks`.

## Testing

```bash
cargo test -p consortium-codec-macros-impl   # unit + insta snapshots
cargo test -p consortium-codec               # trybuild rejection cases
```

Review intentional expansion changes with `cargo insta review`.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
