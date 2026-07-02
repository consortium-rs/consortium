"""Common utilities for CMSIS header parsing."""

import re
import json
import yaml
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple


def safe_hex_to_int(hex_str: str) -> Optional[int]:
    """Convert hex string to int, handling UL/u suffixes."""
    if not hex_str:
        return None
    # Remove UL, u, L suffixes
    clean = re.sub(r"[uUlL]+$", "", hex_str.strip())
    try:
        return int(clean, 0)
    except ValueError:
        return None


def evaluate_expr(expr: str, symbols: Dict[str, int]) -> Optional[int]:
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


def write_yaml(data: Dict[str, Any], path: Path) -> None:
    """Write data to YAML file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False)


def write_json(data: Dict[str, Any], path: Path) -> None:
    """Write data to JSON file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        json.dump(data, f, indent=2)


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
