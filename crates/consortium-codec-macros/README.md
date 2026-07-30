# consortium-codec-macros

Proc-macro facade for [`consortium-codec`](../consortium-codec).

## Why this crate exists

A crate with `proc-macro = true` can export nothing but macros, and its code runs
in the compiler rather than in the target program — a poor home for logic. Each
macro family in this workspace is therefore split in two:

| Crate                          | Role                                                     |
| ------------------------------ | -------------------------------------------------------- |
| `consortium-codec-macros`      | `proc-macro` facade. Converts `TokenStream`, forwards.   |
| `consortium-codec-macros-impl` | Plain library holding the expansion logic and its tests. |

## Contents

- `#[derive(TrustedArchive)]` — assert that a type's rkyv archived form is safe to
  access without `bytecheck` validation, which is what makes `RkyvCodec` usable on
  a `no_std`, no-`alloc` core.

## Usage

Do not depend on this crate directly. `consortium-codec` re-exports the derive
alongside the trait when the `rkyv` feature is on:

```toml
consortium-codec = { version = "0.1", features = ["rkyv"] }
```

```rust,ignore
use consortium_codec::TrustedArchive; // both the trait and the derive

#[derive(rkyv::Archive, rkyv::Serialize, TrustedArchive)]
#[repr(C)]
struct Frame {
    seq: u32,
    samples: [i16; 8],
}
```

The derive requires a stable `repr` (`#[repr(C)]` or `#[repr(align(N))]` for
structs and unions, an integer or `C` repr for enums) and that every field is
itself `TrustedArchive`.

See the [`consortium-codec` README](../consortium-codec/README.md) for why the
marker is needed and what the unvalidated decode path assumes about the peer.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
