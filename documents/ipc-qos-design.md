# IPC QoS Design

**Date:** 2026-04-05

---

## Problem

All channels in the current design are equal. A log-spam channel can starve a safety alarm. There's no way to express "this message is urgent" or "this sender should not exceed N bytes/sec."

---

## Goals

1. **Priority**: High-priority messages preempt low-priority ones at the receiver.
2. **Rate limiting**: Per-channel token bucket so a chatty sender cannot starve others.
3. **Back-pressure**: Receiver can signal "slow down" without dropping messages.
4. **Zero allocation**: All data structures are statically sized; no `alloc` required.
5. **no_std compatible**: Works on bare-metal M-core.

---

## Non-Goals

- Hard real-time guarantees (no RTOS scheduling, no hardware timer enforcement).
- Multi-hop routing or network-style QoS (DSCP, 802.1p).
- Dynamic priority inheritance.

---

## Design

### 1. Priority Levels

Four levels, encoded as 2 bits in the slot header. No allocator needed; priority is a type-level constant at channel construction.

```rust
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    RealTime   = 0,  // alarms, safety interlocks
    High       = 1,  // control commands
    Normal     = 2,  // sensor telemetry (default)
    Background = 3,  // logs, diagnostics
}
```

Priority is stamped into the slot at send time; the receiver reads it before deciding which pending slot to drain next.

---

### 2. Slot Header Extension

Current layout:
```
[len: u32][flags: u32][payload: [u8; mtu]]
```

Extended layout (one extra byte, backward-incompatible version bump):
```
[len: u32][flags: u32][priority: u8][_pad: [u8; 3]][payload: [u8; mtu]]
```

**Flag additions:**

| Bit | Name                | Meaning |
|-----|---------------------|---------|
| 0   | `FLAG_VALID`        | Slot contains a new message |
| 1   | `FLAG_CONSUMED`     | Receiver has read the message |
| 2   | `FLAG_BACKPRESSURE` | Receiver is slow; sender should pause |

The descriptor version field (currently `1`) bumps to `2` to signal the new layout to both sides.

---

### 3. Rate Limiter

A `RateLimiter` is a `no_std`, no-alloc leaky-bucket rate limiter. The caller supplies a monotonic tick source as a type parameter.

```rust
pub struct RateLimiter<C: Clock> {
    allowance: u32,       // remaining byte budget (scaled by 1000 to avoid floats)
    capacity: u32,        // max burst bytes
    refill_rate: u32,     // bytes per millisecond × 1000
    last_tick: u64,       // last refill timestamp (ms)
    _clock: PhantomData<C>,
}

pub trait Clock {
    fn now_ms() -> u64;
}

impl<C: Clock> RateLimiter<C> {
    /// Returns true if `bytes` fit within the current allowance and deducts them.
    pub fn try_consume(&mut self, bytes: u32) -> bool;

    /// Refills allowance based on elapsed time since last call.
    pub fn refill(&mut self);
}
```

A `RateLimitedTransport<Tr, C>` wraps any `Transport` and calls `limiter.try_consume(data.len())` before forwarding a send. If the allowance is exhausted, `SendFut` yields and retries after a platform-supplied delay tick.

```rust
pub struct RateLimitedTransport<Tr: Transport, C: Clock> {
    inner: Tr,
    limiter: RateLimiter<C>,
}

impl<Tr: Transport, C: Clock> Transport for RateLimitedTransport<Tr, C> {
    // send() checks bucket before delegating to inner.send()
    // recv() passes through unchanged
}
```

---

### 4. Priority Scheduler (A-core)

On the A-core (Tokio), multiple channels may have pending data simultaneously. `PriorityMux` polls all registered channels and always returns the highest-priority pending message.

```rust
pub struct PriorityMux<const N: usize> {
    // Parallel arrays, sorted by declared priority at construction.
    slots: [SlotHandle; N],
}

impl<const N: usize> PriorityMux<N> {
    /// Returns the channel index and a reference to the raw slot
    /// with the highest priority among currently-valid slots.
    pub async fn next_ready(&mut self) -> (usize, &SlotHandle);
}
```

**Algorithm:**

1. Snapshot `FLAG_VALID` for all N slots (volatile reads, no lock).
2. Among valid slots, pick the one with the lowest `priority` byte.
3. If none valid, subscribe all slot notifiers and `await` the first to fire.
4. Return to step 1.

Ties within the same priority level are broken by round-robin (a simple `last_served` index).

The scheduler is optional. Callers who only care about a single channel use `Channel` directly as today.

---

### 5. Back-Pressure

When the A-core receiver cannot keep up (e.g., processing takes too long), it sets `FLAG_BACKPRESSURE` in the rx slot it has just consumed but not yet recycled.

On the M-core sender, after `doorbell.ring()`, `SendFut` checks `FLAG_BACKPRESSURE` on the tx slot in its next poll. If set, it yields for one executor cycle before retrying, giving lower-priority work a chance to run.

This is advisory, not blocking. The flag is best-effort; it does not prevent the sender from overwriting a slot. It only requests cooperative throttling.

---

### 6. Type-Level Integration

QoS priority is stamped as a const generic on `Channel` so misconfiguration is caught at compile time:

```rust
pub struct Channel<D, T, Tr, C = PostcardCodec, const P: Priority = Priority::Normal> {
    // ...
}

// Alarm channel — won't compile if you accidentally use it as a background sink
let alarm: Channel<Tx, AlarmEvent, MemTransport<_, _>, PostcardCodec, {Priority::RealTime}> = ...;
```

The `Priority` const flows into `Transport::send()` via a new parameter:

```rust
fn send<'a>(&'a mut self, ch: Chan, data: &[u8], priority: Priority) -> Self::SendFut<'a>;
```

Existing code that omits the const defaults to `Priority::Normal`.

---

## Slot Layout (v2 descriptor, version field = 2)

```
Byte offset  Size  Field       Notes
0x00         4     len         Payload length (volatile u32, LE)
0x04         4     flags       Bits: VALID(0) CONSUMED(1) BACKPRESSURE(2)
0x08         1     priority    0=RealTime … 3=Background
0x09         3     _pad        Reserved, must be zero
0x0C         mtu   payload     Raw encoded bytes
```

Total header overhead: 12 bytes (up from 8).

---

## Crate Changes Summary

| Crate | Change |
|-------|--------|
| `consortium-ipc` | Add `Priority` enum; bump `Transport::send` signature; add `FLAG_BACKPRESSURE` |
| `consortium-ipc-mem` | Write priority byte in slot; check backpressure flag |
| `consortium-ipc-uio` | Read priority byte; expose it from `UioChannel::recv` |
| `consortium-ipc` (new module `qos`) | `RateLimiter`, `RateLimitedTransport`, `PriorityMux` |
| Descriptor (UIO) | Version field: `1 → 2`; slot header size: `8 → 12` |

---

## Open Questions

1. **Clock source on M-core**: `RateLimiter` needs a monotonic millisecond source. The M-core crate must provide a `Clock` impl (SysTick, TIM2, etc.). This is platform-specific and should be injected, not hard-coded.

2. **PriorityMux and pinning**: `N` channel handles in a `const`-generic array require the handles to be `Unpin`. Verify `UioChannel` satisfies this or wrap in `Pin<Box<_>>` (std-only path).

3. **Version negotiation**: The descriptor version bump is backward-incompatible. A migration shim (read old v1 layout, write v2) may be needed during rollout.

4. **Scheduler fairness under saturation**: Round-robin tie-breaking within a priority band is simple but not starvation-proof if a `RealTime` channel fires continuously. Consider a debt-based scheduler (DRR) if saturation is a realistic scenario.
