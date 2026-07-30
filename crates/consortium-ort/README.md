# consortium-ort

ONNX Runtime integration for Consortium.

## Status: reserved, not implemented

**This crate is a placeholder. It currently exports nothing.**

The `ort` dependency is declared so the intended configuration is pinned and the build is
proven, but no wrapper exists yet.

The intent is to let a Consortium application run inference on the A-core alongside its
real-time IPC work — a coprocessor samples sensors and forwards frames over a channel, the
application runs a model on them. [`consortium-runtime-app`](../consortium-runtime-app)
already reserves an `ort` feature that would re-export this.

## The dependency configuration is the deliberate part

```toml
ort = { default-features = false, features = ["api-24", "ndarray", "std"], version = "2.0.0-rc.12" }
```

`default-features = false` matters for a cross-compiled embedded Linux target. The default
features would fetch or link an ONNX Runtime binary chosen for the **host**, whereas an
embedded target needs the one in its own sysroot — the same concern that makes
`consortium-builder` resolve a target sysroot explicitly rather than trusting the ambient
toolchain.

## Features

| Feature   | Effect                     |
| --------- | -------------------------- |
| `tracing` | Forwards to `ort/tracing`. |

With `tracing` on, ORT diagnostics land in the same subscriber
`consortium-runtime-app`'s `#[main]` macro installs, rather than in a second logging
system.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
