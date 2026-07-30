# consortium-ffi

C ABI surface for embedding Consortium runtimes in foreign hosts.

## Status: reserved, not implemented

**This crate is a placeholder. It currently exports nothing.**

The intent is an `extern "C"` surface so a non-Rust host — a C or C++ application, or any
language with a C FFI — can drive a Consortium runtime: open a link, send and receive
typed messages, invoke a TEE command.
[`consortium-runtime-app`](../consortium-runtime-app) already reserves an `ffi` feature
that would re-export it.

Nothing depends on it yet, and its shape is undecided.

## Two constraints worth recording before it grows

**Message types cannot cross a C boundary as generics.** The IPC layer's design is
compile-time — `IpcSafe` proves a type is wire-portable, `CodecFor<T>` picks its
serialization, and `Channel<D, T, Tr, C>` monomorphizes over both. None of that survives
`extern "C"`. An FFI surface has to expose either raw byte buffers (pushing serialization
onto the caller) or a generated per-message shim (keeping type safety at the cost of a
codegen step). That choice determines the whole API.

**Async does not cross either.** Every send and receive in the runtime is a future driven
by Tokio or Embassy. A C API therefore needs one of: a blocking wrapper that owns a
runtime, a callback registration, or a poll-style handle the host integrates into its own
loop. Again, this decides the shape rather than being a detail to settle later.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
