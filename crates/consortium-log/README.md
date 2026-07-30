# consortium-log

A tiny logging facade that forwards to `defmt`, `tracing`, both, or neither —
chosen by Cargo features, resolved at compile time, zero cost when off.

## The problem it solves

Consortium code spans two worlds that log differently and cannot log each
other's way:

| World                              | Logs with | Why not the other                                                                   |
| ---------------------------------- | --------- | ----------------------------------------------------------------------------------- |
| `no_std` firmware (M-core, R-core) | `defmt`   | `tracing` needs `alloc` and formatting machinery a Cortex-M image should not carry. |
| `std` host tooling (codegen, CLI)  | `tracing` | `defmt` needs a decoder on the other end of a wire.                                 |

But plenty of code is _shared_ between them — the codec families, the IPC trait
stack, the shared-memory protocol. Without a facade, every such crate needs
`#[cfg]` around every log line, twice.

This crate makes one call site work in both:

```rust,ignore
consortium_log::trace!("channel {} lowered", name);
```

- `defmt` on → routes to the matching `defmt` macro.
- `tracing` on → routes to the matching `tracing` macro.
- both on → the event goes to both backends.
- neither on → the macro expands to **nothing**.

## Format syntax: the portable subset

Write a format string plus positional arguments, or Rust-style implicit captures:

```rust,ignore
trace!("channel {} lowered", name);      // positional
trace!("channel {name} lowered");        // implicit capture
```

Implicit captures are normalized to indexed arguments by
[`consortium-log-macros`](../consortium-log-macros) before being forwarded,
because `defmt` does not parse them itself.

Each argument must satisfy whatever the enabled backends require —
`defmt::Format` for `defmt`, `Display`/`Debug` for `tracing`. Backend-specific
extensions are only safe when that backend is the **sole** one enabled:

| Extension                         | Belongs to | Portable? |
| --------------------------------- | ---------- | --------- |
| `{=u8}`, `{=[u8]}` type hints     | `defmt`    | no        |
| `field = value` structured fields | `tracing`  | no        |
| `{}`, `{:?}`, `{name}`            | both       | yes       |

**Arguments must be side-effect free.** With no backend enabled the macros expand
to nothing and the arguments are never evaluated, so `trace!("n={}", counter.next())`
silently changes behaviour between builds.

## Features

| Feature   | Effect                                                              |
| --------- | ------------------------------------------------------------------- |
| `defmt`   | Forward to the `defmt` macros.                                      |
| `tracing` | Forward to the `tracing` macros. Implies `alloc`.                   |
| `alloc`   | Marker capability required by `tracing`; alone it pulls no backend. |

Default is **neither** — a crate that only wants to be loggable pays nothing
until a consumer picks a backend.

### The `defmt` dependency requirement

`defmt`'s proc-macros expand to literal `defmt::…` paths resolved at the _call
site_, not `$crate`-hygienic ones. So:

> **A crate that enables `consortium-log/defmt` must also depend on `defmt`
> directly.** The facade cannot supply the name.

This is the one sharp edge in the design, and it interacts badly with Cargo
feature unification: building several workspace crates together can enable
`consortium-log/defmt` from one crate's feature graph while a _different_ crate
in the same build has no `defmt` dependency of its own, giving
`failed to resolve: use of unresolved module or unlinked crate defmt` at that
crate's log call sites. If you hit that, the fix is a direct `defmt` dependency
in the crate that failed — matching what the firmware crates in this workspace
already do.

`tracing`'s macros are `$crate`-hygienic, so the `tracing` backend needs no such
companion dependency.

## How the dispatch works

Worth knowing before editing this crate: `#[cfg]` **inside** an exported macro's
body is resolved against the _calling_ crate's features, not this crate's. So
feature selection cannot live in the public macros. Instead:

1. Each backend's macros are re-exported by name under hidden aliases
   (`__defmt_trace`, `__tracing_trace`, …). Re-exporting individual macros rather
   than the whole crate is what makes them resolvable through `$crate::…` from a
   downstream crate.
2. Two hidden dispatch macros, `__defmt!` and `__tracing!`, are each defined
   twice — once gated on the backend feature, once as a no-op arm. That gating
   happens on the _definition_, where this crate's features are visible.
3. The public level macros carry no `#[cfg]` at all; they call both dispatchers
   and let the definitions decide.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
