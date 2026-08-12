# User Guide

Consortium treats a multi-processor product as one buildable system. The normal workflow
is deliberately short:

1. describe the topology and resource ownership in `Consortium.toml`;
2. define boundary-safe message types in a shared crate;
3. write application and firmware logic against generated `Peripherals` and `Channels`;
4. run `csti build --linux <KERNEL_TREE>`; and
5. integrate the resulting `dist/` tree with the target image.

The chapters follow that sequence:

- [Getting Started](getting-started.md) tours the workspace and runs the first checks.
- [System Architecture](architecture.md) explains how the crates and generated code fit
  together.
- [Project Manifest](manifest.md) documents the current `Consortium.toml` schema.
- [Generated Runtime](runtime.md) shows the endpoint entry points and IPC surface.
- [Build and Deployment](build-and-deploy.md) explains `csti build`, its artifacts, and
  the packaging boundary.

> The development notes later in the book record design work and hardware bring-up.
> They are useful context, but they are not the authoritative API reference.
