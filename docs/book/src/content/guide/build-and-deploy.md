# Build and Deployment

`csti build` is the user-facing system build. It compiles every endpoint described by
the manifest and assembles deployable inputs, but deliberately stops at a local staging
directory.

## Command

```bash
csti build --linux ~/src/linux --manifest Consortium.toml --dist dist --release
```

Inside the Consortium workspace, the equivalent command is:

```bash
cargo run -p csti -- build --linux ~/src/linux --manifest Consortium.toml --dist dist --release
```

| Flag                        | Purpose                                                       |
| --------------------------- | ------------------------------------------------------------- |
| `--manifest <PATH>`         | Manifest to build; defaults to `Consortium.toml`              |
| `--dist <DIR>`              | Local staging root; defaults to `dist`                        |
| `--linux <DIR>`             | Required Linux checkout supplying BSP DTS and dt-bindings     |
| `--release`                 | Build endpoint artifacts in release mode                      |
| `--offline`                 | Run every cargo invocation with `--offline`                   |
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

## Offline builds

A build spans several independent Cargo projects plus a frontend package manager, so no
single `cargo fetch` covers the system. `csti fetch` resolves the same project set the
build plan does and downloads all of it in advance:

```bash
csti fetch
csti build --linux ~/src/linux --offline
```

`fetch` installs each cross target's Rust standard library with `rustup target add`,
runs `cargo fetch --target <triple>` per Cargo project, and installs the HMI frontend's
packages. `--skip-hmi` and `--skip-rust-targets` narrow it; a host without `rustup`
skips the target step with a note rather than failing.

`--offline` constrains cargo only. The frontend build command is the project's own and
runs unchanged, which is why its package install belongs to `fetch`.
`CONSORTIUM_HMI_FETCH_CMD` overrides that install the way `CONSORTIUM_HMI_BUILD_CMD`
overrides the build.

Host provisioning stays outside this: the OP-TEE dev kit, a Yocto sysroot, kernel sources
for `dtc`, and BitBake's own downloads each have their own acquisition path.

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

Pass a kernel source checkout to `csti build --linux <DIR>`. The build reads the
selected BSP DTS/DTSI files, their quoted includes, and `dt-bindings` from that
checkout. Consortium does not bundle Linux device-tree sources.

Cog HMI builds use `vp build` when available, then `npm run build`. Pocket builds use
`bun tools/build.ts`. `CONSORTIUM_HMI_BUILD_CMD` overrides either command for custom or
split-host workflows.

## BitBake hand-off

To stage the generated files into an existing layer without running BitBake:

```bash
csti build \
  --linux ~/src/linux \
  --bitbake-files-dir ../meta-product/recipes-apps/consortium/files
```

To stage and build in one flow, also provide the recipe and Yocto build directory:

```bash
csti build \
  --linux ~/src/linux \
  --bitbake-files-dir ../meta-product/recipes-apps/consortium/files \
  --bitbake-recipe consortium-app \
  --bitbake-build-dir ../build
```

Add `--bitbake-dry-run` when the Consortium build and Yocto build happen on different
machines. It records the resolved command without executing it.

Deployment remains an integration decision: the generated recipe, device tree, kernel
configuration, boot arguments, and systemd unit are inputs to the product image rather
than an imperative board updater.
