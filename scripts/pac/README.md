# CMSIS Header Parsers

Python scripts to parse CMSIS C headers into chiptool IR YAML and chip descriptor JSON files.

## Reproducible regeneration

Every committed PAC output is exactly what `just pac-regen` produces from the pinned
submodules, and `just pac-verify` fails when it is not. The pipeline, in order:

1. Apply vendor-header patches (`patches/<submodule>/*.diff`), stage the headers into
   `./data` (`prepare.sh`), and revert the patches.
2. Rewrite `crates/consortium-data/database/{chip,peri}/` from scratch with
   `parse_stm32.py --all` and `parse_nxp.py --all`.
3. Apply IR corrections (`patches/ir/*.diff`), before anything reads the chip dumps.
4. `gen_interrupts.py` and `gen_peripherals.py` write the PAC interrupt tables and the
   HAL singleton maps; `gen_blocks.py` runs the pinned chiptool over the `BLOCKS` map.
5. Format the Rust (`rustfmt`), apply corrections to it (`patches/generated/*.diff`),
   and format the IR with `vp fmt`.

Correct the parsers where you can. Where you cannot — a register the header omits, an
API the HAL relies on that the IR folds away — add a patch whose header says why. A
patch is the diff from the raw pipeline output to the committed file, so regenerate,
edit, and capture it with `diff -u --label a/<path> --label b/<path>`.

`gen_blocks.py` refuses to run unless chiptool is the revision pinned by
`just setup-chiptool`, because chiptool stamps its revision into every file.

## Scripts

### `parse_stm32.py`

Parses STM32MP2 CMSIS headers (monolithic format).

**Usage:**

```bash
python scripts/parse_stm32.py --family stm32mp21_cm33   # Parse one family
python scripts/parse_stm32.py --all                     # Parse all families
```

**Supported families:**

- `stm32mp21_cm33` → stm32mp215fxx_cm33.h
- `stm32mp23_cm33` → stm32mp235fxx_cm33.h
- `stm32mp25_cm33` → stm32mp257fxx_cm33.h
- `stm32mp25_cm0p` → stm32mp257fxx_cm0.h

**Parsing phases:**

1. Strip preprocessor conditionals (#if/#else/#endif)
2. Extract IRQ enum entries
3. Parse peripheral struct definitions
4. Parse bitfield macro definitions (\_Pos and \_Msk)
5. Extract base addresses and instance casts
6. Link structs to fieldsets
7. Emit YAML and JSON outputs

Fields that `chiptool check` would reject as overlapping are resolved while linking:

- A mask with gaps (`TIM_SMCR_TS` is bits 4-6 and 20-21) becomes a split field, whose
  `bit_offset` is a list of `{start, end}` ranges.
- Two names for exactly the same bits (an alias such as `PPSCTRL`/`PPSCMD`, or a value
  constant such as `VLC_VLANTAGREPLACE`) keep only the one the header defines first.
- A capture/compare mode register (`TIM_CCMRx`) keeps its output-compare fields, and
  its input-capture fields become a separate `<register>_INPUT` fieldset.

Errors in the header itself are fixed by a patch under `patches/STM32CubeMP2/`.

### `parse_nxp.py`

Parses NXP MCUX CMSIS headers (split format: PERI\_\*.h + COMMON.h).

**Usage:**

```bash
python scripts/parse_nxp.py --family imx93_cm33    # Parse one family
python scripts/parse_nxp.py --all                  # Parse all families
```

**Supported families:**

- `imx93_cm33` → MIMX9352_cm33 chip + MIMX9352_cm33_COMMON.h
- `imx95_cm7` → MIMX9596_cm7 chip + MIMX9596_cm7_COMMON.h

**Parsing phases:**

1. Parse all PERI\_\*.h files for register definitions
2. Parse bitfield macro definitions (\_MASK and \_SHIFT)
3. Parse COMMON.h for IRQ enum
4. Extract base addresses and instance mappings
5. Link chip includes to peripherals
6. Emit YAML and JSON outputs

### `gen_blocks.py`

Generates each PAC register-block module with chiptool from the IR file named in its
`BLOCKS` map, then runs `rustfmt`. Modules listed in `HAND_MAINTAINED` (STM32MP2
`timer`, i.MX 9 `dma`) are skipped; any other module missing from the map is an error.

### `common.py`

Shared utilities:

- Hex number parsing and evaluation
- Arithmetic expression evaluation
- Bitfield mask analysis
- YAML/JSON file writing
- C comment extraction

## Output Format

### Register YAML (`crates/consortium-data/database/peri/imx9/<periph>_<suffix>.yaml`)

Chiptool IR format with blocks and fieldsets. Shared i.MX9 blocks use `_v1`;
chip-specific blocks use `_mimx93` or `_mimx95`, and repeated numbered blocks
use `_n` in the stem:

```yaml
block/LPUART:
  description: Low Power UART
  items:
    - name: VERID
      byte_offset: '0x0'
      access: read-only
      fieldset: LPUART_VERID

    - name: DATA
      byte_offset: '0x1c'
      access: read-write
      array:
        len: 128
        stride: 4
      fieldset: LPUART_DATA

fieldset/LPUART_VERID:
  fields:
    - name: FEATURE
      bit_offset: 0
      bit_size: 16
      description: Feature Identification Number
```

### Chip Descriptors (`crates/consortium-data/database/chip/<CHIP>.json`)

Chip metadata with IRQs and peripheral mappings:

```json
{
  "name": "MIMX9352",
  "family": "imx93",
  "core": "cm33",
  "irqs": [
    { "name": "DAP", "value": 1, "description": "DAP interrupt" },
    { "name": "CAN1", "value": 8, "description": "CAN1 interrupt" }
  ],
  "peripherals": [
    { "name": "LPUART1", "base_address": "0x44380000", "block": "lpuart/LPUART" },
    { "name": "LPI2C1", "base_address": "0x44340000", "block": "lpi2c/LPI2C" }
  ]
}
```

## Statistics

- **STM32MP registers:** 90 YAML files (4 families)
- **i.MX93 registers:** 72 YAML files
- **i.MX95 registers:** 227 YAML files
- **Chip descriptors:** 6 JSON files
- **Total:** 395 files

### Chip Coverage

| Chip      | Family    | Core | IRQs | Peripherals |
| --------- | --------- | ---- | ---- | ----------- |
| MIMX9352  | imx93     | cm33 | 202  | 154         |
| MIMX9596  | imx95     | cm7  | 239  | 393         |
| stm32mp21 | stm32mp21 | cm33 | 217  | 217         |
| stm32mp23 | stm32mp23 | cm33 | 245  | 233         |
| stm32mp25 | stm32mp25 | cm33 | 265  | 251         |
| stm32mp25 | stm32mp25 | cm0p | 26   | 0\*         |

\* cm0p variant in the header set doesn't define peripheral instances

## Requirements

- Python 3.14+ (matches the workspace `pyproject.toml` `requires-python`; run via
  `uv run --with pyyaml python scripts/pac/parse_stm32.py --all`)
- PyYAML
- Standard library: re, json, pathlib

## Development Notes

### Key Parsing Challenges Handled

1. **STM32 preprocessor conditionals**: Both Secure (\_S) and Non-Secure (\_NS) variants present; script merges both branches and prefers \_NS addresses.

2. **NXP address resolution**: Base address defines may reference other macros; script builds a symbol table for recursive evaluation.

3. **Bitfield field size calculation**: Both STM32 (\_Pos/\_Msk) and NXP (\_SHIFT/\_MASK) patterns require computing bit width from mask values.

4. **Array field handling**: Symbolic array counts (e.g., `TIMEOUT[LPUART_TIMEOUT_COUNT]`) resolved from local #define declarations.

5. **Reserved field tracking**: Gaps in register space tracked through byte_offset comments or explicit RESERVED fields.

### Edge Cases

- Registers with no bitfield macros are still emitted (fieldset references omitted)
- A few structs glue their qualifier onto `TypeDef` without an underscore (e.g.
  `FDCAN_GlobalTypeDef`); `normalize_block_name` maps the recognized ones to a canonical
  block name (`FDCAN`) so the block and its `FDCAN1..3` instances are emitted, and skips
  unrecognized variants
- Instance names with \_NS suffix normalized to canonical names in chip descriptors
- Multi-line struct definitions and comments handled via regex DOTALL mode
- Hex expressions with UL/u suffixes properly stripped and evaluated
