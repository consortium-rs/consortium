"""Common utilities for CMSIS header parsing."""

import json
import re
from pathlib import Path
from typing import Any

import yaml


def safe_hex_to_int(hex_str: str) -> int | None:
    """Convert hex string to int, handling UL/u suffixes."""
    if not hex_str:
        return None
    # Remove UL, u, L suffixes
    clean = re.sub(r"[uUlL]+$", "", hex_str.strip())
    try:
        return int(clean, 0)
    except ValueError:
        return None


def evaluate_expr(expr: str, symbols: dict[str, int]) -> int | None:
    """Evaluate simple arithmetic expressions like 'BASE + 0x1000'."""
    expr = expr.replace("UL", "").replace("u", "").replace("L", "").strip()

    # Handle simple addition/subtraction
    terms = re.split(r"([+-])", expr)
    if not terms:
        return None

    result = 0
    op = "+"

    for term in terms:
        term = term.strip()
        if not term:
            continue
        if term in ("+", "-"):
            op = term
            continue

        # Try to resolve symbol or parse as literal
        if term in symbols:
            val = symbols[term]
        else:
            val = safe_hex_to_int(term)

        if val is None:
            return None

        if op == "+":
            result += val
        else:
            result -= val

    return result


def popcount(n: int) -> int:
    """Count set bits in integer."""
    count = 0
    while n:
        count += n & 1
        n >>= 1
    return count


def calc_bit_size_from_mask(mask_val: int) -> int:
    """Calculate bit width from mask value (e.g., 0xFF → 8 bits)."""
    if mask_val == 0:
        return 0
    # Find highest set bit
    bit_size = mask_val.bit_length()
    # Verify it's a contiguous mask (no gaps)
    expected = (1 << bit_size) - 1
    if mask_val == expected:
        return bit_size
    # If not contiguous, just count set bits
    return popcount(mask_val)


def mask_ranges(mask_val: int) -> list[tuple[int, int]]:
    """Split a shifted mask into its runs of set bits, lowest first.

    `0x00300070` (bits 4-6 and 20-21) gives `[(4, 6), (20, 21)]`.
    """
    ranges: list[tuple[int, int]] = []
    bit = 0
    while mask_val >> bit:
        if (mask_val >> bit) & 1:
            start = bit
            while (mask_val >> (bit + 1)) & 1:
                bit += 1
            ranges.append((start, bit))
        bit += 1
    return ranges


class _IndentDumper(yaml.Dumper):
    """Dumper that indents block sequences under their parent key.

    PyYAML's default dedents sequence dashes to their parent's column; the
    committed register YAML instead indents them one level (a 4-space `-`), so
    force `indentless=False` to reproduce that style and keep regenerated files
    free of whitespace-only churn.
    """

    def increase_indent(self, flow: bool = False, indentless: bool = False) -> Any:
        return super().increase_indent(flow, indentless=False)


def write_yaml(data: dict[str, Any], path: Path) -> None:
    """Write data to YAML file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        yaml.dump(
            data, f, Dumper=_IndentDumper, default_flow_style=False, sort_keys=False
        )


def write_json(data: dict[str, Any], path: Path) -> None:
    """Write data to JSON file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        json.dump(data, f, indent=2)
        # Committed chip descriptors are newline-terminated; match them so
        # regenerated files don't churn a trailing-newline-only diff.
        f.write("\n")


def extract_description(line: str) -> str:
    """Extract description from C comment (/*!< ... */ or /** ... */)."""
    # Match /* ... */ with optional !, <, and whitespace
    match = re.search(r"/\*\s*!?<?[^\*]*?\*/", line)
    if not match:
        return ""

    desc = match.group(0)

    # Remove comment markers and artifacts
    desc = desc.replace("/*", "").replace("*/", "").strip()
    desc = re.sub(r"^[*!<\s]+", "", desc).strip()

    # Remove Doxygen tags like @name, @brief, etc
    desc = re.sub(r"\s*@\w+.*$", "", desc)

    # Remove Address offset and array metadata
    desc = re.sub(r"[,;]*\s*(?:[aA]ddress\s+)?offset:.*$", "", desc)
    desc = re.sub(r"[,;]*\s*(?:array\s+)?step:.*$", "", desc)

    # Normalize whitespace
    desc = re.sub(r"\s+", " ", desc).strip()

    return desc


def strip_suffix(name: str, suffix: str) -> str:
    """Remove suffix from name if present."""
    if name.endswith(suffix):
        return name[: -len(suffix)]
    return name
