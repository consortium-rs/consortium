# CLAUDE.md

## Commands

```bash
cargo build
cargo test
cargo test -p <crate>
cargo clippy
cargo fmt
mdbook serve docs/
```

**Platform notes:** Linux-only: `consortium-ipc-transport-uio`, `consortium-tee`, `consortium-runtime-app`, `consortium-rpi`. Bare-metal (Cortex-M): `consortium-ipc-transport-mem`, `consortium-runtime-mcu`.

## Architecture

Rust workspace — async IPC abstraction for HMP SoCs (STM32MP2, i.MX 95; Cortex-A + Cortex-M).

| Category | Crates |
|----------|--------|
| Core IPC | `consortium-ipc` |
| Transports | `consortium-ipc-transport-mem`, `consortium-ipc-transport-uio` |
| Doorbells | `consortium-ipc-doorbell-hsem`, `-mu`, `-gpio` |
| Codecs | `consortium-codec-postcard`, `-prost`, `-rkyv` |
| Runtimes | `consortium-runtime-mcu`, `consortium-runtime-app` |
| Config & Build | `consortium-common`, `consortium-builder`, `consortium-cli` |
| RPI | `consortium-rpi`, `consortium-rpi-meta` |
| TEE | `consortium-tee`, `consortium-tee-proc-macros`, `consortium-tee-test` |
| Utilities | `consortium-ffi`, `consortium-hmi`, `consortium-ort` |

**Core trait hierarchy:**
```
Doorbell  — ring/wait/pending
Region    — memory area with optional cache management
Transport — owns a Doorbell; async send/recv
Codec     — serialize/deserialize
Channel<D, T, Tr, C> — typed endpoint (D = Tx|Rx)
```

**Doorbell status:** HSEM (STM32MP2x) ✅ · MU (i.MX 9x) ✅ · GPIO 🔄

## Docs

`docs/src/arch/` — design overviews · `docs/src/dev/` — implementation guides (ipc, tee, rpi)
