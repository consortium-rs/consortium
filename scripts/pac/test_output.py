#!/usr/bin/env python3
"""Test and validate all generated register YAML, chip JSON, and signal JSON files."""

import json
import re
import sys
from collections import Counter
from pathlib import Path

import yaml
from signal_db import (
    DMA_KEYS,
    MUX_KINDS,
    PACKAGE_KEYS,
    PAD_KEYS,
    SIGNAL_KEYS,
    TOP_KEYS,
    family_peripherals,
)

OUTPUT_DIR = (
    Path(__file__).resolve().parents[2] / "crates" / "consortium-data" / "database"
)


def validate_yaml_syntax(file_path: Path) -> tuple[bool, str]:
    """Check if YAML file is valid."""
    try:
        with open(file_path) as f:
            yaml.safe_load(f)
        return True, "OK"
    except (OSError, UnicodeError, yaml.YAMLError) as e:
        return False, str(e)


def validate_json_syntax(file_path: Path) -> tuple[bool, str]:
    """Check if JSON file is valid."""
    try:
        with open(file_path) as f:
            json.load(f)
        return True, "OK"
    except (OSError, UnicodeError, json.JSONDecodeError) as e:
        return False, str(e)


def check_descriptions(data: dict) -> list[str]:
    """Check for common description issues."""
    issues = []

    # Check blocks and fieldsets for bad descriptions
    for key, value in data.items():
        if not isinstance(value, dict):
            continue

        desc = value.get("description", "")
        if not isinstance(desc, str):
            continue

        # Check for comment artifacts
        if "/*!<" in desc or "/*!" in desc or "*/" in desc:
            issues.append(f"  {key}: has comment artifact in description: {desc[:60]}")

        # Check for Doxygen tags
        if re.search(r"@\w+", desc):
            issues.append(f"  {key}: has Doxygen tag in description: {desc[:60]}")

        # Check for Address offset metadata
        if "Address offset" in desc or "address offset" in desc:
            issues.append(f"  {key}: has offset metadata in description: {desc[:60]}")

        # Check for array metadata
        if "array step" in desc.lower() or "step:" in desc:
            issues.append(f"  {key}: has array metadata in description: {desc[:60]}")

        # Check for excessive whitespace
        if "  " in desc:
            issues.append(
                f"  {key}: has excessive whitespace in description: {desc[:60]}"
            )

        # Check fields in fieldsets
        if "fields" in value:
            for field in value["fields"]:
                field_desc = field.get("description", "")
                if isinstance(field_desc, str) and (
                    "/*!<" in field_desc or "@" in field_desc
                ):
                    issues.append(
                        f"  {key}.{field['name']}: bad description: {field_desc[:60]}"
                    )

    return issues


def check_bitfield_widths(data: dict) -> list[str]:
    """Check for invalid bitfield widths."""
    issues = []

    for key, value in data.items():
        if not isinstance(value, dict) or "fields" not in value:
            continue

        # Check for zero or negative bit sizes
        for field in value["fields"]:
            bit_size = field.get("bit_size")

            if not isinstance(bit_size, int):
                continue

            if bit_size <= 0:
                issues.append(f"  {key}.{field['name']}: invalid bit_size: {bit_size}")

            # NXP headers include legitimate 64-bit registers, so only flag
            # widths that exceed the largest scalar register size we model.
            if bit_size > 64:
                issues.append(
                    f"  {key}.{field['name']}: suspiciously large bit_size: {bit_size}"
                )

    return issues


def check_field_overlaps(data: dict) -> list[str]:
    """Report fields starting at the same bit.

    CMSIS headers define some aliases at one offset (single-bit/array pairs such
    as RLOCK/RLOCK0), so these are informational warnings rather than errors;
    `just pac-validate` is what rejects genuine overlaps. A split field's
    `bit_offset` is a list of `{start, end}` ranges, keyed here by its first bit.
    """
    warnings = []

    for key, value in data.items():
        if not isinstance(value, dict) or "fields" not in value:
            continue

        fields_by_offset: dict[int, list] = {}
        for field in value["fields"]:
            offset = field.get("bit_offset", 0)
            if isinstance(offset, list):
                offset = offset[0]["start"]
            fields_by_offset.setdefault(offset, []).append(field)

        for offset, fields in fields_by_offset.items():
            if len(fields) > 1:
                names = [f["name"] for f in fields]
                warnings.append(f"  {key}: multiple fields at offset {offset}: {names}")

    return warnings


def check_chip_json(data: dict) -> list[str]:
    """Check chip JSON structure."""
    issues = []

    if "name" not in data:
        issues.append("  Missing 'name' field")

    if "peripherals" in data:
        for periph in data["peripherals"]:
            # Check base_address format
            if "base_address" in periph:
                addr = periph["base_address"]
                if not re.match(r"^0x[0-9a-f]{8}$", addr, re.IGNORECASE):
                    issues.append(
                        f"  {periph.get('name', '?')}: invalid base_address format: {addr}"
                    )

            # Check block reference
            if "block" not in periph:
                issues.append(f"  {periph.get('name', '?')}: missing block reference")

    return issues


def test_register_yamls():
    """Test all register YAML files."""
    print("=" * 70)
    print("Testing Register YAML Files")
    print("=" * 70)

    registers_dir = OUTPUT_DIR / "peri"
    yaml_files = list(registers_dir.rglob("*.yaml"))

    if not yaml_files:
        print("No register YAML files found")
        return 0, 0

    passed = 0
    failed = 0

    for yaml_file in sorted(yaml_files):
        rel_path = yaml_file.relative_to(OUTPUT_DIR)

        # Check syntax
        ok, msg = validate_yaml_syntax(yaml_file)
        if not ok:
            print(f"❌ {rel_path}: YAML syntax error: {msg}")
            failed += 1
            continue

        # Load and check content
        try:
            with open(yaml_file) as f:
                data = yaml.safe_load(f)
        except (OSError, UnicodeError, yaml.YAMLError) as e:
            print(f"❌ {rel_path}: Failed to load: {e}")
            failed += 1
            continue

        issues = []
        issues.extend(check_descriptions(data))
        issues.extend(check_bitfield_widths(data))

        # Non-fatal: legitimate CMSIS alternate-function / alias overlaps.
        warnings = check_field_overlaps(data)

        if issues:
            print(f"❌ {rel_path}:")
            for issue in issues[:5]:  # Show first 5 issues
                print(issue)
            if len(issues) > 5:
                print(f"  ... and {len(issues) - 5} more issues")
            failed += 1
        elif warnings:
            print(f"⚠️  {rel_path}: {len(warnings)} overlapping field view(s)")
            for warning in warnings[:5]:
                print(warning)
            if len(warnings) > 5:
                print(f"  ... and {len(warnings) - 5} more")
            passed += 1
        else:
            print(f"✓ {rel_path}")
            passed += 1

    return passed, failed


def test_chip_jsons():
    """Test all chip JSON files."""
    print("\n" + "=" * 70)
    print("Testing Chip JSON Files")
    print("=" * 70)

    chips_dir = OUTPUT_DIR / "chip"
    json_files = list(chips_dir.glob("*.json"))

    if not json_files:
        print("No chip JSON files found")
        return 0, 0

    passed = 0
    failed = 0

    for json_file in sorted(json_files):
        rel_path = json_file.relative_to(OUTPUT_DIR)

        # Check syntax
        ok, msg = validate_json_syntax(json_file)
        if not ok:
            print(f"❌ {rel_path}: JSON syntax error: {msg}")
            failed += 1
            continue

        # Load and check content
        try:
            with open(json_file) as f:
                data = json.load(f)
        except (OSError, UnicodeError, json.JSONDecodeError) as e:
            print(f"❌ {rel_path}: Failed to load: {e}")
            failed += 1
            continue

        issues = check_chip_json(data)

        if issues:
            print(f"⚠️  {rel_path}:")
            for issue in issues[:5]:
                print(issue)
            if len(issues) > 5:
                print(f"  ... and {len(issues) - 5} more issues")
            failed += 1
        else:
            # Check IRQ count
            irq_count = len(data.get("irqs", []))
            periph_count = len(data.get("peripherals", []))
            print(f"✓ {rel_path}: {irq_count} IRQs, {periph_count} peripherals")
            passed += 1

    return passed, failed


REGISTER = re.compile(r"0x[0-9a-f]{8}")


def check_function(record: dict, peripherals: set[str], where: str) -> list[str]:
    """`peripheral`/`signal` must be a real split of `function`, or both null."""
    function = record["function"]
    peripheral = record["peripheral"]
    signal = record["signal"]
    if not isinstance(function, str) or not function:
        return [f"  {where}: function {function!r} is not a name"]
    if peripheral is None:
        if signal is not None:
            return [f"  {where}: {function} has a signal but no peripheral"]
        return []
    if peripheral not in peripherals:
        return [f"  {where}: {function}: peripheral {peripheral} is in no chip dump"]
    expected = function[len(peripheral) + 1 :] or None
    if not (function == peripheral or function.startswith(peripheral + "_")):
        return [f"  {where}: {function} does not start with {peripheral}"]
    if signal != expected:
        return [f"  {where}: {function}: signal {signal!r}, expected {expected!r}"]
    return []


def check_mux(mux: object, where: str) -> list[str]:
    if not isinstance(mux, dict) or mux.get("kind") not in MUX_KINDS:
        return [f"  {where}: unknown mux {mux!r}"]
    kind = mux["kind"]
    if sorted(mux) != sorted(("kind", *MUX_KINDS[kind])):
        return [f"  {where}: {kind} mux has keys {sorted(mux)}"]
    if kind == "af" and not (isinstance(mux["af"], int) and 0 <= mux["af"] <= 15):
        return [f"  {where}: AF {mux['af']!r} is not 0-15"]
    if kind != "iomuxc":
        return []
    issues = []
    for key in ("mux_register", "config_register"):
        if not (isinstance(mux[key], str) and REGISTER.fullmatch(mux[key])):
            issues.append(f"  {where}: {key} {mux[key]!r} is not a register")
    if not (isinstance(mux["mux_mode"], int) and 0 <= mux["mux_mode"] <= 15):
        issues.append(f"  {where}: mux_mode {mux['mux_mode']!r} is not 0-15")
    has_input = mux["input_register"] is not None
    if has_input != (mux["input_daisy"] is not None):
        issues.append(f"  {where}: input_register and input_daisy disagree")
    elif has_input and not REGISTER.fullmatch(str(mux["input_register"])):
        issues.append(f"  {where}: input_register {mux['input_register']!r}")
    return issues


def check_signal_db(family: str, data: dict, peripherals: set[str]) -> list[str]:
    """Check one `database/signal/<family>.json` against the shared schema."""
    if not isinstance(data, dict) or tuple(data) != TOP_KEYS:
        return [f"  top-level keys are {list(data)}, expected {list(TOP_KEYS)}"]
    if data["family"] != family:
        return [f"  family is {data['family']!r}, file is {family}"]

    issues: list[str] = []

    def keyed(records: list, keys: tuple[str, ...], what: str) -> list[dict]:
        good = []
        for i, record in enumerate(records):
            if isinstance(record, dict) and tuple(record) == keys:
                good.append(record)
            else:
                issues.append(f"  {what}[{i}]: keys are not {list(keys)}")
        return good

    packages = keyed(data["packages"], PACKAGE_KEYS, "packages")
    pads = keyed(data["pads"], PAD_KEYS, "pads")
    signals = keyed(data["signals"], SIGNAL_KEYS, "signals")
    dma = keyed(data["dma"], DMA_KEYS, "dma")

    package_names = [p["name"] for p in packages]
    if len(set(package_names)) != len(package_names):
        issues.append("  duplicate package names")
    parts = [part for p in packages for part in p["parts"]]
    if len(set(parts)) != len(parts):
        issues.append("  a part number is listed under two packages")

    pad_names = [p["name"] for p in pads]
    if len(set(pad_names)) != len(pad_names):
        issues.append("  duplicate pad names")
    for pad in pads:
        bonded = pad["packages"]
        if not package_names:
            if bonded is not None:
                issues.append(f"  pad {pad['name']}: packages listed but none declared")
        elif not bonded or not set(bonded) <= set(package_names):
            issues.append(f"  pad {pad['name']}: packages {bonded!r} not declared")

    declared = set(pad_names)
    seen_signals: set[tuple[str, str]] = set()
    for s in signals:
        where = f"signal {s['pad']}/{s['function']}"
        if s["pad"] not in declared:
            issues.append(f"  {where}: pad is not declared")
        if (s["pad"], s["function"]) in seen_signals:
            issues.append(f"  {where}: listed twice")
        seen_signals.add((s["pad"], s["function"]))
        issues += check_function(s, peripherals, where)
        issues += check_mux(s["mux"], where)

    seen_requests: set[tuple[str, int]] = set()
    for d in dma:
        where = f"dma {d['controller']}/{d['request']}"
        if not (isinstance(d["request"], int) and d["request"] >= 0):
            issues.append(f"  {where}: request is not a non-negative integer")
        if (d["controller"], d["request"]) in seen_requests:
            issues.append(f"  {where}: request number used twice")
        seen_requests.add((d["controller"], d["request"]))
        issues += check_function(d, peripherals, where)

    return issues


def unmatched_summary(data: dict) -> str:
    """Function prefixes no chip dump names, for the report (not a failure)."""
    prefixes: Counter[str] = Counter(
        r["function"].split("_")[0]
        for r in (*data["signals"], *data["dma"])
        if r["peripheral"] is None
    )
    if not prefixes:
        return ""
    top = ", ".join(f"{p} {n}" for p, n in prefixes.most_common(6))
    return f"; unmatched: {sum(prefixes.values())} ({top})"


def test_signal_dbs():
    """Test every `database/signal/<family>.json`."""
    print("\n" + "=" * 70)
    print("Testing Signal Databases")
    print("=" * 70)

    signal_dir = OUTPUT_DIR / "signal"
    json_files = sorted(signal_dir.glob("*.json"))
    if not json_files:
        print("No signal databases found")
        return 0, 0

    passed = failed = 0
    for json_file in json_files:
        rel_path = json_file.relative_to(OUTPUT_DIR)
        family = json_file.stem
        try:
            data = json.loads(json_file.read_text())
            peripherals = family_peripherals(family)
        except (OSError, UnicodeError, json.JSONDecodeError, SystemExit) as e:
            print(f"❌ {rel_path}: {e}")
            failed += 1
            continue

        issues = check_signal_db(family, data, peripherals)
        if issues:
            print(f"❌ {rel_path}:")
            for issue in issues[:10]:
                print(issue)
            if len(issues) > 10:
                print(f"  ... and {len(issues) - 10} more issues")
            failed += 1
        else:
            print(
                f"✓ {rel_path}: {len(data['pads'])} pads, "
                f"{len(data['signals'])} signals, {len(data['dma'])} DMA requests"
                f"{unmatched_summary(data)}"
            )
            passed += 1

    return passed, failed


def main():
    """Run all tests."""
    if not OUTPUT_DIR.exists():
        print(f"Error: output directory not found at {OUTPUT_DIR}")
        sys.exit(1)

    yaml_passed, yaml_failed = test_register_yamls()
    json_passed, json_failed = test_chip_jsons()
    signal_passed, signal_failed = test_signal_dbs()

    total_passed = yaml_passed + json_passed + signal_passed
    total_failed = yaml_failed + json_failed + signal_failed

    print("\n" + "=" * 70)
    print("Summary")
    print("=" * 70)
    print(f"Register YAMLs: {yaml_passed} passed, {yaml_failed} failed")
    print(f"Chip JSONs:    {json_passed} passed, {json_failed} failed")
    print(f"Signal DBs:    {signal_passed} passed, {signal_failed} failed")
    print(f"Total:         {total_passed} passed, {total_failed} failed")

    if total_failed == 0:
        print("\n✓ All tests passed!")
        return 0
    else:
        print(f"\n✗ {total_failed} file(s) have issues")
        return 1


if __name__ == "__main__":
    sys.exit(main())
