# TEE Rust Macro System — Advanced Type System

## Overview

The `fee_function` proc-macro provides a zero-boilerplate bridge between Trusted Application (TA) and Client Application (CA) code in a TEE environment. This document covers the full type system: from primitive scalars through to arbitrary serializable types.

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Developer writes one function signature                │
│                                                         │
│  #[fee_function]                                        │
│  fn process(ctx: &mut Ctx, cfg: MyConfig) -> Result<()> │
└────────────────────┬────────────────────────────────────┘
                     │  macro expands to
          ┌──────────┴───────────┐
          │                      │
          ▼                      ▼
  process_dispatched()      call_process()        [cfg(feature="ca")]
  (TA side)                 (CA side)
  unpack params             pack params
  → call process()          → invoke_command()
```

The original function is always emitted untouched, so internal TA-to-TA calls pay zero marshalling overhead.

---

## Parameter Classification

The macro classifies each parameter by its Rust type and maps it to a TEE parameter slot. A TEE function has at most **4 parameter slots**.

### Context Parameter

A leading `&mut SomePath` parameter (where `SomePath` is not a primitive) is treated as the TA context. It is forwarded directly to the inner function and never packed into a TEE slot.

```rust
#[fee_function]
fn my_fn(ctx: &mut AppContext, value: u32) -> Result<()> { ... }
//       ^^^^^^^^^^^^^^^^^^^^^^^^
//       context — not a TEE slot
```

---

## Supported Types

### Primitive Values (`ParamValue` slot)

These fit into the `a` and `b` fields of a single `ParamValue` TEE slot.

| Rust type    | Slot kind        | Encoding                                 |
|--------------|------------------|------------------------------------------|
| `u32`        | `ValueInput`     | `a = value`, `b = 0`                     |
| `i32`        | `ValueInput`     | `a = value as u32`, `b = 0`              |
| `bool`       | `ValueInput`     | `a = value as u32` (0 or 1), `b = 0`    |
| `u64`        | `ValueInput`     | `a = low 32 bits`, `b = high 32 bits`   |
| `(u32, u32)` | `ValueInput`     | `a = .0`, `b = .1`                       |

> **Note:** `u64` consumes only **one** TEE slot by splitting across `a` and `b`.

#### Example

```rust
#[fee_function]
fn set_threshold(ctx: &mut Ctx, limit: u64, flags: (u32, u32)) -> Result<()> {
    // limit  → slot 0: a = low bits, b = high bits
    // flags  → slot 1: a = flags.0,  b = flags.1
}
```

---

### Output Value Parameters (`ValueInout` slot)

A `&mut T` where `T` is a primitive scalar is treated as an **output parameter**. The TA writes a result back through the pointer; the CA reads it after `invoke_command` returns.

| Rust type   | Slot kind      | TA behaviour                          | CA behaviour                          |
|-------------|----------------|---------------------------------------|---------------------------------------|
| `&mut u32`  | `ValueInout`   | reads `a`, writes back via `set_a`    | reads `a` into `*ptr` after call      |
| `&mut i32`  | `ValueInout`   | reads `a as i32`, writes back cast    | reads `a as i32` into `*ptr` after call |
| `&mut u64`  | `ValueInout`   | reconstructs from `a`/`b`, writes back | reconstructs from `a`/`b` after call |

#### Example

```rust
#[fee_function]
fn compute(ctx: &mut Ctx, input: u32, result: &mut u64) -> Result<()> {
    *result = (input as u64) * 0xDEAD_BEEF;
}

// CA side (generated):
// #[cfg(feature = "ca")]
// fn call_compute(input: u32, result: &mut u64) -> Result<()> {
//     let _p0 = ParamValue::new(input, 0, ParamType::ValueInput);
//     let _p1 = ParamValue::new(0u32, 0u32, ParamType::ValueInout);
//     invoke_command(Command::Compute, &mut Parameters::new(_p0, _p1, None, None))?;
//     // read back:
//     *result = ((v.b() as u64) << 32) | (v.a() as u64);
//     Ok(())
// }
```

---

### Memory Buffer Parameters (`ParamMemref` slot)

Byte slices are passed as shared memory references.

| Rust type    | Slot kind       | Direction        |
|--------------|-----------------|------------------|
| `&[u8]`      | `MemrefInput`   | CA → TA (read)   |
| `&mut [u8]`  | `MemrefInout`   | CA ↔ TA (r/w)    |

#### Example

```rust
#[fee_function]
fn encrypt(ctx: &mut Ctx, plaintext: &[u8], ciphertext: &mut [u8]) -> Result<()> {
    // plaintext  → slot 0: MemrefInput
    // ciphertext → slot 1: MemrefInout
}
```

---

### Serialized Types (`TeeParam` — `ParamMemref` slot)

Any type implementing `serde::Serialize + serde::Deserialize` can be passed through a memref slot. The macro serializes it using [`postcard`](https://docs.rs/postcard) (a compact, `no_std`-friendly binary format).

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

| Rust type    | Slot kind       | TA behaviour                             | CA behaviour                              |
|--------------|-----------------|------------------------------------------|-------------------------------------------|
| `T: TeeParam`     | `MemrefInput`   | deserialize from buffer with `postcard`  | serialize to `Vec<u8>`, pass as memref    |
| `&mut T: TeeParam`| `MemrefInout`   | deserialize, pass `&mut`, re-serialize   | serialize, pass as memref, deserialize back |

#### Example

```rust
#[derive(TeeParam, Serialize, Deserialize)]
struct SessionConfig {
    timeout_ms: u32,
    retry_limit: u8,
    key_id: u64,
}

#[fee_function]
fn open_session(ctx: &mut Ctx, cfg: SessionConfig) -> Result<()> {
    // cfg is deserialized from a MemrefInput slot on the TA side
    // caller serialized it with postcard on the CA side
}
```

#### Inout (Read-Modify-Write)

```rust
#[derive(TeeParam, Serialize, Deserialize)]
struct Counter {
    value: u64,
    overflows: u32,
}

#[fee_function]
fn increment(ctx: &mut Ctx, counter: &mut Counter) -> Result<()> {
    counter.value = counter.value.wrapping_add(1);
    if counter.value == 0 { counter.overflows += 1; }
}
// TA deserializes Counter from the slot, calls increment(), then re-serializes back.
// CA sends current state, then reads updated state after invoke_command returns.
```

---

## The `TeeParam` Trait

```rust
pub trait TeeParam: Serialize + for<'de> Deserialize<'de> {
    /// Maximum serialized size in bytes.
    /// Used to pre-size the inout buffer on the CA side.
    /// Override this to match the worst-case postcard output for your type.
    const MAX_SIZE: usize;
}
```

The `MAX_SIZE` associated constant solves the inout buffer sizing problem: since the TA may return a value larger than what the CA sent, the CA must pre-allocate a buffer sized for the response. Defining it on the type means the size contract lives next to the type definition.

```rust
#[derive(TeeParam, Serialize, Deserialize)]
struct Counter {
    value: u64,
    overflows: u32,
}

impl TeeParam for Counter {
    // u64 (8) + u32 (4) + postcard framing (≤4) = 16 bytes, rounded up generously
    const MAX_SIZE: usize = 32;
}
```

If you use `#[derive(TeeParam)]` without a manual `impl`, the macro emits a compile-time assertion that the type is `Serialize + Deserialize`, and uses a conservative default of `1024` bytes for `MAX_SIZE`. Override it with a manual impl when the default is wasteful or insufficient.

---

## Slot Limit Enforcement

TEE permits at most **4 parameter slots** per call (the context never counts as a slot). The macro enforces this at compile time:

```
error: fee_function: too many TEE parameter slots used (5), maximum is 4
```

If you exceed 4 slots, restructure your parameters into a serialized struct instead:

```rust
// ❌ Too many slots
#[fee_function]
fn bad(ctx: &mut Ctx, a: u32, b: u64, c: u32, d: &[u8], e: &mut [u8]) -> Result<()> { ... }
//                   [0]    [1]    [2]   [3]          [4]  ← 5 slots, compile error

// ✅ Pack logically related scalars into a struct
#[derive(TeeParam, Serialize, Deserialize)]
struct Params { a: u32, b: u64, c: u32 }

#[fee_function]
fn good(ctx: &mut Ctx, params: Params, input: &[u8], output: &mut [u8]) -> Result<()> { ... }
//                     [0]             [1]            [2]   ← 3 slots
```

---

## Type Classification Priority

When the macro inspects a parameter type, it tests these rules **in order** and uses the first match:

```
1. &[u8]              → MemrefIn
2. &mut [u8]          → MemrefInout
3. &mut u32/i32/u64   → OutputU32/OutputI32/OutputU64
4. u32                → ValueA(U32)
5. i32                → ValueA(I32)
6. bool               → ValueA(Bool)
7. u64                → ValueU64
8. (u32, u32)         → ValuePair
9. &mut T (Path)      → SerializedInout
10. T (Path)          → SerializedIn
11. anything else     → compile error
```

This means a `&mut MyStruct` is never confused with a `&mut [u8]` (caught by rule 2 first), and `&mut u32` is never confused with `&mut SomeStruct` (caught by rule 3 first).

---

## Serialization Format: `postcard`

[`postcard`](https://docs.rs/postcard) was chosen over alternatives for the following reasons:

| Property             | `postcard` | `bincode` | `serde_json` |
|----------------------|-----------|-----------|--------------|
| `no_std` support     | ✅        | partial   | ❌           |
| Output size          | minimal   | small     | large        |
| Schema evolution     | manual    | manual    | flexible     |
| Allocation-free mode | ✅ (heapless) | ❌    | ❌           |

On the TA side (no allocator), `postcard::from_bytes` deserializes directly from the shared memref buffer. On the CA side, `postcard::to_allocvec` serializes into a `Vec<u8>` which is handed to `ParamMemref`.

> **Warning:** `postcard` is not self-describing. If you change a `TeeParam` type (add/remove/reorder fields), both CA and TA must be recompiled together. There is no forward compatibility.

---

## Complete Example

```rust
// Shared between CA and TA (e.g. in a common crate)
#[derive(TeeParam, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub algorithm: u8,
    pub key_id: u32,
    pub nonce: [u8; 12],
}

impl TeeParam for EncryptRequest {
    const MAX_SIZE: usize = 64;
}

// TA crate
tee_service! {
    context TaContext
    commands { encrypt_data, get_counter }
}

#[fee_function]
pub fn encrypt_data(
    ctx:       &mut TaContext,
    request:   EncryptRequest,   // slot 0 — SerializedIn
    plaintext: &[u8],            // slot 1 — MemrefInput
    output:    &mut [u8],        // slot 2 — MemrefInout
) -> Result<()> {
    let key = ctx.key_store.get(request.key_id)?;
    cipher::encrypt(request.algorithm, &key, &request.nonce, plaintext, output)
}

#[fee_function]
pub fn get_counter(
    ctx:     &mut TaContext,
    counter: &mut u64,           // slot 0 — ValueInout (output)
) -> Result<()> {
    *counter = ctx.invocation_count;
    ctx.invocation_count += 1;
    Ok(())
}

// CA crate (feature = "ca")
fn main() -> Result<()> {
    let req = EncryptRequest { algorithm: 1, key_id: 42, nonce: [0u8; 12] };
    let plaintext = b"hello world";
    let mut ciphertext = vec![0u8; 64];

    call_encrypt_data(&req, plaintext, &mut ciphertext)?;

    let mut counter = 0u64;
    call_get_counter(&mut counter)?;
    println!("invocation #{counter}");

    Ok(())
}
```

---

## Summary Table

| Type            | Direction | TEE Slot Kind    | Notes                                  |
|-----------------|-----------|------------------|----------------------------------------|
| `u32`           | in        | `ValueInput`     | `a` field                             |
| `i32`           | in        | `ValueInput`     | `a` field, bit-cast                   |
| `bool`          | in        | `ValueInput`     | `a` = 0 or 1                          |
| `u64`           | in        | `ValueInput`     | `a` = low, `b` = high                 |
| `(u32, u32)`    | in        | `ValueInput`     | `a` = `.0`, `b` = `.1`               |
| `&mut u32`      | out       | `ValueInout`     | written back via `a`                  |
| `&mut i32`      | out       | `ValueInout`     | written back via `a`, bit-cast        |
| `&mut u64`      | out       | `ValueInout`     | written back via `a`/`b`             |
| `&[u8]`         | in        | `MemrefInput`    | raw bytes, zero-copy                  |
| `&mut [u8]`     | in/out    | `MemrefInout`    | raw bytes, zero-copy                  |
| `T: TeeParam`   | in        | `MemrefInput`    | postcard-serialized                   |
| `&mut T: TeeParam` | in/out | `MemrefInout`   | postcard round-trip, `T::MAX_SIZE` pre-allocated |
