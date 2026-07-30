# consortium-tee

Trusted Execution Environment contracts for Consortium: typed OP-TEE commands whose
parameter marshalling is generated rather than hand-written.

## The problem

A TEE call crosses a privilege boundary between a Client Application (CA) in Linux
userspace and a Trusted Application (TA) in the secure world. The GlobalPlatform ABI
for that crossing is **four untyped parameter slots**, each either a pair of `u32`s
(a *Value*) or a shared buffer (a *Memref*).

Both sides must agree, by convention only, on what each slot holds. Nothing checks
it. And a disagreement is a security-relevant bug in the most sensitive code in the
system — the CA reads a length from a slot the TA filled with something else, and now
there is an out-of-bounds read across a trust boundary.

This crate replaces that convention with a Rust signature. One function definition
generates both ends of the call.

## Architecture

```text
                     one #[tee_command] fn
                             │
           ┌─────────────────┴─────────────────┐
   CA side (`ca`)                        TA side (`ta`)
   call_<name>()                         <name>_dispatched()
     pack args → slots                     slots → unpack args
     invoke_command                        call the real fn
     read output slots back                flush out/inout slots
```

| Item                | Role                                                            |
| ------------------- | --------------------------------------------------------------- |
| `TeeParamSlot`      | The wire shape of one slot: `Primitive { a, b }` or `Serialized(bytes)`. |
| `TeeParam<C>`       | "This Rust type can ride in a slot", parameterized by codec.      |
| `#[derive(TeeParam)]` | Generates the serialized impl for a user struct.                 |
| `#[tee_command]`    | Generates the CA caller and TA dispatcher for one function.        |
| `tee_service!`      | Generates the `Command` enum and the TA's `invoke_command` dispatch. |

## Two slot kinds, chosen by the type

| Rust value                        | Slot                         | Cost                 |
| --------------------------------- | ---------------------------- | -------------------- |
| `u8`…`i64`, `bool`, `(u32, u32)`   | `Primitive` — a TEE *Value*   | no serialization      |
| a `#[derive(TeeParam)]` struct     | `Serialized` — a TEE *Memref* | one codec round-trip  |

Primitives are **not** forced through a buffer: a `u16` becomes
`Primitive { a: v as u32, b: 0 }`. A command taking two integers allocates nothing and
copies nothing across the boundary, which is the difference between a cheap call and
one that touches shared memory.

`TeeParamSlot` is generic over its byte buffer so the same enum serves both
directions — `TeeParamRepr` owns a `Vec<u8>` outbound, `TeeParamReprRef` borrows a
`&[u8]` inbound, so reading an incoming Memref copies nothing.

## The codec is not baked into the derive

`TeeParam` is generic over its codec `C`, and `#[derive(TeeParam)]` emits
`impl<C: CodecFor<Self>> TeeParam<C>` rather than committing to a family. The choice is
made once, at the call site:

```rust,ignore
use consortium_tee::{TeeParam, tee_command};

#[derive(TeeParam, serde::Serialize, serde::Deserialize)]
struct MotorCommand { motor_id: u8, target_rpm: i32 }

#[tee_command(codec = PostcardCodec)]
fn set_motor(cmd: MotorCommand) -> Result<(), TeeError> { /* … */ }
```

That keeps a type reusable across commands with different codecs, and lets a type be
shared with the IPC layer without inheriting its codec choice.

Primitive types implement `TeeParam<C>` for **any** `C`, so a primitive-only command
needs no `codec` argument. It defaults to the `NoCodec` marker, which implements no
`CodecFor` — so forgetting `codec` on a command that needs one is a compile error
naming the missing impl, rather than a confusing type mismatch.

## Parameter type support

| Rust type              | TEE slot        | Notes                                 |
| ---------------------- | --------------- | ------------------------------------- |
| `&[u8]`                | `MemrefInput`   |                                       |
| `&mut [u8]`            | `MemrefInout`   |                                       |
| `&mut u32`             | `ValueInout`    | TA reads initial, writes result back   |
| `&mut i32`             | `ValueInout`    | bit-cast through `u32`                 |
| `&mut u64`             | `ValueInout`    | split across `a` (low) / `b` (high)    |
| `T: TeeParam<C>`       | Value or Memref | dispatched via `into`/`from_tee_param` |
| `&mut T: TeeParam<C>`  | `MemrefInout`   | inout codec round-trip                 |

`ctx` marks the first parameter as TA context; it must be a mutable reference to a
named path type and is skipped when assigning slots. Return values use the same
`TeeParam<C>` dispatch. At most four parameters, per the ABI.

## Buffer sizes are declared, not discovered

A Memref buffer is allocated by the CA **before** the call, so the worst-case
serialized size has to be known statically. `TeeParam::MAX_SIZE` carries it:
`#[derive(TeeParam)]` defaults to 1024 bytes, `#[tee(max_size = N)]` overrides.

Set it deliberately. Too small fails the call; too large wastes shared memory on every
invocation.

## Field rules

`#[derive(TeeParam)]` rejects the same constructs as `IpcSafe`, for the same reason:
raw pointers, references, function pointers, and `usize`/`isize`. The last is not
hypothetical here — a **32-bit TA and a 64-bit CA genuinely disagree** on the width, so
a `usize` field silently shifts every field after it.

## Error reporting

`TeeError` separates failures by *stage* — context initialization (is `/dev/tee0` even
there?), session opening (is the TA installed, does the UUID match?), and invocation
(did the TA reject the command?) — because each points at a different thing to fix.
`TeeErrorOrigin` adds the orthogonal axis for an invocation failure: whether the code
came from the client library, the trusted OS, or the TA itself.

Both are re-declared rather than re-exported from `optee-teec`, so raw `optee-teec`
types stay out of the public API; the original is preserved as a `#[source]`.

## Features

`no_std` with `alloc`. Default is `ca`.

| Feature | Effect                                                                  |
| ------- | ----------------------------------------------------------------------- |
| `ca`    | Client-Application side: `optee-teec`, Tokio, and the error types.        |
| `ta`    | Trusted-Application side dispatchers. Host-agnostic.                     |
| `defmt` | Route diagnostics to `defmt` on TA builds.                               |

CA error exports are additionally gated to Linux non-test builds, because
`optee-teec` links a client library needing a real TEE device. `defmt` is deliberately
**not** implied by `ta`: it requires a linked global logger an OP-TEE TA may not
provide, and its macros expand to literal `defmt::` paths (hence the direct
dependency).

Because `ta` is host-agnostic, it is what host-side macro and parameter tests build
against — the TA dispatch logic is testable without an OP-TEE dev kit.

## Configuration

The TA UUID comes from `[tee]` in `Consortium.toml`; the builder injects it as
`CONSORTIUM_TEE_UUID`.

## Testing

```bash
cargo test -p consortium-tee --no-default-features --features ta
just test tee host
```

When changing the macros, update the `trybuild` cases and `insta` snapshots in
[`consortium-tee-macros-impl`](../consortium-tee-macros-impl) before relying on the
proc-macro facade.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
