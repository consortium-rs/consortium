# Plan: Split Transport into SendTransport + RecvTransport Halves

## Context

`Channel<Tx>` and `Channel<Rx>` both own a `Tr: Transport`. Because `Transport::send` and `Transport::recv` both take `&mut self`, two channels cannot hold the same transport simultaneously. The current workaround in tests is constructing two separate `SharedMemoryTransport` instances (with `doorbell.clone()`) to get a Tx/Rx channel pair. The cleaner design is to split `Transport` into `SendTransport` + `RecvTransport` half-traits, add a concrete split type pair for `SharedMemoryTransport`, and have each `Channel` direction own only the half it needs.

---

## Step 1 — Split the Transport trait (`crates/consortium-ipc/src/transport.rs`)

Add a `TransportError` supertrait (just the `Error` associated type) so `ChannelError` can remain bounded on a single trait:

```rust
pub trait TransportError {
    type Error;
}

pub trait SendTransport: TransportError {
    type SendFut<'a>: Future<Output = Result<(), Self::Error>> + MaybeSend where Self: 'a;
    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a>;
    fn max_send_size(&self) -> usize;
}

pub trait RecvTransport: TransportError {
    type RecvFut<'a>: Future<Output = Result<usize, Self::Error>> + MaybeSend where Self: 'a;
    fn recv<'a>(&mut self, buf: &mut [u8]) -> Self::RecvFut<'a>;
    fn max_recv_size(&self) -> usize;
}

/// Combined marker — implement for types that cover both directions.
pub trait Transport: SendTransport + RecvTransport {}
```

Keep `Transport` as the combined marker for backwards compatibility. Any type implementing both halves can explicitly `impl Transport for T {}`.

---

## Step 2 — Update `Channel` and `ChannelError` (`crates/consortium-ipc/src/channel.rs`)

- Remove the `Tr: Transport` bound from the struct definition; push bounds to `impl` blocks.
- `impl<Tx>` block: `Tr: SendTransport`
- `impl<Rx>` block: `Tr: RecvTransport`
- Change `ChannelError<C: Codec, Tr: Transport>` → `ChannelError<C: Codec, Tr: TransportError>` (same callers, relaxed bound).
- `Debug`/`Display` impls stay structurally identical.

---

## Step 3 — Add `Clone` to region types (`crates/consortium-ipc/src/regions.rs`)

`UncachedRegion` and `CachedRegion` hold a raw pointer + length. Add manual `Clone` impls (both halves intentionally alias the same shared memory region):

```rust
impl Clone for UncachedRegion { fn clone(&self) -> Self { Self { ptr: self.ptr, len: self.len } } }
impl Clone for CachedRegion  { fn clone(&self) -> Self { Self { ptr: self.ptr, len: self.len } } }
```

---

## Step 4 — Add split halves and `split()` to `SharedMemoryTransport` (`crates/consortium-ipc-transport-memory/src/transport.rs`)

Add two new concrete types that mirror the existing struct fields, keeping only what each direction needs:

```rust
pub struct SharedMemoryTransportTx<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan, doorbell: D, region: R, base: *mut u8, tx_mtu: u32,
}

pub struct SharedMemoryTransportRx<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan, doorbell: D, region: R, base: *mut u8, rx_mtu: u32,
}
```

- `SharedMemoryTransportTx` implements `TransportError` + `SendTransport` (moves the `send` logic from the current `Transport` impl).
- `SharedMemoryTransportRx` implements `TransportError` + `RecvTransport` (moves the `recv` + `RecvFut::poll` logic).
- Both need their own `SendFut`/`RecvFut` future types (copy the existing ones, updating the pointer type).
- Add `split()` on `SharedMemoryTransport` (requires `D: Clone, R: Clone`):

```rust
pub fn split(self) -> (SharedMemoryTransportTx<D, R>, SharedMemoryTransportRx<D, R>)
where
    D: Clone, R: Clone,
```

The doorbell and region are cloned for the Rx half; the originals go to the Tx half. The `base` raw pointer is copied to both (intentional shared-memory aliasing, same safety contract as `SharedMemoryTransport::new`).

Keep `impl Transport for SharedMemoryTransport<D, R>` by delegating to `SendTransport`/`RecvTransport` impls, or have it implement all three traits directly. The combined `SharedMemoryTransport` is still useful for callers that don't need to split.

---

## Step 5 — Update re-exports (`crates/consortium-ipc/src/lib.rs`)

Add `SendTransport`, `RecvTransport`, `TransportError` to the `pub use` list alongside the existing `Transport`.

---

## Step 6 — Update tests (`crates/consortium-ipc-transport-memory/tests/channel_codec_test.rs`)

Replace the two-transport workaround with `split()`:

```rust
// Before
let tx_transport = unsafe { SharedMemoryTransport::new(ch, a_doorbell.clone(), tx_region, a_base) };
let mut tx = Channel::<Tx, Message, _, C>::new(ch, tx_transport, scratch::<MTU>());
// ... (sequential; can't hold both simultaneously)
let rx_transport = unsafe { SharedMemoryTransport::new(ch, a_doorbell, rx_region, a_base) };
let mut rx = Channel::<Rx, Message, _, C>::new(ch, rx_transport, scratch::<MTU>());

// After
let transport = unsafe { SharedMemoryTransport::new(ch, a_doorbell, region, a_base) };
let (tx_half, rx_half) = transport.split();
let mut tx = Channel::<Tx, Message, _, C>::new(ch, tx_half, scratch::<MTU>());
let mut rx = Channel::<Rx, Message, _, C>::new(ch, rx_half, scratch::<MTU>());
```

Also update the mock `MockTransport` in `channel.rs` unit tests to implement `SendTransport`/`RecvTransport` (or keep `Transport` impl and add blanket forward — whichever is less churn).

---

## Verification

```bash
cargo test -p consortium-ipc
cargo test -p consortium-ipc-transport-memory
just test ipc host
cargo clippy -p consortium-ipc -p consortium-ipc-transport-memory
```
