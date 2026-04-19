# Testing and CI Guide

## Quick Commands

All testing commands are available via `just`:

```bash
# Parse CMSIS headers
just parse-stm32      # Parse STM32MP families
just parse-nxp        # Parse NXP families
just parse-all        # Parse both STM32 and NXP

# Run tests
just test             # Run custom validation checks
just validate         # Run chiptool validation
just summary          # Show validation summary

# Full pipeline
just ci               # Parse all + test + validate
```

## Manual Testing

### Parse CMSIS Headers

```bash
# Parse STM32MP families
python scripts/parse_stm32.py --all

# Parse NXP families
python scripts/parse_nxp.py --all

# Parse a specific family
python scripts/parse_stm32.py --family stm32mp25_cm33
python scripts/parse_nxp.py --family imx93_cm33
```

Output files:
- `output/registers/<periph>_v1.yaml` - Register definitions in chiptool IR format
- `output/chips/<chip>.json` - Chip descriptor with IRQs and peripherals

### Run Validation Tests

```bash
# Custom validation (syntax, descriptions, bitfield widths)
python scripts/test_output.py

# Chiptool validation (full IR checking)
python scripts/validate_with_chiptool.py

# Show summary
bash scripts/test_summary.sh
```

### Requirements

Install dependencies:

```bash
pip install pyyaml

# For chiptool validation:
cargo install --git https://github.com/embassy-rs/chiptool --locked
```

## Test Results

Current status: **352/389 files (90.5%) pass chiptool validation**

### Test Suites

1. **Custom Validation** (`test_output.py`)
   - YAML syntax checking
   - Description artifact detection
   - Bitfield width validation
   - Chip JSON structure validation

2. **Chiptool Validation** (`validate_with_chiptool.py`)
   - Full chiptool IR compatibility
   - Field overlap detection
   - Register structure validation

## Known Limitations

The 37 files that don't pass chiptool validation contain register-level overlaps that reflect hardware design patterns in the CMSIS headers (e.g., CAN mailbox registers, ENET queue configuration). These are architectural patterns and not parsing errors.

## CI/CD

GitHub Actions workflow runs automatically on:
- Push to `main` or `develop` branches
- Pull requests to `main`
- Changes to `scripts/`, `data/`, or CI configuration

The workflow:
1. Installs Python and Rust dependencies
2. Parses all CMSIS headers
3. Runs validation tests
4. Uploads output artifacts

View results in the **Actions** tab on GitHub.
