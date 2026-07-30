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
csti build [--manifest Consortium.toml] [--dist dist] [--release]
```

One command drives the whole pipeline: it compiles the Linux application and
controller firmware, plus the OP-TEE TA and HMI frontend if the manifest declares them;
post-processes the firmware for Linux remoteproc; generates the endpoint modules, device
tree, Kconfig fragment, boot integration, and a systemd unit; then assembles a local
`dist/` staging tree with a BitBake recipe.

**It does not deploy.** Nothing is written to a live target filesystem — `dist/` is a
local staging directory a downstream packaging recipe consumes.

### Flags

| Flag                        | Default           | Effect                                                                   |
| --------------------------- | ----------------- | ------------------------------------------------------------------------ |
| `--manifest <PATH>`         | `Consortium.toml` | The manifest to build.                                                   |
| `--dist <DIR>`              | `dist`            | Where to assemble the staging tree.                                      |
| `--release`                 | off               | Build in release mode.                                                   |
| `--skip-tee-sign`           | off               | Build the TA but stage the unsigned ELF instead of a signed `<uuid>.ta`. |
| `--bitbake-files-dir <DIR>` | —                 | Stage `dist/` under a recipe's `files/` directory.                       |
| `--bitbake-recipe <RECIPE>` | —                 | Invoke BitBake on this recipe (needs `--bitbake-build-dir`).             |
| `--bitbake-build-dir <DIR>` | —                 | The Yocto build directory to run BitBake in.                             |
| `--bitbake-dry-run`         | off               | Resolve and report the BitBake command without running it.               |

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
