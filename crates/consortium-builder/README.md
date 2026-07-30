# consortium-builder

Compiles, post-processes, and stages every artifact a `Consortium.toml` describes.

[`consortium-cfg`](../consortium-cfg) answers _"what does this system look like, and is
it valid?"_ and emits generated source. This crate answers _"now actually build it"_:
four compilers, a post-processing step remoteproc requires, and a staging tree a
packaging recipe can consume.

It stops short of deployment. **Nothing here writes to a live target filesystem** — the
output is a local `dist/` tree plus an optional BitBake hand-off.

The user-facing entry point is [`csti build`](../consortium-cli); this crate is the
library behind it.

## Pipeline

```text
Consortium.toml
  ├─ 1. firmware      cargo, bare-metal target → ELF → rename .vector_table
  ├─ 2. elf_map       the defmt map the app embeds for decoding
  ├─ 3. hmi           vp / npm / bun → frontend bundle
  ├─ 4. tee           OP-TEE TA dev kit → signed <uuid>.ta
  ├─ 5. application   cargo, usually cross-compiled → Linux executable
  ├─ 6. systemd       render the unit
  ├─ 7. dist          assemble dist/ + a BitBake recipe
  └─ 8. bitbake       optional stage / invoke / dry-run hand-off
```

### Plan, then run

`pipeline::plan` resolves every stage's inputs — paths, env vars, artifact locations —
**without running any build**, threading the data flow between stages. `pipeline::run`
plans, then executes.

That split is what makes the pipeline debuggable. A wrong sysroot or a missing firmware
ELF surfaces as a planning failure naming the stage, rather than as a cargo error
several minutes into a cross-compile.

## The `Compiler` trait

Firmware, the Linux application, the OP-TEE TA, and the HMI frontend are built by very
different toolchains:

| Module        | Artifact            | Built with                           |
| ------------- | ------------------- | ------------------------------------ |
| `firmware`    | coprocessor ELF     | `cargo`, bare-metal target           |
| `application` | Linux executable    | `cargo`, usually cross-compiled      |
| `optee`       | signed OP-TEE TA    | the OP-TEE TA dev kit                |
| `web`         | HMI frontend bundle | `vp` / `npm` (Cog) or `bun` (Pocket) |

`Compiler` makes them uniform: each implementor owns a `Configuration` carrying all its
parameters, a typed `Error`, and an `Output`. Its methods take the config rather than
`&self`, so every implementor has the same `.build(cfg)` / `.output_path(&cfg)` shape
and the pipeline needs no per-kind special casing.

## Compile-time configuration

Much of what generated code needs arrives as `CONSORTIUM_*` environment variables read
with `option_env!`/`env!`, not as generated source. `compile::env` describes what gets
injected: `SharedMemConfig`, `DbgConfig`, `HmiConfig`.

Two details worth knowing:

- **Numeric values are emitted as decimal strings**, so a firmware-side consumer that
  needs const parsing can use `const-str` without depending on the std-oriented schema
  crates.
- **There is deliberately no `CONSORTIUM_IPC_BASE`.** Controller firmware gets its
  region base baked into generated init code, and the Linux application reaches the same
  memory through its UIO device — so neither side needs the physical address from the
  environment. Channel-local metadata (`CONSORTIUM_IPC_LENGTH`, `CONSORTIUM_IPC_UIO`,
  `CONSORTIUM_IPC_MTU`) is injected; the base is not.

## Things that are easy to get wrong

**Firmware post-processing is mandatory.** Linux remoteproc expects a coprocessor's
vector table in a section named `.isr_vectors`, but `cortex-m-rt` emits
`.vector_table`. The pipeline renames it and keeps the original alongside the
deployable ELF — an ELF that boots and an ELF a debugger understands are not quite the
same file.

**Sysroot resolution has an order.** `[profile] sysroot` first, then
`SDKTARGETSYSROOT` / `OECORE_TARGET_SYSROOT`, so a manifest can pin a sysroot without
the developer having sourced an SDK script. On an arm64 Linux host,
`aarch64-unknown-linux-gnu` is a _native_ build and `sysroot = "/"` resolves to the
image root — which is how the melt-pot manifests build inside an Apple `container` VM
with no SDK at all.

**TA signing is optional on purpose.** Signing needs the dev kit's signer script and
keys, which a laptop or CI runner usually lacks, so the unsigned ELF can be staged
instead (`--no-sign-ta`). Compiling the TA and signing it are separable steps.

**Device-tree compilation is opt-in, for licensing reasons.** The generated `.dts`
includes vendor `.dtsi` (BSD/MIT, vendored in `consortium-data`) _and_
`dt-bindings/*.h` (GPL-2.0 kernel headers, **not** vendored). So `dtc` compilation
requires `[dts] linux` pointing at the user's own kernel checkout. `dtc` verification
of an overlay works without it, using preprocessor defines that reproduce the
interrupt macros.

**The HMI build command can be overridden.** `CONSORTIUM_HMI_BUILD_CMD` takes
precedence over `vp`/`npm`. That exists because the frontend sometimes has to be built
elsewhere: inside a Linux container the host's `node_modules/` is already populated, so
pointing the command at `/bin/true` lets the pipeline consume the existing dist.

## Staging

The assembled tree can contain `app/`, `firmware/`, `ta/`, `www/`, `dts/linux/`,
`kernel/`, `boot/`, and `systemd/`, plus a generated BitBake recipe.

Firmware ELFs land at `dist/firmware/<core>.elf` so a firmware-packaging recipe can
install them to `/lib/firmware/<core>` for remoteproc's kernel-side loader. That is
unrelated to the defmt ELF map, which is embedded in the application binary.

`[profile] deploy_root` is a **naming convention only** — it composes the HMI dist
path, the systemd unit's `ExecStart=`/`WorkingDirectory=`, and the firmware install
location a recipe consumes. The pipeline never writes there.

## BitBake hand-off

Staging (`--bitbake-files-dir`) and invoking (`--bitbake-recipe` +
`--bitbake-build-dir`) compose: given both, `dist/` is staged into the recipe's
`files/` directory first, then BitBake runs. `--bitbake-dry-run` records the command
without running it — that is the split-machine flow, where `dist/` is produced on one
host and the Yocto build happens on another.

## Extending

Prefer extending `consortium-cfg` lowering first, then this pipeline. When working
here, keep the four boundaries distinct: **planning**, **compilation**, **staging**,
and the **external BitBake hand-off**. Mixing them is what makes a build pipeline
untestable.

`progress::BuildReporter` is deliberately minimal — library callers can ignore it and
keep inherited subprocess output, while the CLI passes a reporter that holds a
persistent step prompt under cargo/vp/bitbake output.

## Testing

```bash
cargo test -p consortium-builder
```

Note that running the full pipeline against the melt-pot examples rewrites their
committed `src/consortium.gen.rs` with absolute `include_bytes!` paths. Revert that
afterwards unless the change is intended.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
