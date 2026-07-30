#!/usr/bin/env python3
"""Publish a Cargo workspace in dependency order without wasting rate limits.

Each publishable workspace package is classified from crates.io as:

* ``new``: the crate name does not exist;
* ``update``: the workspace version is newer than the newest registry version;
* ``skip``: that exact version is already present.

New crates and new versions use independent token buckets matching crates.io's
documented per-account limits.  A wait happens only immediately before a task
that needs a token (or while that task waits for a just-published dependency to
become visible); the script never sleeps merely because a publish completed.
"""

from __future__ import annotations

import argparse
import http.client
import json
import re
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from datetime import UTC, datetime
from email.utils import parsedate_to_datetime
from pathlib import Path
from typing import Any

USER_AGENT = "consortium-release/1 (https://github.com/consortium-rs/consortium)"
NEW_CRATE_BURST = 5
NEW_CRATE_INTERVAL = 10 * 60
UPDATE_BURST = 30
UPDATE_INTERVAL = 60
SAFETY_SECONDS = 2
SEMVER_RE = re.compile(
    r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)"
    r"(?:-(?P<pre>[0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?$"
)
RETRY_DATE_RE = re.compile(
    r"try again after ([A-Za-z]{3}, \d{2} [A-Za-z]{3} \d{4} [\d:]{8} GMT)",
    re.IGNORECASE,
)


@dataclass(frozen=True)
class SemVer:
    major: int
    minor: int
    patch: int
    pre: tuple[str, ...] = ()

    @classmethod
    def parse(cls, value: str) -> SemVer:
        match = SEMVER_RE.fullmatch(value)
        if not match:
            raise ValueError(f"unsupported Cargo version {value!r}")
        pre = tuple((match.group("pre") or "").split(".")) if match.group("pre") else ()
        return cls(
            int(match.group("major")),
            int(match.group("minor")),
            int(match.group("patch")),
            pre,
        )

    def _pre_key(self) -> tuple[tuple[int, int | str], ...]:
        # A release sorts after every prerelease. Numeric identifiers sort
        # before non-numeric identifiers, as required by SemVer.
        if not self.pre:
            return ((2, ""),)
        return tuple(
            (0, int(part)) if part.isdigit() else (1, part) for part in self.pre
        )

    def __lt__(self, other: SemVer) -> bool:
        core = (self.major, self.minor, self.patch)
        other_core = (other.major, other.minor, other.patch)
        return core < other_core or (
            core == other_core and self._pre_key() < other._pre_key()
        )


@dataclass
class Package:
    name: str
    version: str
    manifest: Path
    dependencies: set[str] = field(default_factory=set)
    mode: str = ""
    registry_version: str | None = None


@dataclass
class TokenBucket:
    label: str
    capacity: int
    interval: float
    tokens: float | None = None
    updated_at: float = field(default_factory=time.monotonic)

    def __post_init__(self) -> None:
        if self.tokens is None:
            self.tokens = float(self.capacity)

    def seconds_until_token(self, now: float | None = None) -> float:
        now = time.monotonic() if now is None else now
        elapsed = max(0.0, now - self.updated_at)
        available = min(
            float(self.capacity), float(self.tokens) + elapsed / self.interval
        )
        return 0.0 if available >= 1 else (1 - available) * self.interval

    def acquire(self, dry_run: bool = False) -> None:
        wait = self.seconds_until_token()
        if wait > 0:
            print(
                f"  {self.label} allowance replenishes in {wait:.1f}s; "
                "waiting before this publish"
            )
            if not dry_run:
                time.sleep(wait + SAFETY_SECONDS)
        now = time.monotonic()
        elapsed = max(0.0, now - self.updated_at)
        self.tokens = min(
            float(self.capacity), float(self.tokens) + elapsed / self.interval
        )
        self.updated_at = now
        if dry_run and self.tokens < 1:
            # Model the future wait without making a dry-run take real time.
            self.tokens = 1
        self.tokens -= 1


class CratesIo:
    def __init__(self, base_url: str = "https://crates.io/api/v1") -> None:
        self.base_url = base_url.rstrip("/")

    def _get(self, path: str, attempts: int = 5) -> dict[str, Any] | None:
        url = f"{self.base_url}/{path.lstrip('/')}"
        for attempt in range(attempts):
            request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
            try:
                with urllib.request.urlopen(request, timeout=30) as response:
                    return json.load(response)
            except urllib.error.HTTPError as error:
                if error.code == 404:
                    return None
                if error.code != 429 and error.code < 500:
                    raise
                wait = retry_delay(error.headers.get("Retry-After")) or 2**attempt
            except (
                urllib.error.URLError,
                http.client.HTTPException,
                json.JSONDecodeError,
            ) as error:
                if attempt + 1 == attempts:
                    raise RuntimeError(
                        f"unable to query {url} after {attempts} attempts: {error}"
                    ) from error
                wait = 2**attempt
            print(f"  crates.io lookup delayed; retrying in {wait:.1f}s")
            time.sleep(wait + SAFETY_SECONDS)
        raise RuntimeError(f"unable to query {url}")

    def versions(self, name: str) -> list[str] | None:
        payload = self._get(f"crates/{urllib.parse.quote(name, safe='')}")
        if payload is None:
            return None
        return [item["num"] for item in payload.get("versions", [])]

    def has_version(self, name: str, version: str) -> bool:
        path = (
            f"crates/{urllib.parse.quote(name, safe='')}/"
            f"{urllib.parse.quote(version, safe='')}"
        )
        return self._get(path, attempts=3) is not None


def retry_delay(value: str | None) -> float | None:
    if not value:
        return None
    if value.isdigit():
        return float(value)
    try:
        when = parsedate_to_datetime(value).astimezone(UTC)
    except TypeError:
        return None
    except ValueError:
        return None
    return max(0.0, (when - datetime.now(UTC)).total_seconds())


def cargo_metadata(workspace: Path) -> dict[str, Any]:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=workspace,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def discover_packages(workspace: Path) -> list[Package]:
    metadata = cargo_metadata(workspace)
    members = set(metadata["workspace_members"])
    raw_packages = [item for item in metadata["packages"] if item["id"] in members]
    publishable_names = {
        item["name"] for item in raw_packages if item.get("publish") != []
    }
    packages = []
    for item in raw_packages:
        if item.get("publish") == []:
            continue
        dependencies = {
            dep["name"]
            for dep in item["dependencies"]
            if dep["name"] in publishable_names and dep.get("kind") != "dev"
        }
        packages.append(
            Package(
                name=item["name"],
                version=item["version"],
                manifest=Path(item["manifest_path"]),
                dependencies=dependencies,
            )
        )
    return topological_order(packages)


def topological_order(packages: list[Package]) -> list[Package]:
    by_name = {package.name: package for package in packages}
    pending = {
        package.name: package.dependencies & by_name.keys() for package in packages
    }
    ordered: list[Package] = []
    while pending:
        ready = sorted(
            name for name, dependencies in pending.items() if not dependencies
        )
        if not ready:
            cycle = ", ".join(sorted(pending))
            raise RuntimeError(f"publish dependency cycle among: {cycle}")
        for name in ready:
            ordered.append(by_name[name])
            del pending[name]
        for dependencies in pending.values():
            dependencies.difference_update(ready)
    return ordered


def classify(package: Package, registry: CratesIo) -> None:
    versions = registry.versions(package.name)
    if versions is None:
        package.mode = "new"
        return
    parsed = [(SemVer.parse(version), version) for version in versions]
    if not parsed:
        package.mode = "new"
        return
    newest, newest_text = max(parsed, key=lambda item: item[0])
    package.registry_version = newest_text
    current = SemVer.parse(package.version)
    if current == newest:
        package.mode = "skip"
        return
    if newest < current:
        package.mode = "update"
        return
    raise RuntimeError(
        f"{package.name}: workspace version {package.version} is older than "
        f"crates.io version {newest_text}"
    )


def cargo_publish(workspace: Path, package: Package, dry_run: bool) -> tuple[bool, str]:
    command = ["cargo", "publish", "--locked", "--package", package.name]
    if dry_run:
        print(f"  [dry-run] {' '.join(command)}")
        return True, ""
    process = subprocess.run(
        command, cwd=workspace, capture_output=True, text=True, check=False
    )
    output = process.stdout + process.stderr
    print(output, end="" if output.endswith("\n") else "\n")
    return process.returncode == 0, output


def cargo_retry_delay(output: str) -> float | None:
    match = RETRY_DATE_RE.search(output)
    if match:
        return retry_delay(match.group(1))
    match = re.search(r"retry.after:?\s*(\d+)", output, re.IGNORECASE)
    return float(match.group(1)) if match else None


def choose_next_task(
    pending: dict[str, Package], buckets: dict[str, TokenBucket]
) -> Package:
    ready = [
        package
        for package in pending.values()
        if not (package.dependencies & pending.keys())
    ]
    if not ready:
        raise RuntimeError("no dependency-ready publish task")
    # Do useful work from another rate-limit scope instead of waiting behind an
    # independent task whose allowance is exhausted.
    return min(
        ready,
        key=lambda package: (
            buckets[package.mode].seconds_until_token() > 0,
            buckets[package.mode].seconds_until_token(),
            package.name,
        ),
    )


def wait_for_dependencies(
    package: Package,
    published: set[str],
    registry: CratesIo,
    timeout: float,
) -> None:
    waiting_for = sorted(package.dependencies & published)
    if not waiting_for:
        return
    deadline = time.monotonic() + timeout
    while True:
        missing = [
            name
            for name in waiting_for
            if not registry.has_version(name, package.version)
        ]
        if not missing:
            return
        if time.monotonic() >= deadline:
            raise RuntimeError(
                f"timed out waiting for registry visibility: {', '.join(missing)}"
            )
        print(
            f"  next publish needs {', '.join(missing)} in the registry; "
            "checking again in 5s"
        )
        time.sleep(5)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--workspace-root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
    )
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--start-at", metavar="CRATE")
    parser.add_argument(
        "--visibility-timeout",
        type=float,
        default=300,
        help="seconds to wait before a dependent publish (default: 300)",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    workspace = args.workspace_root.resolve()
    packages = discover_packages(workspace)
    if args.start_at:
        names = [package.name for package in packages]
        if args.start_at not in names:
            raise SystemExit(f"--start-at {args.start_at!r} is not publishable")
        packages = packages[names.index(args.start_at) :]

    registry = CratesIo()
    for package in packages:
        classify(package, registry)
        detail = (
            ""
            if package.registry_version is None
            else f" (crates.io: {package.registry_version})"
        )
        print(f"{package.mode:>6}  {package.name} {package.version}{detail}")

    tasks = [package for package in packages if package.mode != "skip"]
    if not tasks:
        print("Everything is already published.")
        return 0

    buckets = {
        "new": TokenBucket("new-crate", NEW_CRATE_BURST, float(NEW_CRATE_INTERVAL)),
        "update": TokenBucket(
            "existing-crate version", UPDATE_BURST, float(UPDATE_INTERVAL)
        ),
    }
    published: set[str] = set()
    pending = {package.name: package for package in tasks}
    while pending:
        package = choose_next_task(pending, buckets)
        print(f"== {package.mode}: {package.name} {package.version} ==")
        if not args.dry_run:
            wait_for_dependencies(package, published, registry, args.visibility_timeout)
        buckets[package.mode].acquire(args.dry_run)
        while True:
            ok, output = cargo_publish(workspace, package, args.dry_run)
            if ok:
                published.add(package.name)
                del pending[package.name]
                break
            wait = cargo_retry_delay(output)
            if wait is None:
                print(f"publish failed for {package.name}; stopping", file=sys.stderr)
                return 1
            print(f"  crates.io rate limit: retrying this publish in {wait:.1f}s")
            time.sleep(wait + SAFETY_SECONDS)

    print("All publish tasks completed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (
        RuntimeError,
        subprocess.CalledProcessError,
        urllib.error.URLError,
    ) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
