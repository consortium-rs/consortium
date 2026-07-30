# Publishing to crates.io

This is the release runbook for the `consortium` workspace. It records what is
published, what is deliberately not, and the constraints that are easy to trip over.

## What ships

46 of the 48 workspace packages publish. Two do not:

| Package                 | Why it is `publish = false`                                    |
| ----------------------- | -------------------------------------------------------------- |
| `consortium-hmi-pocket` | Depends on PocketJS via git; cargo refuses to publish git deps |
| `melt-pot-shared`       | Example support crate                                          |

## Name mapping

Two of the natural names were already taken on crates.io by unrelated projects:

- `consortium` — taken. There is no umbrella facade crate, so nothing to rename;
  do not add one expecting to claim the bare name.
- `consortium-cli` — taken. The CLI publishes as **`csti`**, matching the binary
  it has always installed. Its source still lives in `crates/consortium-cli/`,
  so `cargo` invocations use `-p csti` against a directory of the older name.

Every other `consortium-*` name, plus `cogcore` and `cogcore-sys`, was free as of
the last check. Re-verify before a first publish:

```bash
for d in crates/*/; do n=$(basename "$d"); curl -sf "https://crates.io/api/v1/crates/$n" >/dev/null && echo "TAKEN: $n"; done
```

## PocketJS

`consortium-hmi-pocket` depends on `pocket-mod`, `pocket-ui-wgpu`, and `pocket3d`,
none of which are on crates.io. They are pinned to the upstream GitHub release
tag `v0.7.0` rather than to the local `submodules/pocketjs` checkout, so the
build is reproducible off a plain `git clone` with no submodule init.

The submodule is still required for the JS side — `bun tools/build.ts` produces
the `<app>.js`/`.pak` pair the builder stages — so it does not go away.

Note that the submodule HEAD is ahead of `v0.7.0`; the crate compiles against
both, but if you take new upstream Rust API, bump the tag in the manifest rather
than relying on the submodule. When PocketJS reaches crates.io, replace the three
`git`/`tag` entries with version requirements and drop the `publish = false`.

## Ordering

`scripts/publish.py` discovers publishable workspace crates with Cargo metadata,
orders them by their non-dev workspace dependencies, and checks crates.io before
each run. It reports each package as `new`, `update`, or `skip`; it refuses to
publish when a workspace version is older than the newest registry version.

```bash
just publish-dry    # verify
python3 scripts/publish.py --dry-run  # inspect the registry-aware plan
just publish        # publish with a crates.io token in place
```

**Both run on Linux only, by design** — see below. Each refuses to run on another
host rather than silently covering less than it appears to.

The publisher maintains separate allowances for new crate names (burst of 5,
then one every 10 minutes) and new versions (burst of 30, then one per minute).
It waits only before the next task that needs an exhausted allowance. A new
crate therefore does not delay an existing-crate update, and the final publish
never causes a pointless trailing sleep. If a dependency was just published,
its dependent checks registry visibility immediately before its own publish.

The complete release flow is:

```bash
uv run --project scripts --locked python scripts/release.py patch
# Use major/minor, or omit the component for the SemVer default.
```

It requires a clean Linux checkout, applies `just fix`, asks for confirmation,
updates Cargo/uv/Vite+ manifests and lockfiles, runs `just lint`, commits and
tags, creates the categorized GitHub release, dispatches the five-platform
`csti` binary workflow, and finally runs the rate-aware publisher.

The workflow uploads cargo-binstall-compatible archives named
`csti-<target>-v<version>.tgz` on Unix and
`csti-<target>-v<version>.zip` on Windows. The `csti` Cargo manifest declares
the matching package URL, archive format, and root-level binary path.

Intra-workspace dependencies carry both `path` and `version = "0.1.0"`, which is
what makes this work — cargo rewrites `path` to a registry requirement on upload.
**Dev-dependencies are deliberately left path-only.** Cargo strips a path-only
dev-dependency from the published manifest entirely, which keeps test-only edges
(such as `consortium-cfg` → `consortium-runtime-app`) out of the publish graph.
If you add a `version` to a dev-dependency, you pull it back into the ordering
and risk a cycle.

## Release from Linux

`cogcore`, `cogcore-sys`, `consortium-hmi-webkit`, and `consortium-runtime-app`
cannot compile on macOS at all — `cogcore-sys`'s build script panics outright with
"cogcore-sys only supports Linux". A macOS run has to exclude those four, so it
verifies 42 of the 46 crates while looking like a full pass. `just publish-dry`
and `just publish` therefore both refuse to run off Linux.

On an Apple Silicon Mac, the Apple `container` flow boots a native arm64 Linux VM
and is the supported path:

```bash
just linux-image-full                                          # once; fetches the WPE .deb
just linux-full 'cargo publish --workspace --dry-run --allow-dirty'
```

`just publish-dry-host` still exists for a quick partial check while iterating on
manifests, but it is not a release gate:

```bash
just publish-dry-host   # macOS; skips the four Linux-only crates
```

If a container run dies with `VZErrorDomain Code=2 "The storage device attachment
is invalid."`, an orphaned container is still holding the `consortium-target`
volume. `container list --all`, then `container stop <id>`, then retry.

See the AGENTS.md section on Linux builds on macOS for the DNS and target-volume
constraints.

## MSRV

`rust-version = "1.87"` in `[workspace.package]`. The edition-2024 floor is 1.85,
but `consortium-cfg` uses `u64::is_multiple_of`, stable since 1.87.

This is enforced by clippy's `incompatible_msrv` lint on every `just lint`, not by
a separate CI job — so a call to a newer API fails the lint bundle rather than
silently breaking downstream users. Raise `rust-version` when that lint fires;
do not add an `#[allow]`.

## Third-party content and licenses

Three crates redistribute vendor material and carry compound SPDX expressions
instead of inheriting the workspace `Apache-2.0`:

| Crate                    | License                               | Vendored content                                  |
| ------------------------ | ------------------------------------- | ------------------------------------------------- |
| `consortium-data`        | `Apache-2.0 AND BSD-3-Clause AND MIT` | `database/dts/` vendor device trees               |
| `cogcore-sys`            | `Apache-2.0 AND MIT`                  | `vendor/include/` Cog sources                     |
| `consortium-runtime-sys` | `Apache-2.0 AND BSD-3-Clause`         | `include/rsc_table.h` (Texas Instruments, Google) |

The device trees are dual-licensed and are redistributed under the permissive
half — BSD-3-Clause for STM32MP2, MIT for i.MX 9. **Files offered under GPL alone
cannot ship.** Two were removed while preparing for publish:
`imx93-11x11-frdm-pixpaper.dtso` (GPL-2.0) and `fsl,qoriq-clockgen.h`
(GPL-2.0-only). Neither was referenced by any other file in the database, so no
board selector or `#include` broke; the pixpaper board is simply no longer an
available `[profile] devkit`. `crates/consortium-data/database/dts/README.md`
carries the check to run when importing new vendor sources.

Unreviewed: `consortium-pac-imx9`, `consortium-pac-stm32mp2`, and the
`database/peri/*.yaml` chiptool metadata are generated from vendor CMSIS headers
and SVDs. This follows normal practice for the embedded Rust PAC ecosystem, but
the generated register and bitfield names are vendor-derived and the crates claim
plain `Apache-2.0`. Worth a licensing decision before a 1.0.

Every crate directory carries its own copy of the Apache-2.0 `LICENSE`, so the
text travels with the `.crate` tarball as Apache-2.0 §4(a) asks — the SPDX
`license` field alone would satisfy crates.io but ship no license text. Cargo
picks the file up automatically for crates with no `include` key; `cogcore-sys`
and `consortium-runtime-sys` have explicit `include` lists and name `LICENSE`
there. **A new crate needs `cp LICENSE crates/<name>/LICENSE`**, and an `include`
list needs the entry.

## Known gaps

- **`docs.rs` will fail** for the Linux-only crates unless their `[package.metadata.docs.rs]`
  pins a Linux target and, for `cogcore-sys`, the system Cog/WPE packages are
  unavailable there regardless.
