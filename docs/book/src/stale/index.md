# Stale & Superseded Drafts

This section archives design and planning drafts whose work has since landed in the
workspace. They are kept for provenance — to show how a subsystem arrived at its current
shape — but they no longer describe the code as it exists today, and in several places
they describe approaches that were deliberately replaced.

Do not treat anything here as normative. When a detail in one of these drafts disagrees
with the current crate sources, the sources win. For the present picture of each
subsystem, use the architecture chapters, the live subsystem guides, and the crate
documentation.

## What lives here

- [UART Transport PoC Plan](uart-transport-poc-plan.md) — the proof-of-concept plan for
  the UART IPC family. The transport now ships across `consortium-ipc-transport-uart`,
  `consortium-ipc-transport-uart-embedded`, and `consortium-ipc-transport-uart-unix`.
  The plan's proposed GAT-to-RPITIT transport refactor is complete, so its
  before/after trait sketches no longer match `consortium-ipc`.
- [Buffered UART RX Plan](buffered-uart-rx-plan.md) — the plan to bump the chip HALs to
  `embedded-io(-async)` 0.7 and add an IRQ-buffered `BufferedUart` RX driver. Both are
  in place. The draft also refers to a scratch path on the original author's machine.
- [Split Transport Halves Plan](split-transport-halves-plan.md) — the plan to split
  `Transport` into `SendTransport` and `RecvTransport`. Those half-traits and
  `SharedMemoryTransport::split()` now exist; the draft's associated-type future
  signatures predate the RPITIT refactor.
- [Early Runtime Initialization Plan](runtime-init-early-plan.md) — an early sketch of
  runtime bring-up. It predates the `#[consortium_runtime_*::main]` entry macros and the
  generated `init()`/`Context` flow, and it assumes doorbell crates install their own
  `#[no_mangle]`/`#[interrupt]` handlers, which is the opposite of the current
  convention: the application owns the vector table and forwards each IRQ into the
  doorbell's exported notifier.
- [Entry Macros Plan](entry-macros-plan.md) — the plan for the runtime entry-point
  attribute macros. `#[consortium_runtime_mcu::main]` and `#[consortium_runtime_app::main]`
  are implemented and hand the body a single `Context` aggregate.
