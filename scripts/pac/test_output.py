#!/usr/bin/env python3
"""Test and validate all generated register YAML and chip JSON files."""

import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Set

import yaml

OUTPUT_DIR = Path(__file__).resolve().parents[2] / "output"


def validate_yaml_syntax(file_path: Path) -> Tuple[bool, str]:
    """Check if YAML file is valid."""
    try:
        with open(file_path) as f:
            yaml.safe_load(f)
        return True, "OK"
    except Exception as e:
        return False, str(e)


def validate_json_syntax(file_path: Path) -> Tuple[bool, str]:
    """Check if JSON file is valid."""
    try:
        with open(file_path) as f:
            json.load(f)
        return True, "OK"
    except Exception as e:
        return False, str(e)


def check_descriptions(data: Dict) -> List[str]:
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
                if isinstance(field_desc, str):
                    if "/*!<" in field_desc or "@" in field_desc:
                        issues.append(
                            f"  {key}.{field['name']}: bad description: {field_desc[:60]}"
                        )

    return issues


def check_bitfield_widths(data: Dict) -> List[str]:
    """Check for invalid bitfield widths."""
    issues = []

    for key, value in data.items():
        if not isinstance(value, dict) or "fields" not in value:
            continue

        # Check for zero or negative bit sizes
        for field in value["fields"]:
            bit_size = field.get("bit_size")
            bit_offset = field.get("bit_offset", 0)

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


def check_field_overlaps(data: Dict) -> List[str]:
    """Report fields sharing a bit offset.

    CMSIS headers legitimately define multiple views at the same offset (e.g.
    timer CCMRx input-capture vs output-compare modes, or single-bit/array
    aliases like RLOCK/RLOCK0). chiptool represents these as distinct fields, so
    these are informational warnings rather than errors.
    """
    warnings = []

    for key, value in data.items():
        if not isinstance(value, dict) or "fields" not in value:
            continue

        fields_by_offset: Dict[int, List] = {}
        for field in value["fields"]:
            offset = field.get("bit_offset", 0)
            fields_by_offset.setdefault(offset, []).append(field)

        for offset, fields in fields_by_offset.items():
            if len(fields) > 1:
                names = [f["name"] for f in fields]
                warnings.append(f"  {key}: multiple fields at offset {offset}: {names}")

    return warnings


def check_chip_json(data: Dict) -> List[str]:
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

    registers_dir = OUTPUT_DIR / "registers"
    yaml_files = list(registers_dir.rglob("*_v1.yaml"))

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
        except Exception as e:
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

    chips_dir = OUTPUT_DIR / "chips"
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
        except Exception as e:
            print(f"❌ {rel_path}: Failed to load: {e}")
            failed += 1
            continue

        issues = issues = check_chip_json(data)

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


def main():
    """Run all tests."""
    if not OUTPUT_DIR.exists():
        print(f"Error: output directory not found at {OUTPUT_DIR}")
        sys.exit(1)

    yaml_passed, yaml_failed = test_register_yamls()
    json_passed, json_failed = test_chip_jsons()

    total_passed = yaml_passed + json_passed
    total_failed = yaml_failed + json_failed

    print("\n" + "=" * 70)
    print("Summary")
    print("=" * 70)
    print(f"Register YAMLs: {yaml_passed} passed, {yaml_failed} failed")
    print(f"Chip JSONs:    {json_passed} passed, {json_failed} failed")
    print(f"Total:         {total_passed} passed, {total_failed} failed")

    if total_failed == 0:
        print("\n✓ All tests passed!")
        return 0
    else:
        print(f"\n✗ {total_failed} file(s) have issues")
        return 1


if __name__ == "__main__":
    sys.exit(main())
