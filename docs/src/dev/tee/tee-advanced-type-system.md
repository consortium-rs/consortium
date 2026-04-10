# TEE Rust Macro System — Advanced Type System

## Status

This document describes the **advanced type system** for `#[tee_command]`. Most features listed below are implemented; some remain planned.

### What is already implemented

See `tee-sdk-design.md` for the baseline. Advanced types now fully implemented:

| Category | Implemented types |
|---|---|
| Input parameters | `bool`, `u32`, `i32`, `u64`, `(u32, u32)`, `&[u8]`, `&mut [u8]`, `&mut u32`, `&mut i32`, `&mut u64`, `T: TeeParam` |
| Return / output | `u32`, `u64`, `bool`, `Vec<u8>`, `T: TeeParam` |
| Serialization | Pluggable codec via `#[tee_command(codec = MyCodec)]`; any codec implementing `consortium_common::codec::CodecFor<T>` |

### What this document covers

- `i32`, `(u32, u32)` value pairs ✅
- `&mut u32` / `&mut u64` output parameters (inout slots) ✅
- Arbitrary serializable types via `TeeParam` + pluggable codec system ✅

---

## Architecture

```
┌────────────────────────────────────────────────────────────────┐
│  Developer writes one function signature                       │
│                                                                │
│  #[tee_command(codec = MyCodec)]  ← required if using TeeParam │
│  fn process(ctx: &mut Ctx, cfg: MyConfig) -> Result<()>        │
└────────────────────┬───────────────────────────────────────────┘
                     │  macro expands to
                     │
          ┌──────────┴───────────┐
          │                      │
          ▼                      ▼
  process_dispatched()      call_process()        [cfg(feature="ca")]
  (TA side)                 (CA side)
  unpack params             pack params
  → call process()          → invoke_command()
```

The original function is always emitted untouched, so internal TA-to-TA calls pay zero marshalling overhead.

The `codec` attribute is required when any parameter or return type implements `TeeParam`. It specifies which codec to use for serialization/deserialization of those types.

---

## Parameter Classification

The macro classifies each parameter by its Rust type and maps it to a TEE parameter slot. A TEE function has at most **4 parameter slots**.

### Context Parameter

A leading `&mut SomePath` parameter (where `SomePath` is not a primitive or slice) is treated as the TA context. It is forwarded directly to the inner function and never packed into a TEE slot. This is already implemented — documented here for completeness.

---

## Planned Types

### Additional Primitive Values (`ParamValue` slot)

| Rust type    | Slot kind        | Encoding                                 | Status |
|--------------|------------------|------------------------------------------|--------|
| `u32`        | `ValueInput`     | `a = value`, `b = 0`                     | ✅ done |
| `bool`       | `ValueInput`     | `a = value as u32` (0 or 1), `b = 0`    | ✅ done |
| `u64`        | ValueInput/Return | `a = low 32`, `b = high 32`             | ✅ done |
| `i32`        | `ValueInput`     | `a = value as u32`, `b = 0`              | ✅ done |
| `(u32, u32)` | `ValueInput`     | `a = .0`, `b = .1`                       | ✅ done |

---

### Output Value Parameters (`ValueInout` slot)

A `&mut T` where `T` is a primitive scalar is treated as an **output parameter**. The TA writes a result back through the pointer; the CA reads it after `invoke_command` returns.

| Rust type   | Slot kind      | TA behaviour                          | CA behaviour                          | Status |
|-------------|----------------|---------------------------------------|---------------------------------------|--------|
| `&mut u32`  | `ValueInout`   | reads `a`, writes back via `set_a`    | reads `a` into `*ptr` after call      | ✅ done |
| `&mut i32`  | `ValueInout`   | reads `a as i32`, writes back cast    | reads `a as i32` into `*ptr` after call | ✅ done |
| `&mut u64`  | `ValueInout`   | reconstructs from `a`/`b`, writes back | reconstructs from `a`/`b` after call | ✅ done |

#### Example

```rust
#[tee_command]
fn compute(ctx: &mut Ctx, input: u32, result: &mut u64) -> Result<()> {
    *result = (input as u64) * 0xDEAD_BEEF;
    Ok(())
}

// CA side (generated):
// #[cfg(feature = "ca")]
// fn call_compute(input: u32, result: &mut u64) -> consortium_tee::TeeResult<()> {
//     let _p0 = ParamValue::new(input, 0, ParamType::ValueInput);
//     let _p1 = ParamValue::new(0u32, 0u32, ParamType::ValueInout);
//     let mut _params = Parameters::new(_p0, _p1, ParamNone, ParamNone);
//     invoke_command(Command::Compute, &mut _params)?;
//     *result = ((_params.param_1().as_value().b() as u64) << 32)
//              | (_params.param_1().as_value().a() as u64);
//     Ok(())
// }
```

---

### Serialized Types (`TeeParam` — `ParamMemref` slot)

Any type implementing `TeeParam` can be passed through a memref slot. The macro serializes it using a pluggable codec specified via the `#[tee_command(codec = ...)]` attribute.

Derive `TeeParam` on your type to opt in:

```rust
#[derive(TeeParam, Serialize, Deserialize)]
struct MyConfig {
    threshold: u32,
    flags: u64,
    mode: OperationMode,
    label: heapless::String<32>,
}
```

The codec type must implement `consortium_common::codec::CodecFor<T>` for your serialization format (e.g., `PostcardCodec` for [`postcard`](https://docs.rs/postcard)).

| Rust type          | Slot kind       | TA behaviour                             | CA behaviour                              | Status |
|--------------------|-----------------|------------------------------------------|-------------------------------------------|--------|
| `T: TeeParam`      | `MemrefInput`   | deserialize from buffer with codec       | serialize to buffer, pass as memref       | ✅ done |
| `&mut T: TeeParam` | `MemrefInout`   | deserialize, pass `&mut`, re-serialize   | serialize, pass as memref, deserialize back | ✅ done |

---

## The `TeeParam` Trait

```rust
pub trait TeeParam {
    /// Maximum serialized size in bytes.
    /// Used to pre-size the inout buffer on the CA side.
    const MAX_SIZE: usize;
}
```

`TeeParam` is a codec-agnostic marker trait. Derive it on any type you want to pass through a memref slot. The actual serialization is handled by the codec you specify in `#[tee_command(codec = ...)]`.

`MAX_SIZE` solves the inout buffer sizing problem: since the TA may return a value larger than what the CA sent, the CA must pre-allocate a buffer sized for the response. Defining it on the type means the size contract lives next to the type definition.

If `#[derive(TeeParam)]` is used without a manual `impl`, the macro uses a conservative default of `1024` bytes.

---

## Slot Limit Enforcement

TEE permits at most **4 parameter slots** per call (the context never counts as a slot). This is already enforced for current types via `compile_error!`. The enforcement will extend to cover all planned types.

If you exceed 4 slots, restructure parameters into a serialized struct instead:

```rust
// Too many slots
#[tee_command]
fn bad(ctx: &mut Ctx, a: u32, b: u64, c: u32, d: &[u8], e: &mut [u8]) -> Result<()> { ... }
//                   [0]    [1]    [2]   [3]          [4]  ← 5 slots, compile error

// Pack logically related scalars into a struct
#[derive(TeeParam, Serialize, Deserialize)]
struct Params { a: u32, b: u64, c: u32 }

#[tee_command]
fn good(ctx: &mut Ctx, params: Params, input: &[u8], output: &mut [u8]) -> Result<()> { ... }
//                     [0]             [1]            [2]   ← 3 slots
```

---

## Type Classification Priority (full order)

When the macro inspects a parameter type, it will test these rules **in order**:

```
1. &[u8]              → MemrefInput
2. &mut [u8]          → MemrefInout
3. &mut u32/i32/u64   → ValueInout output
4. u32                → ValueInput (a)
5. i32                → ValueInput (a, bit-cast)
6. bool               → ValueInput (a, 0/1)
7. u64                → ValueInput (a=low, b=high)
8. (u32, u32)         → ValueInput (a=.0, b=.1)
9. &mut T (Path)      → MemrefInout (TeeParam, codec round-trip)
10. T (Path)          → MemrefInput (TeeParam, codec deserialize)
11. anything else     → compile error
```

**All rules are now implemented.** Rules 9–10 require `#[tee_command(codec = MyCodec)]` to be specified.

---

## Codec Selection and Requirements

### Pluggable Codec System

The `#[tee_command(codec = MyCodec)]` attribute lets you choose how `TeeParam` types are serialized. Your codec type must implement `consortium_common::codec::CodecFor<T>` for each `TeeParam` type you use.

### Example: Using `PostcardCodec`

```rust
use consortium_common::codec::PostcardCodec;

#[derive(TeeParam, Serialize, Deserialize)]
struct Config {
    threshold: u32,
    enabled: bool,
}

#[tee_command(codec = PostcardCodec)]
fn process(ctx: &mut Ctx, cfg: Config) -> Result<(), TeeError> {
    // ...
}
```

### Considerations for Codec Choice

| Property             | `postcard` | `bincode` | Custom |
|----------------------|-----------|-----------|--------|
| `no_std` support     | ✅        | partial   | ✅     |
| Output size          | minimal   | small     | varies |
| Allocation-free mode | ✅        | ❌        | ✅     |
| Schema evolution     | manual    | manual    | custom |

### Schema Evolution Warning

Most binary codecs (including `postcard` and `bincode`) are **not self-describing**. If you change a `TeeParam` type (add/remove/reorder fields), both CA and TA must be recompiled together to stay in sync.

---

## Codec Implementation Reference

To implement a custom codec for your type `T`:

```rust
impl consortium_common::codec::CodecFor<MyType> for MyCodec {
    type Error = MyError;
    type Decoded<'buf> = MyType;
    
    fn encode(msg: &MyType, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Serialize msg into buf, return number of bytes written
    }
    
    fn decode(buf: &[u8]) -> Result<Self::Decoded, Self::Error> {
        // Deserialize from buf
    }
}
```
