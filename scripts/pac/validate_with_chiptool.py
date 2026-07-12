#!/usr/bin/env python3
"""Validate all generated register YAML files using chiptool."""

import subprocess
import sys
from pathlib import Path
from typing import Tuple

OUTPUT_DIR = (
    Path(__file__).resolve().parents[2] / "crates" / "consortium-data" / "database"
)


def run_chiptool_validate(yaml_file: Path) -> Tuple[bool, str]:
    """Run chiptool check on a YAML file."""
    try:
        result = subprocess.run(
            [
                "chiptool",
                "check",
                str(yaml_file),
                "--allow-unused-enums",
                "--allow-unused-fieldsets",
            ],
            capture_output=True,
            text=True,
            timeout=10,
        )
        if result.returncode == 0:
            return True, "OK"
        else:
            # Capture error message
            error = result.stderr if result.stderr else result.stdout
            return False, error.strip()
    except FileNotFoundError:
        return False, "chiptool not found (install with: pip install chiptool)"
    except subprocess.TimeoutExpired:
        return False, "chiptool timeout"
    except Exception as e:
        return False, str(e)


def validate_all_yamls():
    """Validate all register YAML files."""
    print("=" * 80)
    print("Validating All Register YAML Files with chiptool")
    print("=" * 80)

    registers_dir = OUTPUT_DIR / "peri"
    yaml_files = sorted(registers_dir.rglob("*.yaml"))

    if not yaml_files:
        print("No register YAML files found")
        return 0, 0

    passed = 0
    failed = 0
    failed_files = []

    for i, yaml_file in enumerate(yaml_files, 1):
        rel_path = yaml_file.relative_to(OUTPUT_DIR)

        ok, msg = run_chiptool_validate(yaml_file)

        if ok:
            print(f"[{i:3d}/{len(yaml_files)}] ✓ {rel_path}")
            passed += 1
        else:
            print(f"[{i:3d}/{len(yaml_files)}] ✗ {rel_path}")
            # Show first line of error
            error_line = msg.split("\n")[0] if msg else "Unknown error"
            print(f"           {error_line[:70]}")
            failed += 1
            failed_files.append((rel_path, msg))

    return passed, failed, failed_files


def main():
    """Run validation."""
    if not OUTPUT_DIR.exists():
        print(f"Error: output directory not found at {OUTPUT_DIR}")
        sys.exit(1)

    passed, failed, failed_files = validate_all_yamls()

    print("\n" + "=" * 80)
    print("Summary")
    print("=" * 80)
    print(f"Passed: {passed}/{passed + failed}")
    print(f"Failed: {failed}/{passed + failed}")

    if failed > 0:
        print("\n" + "=" * 80)
        print("Failed Files (first 10)")
        print("=" * 80)
        for rel_path, error in failed_files[:10]:
            print(f"\n{rel_path}:")
            # Show first 3 lines of error
            error_lines = error.split("\n")[:3]
            for line in error_lines:
                if line.strip():
                    print(f"  {line}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
