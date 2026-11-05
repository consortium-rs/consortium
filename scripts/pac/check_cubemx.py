#!/usr/bin/env python3
"""Cross-check the STM32 signal databases against a local STM32CubeMX install.

`STM32_open_pin_data` is ST's export of the CubeMX pin database. This re-reads
each STM32 family's pins straight from an installed CubeMX (`db/mcu/*.xml` and
`db/mcu/IP/GPIO-*_Modes.xml`) with the same `collect_pins` the pipeline uses,
and compares packages, pads, and pad signals with the committed
`database/signal/<family>.json`. DMA requests come from STM32CubeMP2, not
CubeMX, so they are not compared.

Local only, never in CI: CubeMX is ST-licensed and not redistributable, so
nothing here is copied into the repository. A newer CubeMX than the pinned
`STM32CubeMX-DB.*` tag can differ legitimately; the database release of each
side is printed so a difference can be told apart from a parser bug.

    python3 scripts/pac/check_cubemx.py [--db <CubeMX>/Contents/Resources/db]

The database path defaults to `$CUBEMX_DB`, then the macOS install location.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path

from parse_signals_stm32 import FAMILIES, collect_pins
from signal_db import OUTPUT_DIR, ROOT, family_peripherals

DEFAULT_DB = Path(
    "/Applications/STMicroelectronics/STM32CubeMX.app/Contents/Resources/db"
)


def cubemx_release(db: Path) -> str:
    match = re.search(
        r'Release="([^"]+)"', (db / "package.xml").read_text(encoding="latin-1")
    )
    return match.group(1) if match else "unknown"


def pinned_release() -> str:
    result = subprocess.run(
        ["git", "-C", "submodules/STM32_open_pin_data", "describe", "--tags"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    return result.stdout.strip() or "unknown"


def signal_rows(signals: list[dict]) -> set[tuple[str, str, str]]:
    return {
        (s["pad"], s["function"], json.dumps(s["mux"], sort_keys=True)) for s in signals
    }


def compare(family: str, db: Path) -> list[str]:
    committed = json.loads((OUTPUT_DIR / f"{family}.json").read_text())
    pins = collect_pins(
        family, db / "mcu", db / "mcu" / "IP", family_peripherals(family)
    )

    issues = []
    ours = {p["name"]: sorted(p["parts"]) for p in committed["packages"]}
    theirs = {p["name"]: sorted(p["parts"]) for p in pins.packages}
    if ours != theirs:
        issues.append(f"  packages differ: {sorted(ours)} vs CubeMX {sorted(theirs)}")

    ours_pads = {p["name"]: p["packages"] for p in committed["pads"]}
    theirs_pads = {p["name"]: p["packages"] for p in pins.pads}
    for pad in sorted(ours_pads.keys() ^ theirs_pads.keys()):
        side = "only committed" if pad in ours_pads else "only in CubeMX"
        issues.append(f"  pad {pad}: {side}")
    for pad in sorted(ours_pads.keys() & theirs_pads.keys()):
        if ours_pads[pad] != theirs_pads[pad]:
            issues.append(
                f"  pad {pad}: packages {ours_pads[pad]} vs CubeMX {theirs_pads[pad]}"
            )

    ours_rows = signal_rows(committed["signals"])
    theirs_rows = signal_rows(pins.signals)
    for pad, function, mux in sorted(ours_rows - theirs_rows):
        issues.append(f"  signal {pad}/{function} {mux}: only committed")
    for pad, function, mux in sorted(theirs_rows - ours_rows):
        issues.append(f"  signal {pad}/{function} {mux}: only in CubeMX")
    return issues


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--db",
        type=Path,
        default=Path(os.environ.get("CUBEMX_DB", DEFAULT_DB)),
        help="CubeMX database directory (the one holding package.xml and mcu/)",
    )
    args = parser.parse_args()
    if not (args.db / "mcu").is_dir():
        print(f"No CubeMX database at {args.db}; pass --db or set CUBEMX_DB.")
        return 2

    print(f"CubeMX {cubemx_release(args.db)} vs pinned {pinned_release()}")
    failed = 0
    for family in FAMILIES:
        issues = compare(family, args.db)
        if issues:
            failed += 1
            print(f"❌ {family}: {len(issues)} differences")
            for issue in issues[:20]:
                print(issue)
            if len(issues) > 20:
                print(f"  ... and {len(issues) - 20} more")
        else:
            print(f"✓ {family}: packages, pads, and signals match")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
