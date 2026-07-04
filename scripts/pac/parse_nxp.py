#!/usr/bin/env python3
"""Parse NXP MCUX CMSIS headers into chiptool IR YAML and chip JSON."""

import re
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple, Set
import argparse

from common import (
    safe_hex_to_int,
    evaluate_expr,
    popcount,
    calc_bit_size_from_mask,
    write_yaml,
    write_json,
    extract_description,
    strip_suffix,
)


DATA_DIR = Path(__file__).resolve().parents[2] / "data"
OUTPUT_DIR = Path(__file__).resolve().parents[2] / "crates" / "consortium-data" / "database"

FAMILY_REPRESENTATIVES = {
    "imx93_cm33": ("MIMX9352_cm33", "MIMX9352_cm33_COMMON.h"),
    "imx95_cm7": ("MIMX9596_cm7", "MIMX9596_cm7_COMMON.h"),
}


class NXPPeripheralParser:
    """Parse a single PERI_*.h file for register definitions."""

    def __init__(self, peri_header_path: Path):
        self.peri_header_path = peri_header_path
        with open(peri_header_path) as f:
            self.content = f.read()

        # Extract peripheral name from filename (PERI_LPUART.h → LPUART)
        match = re.search(r"PERI_(\w+)\.h", peri_header_path.name)
        self.peripheral_name = match.group(1) if match else "UNKNOWN"

        self.block_name = self.peripheral_name
        self.block_data: Dict[str, Any] = {"description": "", "items": []}
        self.fieldsets: Dict[str, Dict[str, Any]] = {}

    @staticmethod
    def _item_size(item: Dict[str, Any]) -> int:
        """Return the address span occupied by a register item."""
        array = item.get("array")
        if isinstance(array, dict):
            return (int(array.get("len", 1)) - 1) * int(array.get("stride", 4)) + 4
        return 4

    @classmethod
    def _prune_overlapping_items(
        cls, items: List[Dict[str, Any]]
    ) -> List[Dict[str, Any]]:
        """Drop alternate register views that overlap in chiptool IR."""
        by_name_offset: Dict[Tuple[str, int], Dict[str, Any]] = {}
        order: List[Tuple[str, int]] = []

        for item in items:
            key = (item["name"], int(item["byte_offset"]))
            if key not in by_name_offset:
                by_name_offset[key] = item
                order.append(key)
                continue

            existing = by_name_offset[key]
            if cls._item_size(item) > cls._item_size(existing):
                by_name_offset[key] = item

        compacted: List[Dict[str, Any]] = []
        for key in order:
            item = by_name_offset[key]
            start = int(item["byte_offset"])
            end = start + cls._item_size(item) - 1

            if any(
                max(start, int(existing["byte_offset"]))
                <= min(
                    end,
                    int(existing["byte_offset"]) + cls._item_size(existing) - 1,
                )
                for existing in compacted
            ):
                continue

            compacted.append(item)

        return compacted

    @staticmethod
    def _prune_overlapping_fields(fields: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Drop alternate bitfield views that overlap in chiptool IR."""
        pruned: List[Dict[str, Any]] = []
        used_bits: Set[int] = set()

        for field in sorted(fields, key=lambda f: (f["bit_offset"], f["name"])):
            offset = int(field["bit_offset"])
            size = int(field["bit_size"])
            bits = set(range(offset, offset + size))

            if used_bits & bits:
                continue

            pruned.append(field)
            used_bits.update(bits)

        return pruned

    def parse_array_counts(self) -> Dict[str, int]:
        """Extract #define FOO_COUNT N declarations."""
        counts = {}
        for match in re.finditer(r"#define\s+(\w+)\s+(\d+)u?", self.content):
            name = match.group(1)
            count = int(match.group(2))
            counts[name] = count
        return counts

    def parse_struct(self):
        """Extract typedef struct definition."""
        # Find struct matching Type suffix
        pattern = rf"typedef\s+struct\s*\{{(.*?)\}}\s*(\w+_Type)\s*;"
        match = re.search(pattern, self.content, re.DOTALL)
        if not match:
            return

        struct_body = match.group(1)
        struct_name = match.group(2)

        # Verify struct name matches peripheral
        if self.peripheral_name not in struct_name:
            return

        array_counts = self.parse_array_counts()
        current_offset = 0
        lines = struct_body.split("\n")
        i = 0

        while i < len(lines):
            line = lines[i]
            line_stripped = line.strip()
            i += 1

            if (
                not line_stripped
                or line_stripped.startswith("/*")
                or line_stripped.startswith("//")
            ):
                continue

            # Check for nested struct declaration: struct { ... } NAME[array];
            struct_match = re.match(r"struct\s*\{", line_stripped)
            if struct_match:
                # Extract struct array step from comment
                array_step = None
                step_match = re.search(r"array\s+step:\s*0x([0-9a-fA-F]+)", line)
                if step_match:
                    array_step = int(step_match.group(1), 16)

                struct_base_offset = current_offset
                offset_match = re.search(r"offset:\s*0x([0-9a-fA-F]+)", line)
                if offset_match:
                    struct_base_offset = int(offset_match.group(1), 16)
                    current_offset = struct_base_offset

                # Collect all nested structs and top-level fields until the closing brace of the outer struct
                nested_structs: List[
                    Tuple[str, List[str], int]
                ] = []  # (struct_name, body, offset)
                top_level_fields: List[
                    str
                ] = []  # Fields at the top level of the array struct
                outer_brace_depth = 0
                current_nested_body: List[str] = []
                current_nested_name: str = ""
                current_nested_offset = 0

                while i < len(lines):
                    nested_line = lines[i]
                    i += 1

                    nested_stripped = nested_line.strip()

                    # Check for inner struct declaration
                    inner_struct_match = re.match(r"struct\s*\{", nested_stripped)
                    if inner_struct_match:
                        inner_offset_match = re.search(
                            r"offset:\s*0x([0-9a-fA-F]+)", nested_line
                        )
                        if inner_offset_match:
                            current_nested_offset = int(inner_offset_match.group(1), 16)
                        else:
                            current_nested_offset = 0
                        current_nested_body = []
                        outer_brace_depth += 1
                        continue

                    # Check for closing brace of inner struct: } NAME;
                    if outer_brace_depth > 0 and nested_stripped.startswith("}"):
                        struct_member_match = re.match(
                            r"\}\s*(\w+)\s*;", nested_stripped
                        )
                        if struct_member_match:
                            current_nested_name = struct_member_match.group(1)
                            nested_structs.append(
                                (
                                    current_nested_name,
                                    current_nested_body,
                                    current_nested_offset,
                                )
                            )
                            outer_brace_depth -= 1
                            current_nested_body = []
                            continue

                    # Check for closing brace of outer array struct: } NAME[array];
                    if outer_brace_depth == 0 and nested_stripped.startswith("}"):
                        outer_struct_match = re.search(
                            r"\}\s*(\w+)(?:\[(\w+)\])?\s*;", nested_line
                        )

                        # Initialize array length - will be updated if this is an array struct
                        struct_array_len = 1

                        if outer_struct_match:
                            struct_member_name = outer_struct_match.group(1)
                            struct_array_name = outer_struct_match.group(2)

                            # Determine if this struct is an array
                            if struct_array_name:
                                struct_array_len = array_counts.get(
                                    struct_array_name, 1
                                )

                            # Process each nested struct in the array
                            for (
                                nested_name,
                                nested_body,
                                nested_offset,
                            ) in nested_structs:
                                for nested_line_str in nested_body:
                                    nested_stripped = nested_line_str.strip()
                                    if not nested_stripped:
                                        continue

                                    field_match = re.match(
                                        r"(__IO|__I|__O)?\s*(uint8_t|uint16_t|uint32_t|uint64_t)\s+(\w+)(?:\[(\w+)\])?\s*;",
                                        nested_stripped,
                                    )
                                    if not field_match:
                                        continue

                                    qualifier = field_match.group(1) or ""
                                    field_type = field_match.group(2)
                                    field_name = field_match.group(3)
                                    field_array_name = field_match.group(4)

                                    if field_name.startswith("RESERVED"):
                                        continue

                                    field_offset_match = re.search(
                                        r"offset:\s*0x([0-9a-fA-F]+)", nested_line_str
                                    )
                                    if field_offset_match:
                                        absolute_field_offset = int(
                                            field_offset_match.group(1), 16
                                        )
                                        field_offset = (
                                            absolute_field_offset - nested_offset
                                        )
                                    else:
                                        field_offset = 0

                                    if (
                                        qualifier not in ("__IO", "__I", "__O")
                                        and not field_offset_match
                                    ):
                                        continue

                                    description = extract_description(nested_line_str)
                                    reg_name = f"{nested_name}_{field_name}"
                                    reg_offset = (
                                        absolute_field_offset
                                        if field_offset_match
                                        else struct_base_offset + field_offset
                                    )

                                    item: Dict[str, Any] = {
                                        "name": reg_name,
                                        "byte_offset": reg_offset,
                                    }

                                    if struct_array_len > 1 and array_step:
                                        item["array"] = {
                                            "len": struct_array_len,
                                            "stride": array_step,
                                        }

                                    if description:
                                        item["description"] = description

                                    self.block_data["items"].append(item)

                            # Process top-level fields in the array struct
                            for top_line_str in top_level_fields:
                                top_stripped = top_line_str.strip()
                                if not top_stripped:
                                    continue

                                field_match = re.match(
                                    r"(__IO|__I|__O)?\s*(uint8_t|uint16_t|uint32_t|uint64_t)\s+(\w+)(?:\[(\w+)\])?\s*;",
                                    top_stripped,
                                )
                                if not field_match:
                                    continue

                                qualifier = field_match.group(1) or ""
                                field_type = field_match.group(2)
                                field_name = field_match.group(3)
                                field_array_name = field_match.group(4)

                                if field_name.startswith("RESERVED"):
                                    continue

                                field_offset_match = re.search(
                                    r"offset:\s*0x([0-9a-fA-F]+)", top_line_str
                                )
                                if field_offset_match:
                                    absolute_field_offset = int(
                                        field_offset_match.group(1), 16
                                    )
                                    field_offset = (
                                        absolute_field_offset - struct_base_offset
                                    )
                                else:
                                    continue

                                if (
                                    qualifier not in ("__IO", "__I", "__O")
                                    and not field_offset_match
                                ):
                                    continue

                                description = extract_description(top_line_str)
                                reg_offset = struct_base_offset + field_offset

                                item = {
                                    "name": f"{struct_member_name}_{field_name}",
                                    "byte_offset": reg_offset,
                                }

                                if struct_array_len > 1 and array_step:
                                    item["array"] = {
                                        "len": struct_array_len,
                                        "stride": array_step,
                                    }

                                if description:
                                    item["description"] = description

                                self.block_data["items"].append(item)

                        # Update offset after struct
                        if array_step and struct_array_len > 0:
                            current_offset = (
                                struct_base_offset + struct_array_len * array_step
                            )
                        else:
                            current_offset = struct_base_offset + 0x80  # Default step
                        break
                    else:
                        if outer_brace_depth > 0:
                            current_nested_body.append(nested_line)
                        else:
                            # Top-level field in the array struct
                            top_level_fields.append(nested_line)
                continue

            # Handle non-array struct: struct { ... } NAME;
            if line_stripped.startswith("}") and "{" not in line_stripped:
                struct_match = re.search(r"\}\s*(\w+)\s*;", line_stripped)
                if struct_match:
                    struct_name = struct_match.group(1)

                    # This closes a struct - but we need to know if it had nested structs
                    # For simple cases without nesting, we just skip (handled by field parsing)
                    continue

            # Parse regular field: __IO uint32_t NAME; or uint8_t RESERVED[N];
            field_match = re.match(
                r"(__IO|__I|__O)?\s*(uint8_t|uint16_t|uint32_t|uint64_t)\s+(\w+)(?:\[(\w+)\])?\s*;",
                line_stripped,
            )
            if not field_match:
                continue

            qualifier = field_match.group(1) or ""
            field_type = field_match.group(2)
            field_name = field_match.group(3)
            array_name = field_match.group(4)

            # Skip reserved fields
            if field_name.startswith("RESERVED"):
                # Track gap
                type_size = {"uint8_t": 1, "uint16_t": 2, "uint32_t": 4, "uint64_t": 8}[
                    field_type
                ]
                if array_name:
                    array_len = array_counts.get(array_name, 0)
                    current_offset += array_len * type_size
                else:
                    current_offset += type_size
                continue

            # Determine access - allow fields without qualifier if they have explicit offset info
            offset_match_here = re.search(r"offset:\s*0x([0-9a-fA-F]+)", line)
            if qualifier == "__IO":
                access = "read-write"
            elif qualifier == "__I":
                access = "read-only"
            elif qualifier == "__O":
                access = "write-only"
            elif offset_match_here:
                # Field has explicit offset, include it even without qualifier
                access = "read-write"  # Default to read-write for unqualified fields
            else:
                continue

            # Extract byte offset from comment
            byte_offset = current_offset
            offset_match = re.search(r"offset:\s*0x([0-9a-fA-F]+)", line)
            if offset_match:
                byte_offset = int(offset_match.group(1), 16)
                current_offset = byte_offset

            description = extract_description(line)

            item: Dict[str, Any] = {
                "name": field_name,
                "byte_offset": byte_offset,
            }

            # Handle arrays
            if array_name:
                array_len = array_counts.get(array_name, 0)
                if array_len > 0:
                    type_size = {
                        "uint8_t": 1,
                        "uint16_t": 2,
                        "uint32_t": 4,
                        "uint64_t": 8,
                    }[field_type]
                    item["array"] = {
                        "len": array_len,
                        "stride": type_size,
                    }
                    current_offset = byte_offset + array_len * type_size
            else:
                type_size = {"uint8_t": 1, "uint16_t": 2, "uint32_t": 4, "uint64_t": 8}[
                    field_type
                ]
                current_offset = byte_offset + type_size

            if description:
                item["description"] = description

            self.block_data["items"].append(item)

    def parse_bitfield_macros(self):
        """Extract MASK and SHIFT defines for bitfields."""
        # NXP uses _MASK and _SHIFT naming
        mask_defs: Dict[str, int] = {}
        shift_defs: Dict[str, int] = {}
        descriptions: Dict[str, str] = {}

        # Find all _MASK defines
        for match in re.finditer(
            r"#define\s+(\w+)_MASK\s+\((0x[0-9a-fA-F]+)U?\)", self.content
        ):
            name = match.group(1)
            mask_val = safe_hex_to_int(match.group(2))
            if mask_val is not None:
                mask_defs[name] = mask_val

                # Extract description from preceding comment
                line_start = self.content.rfind("\n", 0, match.start()) + 1
                line_end = match.end()
                context = self.content[max(0, line_start - 200) : line_end + 100]

                # Look for /*! FIELDNAME - description */ comment
                desc_match = re.search(r"/\*!\s*(\w+)\s*-\s*(.*?)\s*\*/", context)
                if desc_match:
                    descriptions[name] = desc_match.group(2).strip()
                else:
                    descriptions[name] = ""

        # Find all _SHIFT defines
        for match in re.finditer(r"#define\s+(\w+)_SHIFT\s+\((\d+)U?\)", self.content):
            name = match.group(1)
            shift_val = int(match.group(2))
            shift_defs[name] = shift_val

        # Group by register (PERIPH_REG)
        fieldsets_dict: Dict[str, List[Dict[str, Any]]] = {}

        for field_name, mask_val in mask_defs.items():
            if field_name not in shift_defs:
                continue

            bit_offset = shift_defs[field_name]
            description = descriptions.get(field_name, "")

            # Extract register name (PERIPH_REG_FIELD → PERIPH_REG)
            parts = field_name.rsplit("_", 1)
            if len(parts) != 2:
                continue

            reg_name = parts[0]
            field_short = parts[1]

            # Skip purely numeric field names (array indices, not real fields)
            if field_short.isdigit():
                continue

            if reg_name not in fieldsets_dict:
                fieldsets_dict[reg_name] = []

            # NXP masks are pre-shifted; unshift before calculating bit size
            unshifted_mask = mask_val >> bit_offset
            bit_size = calc_bit_size_from_mask(unshifted_mask)

            field: Dict[str, Any] = {
                "name": field_short,
                "bit_offset": bit_offset,
                "bit_size": bit_size,
            }

            if description:
                field["description"] = description

            fieldsets_dict[reg_name].append(field)

        # Convert to fieldsets
        for reg_name, fields in fieldsets_dict.items():
            # reg_name already includes block name (e.g., LPUART_VERID)
            # so use it directly
            self.fieldsets[reg_name] = {
                "description": "",
                "fields": self._prune_overlapping_fields(fields),
            }

    def parse(self):
        """Parse the peripheral file."""
        self.parse_struct()
        self.parse_bitfield_macros()

    def emit_yaml(self):
        """Write YAML output."""
        if not self.block_data["items"]:
            return

        # Link fieldsets to items
        for item in self.block_data["items"]:
            reg_name = item["name"]
            # Fieldsets are named BLOCKNAME_REGNAME
            fieldset_name = f"{self.block_name}_{reg_name}"
            # Check both with and without block name prefix since NXP stores them as BLOCKNAME_REGNAME
            if fieldset_name in self.fieldsets:
                item["fieldset"] = fieldset_name

        self.block_data["items"] = self._prune_overlapping_items(
            self.block_data["items"]
        )

        data = {f"block/{self.block_name}": self.block_data}
        # fieldsets are already named BLOCKNAME_REGNAME, so add them with fieldset/ prefix
        data.update({f"fieldset/{name}": fs for name, fs in self.fieldsets.items()})

        yaml_path = (
            OUTPUT_DIR / "peri" / "imx93" / f"{self.block_name.lower()}_v1.yaml"
        )
        write_yaml(data, yaml_path)
        print(f"Wrote {yaml_path}")


class NXPCommonParser:
    """Parse *_COMMON.h file for IRQs and base addresses."""

    def __init__(self, common_header_path: Path):
        self.common_header_path = common_header_path
        with open(common_header_path) as f:
            self.content = f.read()

        self.irqs: List[Dict[str, Any]] = []
        self.instances: Dict[str, Tuple[str, int]] = {}  # name → (type, address)

    def parse_irq_enum(self):
        """Extract IRQ enum."""
        # Find enum block
        match = re.search(
            r"typedef\s+enum\s+IRQn\s*\{(.*?)\}\s*(?:IRQn_Type|IRQn)\s*;",
            self.content,
            re.DOTALL,
        )
        if not match:
            return

        enum_body = match.group(1)

        for line in enum_body.split("\n"):
            line = line.strip()
            if not line or line.startswith("/*"):
                continue

            # Match: NAME_IRQn = VALUE, /*!< description */
            entry_match = re.match(
                r"(\w+_IRQn)\s*=\s*(-?\d+)\s*,?\s*(?:/\*\*<?\s*(.*?)\s*\*/)?", line
            )
            if not entry_match:
                continue

            name_with_suffix = entry_match.group(1)
            value = int(entry_match.group(2))
            description = entry_match.group(3) or ""

            # Skip auxiliary constants and reserved entries
            if "NotAvail" in name_with_suffix or "RESERVED" in name_with_suffix.upper():
                continue

            # Skip negative values (ARM core exceptions)
            if value < 0:
                continue

            name = strip_suffix(name_with_suffix, "_IRQn")
            self.irqs.append(
                {
                    "name": name,
                    "value": value,
                    "description": description.strip(),
                }
            )

    def parse_base_addresses(self):
        """Extract base addresses and instance casts."""
        # Find _BASE and _BASE_NS defines
        bases: Dict[str, int] = {}

        for match in re.finditer(
            r"#define\s+(\w+_BASE(?:_NS)?)\s+\((0x[0-9a-fA-F]+)u?\)", self.content
        ):
            name = match.group(1)
            addr = safe_hex_to_int(match.group(2))
            if addr is not None:
                bases[name] = addr

        # Find instance casts: NAME or NAME_NS -> (TYPE_Type *) BASE
        for match in re.finditer(
            r"#define\s+(\w+)(?:_NS)?\s+\(\((\w+)_Type\s*\*\)\s*(\w+_BASE(?:_NS)?)\)",
            self.content,
        ):
            instance_name = match.group(1)
            block_type = match.group(2)
            base_macro = match.group(3)

            # Resolve base address - try _NS variant first, then fallback to non-NS
            base_addr = None
            if base_macro in bases:
                base_addr = bases[base_macro]
            elif base_macro.endswith("_NS"):
                # Try without _NS suffix
                base_no_ns = base_macro[:-3]  # Remove _NS
                if f"{base_no_ns}_BASE_NS" in bases:
                    base_addr = bases[f"{base_no_ns}_BASE_NS"]
                elif f"{base_no_ns}_BASE" in bases:
                    base_addr = bases[f"{base_no_ns}_BASE"]
            else:
                # Try with _NS suffix
                base_with_ns = f"{base_macro}_NS"
                if base_with_ns in bases:
                    base_addr = bases[base_with_ns]

            if base_addr is not None:
                # Normalize instance name (remove _NS suffix for canonical name)
                canonical_name = instance_name.replace("_NS", "")
                self.instances[canonical_name] = (block_type, base_addr)

    def parse(self):
        """Parse the common file."""
        self.parse_irq_enum()
        self.parse_base_addresses()


class NXPChipParser:
    """Parse chip header file to get included peripherals."""

    def __init__(self, chip_header_path: Path, common_parser: NXPCommonParser):
        self.chip_header_path = chip_header_path
        with open(chip_header_path) as f:
            self.content = f.read()

        self.common = common_parser

        # Extract chip name and core from path
        match = re.search(r"(MIMX\d+)_(\w+)\.h", chip_header_path.name)
        if match:
            self.chip_name = match.group(1)
            self.core = match.group(2).lower()
        else:
            self.chip_name = "UNKNOWN"
            self.core = "cm33"

    def get_included_peripherals(self) -> List[str]:
        """Extract list of included PERI_*.h files."""
        peripherals = []
        for match in re.finditer(r'#include\s+"PERI_(\w+)\.h"', self.content):
            peripherals.append(match.group(1))
        return peripherals

    def emit_json(self, peri_blocks: Dict[str, str]):
        """Write chip descriptor JSON."""
        # Determine family from chip name
        family = (
            "imx93"
            if "9352" in self.chip_name
            or "9351" in self.chip_name
            or "9301" in self.chip_name
            else "imx95"
        )

        # Build peripherals list from instances
        peripherals = []
        for instance_name, (block_type, base_addr) in self.common.instances.items():
            peripherals.append(
                {
                    "name": instance_name,
                    "base_address": f"0x{base_addr:08x}",
                    "block": f"{block_type.lower()}/{block_type}",
                }
            )

        chip_json = {
            "name": self.chip_name,
            "family": family,
            "core": self.core,
            "irqs": self.common.irqs,
            "peripherals": sorted(peripherals, key=lambda x: x["name"]),
        }

        # The bundled file is keyed by family + core, with the canonical `mimx`
        # prefix: e.g. `mimx95_cm7.json`.
        json_path = OUTPUT_DIR / "chip" / f"m{family}_{self.core}.json"
        write_json(chip_json, json_path)
        print(f"Wrote {json_path}")


def main():
    parser = argparse.ArgumentParser(description="Parse NXP MCUX CMSIS headers")
    parser.add_argument(
        "--family", choices=list(FAMILY_REPRESENTATIVES.keys()), help="Family to parse"
    )
    parser.add_argument("--all", action="store_true", help="Parse all families")
    args = parser.parse_args()

    families_to_parse = (
        FAMILY_REPRESENTATIVES.keys()
        if args.all
        else ([args.family] if args.family else [])
    )

    if not families_to_parse:
        print("Error: specify --family or --all")
        sys.exit(1)

    for family in families_to_parse:
        family_dir = DATA_DIR / family
        if not family_dir.exists():
            print(f"Warning: {family_dir} not found")
            continue

        # Parse all PERI_*.h files
        print(f"Parsing {family}...")

        # Determine which subfamily for output (imx93 or imx95)
        subfamily = family.split("_")[0]
        output_subdir = OUTPUT_DIR / "peri" / subfamily

        peri_files = sorted(family_dir.glob("PERI_*.h"))
        for peri_file in peri_files:
            peri_parser = NXPPeripheralParser(peri_file)
            peri_parser.parse()

            # Update output dir for this family
            peri_parser.block_data["items"]  # Check if has content

            yaml_path = output_subdir / f"{peri_parser.block_name.lower()}_v1.yaml"
            if peri_parser.block_data["items"]:
                # Link fieldsets
                for item in peri_parser.block_data["items"]:
                    reg_name = item["name"]
                    # fieldsets stored with full name BLOCKNAME_REGNAME
                    fieldset_name = f"{peri_parser.block_name}_{reg_name}"
                    if fieldset_name in peri_parser.fieldsets:
                        item["fieldset"] = fieldset_name

                peri_parser.block_data["items"] = peri_parser._prune_overlapping_items(
                    peri_parser.block_data["items"]
                )

                data = {f"block/{peri_parser.block_name}": peri_parser.block_data}
                data.update(
                    {
                        f"fieldset/{name}": fs
                        for name, fs in peri_parser.fieldsets.items()
                    }
                )

                write_yaml(data, yaml_path)
                print(f"Wrote {yaml_path}")

        # Parse chip descriptor(s)
        chip_header, common_header_name = FAMILY_REPRESENTATIVES[family]
        chip_header_path = family_dir / f"{chip_header}.h"
        common_header_path = family_dir / common_header_name

        if chip_header_path.exists() and common_header_path.exists():
            common_parser = NXPCommonParser(common_header_path)
            common_parser.parse()

            chip_parser = NXPChipParser(chip_header_path, common_parser)
            chip_parser.emit_json({})


if __name__ == "__main__":
    main()
