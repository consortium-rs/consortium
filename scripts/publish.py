#!/usr/bin/env python3
"""
Resume-safe, rate-limit-aware publisher for a cargo workspace's first-time
crates.io release.

What it does:
  - For each crate (in dependency order below), checks crates.io's API to
    see if that exact name/version is already published. If so, skips it.
  - Otherwise runs `cargo publish` in that crate's directory.
  - crates.io allows a burst of 5 new-crate publishes, then 1 every 10 min.
    After each successful publish of a *new* crate, this script sleeps
    10 minutes (plus a small buffer) before the next attempt.
  - If crates.io still returns 429, it parses the "Please try again after
    <RFC1123 date>" from the error text and sleeps exactly until then
    (plus a buffer), then retries automatically.

Usage:
  python3 publish_remaining.py                # publish everything remaining
  python3 publish_remaining.py --dry-run       # just show what would happen
  python3 publish_remaining.py --start-at consortium-log-macros

Run it from anywhere; it cds into each crate dir itself. Safe to Ctrl-C and
rerun later -- it always re-checks crates.io before attempting a publish.
"""

import argparse
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime

WORKSPACE_ROOT = "/workspace"  # change if your workspace lives elsewhere

# (crate_name_on_crates_io, path_relative_to_WORKSPACE_ROOT, version)
# Order matches the dependency-respecting order cargo itself packaged them in.
CRATES = [
    ("cogcore-sys", "crates/cogcore-sys", "0.1.0"),
    ("cogcore", "crates/cogcore", "0.1.0"),
    ("consortium-dts", "crates/consortium-dts", "0.1.0"),
    ("consortium-cfg-common", "crates/consortium-cfg-common", "0.1.0"),
    ("consortium-data", "crates/consortium-data", "0.1.0"),
    ("consortium-macros-helpers", "crates/consortium-macros-helpers", "0.1.0"),
    ("consortium-codec-macros-impl", "crates/consortium-codec-macros-impl", "0.1.0"),
    ("consortium-codec-macros", "crates/consortium-codec-macros", "0.1.0"),
    ("consortium-log-macros", "crates/consortium-log-macros", "0.1.0"),
    ("consortium-log", "crates/consortium-log", "0.1.0"),
    ("consortium-codec", "crates/consortium-codec", "0.1.0"),
    ("consortium-ipc-macros-impl", "crates/consortium-ipc-macros-impl", "0.1.0"),
    ("consortium-ipc-macros", "crates/consortium-ipc-macros", "0.1.0"),
    ("consortium-ipc", "crates/consortium-ipc", "0.1.0"),
    ("consortium-hal", "crates/consortium-hal", "0.1.0"),
    ("consortium-pac-stm32mp2", "crates/consortium-pac-stm32mp2", "0.1.0"),
    ("consortium-hal-stm32mp2", "crates/consortium-hal-stm32mp2", "0.1.0"),
    ("consortium-ipc-doorbell-hsem", "crates/consortium-ipc-doorbell-hsem", "0.1.0"),
    ("consortium-ipc-doorbell-ipcc", "crates/consortium-ipc-doorbell-ipcc", "0.1.0"),
    ("consortium-pac-imx9", "crates/consortium-pac-imx9", "0.1.0"),
    ("consortium-hal-imx9", "crates/consortium-hal-imx9", "0.1.0"),
    ("consortium-ipc-doorbell-mu", "crates/consortium-ipc-doorbell-mu", "0.1.0"),
    ("consortium-shared-memory", "crates/consortium-shared-memory", "0.1.0"),
    ("consortium-ipc-transport-memory", "crates/consortium-ipc-transport-memory", "0.1.0"),
    ("consortium-cfg", "crates/consortium-cfg", "0.1.0"),
    ("consortium-builder", "crates/consortium-builder", "0.1.0"),
    ("consortium-dbg", "crates/consortium-dbg", "0.1.0"),
    ("consortium-ffi", "crates/consortium-ffi", "0.1.0"),
    ("consortium-hmi", "crates/consortium-hmi", "0.1.0"),
    ("consortium-hmi-input", "crates/consortium-hmi-input", "0.1.0"),
    ("consortium-hmi-webkit", "crates/consortium-hmi-webkit", "0.1.0"),
    ("consortium-ipc-doorbell-gpio", "crates/consortium-ipc-doorbell-gpio", "0.1.0"),
    ("consortium-ipc-transport-uart", "crates/consortium-ipc-transport-uart", "0.1.0"),
    ("consortium-ipc-transport-uart-embedded", "crates/consortium-ipc-transport-uart-embedded", "0.1.0"),
    ("consortium-ipc-transport-uart-unix", "crates/consortium-ipc-transport-uart-unix", "0.1.0"),
    ("consortium-ort", "crates/consortium-ort", "0.1.0"),
    ("consortium-runtime-macros-impl", "crates/consortium-runtime-macros-impl", "0.1.0"),
    ("consortium-runtime-macros", "crates/consortium-runtime-macros", "0.1.0"),
    ("consortium-tee-macros-impl", "crates/consortium-tee-macros-impl", "0.1.0"),
    ("consortium-tee-macros", "crates/consortium-tee-macros", "0.1.0"),
    ("consortium-tee", "crates/consortium-tee", "0.1.0"),
    ("consortium-runtime-app", "crates/consortium-runtime-app", "0.1.0"),
    ("consortium-runtime-sys", "crates/consortium-runtime-sys", "0.1.0"),
    ("consortium-runtime-mcu", "crates/consortium-runtime-mcu", "0.1.0"),
    ("consortium-runtime-spl", "crates/consortium-runtime-spl", "0.1.0"),
    ("csti", "crates/consortium-cli", "0.1.0"),
]

BURST_GAP_SECONDS = 10 * 60 + 15  # 10 min limit + small safety buffer
RETRY_PARSE_RE = re.compile(r"try again after ([A-Za-z]{3}, \d{2} [A-Za-z]{3} \d{4} [\d:]{8} GMT)")


def already_published(name: str, version: str) -> bool:
    url = f"https://crates.io/api/v1/crates/{name}/{version}"
    req = urllib.request.Request(url, headers={"User-Agent": "publish_remaining.py (resume script)"})
    try:
        urllib.request.urlopen(req, timeout=15)
        return True
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return False
        raise
    except urllib.error.URLError as e:
        print(f"  ! could not reach crates.io to check {name} ({e}); assuming NOT published", file=sys.stderr)
        return False


def run_publish(path: str, dry_run: bool) -> tuple[bool, str]:
    cmd = ["cargo", "publish"]
    if dry_run:
        print(f"  [dry-run] would run: {' '.join(cmd)} (in {path})")
        return True, ""
    proc = subprocess.run(cmd, cwd=path, capture_output=True, text=True)
    output = proc.stdout + proc.stderr
    print(output)
    return proc.returncode == 0, output


def sleep_until_retry_time(error_text: str) -> bool:
    """If the error contains a crates.io retry-after date, sleep until then.
    Returns True if it found and slept, False otherwise."""
    m = RETRY_PARSE_RE.search(error_text)
    if not m:
        return False
    retry_at = parsedate_to_datetime(m.group(1)).astimezone(timezone.utc)
    now = datetime.now(timezone.utc)
    wait = (retry_at - now).total_seconds() + 15  # small buffer
    if wait > 0:
        print(f"  Rate limited. crates.io says retry after {retry_at.isoformat()}. "
              f"Sleeping {wait/60:.1f} min...")
        time.sleep(wait)
    return True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true", help="Don't actually publish, just show plan")
    ap.add_argument("--start-at", help="Skip ahead to this crate name in the list")
    ap.add_argument("--workspace-root", default=WORKSPACE_ROOT)
    args = ap.parse_args()

    crates = CRATES
    if args.start_at:
        names = [c[0] for c in crates]
        if args.start_at not in names:
            sys.exit(f"--start-at {args.start_at!r} not found in crate list")
        crates = crates[names.index(args.start_at):]

    for name, rel_path, version in crates:
        path = f"{args.workspace_root}/{rel_path}"
        print(f"== {name} {version} ==")

        if already_published(name, version):
            print("  already on crates.io, skipping.")
            continue

        while True:
            ok, output = run_publish(path, args.dry_run)
            if ok:
                break
            if sleep_until_retry_time(output):
                continue  # retry the same crate immediately after sleeping
            print(f"  publish failed for {name} and no rate-limit info found; stopping.")
            sys.exit(1)

        if not args.dry_run:
            print(f"  published. Waiting {BURST_GAP_SECONDS/60:.1f} min before next new-crate publish...")
            time.sleep(BURST_GAP_SECONDS)

    print("All crates processed.")


if __name__ == "__main__":
    main()