# consortium-log-macros

Internal format-string normalization for [`consortium-log`](../consortium-log).

> **Implementation detail.** Depend on `consortium-log`. This crate's API is not
> stable and its single macro is not meant to be called directly.

## What it does

`consortium-log` forwards one call site to two backends that do not agree about
format strings. Rust's implicit captures —

```rust,ignore
trace!("channel {name} lowered")
```

— work with `tracing`, which is built on `core::fmt`. They do **not** work with
`defmt`, which parses format strings itself and has no notion of pulling `name`
out of the enclosing scope.

Rather than banning the nicer syntax in shared code, `expand_captures` rewrites it
into the indexed form both backends accept:

```text
trace!("channel {name} lowered")
  →  __defmt!(trace, "channel {0} lowered", name)
```

Explicit positional arguments keep their indices; captures are appended after
them, and a name used twice reuses one index.

## Why the callback is an input

The macro takes **three parenthesized groups** — callback path, log level, and the
original log arguments — and re-emits an invocation of that callback.

Threading the callback path *through* the macro is what preserves
`consortium-log`'s `$crate` hygiene. The facade passes `$crate::__defmt`, so the
expansion names the facade's own dispatch macro regardless of which crate the log
call was written in. A proc-macro cannot synthesize a `$crate` token of its own,
so the path has to arrive as data.

## What is deliberately left alone

Two inputs are forwarded byte-for-byte rather than rewritten:

- **Anything that fails to parse** as a format string plus arguments. Emitting the
  original tokens lets the backend produce the real diagnostic, pointing at the
  user's call site instead of at this rewrite.
- **Named arguments** (`name = expression`). Those already mean something to Rust
  and to `tracing`'s structured fields, so `{name}` alongside them is not an
  implicit capture and must not be treated as one.

## Testing

```bash
cargo test -p consortium-log-macros
cargo test -p consortium-log --features defmt      # end-to-end through the facade
cargo test -p consortium-log --features tracing
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
