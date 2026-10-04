#!/usr/bin/env python3
"""Bump the pinned Rust toolchain and triage what the new compiler changes.

The run has six stages, each logged under ``target/rust-update/<version>/``:

1. Resolve the latest stable release (``rustup check``, falling back to the
   dist channel manifest) and compare it with ``rust-toolchain.toml``.
2. Run the host test bundle on the *current* toolchain as a baseline, then
   rewrite the channel, install it, and (on macOS) rebuild the Linux images
   that bake the toolchain in.
3. Re-run the same tests with snapshot updates disabled and diff the failures
   against the baseline, so only regressions introduced by the bump surface.
4. Re-run the failing commands with insta/trybuild in overwrite mode, then once
   more strictly, so accepted snapshot churn is separated from real breakage.
5. Apply Rust/JS/Python fixers and formatters, then run ``just check`` and the
   lint bundle.
6. Fetch the rustc and Clippy changelogs for the bumped range and write a
   prompt asking an AI reviewer where the repository could adopt better style.

Nothing is committed; review the worktree with ``git diff`` afterwards.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path

import tomlkit

ROOT = Path(__file__).resolve().parents[1]
TOOLCHAIN_FILE = ROOT / "rust-toolchain.toml"
WORKSPACE_MANIFEST = ROOT / "Cargo.toml"
LOG_ROOT = ROOT / "target" / "rust-update"

CHANNEL_MANIFEST_URL = "https://static.rust-lang.org/dist/channel-rust-stable.toml"
RUST_RELEASES_URL = (
    "https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md"
)
CLIPPY_CHANGELOG_URL = (
    "https://raw.githubusercontent.com/rust-lang/rust-clippy/master/CHANGELOG.md"
)

# The host bundle CI runs (`.github/workflows/host-tests.yml`), plus HMI, which
# the full Linux image can link. Recipes are expanded into their individual
# cargo commands so one failing crate does not hide the rest.
TEST_RECIPES = (
    "test-dbg-host",
    "test-codec-host",
    "test-cfg-host",
    "test-ipc-host",
    "test-tee-host",
    "test-hmi-host",
)
# Fail on any snapshot drift: insta and `consortium-cfg`'s artifacts snapshot
# otherwise rewrite themselves outside CI, and trybuild fails by default.
STRICT_ENV = {"INSTA_UPDATE": "no", "UPDATE_SNAPSHOTS": "0"}
OVERWRITE_ENV = {"INSTA_UPDATE": "always", "TRYBUILD": "overwrite"}
SNAPSHOT_SUFFIXES = (".snap", ".snap.new", ".stderr", ".stdout")
HOST_LINT_RECIPES = (
    "fmt-check",
    "taplo-check",
    "typos",
    "ls-lint",
    "oxlint",
    "ruff-lint",
)

VERSION_RE = re.compile(r"\b(\d+)\.(\d+)\.(\d+)\b")
CMD_MARK = "::rust-update-cmd::"
STATUS_MARK = "::rust-update-status::"
FAILED_TEST_RE = re.compile(r"^test (?P<name>.+?) \.\.\. (?:FAILED|mismatch|error)\s*$")
RELEASE_HEADER_RE = re.compile(
    r"^Version (\d+\.\d+\.\d+) \(([^)]*)\)\s*$", re.MULTILINE
)
CLIPPY_HEADER_RE = re.compile(r"^## Rust (\d+)\.(\d+)\s*$", re.MULTILINE)


@dataclass(frozen=True, order=True)
class Version:
    major: int
    minor: int
    patch: int

    @classmethod
    def parse(cls, text: str) -> Version:
        match = VERSION_RE.search(text)
        if not match:
            raise ValueError(f"no X.Y.Z version in {text!r}")
        return cls(*(int(part) for part in match.groups()))

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"


@dataclass
class TestRun:
    """Failures of one test pass, keyed as ``<command> :: <test>``."""

    failures: set[str] = field(default_factory=set)
    failed_commands: list[str] = field(default_factory=list)


@dataclass
class Report:
    current: Version
    target: Version
    steps: list[tuple[str, bool, str]] = field(default_factory=list)
    regressions: list[str] = field(default_factory=list)
    fixed: list[str] = field(default_factory=list)
    snapshot_files: list[str] = field(default_factory=list)

    def record(self, step: str, ok: bool, detail: str = "") -> None:
        self.steps.append((step, ok, detail))
        print(
            f"\n==> {step}: {'ok' if ok else 'FAILED'}{f' ({detail})' if detail else ''}"
        )

    @property
    def ok(self) -> bool:
        return all(ok for _, ok, _ in self.steps)


# ── process helpers ──────────────────────────────────────────────────────────


def run_logged(
    command: list[str],
    log: Path | None,
    *,
    dry_run: bool = False,
    env: dict[str, str] | None = None,
) -> tuple[int, str]:
    """Run ``command``, streaming output to the terminal and ``log``."""
    printable = " ".join(command)
    if dry_run:
        print(f"[dry-run] {printable}")
        return 0, ""
    print(f"+ {printable}", flush=True)
    output: list[str] = []
    with subprocess.Popen(
        command,
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env={**os.environ, **env} if env else None,
    ) as process:
        assert process.stdout is not None
        for line in process.stdout:
            sys.stdout.write(line)
            output.append(line)
    text = "".join(output)
    if log is not None:
        log.parent.mkdir(parents=True, exist_ok=True)
        with log.open("a") as handle:
            handle.write(f"+ {printable}\n{text}\n[exit {process.returncode}]\n\n")
    return process.returncode, text


def capture(*command: str) -> str:
    return subprocess.run(
        command, cwd=ROOT, check=True, text=True, capture_output=True
    ).stdout


def rust_command(shell: str) -> list[str]:
    """Wrap a shell snippet so Rust builds run where the repo expects them.

    macOS routes through the full Linux container (`just linux-full`), whose
    recipe interpolates the snippet inside single quotes.
    """
    if sys.platform == "darwin":
        if "'" in shell:
            raise ValueError(
                f"cannot pass a single quote through just linux-full: {shell}"
            )
        return ["just", "linux-full", shell]
    if sys.platform == "win32":
        raise RuntimeError(
            "run this script inside WSL; Rust builds are not native here"
        )
    return ["bash", "-c", shell]


# ── stage 1: versions ────────────────────────────────────────────────────────


def pinned_version(path: Path = TOOLCHAIN_FILE) -> Version:
    return Version.parse(str(tomlkit.parse(path.read_text())["toolchain"]["channel"]))


def parse_rustup_check(output: str) -> Version | None:
    """Return the newest stable version ``rustup check`` reports, if any.

    Lines look like ``stable-<host> - up to date: 1.99.0 (...)`` or
    ``stable-<host> - update available: 1.98.1 (...) -> 1.99.0 (...)``; the
    last version on the line is the newest either way.
    """
    for line in output.splitlines():
        if line.startswith("stable-"):
            versions = VERSION_RE.findall(line)
            if versions:
                return Version(*(int(part) for part in versions[-1]))
    return None


def fetch(url: str) -> str:
    request = urllib.request.Request(
        url, headers={"User-Agent": "consortium-update-rust"}
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read().decode()


def latest_stable() -> Version:
    # `rustup check` exits 100 whenever any toolchain has an update, so read its
    # output regardless of the status.
    try:
        result = subprocess.run(
            ["rustup", "check"], cwd=ROOT, check=False, text=True, capture_output=True
        )
        found = parse_rustup_check(result.stdout)
    except OSError:
        found = None
    if found is not None:
        print(f"rustup check: latest stable is {found}")
        return found
    manifest = tomlkit.parse(fetch(CHANNEL_MANIFEST_URL))
    found = Version.parse(str(manifest["pkg"]["rust"]["version"]))
    print(f"channel manifest: latest stable is {found}")
    return found


def msrv() -> str:
    package = tomlkit.parse(WORKSPACE_MANIFEST.read_text())["workspace"]["package"]
    return str(package["rust-version"])


# ── stage 2: bump ────────────────────────────────────────────────────────────


def rewrite_channel(text: str, version: Version) -> str:
    document = tomlkit.parse(text)
    document["toolchain"]["channel"] = str(version)
    return tomlkit.dumps(document)


def bump_toolchain(version: Version, report: Report, log: Path, dry_run: bool) -> None:
    if dry_run:
        print(f"[dry-run] set {TOOLCHAIN_FILE.name} channel = {version}")
    else:
        TOOLCHAIN_FILE.write_text(rewrite_channel(TOOLCHAIN_FILE.read_text(), version))
    # rustup no longer auto-installs from the toolchain file; the install reads
    # its components and targets as well as the channel.
    code, _ = run_logged(["rustup", "toolchain", "install"], log, dry_run=dry_run)
    report.record("install host toolchain", code == 0)
    if sys.platform == "darwin":
        # Both images copy rust-toolchain.toml at build time, so the bump only
        # reaches the container once they are rebuilt.
        for recipe in ("linux-image", "linux-image-full"):
            code, _ = run_logged(["just", recipe], log, dry_run=dry_run)
            report.record(f"rebuild {recipe}", code == 0)


# ── stages 2-4: tests ────────────────────────────────────────────────────────


def expand_recipes(recipes: tuple[str, ...]) -> list[str]:
    """Expand just recipes into their cargo commands via ``just --dry-run``."""
    commands: list[str] = []
    for recipe in recipes:
        listing = subprocess.run(
            ["just", "--dry-run", recipe],
            cwd=ROOT,
            check=True,
            text=True,
            capture_output=True,
        )
        for line in (listing.stderr + listing.stdout).splitlines():
            line = line.strip()
            if line and not line.startswith("#"):
                commands.append(line)
    return commands


def test_script(commands: list[str], env: dict[str, str]) -> str:
    """One shell snippet that runs every command and marks each exit status."""
    prefix = " ".join(f"{key}={value}" for key, value in env.items())
    parts: list[str] = []
    for command in commands:
        if any(char in command for char in "'\"$`"):
            raise ValueError(
                f"test command needs quoting this runner avoids: {command}"
            )
        if command.startswith("cargo test"):
            command = command.replace("cargo test", "cargo test --no-fail-fast", 1)
        parts.append(f'echo "{CMD_MARK}{command}"')
        parts.append(f"env {prefix} {command}")
        parts.append(f'echo "{STATUS_MARK}$?"')
    return "; ".join(parts)


def parse_test_output(output: str) -> TestRun:
    run = TestRun()
    command: str | None = None
    tests: set[str] = set()
    for line in output.splitlines():
        if line.startswith(CMD_MARK):
            command, tests = line.removeprefix(CMD_MARK).strip(), set()
        elif command is None:
            continue
        elif line.startswith(STATUS_MARK):
            if line.removeprefix(STATUS_MARK).strip() != "0":
                run.failed_commands.append(command)
                run.failures |= {
                    f"{command} :: {test}" for test in tests or {"<build>"}
                }
            command = None
        elif match := FAILED_TEST_RE.match(line):
            tests.add(match["name"])
    return run


def run_tests(
    commands: list[str], env: dict[str, str], log: Path, dry_run: bool
) -> TestRun:
    _, output = run_logged(
        rust_command(test_script(commands, env)), log, dry_run=dry_run
    )
    return parse_test_output(output)


def changed_snapshots() -> list[str]:
    paths = []
    for line in capture("git", "status", "--porcelain").splitlines():
        path = line[3:].split(" -> ")[-1]
        if path.endswith(SNAPSHOT_SUFFIXES):
            paths.append(path)
    return sorted(paths)


# ── stage 5: fixes and lints ─────────────────────────────────────────────────


def fix_and_lint(report: Report, log: Path, dry_run: bool) -> None:
    # Mirrors release.py's apply_release_fixes/run_release_linters, but keeps
    # going after a failure so the report lists every lint that still fails.
    fixes = [
        rust_command("just cargo-fix"),
        ["just", "oxfix"],
        ["just", "ruff-fix"],
        ["just", "fmt"],
    ]
    for command in fixes:
        code, _ = run_logged(command, log, dry_run=dry_run)
        report.record(f"fix: {command[-1]}", code == 0)
    for label, command in [
        ("check", rust_command("just check")),
        ("lint: cargo-lint", rust_command("just cargo-lint")),
        *((f"lint: {recipe}", ["just", recipe]) for recipe in HOST_LINT_RECIPES),
    ]:
        code, _ = run_logged(command, log, dry_run=dry_run)
        report.record(label, code == 0)


# ── stage 6: changelog and prompt ────────────────────────────────────────────


def rust_release_notes(text: str, after: Version, upto: Version) -> str:
    """Sections of rustc's RELEASES.md for versions in ``(after, upto]``."""
    headers = list(RELEASE_HEADER_RE.finditer(text))
    sections = []
    for index, header in enumerate(headers):
        version = Version.parse(header[1])
        if after < version <= upto:
            end = headers[index + 1].start() if index + 1 < len(headers) else len(text)
            sections.append(text[header.start() : end].strip())
    return "\n\n".join(sections)


def clippy_release_notes(text: str, after: Version, upto: Version) -> str:
    """Sections of Clippy's CHANGELOG.md for minor releases in ``(after, upto]``."""
    headers = list(CLIPPY_HEADER_RE.finditer(text))
    sections = []
    for index, header in enumerate(headers):
        minor = (int(header[1]), int(header[2]))
        if (after.major, after.minor) < minor <= (upto.major, upto.minor):
            end = headers[index + 1].start() if index + 1 < len(headers) else len(text)
            sections.append(text[header.start() : end].strip())
    return "\n\n".join(sections)


def build_prompt(
    report: Report, rust_notes: str, clippy_notes: str, rust_msrv: str
) -> str:
    status = (
        "\n".join(
            f"- {'ok' if ok else 'FAILED'}: {step}{f' ({detail})' if detail else ''}"
            for step, ok, detail in report.steps
        )
        or "- (no stages ran)"
    )
    regressions = "\n".join(f"- `{item}`" for item in report.regressions) or "- none"
    snapshots = "\n".join(f"- `{path}`" for path in report.snapshot_files) or "- none"
    return f"""\
# Rust {report.current} → {report.target}: style review

You are reviewing the Consortium Rust workspace (this repository) after its
pinned toolchain moved from Rust {report.current} to {report.target}. Read
`AGENTS.md` first: it describes the crates, the `no_std` firmware constraints,
and the repository conventions your suggestions must respect.

## Goal

Find places where the release notes below enable clearer, safer, or more
idiomatic code in this repository, and report them. **Do not edit files**;
produce the report so a maintainer can choose what to adopt.

## Constraints

- The workspace MSRV is `rust-version = "{rust_msrv}"` (`[workspace.package]`
  in `Cargo.toml`), enforced by Clippy's `incompatible_msrv`. A suggestion that
  uses an API or language feature stabilized after {rust_msrv} must say which
  version it needs; the policy is to raise the MSRV rather than `#[allow]` the
  lint, so group such suggestions by the MSRV they would require and say
  whether the gain justifies the raise.
- Firmware-side crates (`consortium-ipc`, doorbells, HALs, `consortium-runtime-mcu`,
  `consortium-dbg`, ...) are `no_std`; only suggest `core`/`alloc` APIs there.
- Keep unsafe blocks narrow and documented; do not suggest widening them.
- Ignore generated code: PAC crates, `consortium-runtime-sys`, `cogcore-sys`,
  `src/peripherals/<chip>.rs`, and `consortium.gen.rs` files.
- Edition is 2024 everywhere.

## What to look for

1. Newly stabilized std/core/alloc APIs that replace hand-written helpers,
   manual loops, `unsafe` code, or third-party dependencies in this repo.
2. Newly stabilized language features (patterns, `const` capabilities,
   attributes, lints) that simplify existing code.
3. New Clippy lints, including `pedantic`/`restriction` ones that are not on by
   default, that would catch real problems here. Run them
   (`cargo clippy -p <crate> -- -W clippy::<lint>`) and report hit counts;
   recommend which to enable in `[workspace.lints]`.
4. Deprecations, future-incompatibility warnings, or changed lint defaults that
   affect this repository.

## Output format

For each finding: `file:line`, the current code, the proposed code, the minimum
Rust version it needs, and a one-line justification. Order findings by value,
group by category, and finish with a short recommendation on the MSRV.

## Bump results

{status}

Regressions remaining after the snapshot update (also worth a look):

{regressions}

Snapshot files rewritten by the bump:

{snapshots}

## rustc release notes

{rust_notes or "(no rustc release notes found for this range)"}

## Clippy changelog

{clippy_notes or "(no Clippy changelog sections for this range; patch releases rarely add lints)"}
"""


def write_prompt(report: Report, out: Path, dry_run: bool) -> Path | None:
    try:
        rust_notes = rust_release_notes(
            fetch(RUST_RELEASES_URL), report.current, report.target
        )
        clippy_notes = clippy_release_notes(
            fetch(CLIPPY_CHANGELOG_URL), report.current, report.target
        )
    except OSError as error:
        report.record("fetch changelogs", False, str(error))
        return None
    report.record("fetch changelogs", True)
    path = out / "prompt.md"
    if dry_run:
        print(f"[dry-run] write {path}")
        return None
    path.write_text(build_prompt(report, rust_notes, clippy_notes, msrv()))
    return path


# ── driver ───────────────────────────────────────────────────────────────────


def write_summary(report: Report, out: Path) -> Path:
    lines = [f"# Rust {report.current} → {report.target}", "", "## Stages", ""]
    lines += [
        f"- {'ok' if ok else 'FAILED'}: {step}{f' ({detail})' if detail else ''}"
        for step, ok, detail in report.steps
    ]
    for title, items in (
        ("Regressions", report.regressions),
        ("Fixed by the bump", report.fixed),
        ("Snapshot files changed", report.snapshot_files),
    ):
        lines += ["", f"## {title}", ""] + (
            [f"- `{item}`" for item in items] or ["- none"]
        )
    path = out / "summary.md"
    path.write_text("\n".join(lines) + "\n")
    return path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--to", type=Version.parse, help="target version (default: latest stable)"
    )
    parser.add_argument(
        "--from",
        dest="since",
        type=Version.parse,
        help="older version for the changelog range (default: the pinned one)",
    )
    parser.add_argument(
        "--force", action="store_true", help="run even if the pin is already current"
    )
    parser.add_argument(
        "--no-baseline",
        action="store_true",
        help="skip the pre-bump test run; every failure is then reported as a regression",
    )
    parser.add_argument("--skip-fix", action="store_true", help="skip fixers and lints")
    parser.add_argument(
        "--prompt-only",
        action="store_true",
        help="only fetch the changelog and write the AI prompt",
    )
    parser.add_argument(
        "--dry-run", action="store_true", help="print commands without running them"
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    current = args.since or pinned_version()
    target = args.to or latest_stable()
    report = Report(current, target)
    out = LOG_ROOT / str(target)
    out.mkdir(parents=True, exist_ok=True)

    if args.prompt_only:
        prompt = write_prompt(report, out, args.dry_run)
        if prompt:
            print(f"\nPrompt: {prompt.relative_to(ROOT)}")
        return 0 if report.ok else 1

    if target <= current and not args.force:
        print(
            f"rust-toolchain.toml already pins {current} (latest: {target}); nothing to do."
        )
        print(
            "Pass --force to re-run the checks, or --prompt-only --from <old> for a prompt."
        )
        return 0

    commands = expand_recipes(TEST_RECIPES)
    baseline = TestRun()
    if not args.no_baseline:
        baseline = run_tests(
            commands, STRICT_ENV, out / "tests-baseline.log", args.dry_run
        )
        report.record(
            f"baseline tests on {current}", True, f"{len(baseline.failures)} failing"
        )

    bump_toolchain(target, report, out / "bump.log", args.dry_run)

    bumped = run_tests(commands, STRICT_ENV, out / "tests-bumped.log", args.dry_run)
    report.record(
        f"tests on {target}", not bumped.failures, f"{len(bumped.failures)} failing"
    )

    if bumped.failed_commands:
        # Accept snapshot churn only for the commands that failed, then confirm
        # strictly: whatever still fails is a real regression, not a snapshot.
        run_tests(
            bumped.failed_commands,
            OVERWRITE_ENV,
            out / "tests-snapshots.log",
            args.dry_run,
        )
        bumped = run_tests(
            bumped.failed_commands, STRICT_ENV, out / "tests-verify.log", args.dry_run
        )
    report.snapshot_files = [] if args.dry_run else changed_snapshots()
    report.regressions = sorted(bumped.failures - baseline.failures)
    report.fixed = sorted(baseline.failures - bumped.failures)
    report.record(
        "snapshot update",
        not report.regressions,
        f"{len(report.snapshot_files)} files rewritten, {len(report.regressions)} regressions",
    )

    if not args.skip_fix:
        fix_and_lint(report, out / "fix-lint.log", args.dry_run)

    prompt = write_prompt(report, out, args.dry_run)
    summary = write_summary(report, out)
    print(f"\nSummary: {summary.relative_to(ROOT)}")
    if prompt:
        print(
            f"Prompt:  {prompt.relative_to(ROOT)}  (e.g. claude -p < {prompt.relative_to(ROOT)})"
        )
    print("Logs:    " + str(out.relative_to(ROOT)))
    print("Nothing was committed; review the changes with `git diff`.")
    return 0 if report.ok else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
