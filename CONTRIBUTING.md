# Contributing to Consortium

Thanks for your interest. Consortium is early-stage and moving quickly, so the most
useful contributions are usually small, focused, and accompanied by a test that pins
down the behavior you care about.

- Questions and design discussion: open a
  [Discussion](https://github.com/consortium-rs/consortium/discussions).
- Bugs: an issue with the chip, the manifest, and the failing command is ideal — the
  bug report template asks for exactly that.
- Changes: a pull request against `main`.
- Vulnerabilities: **not** a public issue. Follow [SECURITY.md](SECURITY.md).

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting set up

```bash
just setup
```

`just` with no arguments lists every recipe. The `justfile` is the command index, but it
moves faster than the docs — verify a recipe against the current crates before copying it
somewhere new.

Common loops:

```bash
just check
```

```bash
just lint
```

```bash
just fix
```

Use `cargo test -p <crate>` while iterating; full-workspace commands can hit
platform-sensitive crates. Subsystem bundles are `just test <ipc|tee|hmi|cfg|dbg|codec> host`.

### Host execution policy

Run non-Rust-build work directly on the host: Git and GitHub operations, Python scripts,
`uv`/`uvx`, Vite+ (`vp`), manifest and lockfile updates, formatters, and non-Rust linters.

- **macOS** — route Rust builds, tests, Clippy, and publish verification through the
  Linux container recipes (`just linux '...'`, `just linux-full '...'`). Linux-only
  crates (`consortium-runtime-app`, `cogcore`, `cogcore-sys`, `consortium-hmi-webkit`)
  cannot compile natively at all. See the container notes in `AGENTS.md` — in particular,
  do not set a global `CARGO_TARGET_DIR`, and pass a routable `--dns`.
- **Linux** — the standard host environment and normal `just`/`cargo` commands.
- **Windows** — run Rust build transactions inside WSL.

Hardware, SDK, UIO, remoteproc, OP-TEE runtime, and Cog/WebKit checks stay separate from
the default host checks. Do not assume every developer machine can run them.

## Making a change

Work from the current sources rather than from older docs, examples, or design notes —
`docs/book/src/dev/**` and the dated `docs/book/src/log/**` entries are history, not
specification.

A few conventions that come up in review:

- Rust edition 2024, MSRV `1.87` (`rust-version` in `[workspace.package]`). Clippy's
  `incompatible_msrv` enforces it on every `just lint`; raise the MSRV deliberately
  rather than `#[allow]`-ing the lint.
- Workspace lints deny undocumented unsafe blocks, unsafe operations outside an `unsafe`
  block, and multiple unsafe ops per block. Keep unsafe narrow and state the hardware,
  memory, aliasing, or ABI invariant it relies on.
- Every file carries the Apache-2.0 header and an `SPDX-License-Identifier`.
- `cargo fmt` for Rust, Taplo for TOML (two-space indent, 100 columns), `typos.toml` for
  spelling, `.ls-lint.yml` for file and directory naming. `just fix` applies all of them.
- Prefer extending `consortium-cfg` lowering before the builder pipeline, and schema
  types before validation, lowering, and codegen. Update the snapshot tests under
  `crates/consortium-cfg/tests/` when generated output intentionally changes.
- Do not hand-edit generated output (PAC crates, `src/peripherals/<chip>.rs`, chip
  databases). Change the generator or the patch workflow and re-run it.
- New dependencies need Apache-2.0-compatible licensing and must respect the relevant
  `no_std`/`alloc`/`std` constraints. Intra-workspace deps carry both `path` and
  `version`; dev-dependencies stay path-only on purpose (see `docs/PUBLISHING.md`).

### Tests

Every behavioral change should come with a test that fails without it. Prefer host-side
coverage that does not need hardware:

- Fake MMIO, heap-backed regions, and mock doorbells for protocol logic.
- trybuild and insta snapshots for proc-macro behavior.
- `cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test`
  for the no-std-shaped shared-memory future path.
- UART protocol changes should run `cargo test -p consortium-ipc-transport-uart`, the
  same with `--no-default-features`, and `cargo test -p consortium-ipc-transport-uart-unix`.

### Pull requests

- One logical change per PR; keep unrelated formatting churn out of the diff.
- Commit messages follow Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`,
  `ci:`, `release:`), matching the existing history.
- Say which commands you ran and on what host. If something is untested because it needs
  hardware or an SDK you do not have, say so explicitly rather than implying coverage.
- CI runs the linters, the host test bundle, and PAC parse/validate. Green CI is
  necessary, not sufficient — a human reviews the change.

## AI coding assistants

Using AI tools on Consortium is **allowed and encouraged**. They are good at the parts of
this repository that are mechanical and voluminous, and there is a lot of that here.

What does not change is where responsibility sits. This policy follows the spirit of the
Linux kernel's [AI coding assistants
documentation](https://docs.kernel.org/process/coding-assistants.html): the assistant
helps, the human is the author.

### Human sign-off is mandatory

- **Only a human may sign off.** An AI agent must never add a `Signed-off-by:` line.
  Signing off is a legal certification of provenance and licensing that only a person can
  make, and by signing off you take full responsibility for the patch — including for
  every line a tool produced.
- **Attribute the assistance.** Add an `Assisted-by:` trailer naming the tool and model,
  for example:

  ```text
  Assisted-by: claude-code:claude-opus-5
  Signed-off-by: Your Name <you@example.com>
  ```

  Name the assistant and any specialized analysis tooling; ordinary editors, formatters,
  and compilers do not belong there.

- **You must be able to explain the change.** If you cannot say why a line is correct,
  it is not ready to submit. "The model wrote it" is not a review.
- **Licensing is on you.** Generated code must be Apache-2.0-compatible and carry the
  standard header and SPDX identifier. Do not submit output you suspect was reproduced
  verbatim from incompatible sources.

### How much review, and where

Review effort should scale with what the code can break. In this repository the
consequences are unusually asymmetric: a wrong descriptor field or a mis-stated unsafe
invariant is a cross-core corruption that reproduces on hardware and nowhere else.

**Read every line yourself** — line by line, against the datasheet, reference manual, or
protocol description where one applies — for:

- Core functionality: `consortium-ipc` traits and channel logic, transport and descriptor
  protocols, doorbell drivers, shared-memory and cache-maintenance code, HAL drivers,
  `consortium-tee` parameter conversion, and anything `unsafe`.
- Anything that touches a wire format, a memory layout, an ABI, or a safety invariant.
- Configuration schema, validation rules, and lowering — a permissive validator lets a
  broken system build.
- The **intent** of a test: what it asserts, and whether that assertion would actually
  fail if the behavior regressed.

**Unreviewed assistant output is acceptable** — meaning you still read it, but you do not
have to justify it line by line — only for mechanical work whose correctness is checked
by something other than your reading:

- Boilerplate and scaffolding: proc-macro plumbing, code generation, generator scripts'
  repetitive output, repetitive `impl` blocks, manifest wiring, doc formatting.
- Test **helper** functions and fixtures — the harness, not the cases. Test cases
  themselves are specification, and you write those.

Even in that category, the outputs must be reviewed by a human:

- **Snapshots.** Never accept an insta snapshot because the test went green. Read the
  generated DTS, Kconfig, endpoint module, or macro expansion and confirm it is what the
  hardware and the manifest actually call for. A snapshot blessed without reading is how
  a wrong output becomes the expected output.
- **Test results.** Confirm the new test fails without your change. A generated test that
  passes both ways documents nothing.
- **Generated data.** Re-run the generator rather than editing its output, and check the
  diff against the chip dump or vendor source it came from.

If an AI tool produced a substantial part of a patch and you have not been able to review
it to this standard, say so in the PR description. An honest "the codegen path is
assistant-written and I have verified the snapshots but not audited the generator" is
useful information for a reviewer. A silent submission is not.

## License

By contributing, you agree that your contributions are licensed under the Apache License,
Version 2.0, unless you state otherwise. Note the exceptions for crates that redistribute
vendor material (`consortium-data`, `consortium-runtime-sys`, `cogcore-sys`) and the
dual-licensing requirement for vendor device trees under
`crates/consortium-data/database/dts/` — GPL-only files cannot ship.
