# csti

The Consortium command-line interface: build and package a heterogeneous
multiprocessing system from one `Consortium.toml`.

> **Crate name.** This publishes as **`csti`**, matching the binary, because
> `consortium-cli` and the bare `consortium` name are both taken on crates.io by
> unrelated projects. The source directory keeps its descriptive name, so cargo
> invocations use `-p csti`.

## Install

```bash
cargo install csti
```

## Usage

```bash
csti build --linux <KERNEL_TREE> [--manifest Consortium.toml] [--dist dist] [--release]
csti fetch [--manifest Consortium.toml]
```

One command drives the whole pipeline: it compiles the Linux application and
controller firmware, plus the OP-TEE TA and HMI frontend if the manifest declares them;
post-processes the firmware for Linux remoteproc; generates the endpoint modules, device
tree, Kconfig fragment, boot integration, and a systemd unit; then assembles a local
`dist/` staging tree with a BitBake recipe.

**It does not deploy.** Nothing is written to a live target filesystem — `dist/` is a
local staging directory a downstream packaging recipe consumes.

### `build` flags

| Flag                        | Default           | Effect                                                                   |
| --------------------------- | ----------------- | ------------------------------------------------------------------------ |
| `--manifest <PATH>`         | `Consortium.toml` | The manifest to build.                                                   |
| `--dist <DIR>`              | `dist`            | Where to assemble the staging tree.                                      |
| `--linux <DIR>`             | required          | Linux checkout supplying BSP DTS/DTSI files and dt-bindings.             |
| `--release`                 | off               | Build in release mode.                                                   |
| `--offline`                 | off               | Run every cargo invocation with `--offline` (see `fetch` below).         |
| `--skip-tee-sign`           | off               | Build the TA but stage the unsigned ELF instead of a signed `<uuid>.ta`. |
| `--bitbake-files-dir <DIR>` | —                 | Stage `dist/` under a recipe's `files/` directory.                       |
| `--bitbake-recipe <RECIPE>` | —                 | Invoke BitBake on this recipe (needs `--bitbake-build-dir`).             |
| `--bitbake-build-dir <DIR>` | —                 | The Yocto build directory to run BitBake in.                             |
| `--bitbake-dry-run`         | off               | Resolve and report the BitBake command without running it.               |

### `csti fetch` and `--offline`

A build spans several independent Cargo projects — one firmware crate per remote
coprocessor, the TA, the Linux application — plus a frontend with its own package
manager, so no single `cargo fetch` covers the system. `csti fetch` resolves the same
project set the build plan does and downloads all of it up front:

```bash
csti fetch                  # populate every cache
csti build --linux ~/src/linux --offline # build with no network access
```

| Flag                  | Default           | Effect                                                                 |
| --------------------- | ----------------- | ---------------------------------------------------------------------- |
| `--manifest <PATH>`   | `Consortium.toml` | The manifest to resolve.                                               |
| `--skip-hmi`          | off               | Fetch only Rust dependencies; skip the frontend's package install.     |
| `--skip-rust-targets` | off               | Skip `rustup target add` for the cross targets the build compiles for. |

It runs three kinds of download:

1. `rustup target add` for each distinct target triple. A cross build needs that
   target's prebuilt `std`, which `cargo fetch` does not provide. A host with no
   `rustup` (an SDK-managed or distro toolchain) skips this step with a note rather
   than failing.
2. `cargo fetch --target <triple>` per Cargo project, scoped so a bare-metal firmware
   crate does not pull a host target's dependency set. No feature selection is needed:
   `cargo fetch` works from the lock file, which carries optional dependencies whether
   or not their feature is active.
3. The frontend's package install — `vp`/`npm install` for Cog, `bun install` for
   Pocket, overridable with `CONSORTIUM_HMI_FETCH_CMD` (the counterpart to
   `CONSORTIUM_HMI_BUILD_CMD`; point it at `/bin/true` when `node_modules/` is already
   populated, as in the Linux container flow).

`--offline` then adds `--offline` to every cargo invocation, so a missing dependency
fails loudly instead of silently reaching the network. Only cargo is constrained: the
frontend build command is the project's own and is run unchanged, which is why the
package install belongs to `fetch`.

Not covered, because these are host provisioning rather than project dependencies with
their own vendor-specific acquisition paths: the OP-TEE dev kit, a Yocto sysroot, the
Linux checkout required by `csti build --linux`, and BitBake's own downloads.

### `--skip-tee-sign`

Signing a Trusted Application needs the OP-TEE dev kit's signer script and its keys,
which a laptop or CI runner usually does not have. This flag builds the TA and stages
the unsigned ELF instead, so TA compilation stays covered without the signing material.

### The split-machine BitBake flow

Staging and invoking compose. Given both `--bitbake-files-dir` and `--bitbake-recipe`
(+ `--bitbake-build-dir`), `dist/` is staged into the recipe's `files/` directory first,
then BitBake runs.

`--bitbake-dry-run` exists for the case where the Consortium build and the Yocto build
happen on **different machines**: assemble `dist/` here, have the exact BitBake command
reported, and run it later on the Yocto host.

## Where the work happens

`csti` is a thin `clap` front end. The pipeline lives in
[`consortium-builder`](../consortium-builder); validation, lowering, and codegen live in
[`consortium-cfg`](../consortium-cfg).

What the CLI itself adds is progress reporting: it passes a `BuildReporter` that keeps a
persistent step prompt visible underneath cargo, `vp`, and BitBake output, so a
long cross-compile still says which of the eight stages it is in.

## Manifest reference

`Consortium.toml`'s schema is defined by
[`consortium-cfg-common`](../consortium-cfg-common), which is the authoritative
reference. `crates/consortium-cfg/Consortium.example.mimx95.toml` and
`Consortium.example.stm32mp257.toml` are worked examples for the two supported chip
families.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
