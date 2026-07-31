# Introduction

Modern heterogeneous SoCs combine processors with different execution models on one
piece of silicon. Linux may own networking, storage, and the user interface while a
real-time core owns deterministic control loops; a trusted application may protect a
small security boundary alongside both.

That arrangement is powerful, but every boundary needs an explicit contract. The
chapters in this section introduce the four domains Consortium connects:

- [HAMPU](hampu.md) explains the hardware model and why asymmetric systems are useful.
- [IPC](ipc.md) introduces typed communication between processors.
- [TEE](tee.md) describes calls across the normal-world/secure-world boundary.
- [HMI](hmi.md) covers the application-facing user-interface layer.

The [User Guide](../guide/index.md) then maps those ideas to the current manifest,
generated runtime, and build pipeline.
