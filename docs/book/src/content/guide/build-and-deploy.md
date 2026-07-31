# Build and Deployment

`csti build` is the user-facing system build. It compiles every endpoint described by
the manifest and assembles deployable inputs, but deliberately stops at a local staging
directory.

## Command

```bash
csti build --manifest Consortium.toml --dist dist --release
```

Inside the Consortium workspace, the equivalent command is:

```bash
cargo run -p csti -- build --manifest Consortium.toml --dist dist --release
```

| Flag                        | Purpose                                                       |
| --------------------------- | ------------------------------------------------------------- |
| `--manifest <PATH>`         | Manifest to build; defaults to `Consortium.toml`              |
| `--dist <DIR>`              | Local staging root; defaults to `dist`                        |
| `--release`                 | Build endpoint artifacts in release mode                      |
| `--skip-tee-sign`           | Stage an unsigned TA ELF when signing material is unavailable |
| `--bitbake-files-dir <DIR>` | Copy the assembled tree into a recipe's `files/` directory    |
| `--bitbake-recipe <PN>`     | Invoke a BitBake recipe; requires its build directory         |
| `--bitbake-build-dir <DIR>` | Yocto build directory used for BitBake                        |
| `--bitbake-dry-run`         | Resolve and report the BitBake command without running it     |

## Pipeline

The builder keeps planning, compilation, staging, and external packaging as separate
boundaries:

1. parse, validate, and lower `Consortium.toml`;
2. generate endpoint modules and build-time configuration;
3. build controller firmware and post-process it for remoteproc;
4. build the optional HMI and trusted application;
5. build the Linux application;
6. render device-tree, kernel, boot, and systemd integration;
7. assemble `dist/`; and
8. optionally hand the tree to BitBake.

Linux remoteproc expects the firmware vector table as `.isr_vectors`, while
`cortex-m-rt` emits `.vector_table`. The pipeline performs that rename and retains the
original/debug artifact alongside the deployable ELF.

## Staging tree

Only configured subsystems appear:

```text
dist/
├── app/          Linux executable
├── firmware/     deployable and debug firmware ELFs
├── ta/           signed TA or unsigned development ELF
├── www/          Cog/WPE assets
├── hmi/          Pocket bundle and resource pack
├── dts/linux/    generated DTS/DTSO and compiled DTB/DTBO when available
├── kernel/       UIO config fragment and optional kernel bbappend
├── boot/         family-specific boot inputs and integration notes
├── systemd/      generated application unit
└── *.bb          generated BitBake recipe
```

`[profile].deploy_root` describes where downstream packaging will install the
application-facing files. It is used to generate paths in the systemd unit and runtime
configuration; `csti` never writes to that path on the build host or target.

## Toolchain inputs

The Linux application sysroot is resolved in this order:

1. `[profile].sysroot`;
2. `SDKTARGETSYSROOT`; then
3. `OECORE_TARGET_SYSROOT`.

Device-tree source can be emitted without a kernel checkout. To compile it, set
`[dts].linux` to a kernel source tree so the build can use its GPL-licensed
`dt-bindings` headers. Consortium does not vendor those headers.

Cog HMI builds use `vp build` when available, then `npm run build`. Pocket builds use
`bun tools/build.ts`. `CONSORTIUM_HMI_BUILD_CMD` overrides either command for custom or
split-host workflows.

## BitBake hand-off

To stage the generated files into an existing layer without running BitBake:

```bash
csti build \
  --bitbake-files-dir ../meta-product/recipes-apps/consortium/files
```

To stage and build in one flow, also provide the recipe and Yocto build directory:

```bash
csti build \
  --bitbake-files-dir ../meta-product/recipes-apps/consortium/files \
  --bitbake-recipe consortium-app \
  --bitbake-build-dir ../build
```

Add `--bitbake-dry-run` when the Consortium build and Yocto build happen on different
machines. It records the resolved command without executing it.

Deployment remains an integration decision: the generated recipe, device tree, kernel
configuration, boot arguments, and systemd unit are inputs to the product image rather
than an imperative board updater.
