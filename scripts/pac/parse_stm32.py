#!/usr/bin/env python3
"""Parse STM32MP CMSIS headers into chiptool IR YAML and chip JSON."""

import re
import sys
from pathlib import Path
from typing import Dict, List, Any, Tuple, Set
import argparse

from common import (
    safe_hex_to_int,
    evaluate_expr,
    calc_bit_size_from_mask,
    write_yaml,
    write_json,
    extract_description,
    strip_suffix,
)


DATA_DIR = Path(__file__).resolve().parents[2] / "data"
OUTPUT_DIR = (
    Path(__file__).resolve().parents[2] / "crates" / "consortium-data" / "database"
)

FAMILY_REPRESENTATIVES = {
    "stm32mp21_cm33": "stm32mp215fxx_cm33.h",
    "stm32mp23_cm33": "stm32mp235fxx_cm33.h",
    "stm32mp25_cm33": "stm32mp257fxx_cm33.h",
    "stm32mp25_cm0p": "stm32mp257fxx_cm0.h",
}

# A few CMSIS structs glue their qualifier straight onto `TypeDef` instead of
# using the usual `_TypeDef` suffix (e.g. `FDCAN_GlobalTypeDef`,
# `IPCC_CommonTypeDef`), so the generic `_TypeDef` matcher skips them and their
# instances never reach the chip descriptor. Map each malformed type name to the
# canonical block name so it is emitted like every other peripheral. FDCAN's
# bitfield macros are prefixed `FDCAN_<REG>`, so the block must be `FDCAN` (not
# `FDCAN_Global`) for register/fieldset linking to resolve. Types absent from
# this map keep the historical behavior of being skipped.
CAMEL_TYPEDEF_BLOCKS = {
    "FDCAN_Global": "FDCAN",
}


def normalize_block_name(type_name: str) -> str | None:
    """Map a CMSIS `*TypeDef` struct/cast name to its canonical block name.

    Returns the block name for the well-formed `<Block>_TypeDef` spelling and
    for the malformed CamelCase variants listed in `CAMEL_TYPEDEF_BLOCKS`, or
    `None` for an unrecognized CamelCase variant that should be skipped.
    """
    if type_name.endswith("_TypeDef"):
        return strip_suffix(type_name, "_TypeDef")
    return CAMEL_TYPEDEF_BLOCKS.get(strip_suffix(type_name, "TypeDef"))


class STM32HeaderParser:
    def __init__(self, header_path: Path):
        self.header_path = header_path
        with open(header_path) as f:
            self.content = f.read()

        # Extract family and core from path
        parts = header_path.parent.name.split("_")
        self.family_prefix = parts[0]  # stm32mp25, stm32mp21, etc.
        self.core = parts[1]  # cm33, cm0p

        # Extract chip variant from filename (e.g., stm32mp215fxx → mp215f)
        match = re.search(r"stm32(mp\d[a-z]?)xx", header_path.stem)
        self.chip_variant = match.group(1) if match else "unknown"

        self.irqs: List[Dict[str, Any]] = []
        self.blocks: Dict[str, Dict[str, Any]] = {}
        self.fieldsets: Dict[str, Dict[str, Any]] = {}
        self.peripherals: List[Dict[str, Any]] = []

    @staticmethod
    def _is_aggregate_alias(field_name: str, field_names: Set[str]) -> bool:
        """Return true for CMSIS aggregate aliases covered by concrete fields."""
        if field_name.endswith("x"):
            return True
        if field_name == "LPUART":
            return True
        if field_name == "CCC" and {"RNW", "ADD"}.issubset(field_names):
            return True
        return any(
            other.startswith(field_name) and other[len(field_name) :].isdigit()
            for other in field_names
            if other != field_name
        )

    @classmethod
    def _prune_overlapping_fields(
        cls, fields: List[Dict[str, Any]]
    ) -> List[Dict[str, Any]]:
        """Drop aggregate/alternate CMSIS aliases that cannot be represented in chiptool IR."""
        field_names = {field["name"] for field in fields}
        remove: Set[int] = set()

        for i, field in enumerate(fields):
            field_mask = field.get("_mask", 0)
            if not field_mask:
                continue

            for j, other in enumerate(fields):
                if i == j:
                    continue

                other_mask = other.get("_mask", 0)
                if not other_mask or not (field_mask & other_mask):
                    continue

                if cls._is_aggregate_alias(field["name"], field_names):
                    remove.add(i)
                    break

                if cls._is_aggregate_alias(other["name"], field_names):
                    continue

                same_offset = field["bit_offset"] == other["bit_offset"]
                narrower = field["bit_size"] < other["bit_size"]
                if same_offset and narrower:
                    remove.add(i)
                    break

        pruned = []
        for i, field in enumerate(fields):
            if i in remove:
                continue
            field.pop("_mask", None)
            pruned.append(field)

        pruned.sort(key=lambda f: (f["bit_offset"], f["name"]))
        return pruned

    def strip_preprocessor(self):
        """Remove #if/#else/#endif conditionals, keeping all branches."""
        # Simple approach: remove #if/#endif lines but keep content
        lines = []
        skip_depth = 0
        for line in self.content.split("\n"):
            stripped = line.strip()
            if stripped.startswith("#if"):
                skip_depth += 1
                continue
            elif stripped.startswith("#elif"):
                skip_depth = max(0, skip_depth - 1)
                skip_depth += 1
                continue
            elif stripped.startswith("#else"):
                # For #else, we want to keep the else branch
                skip_depth = max(0, skip_depth - 1)
                continue
            elif stripped.startswith("#endif"):
                skip_depth = max(0, skip_depth - 1)
                continue

            lines.append(line)

        self.content = "\n".join(lines)

    def parse_irq_enum(self):
        """Extract IRQ enum entries."""
        # Find enum block
        match = re.search(
            r"typedef\s+enum\s*\{(.*?)\}\s*IRQn_Type\s*;", self.content, re.DOTALL
        )
        if not match:
            return

        enum_body = match.group(1)

        for line in enum_body.split("\n"):
            line = line.strip()
            if not line or line.startswith("/*") or line.startswith("//"):
                continue

            # Match: NAME_IRQn = VALUE, /*!< description */
            entry_match = re.match(
                r"(\w+_IRQn)\s*=\s*(-?\d+)\s*,?\s*(?:/\*!<\s*(.*?)\s*\*/)?", line
            )
            if not entry_match:
                continue

            name_with_suffix = entry_match.group(1)
            value = int(entry_match.group(2))
            description = entry_match.group(3) or ""

            # Skip reserved entries
            if "RESERVED" in name_with_suffix.upper() or "MAX_IRQ" in name_with_suffix:
                continue

            # Skip negative values (ARM core exceptions)
            if value < 0:
                continue

            name = strip_suffix(name_with_suffix, "_IRQn")
            self.irqs.append(
                {"name": name, "value": value, "description": description.strip()}
            )

    def parse_peripheral_structs(self):
        """Extract typedef struct definitions."""
        # Find all struct blocks. Accept both the usual `<Block>_TypeDef`
        # spelling and CamelCase variants like `FDCAN_GlobalTypeDef` (see
        # `normalize_block_name`).
        pattern = r"typedef\s+struct\s*\{(.*?)\}\s*(\w+TypeDef)\s*;"

        for match in re.finditer(pattern, self.content, re.DOTALL):
            struct_body = match.group(1)
            struct_name_with_suffix = match.group(2)
            struct_name = normalize_block_name(struct_name_with_suffix)
            if struct_name is None:
                continue

            items = []
            current_offset = 0

            for line in struct_body.split("\n"):
                line_stripped = line.strip()
                if (
                    not line_stripped
                    or line_stripped.startswith("/*")
                    or line_stripped.startswith("//")
                ):
                    continue

                # Parse field declaration
                # Patterns:
                # __IO uint32_t FIELD;
                # __IO uint32_t FIELD[N];
                # uint32_t RESERVED[N];

                # Extract qualifier and type
                qualifier_match = re.match(
                    r"(__IO|__I|__O)?\s*(uint32_t|uint8_t|uint16_t|uint64_t)\s+(\w+)(?:\[(\d+)\])?\s*;",
                    line_stripped,
                )
                if not qualifier_match:
                    continue

                qualifier = qualifier_match.group(1) or ""
                field_type = qualifier_match.group(2)
                field_name = qualifier_match.group(3)
                array_len = qualifier_match.group(4)

                # Skip reserved fields
                if field_name.startswith("RESERVED"):
                    # Track the gap in offset
                    if array_len:
                        type_size = {
                            "uint8_t": 1,
                            "uint16_t": 2,
                            "uint32_t": 4,
                            "uint64_t": 8,
                        }[field_type]
                        current_offset += int(array_len) * type_size
                    else:
                        type_size = {
                            "uint8_t": 1,
                            "uint16_t": 2,
                            "uint32_t": 4,
                            "uint64_t": 8,
                        }[field_type]
                        current_offset += type_size
                    continue

                # Only fields with an access qualifier are real registers.
                if qualifier not in ("__IO", "__I", "__O"):
                    continue

                # Extract byte offset from comment
                byte_offset = current_offset
                offset_match = re.search(r"Address offset:\s*0x([0-9a-fA-F]+)", line)
                if offset_match:
                    byte_offset = int(offset_match.group(1), 16)
                    current_offset = byte_offset

                # Get description
                description = extract_description(line)

                item: Dict[str, Any] = {
                    "name": field_name,
                    "byte_offset": byte_offset,
                }

                if array_len:
                    type_size = {
                        "uint8_t": 1,
                        "uint16_t": 2,
                        "uint32_t": 4,
                        "uint64_t": 8,
                    }[field_type]
                    item["array"] = {
                        "len": int(array_len),
                        "stride": type_size,
                    }
                    current_offset = byte_offset + int(array_len) * type_size
                else:
                    type_size = {
                        "uint8_t": 1,
                        "uint16_t": 2,
                        "uint32_t": 4,
                        "uint64_t": 8,
                    }[field_type]
                    current_offset = byte_offset + type_size

                if description:
                    item["description"] = description

                items.append(item)

            if items:
                self.blocks[struct_name] = {
                    "description": "",
                    "items": items,
                }

    def parse_bitfield_macros(self):
        """Extract bitfield definitions from _Pos and _Msk macros."""
        # Collect all _Pos defines
        pos_defs: Dict[str, int] = {}
        msk_defs: Dict[str, int] = {}
        descriptions: Dict[str, str] = {}

        # Find all _Pos and _Msk defines
        for match in re.finditer(r"#define\s+(\w+)_Pos\s+\((\d+)U?\)", self.content):
            name = match.group(1)
            pos_defs[name] = int(match.group(2))

        for match in re.finditer(
            r"#define\s+(\w+)_Msk\s+\((0x[0-9a-fA-F]+)UL\s*<<\s*\w+_Pos\)", self.content
        ):
            name = match.group(1)
            mask_val = safe_hex_to_int(match.group(2))
            if mask_val is not None:
                msk_defs[name] = mask_val

        # Extract descriptions from alias macros (e.g., #define FIELD FIELD_Msk /*!< description */)
        for field_name in msk_defs.keys():
            # Look for: #define FIELDNAME FIELDNAME_Msk /*!< description */
            alias_pattern = rf"#define\s+{re.escape(field_name)}\s+{re.escape(field_name)}_Msk\s+(.*?\*/)"
            alias_match = re.search(alias_pattern, self.content)
            if alias_match:
                descriptions[field_name] = extract_description(
                    f"/* {alias_match.group(1)}"
                )
            else:
                # Fall back to _Msk line (may just be hex value like /*!< 0x000000FF */)
                msk_pattern = (
                    rf"#define\s+{re.escape(field_name)}_Msk\s+.*?(\/\*.*?\*\/)"
                )
                msk_match = re.search(msk_pattern, self.content, re.DOTALL)
                if msk_match:
                    descriptions[field_name] = extract_description(msk_match.group(1))

        # Group by register (PERIPH_REG)
        fieldsets_dict: Dict[str, List[Dict[str, Any]]] = {}

        for field_name, bit_offset in pos_defs.items():
            if field_name not in msk_defs:
                continue

            mask_val = msk_defs[field_name]
            description = descriptions.get(field_name, "")

            # Extract register name from field_name (PERIPH_REG_FIELD → PERIPH_REG)
            # Split on last underscore before the field name
            parts = field_name.rsplit("_", 1)
            if len(parts) == 2:
                reg_name = parts[0]
            else:
                continue

            if reg_name not in fieldsets_dict:
                fieldsets_dict[reg_name] = []

            # Calculate bit size from unshifted mask
            bit_size = calc_bit_size_from_mask(mask_val)

            # Some registers (e.g. GPIO_ADVCFGRL/H) name each per-pin nibble with a
            # bare number (macro `GPIO_ADVCFGRL_0_Pos`), which cannot be a Rust ident.
            # Prefix digit-leading field names with the register's last token so the
            # generated PAC compiles (e.g. `0` -> `ADVCFGRL0`).
            field_ident = parts[1]
            if field_ident and field_ident[0].isdigit():
                field_ident = f"{reg_name.rsplit('_', 1)[-1]}{field_ident}"

            field: Dict[str, Any] = {
                "name": field_ident,
                "bit_offset": bit_offset,
                "bit_size": bit_size,
                "_mask": mask_val << bit_offset,
            }

            if description:
                field["description"] = description

            fieldsets_dict[reg_name].append(field)

        # Convert to fieldsets
        for reg_name, fields in fieldsets_dict.items():
            self.fieldsets[reg_name] = {
                "description": "",
                "fields": self._prune_overlapping_fields(fields),
            }

    def parse_base_addresses_and_instances(self):
        """Extract base addresses and instance casts."""
        # Build symbol table for address resolution
        symbols: Dict[str, int] = {}

        # Known anchors
        symbols["PERIPH_BASE_NS"] = 0x40000000
        symbols["MPU_AXI_BUS_MEMORY_BASE"] = 0x50000000

        # Parse all _BASE_NS defines
        for match in re.finditer(r"#define\s+(\w+)_BASE_NS\s+\((.*?)\)", self.content):
            name = match.group(1)
            expr = match.group(2)
            val = evaluate_expr(expr, symbols)
            if val is not None:
                symbols[f"{name}_BASE_NS"] = val

        # Parse instance casts (NAME_NS -> (TYPE_TypeDef *) NAME_BASE_NS)
        instances: Dict[str, Tuple[str, int]] = {}

        for match in re.finditer(
            r"#define\s+(\w+)_NS\s+\(\((\w+TypeDef)\s*\*\)\s*(\w+_BASE_NS)\)",
            self.content,
        ):
            instance_name = match.group(1)
            block_type = normalize_block_name(match.group(2))
            base_macro = match.group(3)

            if block_type is not None and base_macro in symbols:
                base_addr = symbols[base_macro]
                instances[instance_name] = (block_type, base_addr)

        # Build peripherals list from instances
        for instance_name, (block_type, base_addr) in instances.items():
            self.peripherals.append(
                {
                    "name": instance_name,
                    "base_address": f"0x{base_addr:08x}",
                    "block": f"{block_type.lower()}/{block_type}",
                }
            )

    def link_blocks_to_fieldsets(self):
        """Link block items to their fieldsets."""
        for block_name, block_data in self.blocks.items():
            for item in block_data["items"]:
                reg_name = item["name"]
                fieldset_name = f"{block_name}_{reg_name}"

                if fieldset_name in self.fieldsets:
                    item["fieldset"] = fieldset_name

    def emit_outputs(self):
        """Write YAML and JSON files."""
        # Write register YAMLs
        for block_name, block_data in self.blocks.items():
            yaml_path = (
                OUTPUT_DIR / "peri" / "stm32mp" / f"{block_name.lower()}_v1.yaml"
            )
            data = {f"block/{block_name}": block_data}

            # Add corresponding fieldsets
            for item in block_data.get("items", []):
                if "fieldset" in item:
                    fs_name = item["fieldset"]
                    if fs_name in self.fieldsets:
                        data[f"fieldset/{fs_name}"] = self.fieldsets[fs_name]

            write_yaml(data, yaml_path)
            print(f"Wrote {yaml_path}")

        # Write chip JSON
        chip_name = f"{self.family_prefix}_{self.chip_variant}_{self.core}"
        chip_json = {
            "name": chip_name,
            "family": self.family_prefix,
            "core": self.core,
            "irqs": self.irqs,
            "peripherals": sorted(self.peripherals, key=lambda x: x["name"]),
        }

        # The bundled file is keyed by family + core (the chip-variant segment is
        # dropped): e.g. `stm32mp25_cm33.json`.
        json_path = OUTPUT_DIR / "chip" / f"{self.family_prefix}_{self.core}.json"
        write_json(chip_json, json_path)
        print(f"Wrote {json_path}")

    def parse(self):
        """Run full parsing pipeline."""
        self.strip_preprocessor()
        self.parse_irq_enum()
        self.parse_peripheral_structs()
        self.parse_bitfield_macros()
        self.parse_base_addresses_and_instances()
        self.link_blocks_to_fieldsets()
        self.emit_outputs()


def main():
    parser = argparse.ArgumentParser(description="Parse STM32MP CMSIS headers")
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
        header_file = FAMILY_REPRESENTATIVES[family]
        header_path = DATA_DIR / family / header_file

        if not header_path.exists():
            print(f"Warning: {header_path} not found")
            continue

        print(f"Parsing {family}...")
        p = STM32HeaderParser(header_path)
        p.parse()


if __name__ == "__main__":
    main()
