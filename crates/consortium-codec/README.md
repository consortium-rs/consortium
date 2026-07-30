# consortium-codec

Serialization families for Consortium IPC and TEE parameter conversion.

A Consortium link carries typed messages; the wire carries bytes. This crate is
the seam between them, and the only thing `consortium-ipc` and `consortium-tee`
know about serialization.

## Architecture

Two traits, split so that a family declares its error type once:

```text
  Codec              one impl per family — declares Error
    └── CodecFor<T>     one impl per (family, message type) — encode / decode
```

`Codec` implementors are zero-sized markers. They are named as type parameters
(`Channel<Tx, Msg, Transport, PostcardCodec>`) and never instantiated.

### `Decoded<'buf>` is the load-bearing detail

`CodecFor::Decoded` is an associated _type constructor_, not `T`. That is what
lets one trait cover owning and zero-copy codecs at once:

| Codec           | `Decoded<'buf>`     | What a `recv` costs                  |
| --------------- | ------------------- | ------------------------------------ |
| `PostcardCodec` | `T`                 | deserializes into an owned value     |
| `ProstCodec`    | `T`                 | deserializes into an owned value     |
| `RkyvCodec`     | `&'buf T::Archived` | casts and borrows the receive buffer |

So **do not assume `decode` yields an owned `T`**. `consortium_ipc::ReceivedMessage`
wraps `Decoded<'buf>` and `Deref`s through to `T` where the codec permits, which
is why call sites written against it survive a codec swap unchanged.

Encoding always targets a caller-supplied `&mut [u8]` and returns the byte count.
No allocation appears in the trait signature, so one codec serves a `no_std`
firmware channel and a Linux one.

## Choosing a family

There is **no default codec feature** — a consumer names what it wants, so
firmware never pays for a family it does not call.

| Feature    | Codec           | Pick it when                                                                |
| ---------- | --------------- | --------------------------------------------------------------------------- |
| `postcard` | `PostcardCodec` | Both ends are Rust crates in this workspace. Compact, `no_std`, serde-wide. |
| `prost`    | `ProstCodec`    | The schema is shared with a non-Rust or independently shipped peer.         |
| `rkyv`     | `RkyvCodec`     | Messages are large enough that copying them out dominates.                  |

Supporting features: `alloc` enables heap-backed paths (including _validated_
rkyv decode); `std` implies `alloc` and routes diagnostics to `tracing`; `defmt`
routes the same diagnostics to `defmt` for firmware builds.

`Consortium.toml` selects a channel's codec with `[[ipc.channel]] codec`, which
defaults to `postcard`.

## Example

```rust,ignore
use consortium_codec::{CodecFor, PostcardCodec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Telemetry { sequence: u32, ready: bool }

let mut buf = [0u8; 32];
let n = PostcardCodec::encode(&Telemetry { sequence: 7, ready: true }, &mut buf)?;
let back: Telemetry = <PostcardCodec as CodecFor<Telemetry>>::decode(&buf[..n])?;
```

## Per-family notes

### `PostcardCodec`

Not self-describing: both ends must agree on the type, which in this workspace is
arranged by sharing one message crate (see `examples/melt-pot/shared`). Encode
fails rather than truncating when the destination is short. Decode wants an exact
frame — the channel passes `buf[..n]`, never the whole buffer.

### `ProstCodec`

`prost` encodes into a growable `BufMut`, so this codec measures `encoded_len()`
first and returns `ProstError::BufferTooSmall` itself; otherwise an over-long
message would panic or emit a partial frame instead of erroring. `T: Message +
Default` is required because `prost` decodes by filling a default value. Note
that generated protobuf types usually need `alloc` for `String`/`Vec` fields —
check the generated code before choosing this on a bare-metal core.

### `RkyvCodec` and `TrustedArchive`

`RkyvCodec<SCRATCH>` has two paths:

| Build            | Serializer              | Decode                           |
| ---------------- | ----------------------- | -------------------------------- |
| `alloc` enabled  | `AlignedVec` arena      | `rkyv::access` — **validated**   |
| `alloc` disabled | `SubAllocator<SCRATCH>` | `access_unchecked` — **trusted** |

`bytecheck`'s validator needs `alloc`. Without it, decode calls
`access_unchecked`, which is unsound on bytes that are not a correctly aligned
archive of `T` — so the no-`alloc` impl is bounded on the `TrustedArchive`
marker, which carries that obligation. Blanket impls cover fixed-width
primitives, arrays, and tuples up to eight elements; user types use
`#[derive(TrustedArchive)]`, which checks for a stable `repr` and recurses into
every field.

Limitations worth knowing before choosing rkyv: the no-`alloc` path does not
validate, so it assumes a cooperative peer; rkyv archives are not stable across
`rkyv` versions, so both ends must be built against the same one; and `SCRATCH`
too small is a runtime encode error, not a compile error.

## Implementing a codec

1. Add a unit struct and `impl Codec` for it to declare `Error`.
2. `impl<T> CodecFor<T> for MyCodec where T: <your serialization bound>`.
3. Set `Decoded<'buf>` to `T` for an owning codec, or to a borrow of `'buf` for a
   zero-copy one.
4. `encode` must report the exact count written and must **fail rather than
   truncate** — a transport sends `buf[..n]` verbatim.
5. Log failure paths through `consortium-log` so they resolve to `defmt` on
   firmware and `tracing` on the host.

## Testing

```bash
cargo test -p consortium-codec --features postcard
cargo test -p consortium-codec --features rkyv        # no_std, trusted path
cargo test -p consortium-codec --features rkyv,alloc  # validated path
just test codec host
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
