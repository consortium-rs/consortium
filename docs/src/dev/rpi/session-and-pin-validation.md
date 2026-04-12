# RpiSession: Peripheral Configuration & Pin Validation

## Overview

`RpiSession` is the core abstraction for resolving and validating peripheral configurations at the device/chip variant level. It combines three data sources — the project configuration, the RIFSC peripheral map, and the IOMUX pin database — to produce a validated peripheral inventory with complete DTS metadata.

The session ensures that:
- **Peripheral IDs (perid)** are assigned from the chip's RIFSC YAML
- **Pin mappings** are validated against the IOMUX YAML for the specific package variant
- **Alternate function (AF) numbers** are derived correctly from the pin mux list
- **Aliases** are populated for DTS generation

---

## Architecture

### Data Flow

```
Consortium.toml
      ↓
  Config (chip: Chip, variant: Option<String>, peripheral: HashMap)
      ↓
RpiSession::new()
      ├─→ RifConfig (loaded from data/stm32mp/rifsc/stm32mp{21,23,25}xxxx.yaml)
      ├─→ IomuxConfig (loaded from data/stm32mp/iomux/STM32MP{variant}.yaml)
      ↓
  RpiSession { chip, variant, peripherals: HashMap<String, Peripheral> }
      ↓
 DTS generation, build validation, runtime configuration
```

### Key Types

**`GeneralConfig`** (in `consortium-common`):
```rust
pub struct GeneralConfig {
    pub chip: Chip,                    // Family level: Stm32mp25x, Imx95, etc.
    pub variant: Option<String>,       // Package variant: "STM32MP257FAKx"
}
```

**`RpiSession`** (in `consortium-rpi`):
```rust
pub struct RpiSession {
    pub chip: Chip,
    pub variant: Option<String>,
    pub peripherals: HashMap<String, Peripheral>,
}
```

**`Peripheral`** (in `consortium-rpi`):
```rust
pub struct Peripheral {
    pub name: String,
    pub owner: String,
    pub secure: bool,
    pub pins: Vec<PeripheralPinMux>,  // Validated & enriched
    pub dts: Option<String>,
    pub perid: Option<u32>,            // RIFSC ID, assigned from RifConfig
}
```

**`PeripheralPinMux`** (in `consortium-rpi`):
```rust
pub struct PeripheralPinMux {
    pub group: char,                   // GPIO group: 'A', 'B', etc.
    pub pin_id: u8,                    // GPIO pin number
    pub position: String,              // AF function: "AF0", "AF1", ..., "ANALOG"
    pub alias: String,                 // DTS comment: "I2C1_SDA"
}
```

---

## Usage

### Creating an RpiSession

```rust
use consortium_common::config::{
    Config,
    load_rifsc_peripherals,
    load_stm32mp_iomux,
};
use consortium_rpi::RpiSession;

// 1. Load configuration from Consortium.toml
let config: Config = load_config("Consortium.toml")?;

// 2. Load RIFSC peripheral map for the chip family
let chip_family = config.general.chip.rifsc_family_name()
    .ok_or("Chip does not support RIFSC")?;
let rif_path = format!("data/stm32mp/rifsc/{}.yaml", chip_family);
let rif = load_rifsc_peripherals(&rif_path)?;

// 3. Optionally load IOMUX for pin validation
let iomux = if let Some(variant) = &config.general.variant {
    Some(load_stm32mp_iomux(variant)?)
} else {
    None
};

// 4. Create the session
let session = RpiSession::new(&config, Some(&rif), iomux.as_ref())?;

// 5. Iterate over validated peripherals
for (name, periph) in &session.peripherals {
    println!("Peripheral: {}", name);
    println!("  RIFSC ID: {:?}", periph.perid);
    println!("  Owner: {}", periph.owner);
    for pin in &periph.pins {
        println!("  Pin: P{}{} → {} ({})", pin.group, pin.pin_id, pin.position, pin.alias);
    }
}
```

### Optional Parameters

**`rif: Option<&RifConfig>`**
- **Required for STM32MP**: Pass the loaded RIFSC YAML to assign `perid`
- **Not applicable for i.MX**: Pass `None`; peripherals will have `perid = None`

**`iomux: Option<&IomuxConfig>`**
- **Required for pin validation**: Pass the loaded IOMUX YAML for the specific package variant (e.g., `STM32MP257FAKx`)
- **Optional**: Pass `None` to skip pin validation; `position` and `alias` fields remain empty

---

## Chip Family & Variant Mapping

### Chip Family → RIFSC YAML

The `Chip` enum represents the **family level**. Use `rifsc_family_name()` to get the RIFSC YAML stem:

| Chip | RIFSC Family | YAML File |
|------|---|---|
| `Stm32mp21x` | `stm32mp21xxxx` | `data/stm32mp/rifsc/stm32mp21xxxx.yaml` |
| `Stm32mp23x` | `stm32mp23xxxx` | `data/stm32mp/rifsc/stm32mp23xxxx.yaml` |
| `Stm32mp25x` | `stm32mp25xxxx` | `data/stm32mp/rifsc/stm32mp25xxxx.yaml` |
| `Imx93`, `Imx95`, etc. | `None` | No RIFSC (uses TRDC, not yet implemented) |

### Variant → IOMUX YAML

The **variant string** in `GeneralConfig` specifies the exact package. This maps directly to the IOMUX YAML filename:

| Variant | IOMUX File |
|---|---|
| `"STM32MP255AAKx"` | `data/stm32mp/iomux/STM32MP255AAKx.yaml` |
| `"STM32MP257FAKx"` | `data/stm32mp/iomux/STM32MP257FAKx.yaml` |
| `"STM32MP213FANx"` | `data/stm32mp/iomux/STM32MP213FANx.yaml` |

---

## Pin Validation & AF Assignment

### IOMUX YAML Structure

Each pin entry in the IOMUX YAML lists available alternate functions (mux options):

```yaml
PA5:
  position: P11                # Physical package position (BGA ball)
  type: I/O
  mux:
    - name: tim2              # AF0: Timer2 Channel 4
      pin: ch4
    - name: i2c1              # AF1: I2C1 SDA
      pin: sda
    - name: spi2              # AF2: SPI2 MOSI
      pin: mosi
```

### AF Number Assignment

The **AF function number is derived from the mux list index** (STM32MP convention):
- `mux[0]` → `"AF0"` (timer2)
- `mux[1]` → `"AF1"` (i2c1)
- `mux[2]` → `"AF2"` (spi2)

When validating a peripheral pin, `RpiSession::new()` finds the index of the peripheral in the mux list and stores it as `PeripheralPinMux.position`:

```rust
let af_idx = iomux_pin
    .mux
    .iter()
    .position(|sig| sig.name == "i2c1")?;  // Finds index 1

pin.position = format!("AF{}", af_idx);    // Sets "AF1"
pin.alias = "I2C1_SDA";                    // From mux[1].pin
```

### Validation Errors

**`RpiError::InvalidPin(pin_name)`**
- Pin does not exist in the IOMUX YAML for this package variant
- Example: trying to use `"PA100"` on a 176-pin package that only has up to `"PA63"`

**`RpiError::PinNotAvailable(pin, peripheral, allowed_pins)`**
- Pin exists in IOMUX but does not support the configured peripheral
- Example: `"PA5"` supports `tim2`, `i2c1`, `spi2`, but not `uart1`
- The error message includes the list of pins that do support the peripheral

---

## DTS Generation

Once validated, the peripheral and its pins can generate DTS snippets for inclusion in the device tree:

```rust
// Generate DTS iomux definition for each pin
for pin in &periph.pins {
    let dts = pin.generate_dts_iommx(sleep: false);
    // Output: <STM32_PINMUX('A', 5, AF1)>, /* I2C1_SDA */
}
```

The `position` field (e.g., `"AF1"`) is used directly in the DTS macro, ensuring correct pin function configuration.

---

## Design Decisions

1. **Variant is `Option<String>`**: Pin validation is optional. Callers can create a session without specifying the variant; `perid` is still assigned, but `position` and `alias` remain empty.

2. **AF number from mux index**: The IOMUX YAML does not store AF numbers explicitly. We rely on mux list order matching the AF numbering convention (which is universal on STM32 devices).

3. **Silent perid assignment**: When a peripheral is not found in the RIFSC map, `perid` stays `None` rather than erroring. This allows non-RIFSC-controlled peripherals to exist in the config.

4. **No i.MX TRDC yet**: The `rif` parameter is `Option` to prepare for i.MX support, but TRDC loading is not yet implemented.

---

## Testing & Validation

### Manual Integration Test

```bash
# Build and test the RpiSession with real YAML data
cargo test -p consortium-rpi

# Verify a specific variant loads and validates correctly
cargo run --example rpi_session_example STM32MP257FAKx
```

### Checklist

- [ ] Config loads without errors
- [ ] `chip.rifsc_family_name()` returns correct family string
- [ ] RIFSC YAML loads successfully
- [ ] IOMUX YAML loads successfully
- [ ] All configured peripherals exist in RIFSC map
- [ ] All configured pins exist in IOMUX map
- [ ] All pins support the configured peripheral
- [ ] `perid` is assigned correctly from RIFSC ID
- [ ] `position` is derived correctly from mux index
- [ ] `alias` is formatted as `"PERIPHERAL_FUNCTION"`
- [ ] DTS generation produces valid STM32_PINMUX macros
