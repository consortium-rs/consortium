#!/usr/bin/env python3
"""Prepare and publish a Consortium release from the host checkout."""

from __future__ import annotations

import argparse
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from collections.abc import MutableMapping
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import tomlkit
import yaml

ROOT = Path(__file__).resolve().parents[1]
CONVENTIONAL_RE = re.compile(
    r"^(?P<kind>[a-zA-Z]+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?: "
    r"(?P<description>.+)$"
)
NO_GITHUB_LOGIN = {"", "null", "None"}
DEPENDENCY_TABLES = {"dependencies", "build-dependencies", "dev-dependencies"}
RELEASE_PACKAGES = {
    ("windows-latest", "x86_64-pc-windows-msvc", "zip"),
    ("windows-11-arm", "aarch64-pc-windows-msvc", "zip"),
    ("macos-14", "aarch64-apple-darwin", "tgz"),
    ("ubuntu-latest", "x86_64-unknown-linux-gnu", "tgz"),
    ("ubuntu-24.04-arm", "aarch64-unknown-linux-gnu", "tgz"),
}
BINSTALL_PKG_URL = (
    "{ repo }/releases/download/v{ version }/"
    "{ name }-{ target }-v{ version }{ archive-suffix }"
)
CATEGORY_ORDER = (
    "Features",
    "Fixes",
    "Performance",
    "Documentation",
    "Refactoring",
    "Tests",
    "Build",
    "CI",
    "Maintenance",
    "Reverts",
    "Other contributions",
)
CATEGORIES = {
    "feat": "Features",
    "fix": "Fixes",
    "perf": "Performance",
    "docs": "Documentation",
    "refactor": "Refactoring",
    "test": "Tests",
    "build": "Build",
    "ci": "CI",
    "chore": "Maintenance",
    "revert": "Reverts",
}


@dataclass(frozen=True)
class Version:
    major: int
    minor: int
    patch: int

    @classmethod
    def parse(cls, value: str) -> Version:
        match = re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", value)
        if not match:
            raise ValueError(f"release.py requires a stable SemVer, got {value!r}")
        return cls(*(int(part) for part in match.groups()))

    def bump(self, component: str) -> Version:
        if component == "major":
            return Version(self.major + 1, 0, 0)
        if component == "minor":
            return Version(self.major, self.minor + 1, 0)
        if component == "patch":
            return Version(self.major, self.minor, self.patch + 1)
        raise ValueError(f"unknown version component {component!r}")

    def default_component(self) -> str:
        return "patch" if self.major == 0 else "minor"

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"


@dataclass(frozen=True)
class Commit:
    sha: str
    author_name: str
    author_email: str
    subject: str


def run(
    *command: str,
    cwd: Path = ROOT,
    capture: bool = False,
    dry_run: bool = False,
    env: dict[str, str] | None = None,
) -> str:
    printable = " ".join(command)
    if dry_run:
        print(f"[dry-run] {printable}")
        return ""
    print(f"+ {printable}")
    result = subprocess.run(
        command,
        cwd=cwd,
        check=True,
        text=True,
        capture_output=capture,
        env=env,
    )
    return result.stdout.strip() if capture else ""


def require_clean_tree(root: Path, allow_dirty: bool = False) -> None:
    status = run("git", "status", "--porcelain", cwd=root, capture=True)
    if status and not allow_dirty:
        raise RuntimeError(
            "the worktree is not clean; commit or stash existing changes, "
            "or pass --allow-dirty"
        )
    if status:
        print(
            "warning: continuing with a dirty worktree; a real release will "
            "stage all superproject changes"
        )


def workspace_version(root: Path) -> Version:
    document = tomlkit.parse((root / "Cargo.toml").read_text())
    try:
        value = document["workspace"]["package"]["version"]
    except KeyError as error:
        raise RuntimeError("Cargo.toml has no [workspace.package] version") from error
    if not isinstance(value, str):
        raise TypeError("Cargo.toml workspace version is not a string")
    return Version.parse(value)


def cargo_manifests(root: Path) -> list[Path]:
    manifests = [root / "Cargo.toml"]
    for base in (root / "crates", root / "examples"):
        manifests.extend(base.rglob("Cargo.toml"))
    return sorted(set(manifests))


def package_name(manifest: Path) -> str | None:
    document = tomlkit.parse(manifest.read_text())
    package = document.get("package")
    if not isinstance(package, MutableMapping):
        return None
    name = package.get("name")
    return name if isinstance(name, str) else None


def is_local_package_path(
    manifest: Path, raw_path: str, package_dirs: set[Path]
) -> bool:
    target = (manifest.parent / raw_path).resolve()
    return target in package_dirs


def dependency_tables(
    container: MutableMapping[str, Any],
) -> list[tuple[str, MutableMapping[str, Any]]]:
    tables: list[tuple[str, MutableMapping[str, Any]]] = []
    for key, value in container.items():
        if not isinstance(value, MutableMapping):
            continue
        if key in DEPENDENCY_TABLES:
            tables.append((key, value))
        else:
            tables.extend(dependency_tables(value))
    return tables


def update_cargo_manifest(
    manifest: Path,
    version: str,
    package_dirs: set[Path],
    update_package: bool,
) -> bool:
    original = manifest.read_text()
    document = tomlkit.parse(original)
    package = document.get("package")
    if update_package and isinstance(package, MutableMapping):
        package_version = package.get("version")
        if isinstance(package_version, str):
            package["version"] = version

    for table_name, dependencies in dependency_tables(document):
        for dependency in dependencies.values():
            if not isinstance(dependency, MutableMapping):
                continue
            raw_path = dependency.get("path")
            if not isinstance(raw_path, str) or not is_local_package_path(
                manifest, raw_path, package_dirs
            ):
                continue
            if "version" in dependency or table_name != "dev-dependencies":
                dependency["version"] = version

    updated = tomlkit.dumps(document)
    if updated == original:
        return False
    manifest.write_text(updated)
    return True


def update_cargo_versions(root: Path, version: str) -> list[Path]:
    manifests = cargo_manifests(root)
    package_dirs = {
        manifest.parent.resolve()
        for manifest in manifests
        if package_name(manifest) is not None
    }
    root_manifest = root / "Cargo.toml"
    root_original = root_manifest.read_text()
    root_document = tomlkit.parse(root_original)
    try:
        root_document["workspace"]["package"]["version"] = version
    except KeyError as error:
        raise RuntimeError("Cargo.toml has no [workspace.package] version") from error
    root_updated = tomlkit.dumps(root_document)
    changed: list[Path] = []
    if root_updated != root_original:
        root_manifest.write_text(root_updated)
        changed.append(root_manifest)
    for manifest in manifests:
        if manifest == root / "Cargo.toml":
            # Dependency requirements in the virtual root are still handled.
            package_update = False
        else:
            package_update = True
        if update_cargo_manifest(
            manifest, version, package_dirs, update_package=package_update
        ):
            changed.append(manifest)
    return sorted(set(changed))


def update_pyproject(path: Path, version: str) -> bool:
    original = path.read_text()
    document = tomlkit.parse(original)
    project = document.get("project")
    if not isinstance(project, MutableMapping) or not isinstance(
        project.get("version"), str
    ):
        raise TypeError(f"{path} has no string [project] version")
    project["version"] = version
    updated = tomlkit.dumps(document)
    if updated == original:
        return False
    path.write_text(updated)
    return True


def first_party_package_json_files(root: Path) -> list[Path]:
    paths = [root / "package.json"]
    paths.extend((root / "examples").rglob("package.json"))
    return sorted(set(paths))


def update_package_json(path: Path, version: str) -> bool:
    data = json.loads(path.read_text())
    if data.get("version") == version:
        return False
    data["version"] = version
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    return True


def validate_release_workflow(path: Path) -> None:
    workflow = yaml.load(path.read_text(), Loader=yaml.BaseLoader)
    if not isinstance(workflow, dict):
        raise TypeError(f"{path} is not a YAML mapping")
    try:
        tag_input = workflow["on"]["workflow_dispatch"]["inputs"]["tag"]
        permissions = workflow["permissions"]
        build_job = workflow["jobs"]["build"]
        entries = build_job["strategy"]["matrix"]["include"]
        steps = build_job["steps"]
    except (KeyError, TypeError) as error:
        raise RuntimeError(
            f"{path} is missing required release workflow fields"
        ) from error
    if not isinstance(tag_input, dict) or tag_input.get("required") != "true":
        raise RuntimeError(f"{path} must require the workflow_dispatch tag input")
    if not isinstance(permissions, dict) or permissions.get("contents") != "write":
        raise RuntimeError(f"{path} must grant contents: write")
    if not isinstance(entries, list):
        raise TypeError(f"{path} release matrix is not a list")
    packages = {
        (entry.get("runner"), entry.get("target"), entry.get("package"))
        for entry in entries
        if isinstance(entry, dict)
    }
    if packages != RELEASE_PACKAGES:
        raise RuntimeError(
            f"{path} does not define the required five-platform release matrix"
        )
    run_scripts = "\n".join(
        step.get("run", "") for step in steps if isinstance(step, dict)
    )
    required_naming = {
        "csti-${{ matrix.target }}-v${version}.tgz",
        "csti-${{ matrix.target }}-v${version}.zip",
    }
    if not all(name in run_scripts for name in required_naming):
        raise RuntimeError(f"{path} does not use cargo-binstall-compatible names")


def validate_binstall_manifest(path: Path) -> None:
    document = tomlkit.parse(path.read_text())
    try:
        package = document["package"]
        config = package["metadata"]["binstall"]
        windows = config["overrides"]["cfg(windows)"]
    except KeyError as error:
        raise RuntimeError(f"{path} has incomplete cargo-binstall metadata") from error
    expected = {
        "name": "csti",
        "pkg-url": BINSTALL_PKG_URL,
        "bin-dir": "{ bin }{ binary-ext }",
        "pkg-fmt": "tgz",
        "windows-pkg-fmt": "zip",
    }
    actual = {
        "name": package.get("name"),
        "pkg-url": config.get("pkg-url"),
        "bin-dir": config.get("bin-dir"),
        "pkg-fmt": config.get("pkg-fmt"),
        "windows-pkg-fmt": windows.get("pkg-fmt"),
    }
    if actual != expected:
        raise RuntimeError(
            f"{path} cargo-binstall metadata does not match release assets"
        )


def update_all_manifests(root: Path, version: str) -> list[Path]:
    changed = update_cargo_versions(root, version)
    for path in (root / "pyproject.toml", root / "scripts" / "pyproject.toml"):
        if path.exists() and update_pyproject(path, version):
            changed.append(path)
    for path in first_party_package_json_files(root):
        if update_package_json(path, version):
            changed.append(path)
    return sorted(set(changed))


def regenerate_lockfiles(root: Path, dry_run: bool) -> None:
    run(
        "cargo",
        "metadata",
        "--format-version",
        "1",
        cwd=root,
        capture=True,
        dry_run=dry_run,
    )
    run("uv", "lock", cwd=root, dry_run=dry_run)
    run("uv", "lock", cwd=root / "scripts", dry_run=dry_run)
    run("vp", "install", cwd=root, dry_run=dry_run)


def run_rust_recipe(recipe: str, *, dry_run: bool = False) -> None:
    command = ("just", recipe)
    if sys.platform == "darwin":
        run(
            "just",
            "linux-full",
            shlex.join(command),
            cwd=ROOT,
            dry_run=dry_run,
        )
        return
    run(*command, cwd=ROOT, dry_run=dry_run)


def format_release_files(*, dry_run: bool = False) -> None:
    run("just", "rustfmt", cwd=ROOT, dry_run=dry_run)
    run("vp", "fmt", cwd=ROOT, dry_run=dry_run)
    run("uvx", "ruff", "format", cwd=ROOT, dry_run=dry_run)
    run("taplo", "format", cwd=ROOT, dry_run=dry_run)


def apply_release_fixes(*, dry_run: bool = False) -> None:
    run_rust_recipe("cargo-fix", dry_run=dry_run)
    run("just", "oxfix", cwd=ROOT, dry_run=dry_run)
    run("just", "ruff-fix", cwd=ROOT, dry_run=dry_run)
    format_release_files(dry_run=dry_run)


def run_release_linters(*, dry_run: bool = False) -> None:
    run_rust_recipe("cargo-lint", dry_run=dry_run)
    for recipe in (
        "fmt-check",
        "taplo-check",
        "typos",
        "ls-lint",
        "oxlint",
        "ruff-lint",
    ):
        run("just", recipe, cwd=ROOT, dry_run=dry_run)


def prepare_release(version: str) -> None:
    apply_release_fixes()
    changed = update_all_manifests(ROOT, version)
    print(f"Updated {len(changed)} manifests to {version}.")
    regenerate_lockfiles(ROOT, dry_run=False)
    # Version and lockfile generation can produce formatting changes. Format
    # their final state before running format checks as part of the lint bundle.
    format_release_files()
    run_release_linters()


def last_release_tag(root: Path) -> str | None:
    result = subprocess.run(
        ["git", "describe", "--tags", "--abbrev=0", "--match", "v[0-9]*"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() or None


def commits_since(root: Path, tag: str | None) -> list[Commit]:
    revision = f"{tag}..HEAD" if tag else "HEAD"
    record_separator = "\x1e"
    field_separator = "\x1f"
    output = run(
        "git",
        "log",
        revision,
        f"--format={record_separator}%H{field_separator}%aN{field_separator}%aE"
        f"{field_separator}%s",
        cwd=root,
        capture=True,
    )
    commits = []
    for record in output.split(record_separator):
        if not record:
            continue
        fields = record.strip().split(field_separator, 3)
        if len(fields) == 4:
            commits.append(Commit(*fields))
    return commits


def repository_slug(root: Path) -> str:
    url = run("git", "remote", "get-url", "origin", cwd=root, capture=True)
    match = re.search(r"github\.com[/:](?P<slug>[^/]+/[^/]+?)(?:\.git)?$", url)
    if not match:
        raise RuntimeError(f"origin is not a GitHub repository: {url}")
    return match.group("slug")


def current_branch(root: Path) -> str:
    branch = run("git", "branch", "--show-current", cwd=root, capture=True)
    if not branch:
        raise RuntimeError("release from a branch, not a detached HEAD")
    return branch


def require_unused_release(root: Path, tag: str) -> None:
    local = subprocess.run(
        ["git", "rev-parse", "--verify", "--quiet", f"refs/tags/{tag}"],
        cwd=root,
        check=False,
    )
    if local.returncode == 0:
        raise RuntimeError(f"local tag {tag} already exists")
    remote = subprocess.run(
        ["git", "ls-remote", "--exit-code", "--tags", "origin", f"refs/tags/{tag}"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    if remote.returncode == 0:
        raise RuntimeError(f"remote tag {tag} already exists")
    if remote.returncode not in {0, 2}:
        raise RuntimeError("could not check remote tags")


def fallback_login(commit: Commit) -> str:
    match = re.match(
        r"(?:\d+\+)?([^@]+)@users\.noreply\.github\.com", commit.author_email
    )
    if match:
        return match.group(1)
    return re.sub(r"[^A-Za-z0-9-]", "-", commit.author_name).strip("-") or "unknown"


def github_login(
    root: Path, repository: str, commit: Commit, cache: dict[str, str]
) -> str:
    if commit.author_email in cache:
        return cache[commit.author_email]
    if shutil.which("gh") is None:
        login = fallback_login(commit)
        cache[commit.author_email] = login
        return login
    result = subprocess.run(
        [
            "gh",
            "api",
            f"repos/{repository}/commits/{commit.sha}",
            "--jq",
            ".author.login // empty",
        ],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    login = result.stdout.strip()
    if result.returncode != 0 or login in NO_GITHUB_LOGIN:
        login = fallback_login(commit)
    cache[commit.author_email] = login
    return login


def release_notes(root: Path, repository: str, version: str) -> str:
    previous = last_release_tag(root)
    categorized: dict[str, list[str]] = {heading: [] for heading in CATEGORY_ORDER}
    login_cache: dict[str, str] = {}
    for commit in commits_since(root, previous):
        match = CONVENTIONAL_RE.match(commit.subject)
        if match:
            kind = match.group("kind").lower()
            heading = CATEGORIES.get(kind, "Other contributions")
            scope = match.group("scope") or kind
            description = match.group("description")
        else:
            heading = "Other contributions"
            scope = "other"
            description = commit.subject
        login = github_login(root, repository, commit, login_cache)
        categorized[heading].append(
            f"- [{scope}] [{commit.sha[:7]}] by @{login}: {description}"
        )

    sections = [
        f"# {heading}\n\n" + "\n".join(categorized[heading])
        for heading in CATEGORY_ORDER
        if categorized[heading]
    ]
    if not sections:
        sections = ["# Other contributions\n\n- No user-facing changes."]
    comparison = (
        f"\n\n**Full history:** https://github.com/{repository}/compare/{previous}...v{version}"
        if previous
        else ""
    )
    return "\n\n".join(sections) + comparison + "\n"


def confirm(
    current: Version, target: Version, component: str, assume_yes: bool
) -> None:
    print(f"Release bump: {current} -> {target} ({component})")
    if assume_yes:
        return
    answer = input("Continue with this release? [y/N] ").strip().lower()
    if answer not in {"y", "yes"}:
        raise RuntimeError("release cancelled")


def write_notes_file(notes: str) -> Path:
    descriptor, name = tempfile.mkstemp(prefix="consortium-release-", suffix=".md")
    with os.fdopen(descriptor, "w") as file:
        file.write(notes)
    return Path(name)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("component", nargs="?", choices=("major", "minor", "patch"))
    parser.add_argument(
        "--yes", action="store_true", help="accept the proposed version"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="show the version and release notes without changing anything",
    )
    parser.add_argument(
        "--allow-dirty",
        action="store_true",
        help="permit existing changes (a real release stages all of them)",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    require_clean_tree(ROOT, allow_dirty=args.allow_dirty)
    validate_release_workflow(ROOT / ".github" / "workflows" / "release-build.yml")
    validate_binstall_manifest(ROOT / "crates" / "consortium-cli" / "Cargo.toml")
    current = workspace_version(ROOT)
    component = args.component or current.default_component()
    target = current.bump(component)
    confirm(current, target, component, args.yes or args.dry_run)
    repository = repository_slug(ROOT)
    branch = current_branch(ROOT)
    tag = f"v{target}"

    if args.dry_run:
        print("\n" + release_notes(ROOT, repository, str(target)))
        return 0

    run("gh", "auth", "status", cwd=ROOT)
    require_unused_release(ROOT, tag)
    prepare_release(str(target))

    # Generate notes before the release commit so it is not listed as a user
    # contribution to its own release.
    notes = release_notes(ROOT, repository, str(target))
    notes_path = write_notes_file(notes)
    try:
        run("git", "add", "--all", cwd=ROOT)
        run("git", "commit", "-m", f"release: {tag}", cwd=ROOT)
        run("git", "tag", "-a", tag, "-m", f"Release {target}", cwd=ROOT)
        run("git", "push", "origin", f"HEAD:refs/heads/{branch}", cwd=ROOT)
        run("git", "push", "origin", tag, cwd=ROOT)
        run(
            "gh",
            "release",
            "create",
            tag,
            "--title",
            str(target),
            "--notes-file",
            str(notes_path),
            "--verify-tag",
            cwd=ROOT,
        )
        run(
            "gh",
            "workflow",
            "run",
            "release-build.yml",
            "--ref",
            tag,
            "-f",
            f"tag={tag}",
            cwd=ROOT,
        )
        run(sys.executable, str(ROOT / "scripts" / "publish.py"), cwd=ROOT)
    finally:
        notes_path.unlink(missing_ok=True)

    print(f"Release {target} created; binary builds were dispatched.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (
        RuntimeError,
        TypeError,
        ValueError,
        subprocess.CalledProcessError,
    ) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
