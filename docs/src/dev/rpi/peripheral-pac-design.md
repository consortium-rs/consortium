# RIF Subsystem: Peripheral Configuration & PAC Generation Design

## Overview

This document proposes a system for declaring peripheral ownership in `Consortium.toml` and automatically generating:

1. **RIF/RIFSC security configuration** — derived entirely by the builder from the chip mapping already in `consortium-common`; not user-specified.
2. **PAC peripheral objects** (`p.i2c1`, `p.spi2`, …) for M-core firmware, exposed in `embedded-hal`-compatible format backed by `embassy-stm32` or `imxrt-hal`.
3. **Device Tree Source (DTS) fragments** assigning peripherals owned by `"linux"` or `"optee"` to their respective domains.

RIF compartment IDs and DMA routing are derived automatically: the builder knows the chip, the chip mapping knows which CID each core occupies, and the DMA controllers on both STM32MP2x (RISAB) and i.MX 9x (TRDC) are RIF-aware — so claiming a peripheral implicitly covers its DMA access.

The goal is that a developer declares intent once and the build system derives safe, non-overlapping peripheral access across all execution environments.

---

## Motivation

On a heterogeneous SoC such as the STM32MP257 (Cortex-A35 + Cortex-M33 + Cortex-M0+) or i.MX 94 (Cortex-A55 + 2× Cortex-M7 + Cortex-M33), each peripheral must be owned by exactly one execution domain:

| Domain | Mechanism | Current approach |
|--------|-----------|-----------------|
| Linux (NS) | DTS `status = "okay"` | Manual |
| OP-TEE (S) | DTS `secure-status = "okay"` | Manual |
| M-core NS firmware | PAC + HAL + RIFSC/TRDC | Manual |
| M-core S firmware (TF-M) | PAC + HAL + RIFSC/TRDC | Manual |

Today all four configurations are written by hand and must stay in sync. A mismatch causes silent interrupt mis-attribution, broken DMA, or a security violation trap at runtime.

`Consortium.toml` becomes the single source of truth.

---

## Consortium.toml Schema

Peripherals are declared as named subsections `[peripheral.<NAME>]`, where `<NAME>` is the peripheral identifier in lower-snake-case matching the chip's PAC (e.g. `i2c1`, `spi2`, `lptim1`). This mirrors TOML idioms like `[dependencies.<crate>]` and avoids the ordering noise of `[[peripherals]]` arrays.

### Field reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `master` | string | yes | Who owns this peripheral: a core abbreviation (`"cm33"`, `"cm0p"`, `"cm7_0"`), `"linux"`, or `"optee"` |
| `secure` | bool | when master is M-core or `"optee"` | `true` → secure-world (RIFSC SEC bit / TRDC); `false` → non-secure. Defaults to `false` for M-core, must be `true` for `"optee"` |
| `clock` | string | no | Clock source name for this peripheral (chip-specific, e.g. `"pll4_r"`, `"hsi"`). Omit to use the chip reset default. |

No `rif_cid` — the builder derives each core's compartment ID from `Chip::programmable_rt_cores()` and `Chip::remoteproc_name()` in `consortium-common`. No `dma` — DMA is RIF-aware on both STM32MP2x (RISAB) and i.MX 9x (TRDC); assigning a peripheral automatically covers its DMA paths.

**GPIO banks** (`gpioa`, `gpiob`, …) are first-class entries in the RIFSC/TRDC like any other peripheral and are declared the same way — no special handling required:

```toml
[peripheral.gpioa]
master = "linux"

[peripheral.gpiob]
master = "cm33"
secure = false
```

### Core abbreviations

`master` for M-core owners uses `CortexMcuCore::abbr()`:

| Core | Single instance | Multiple instances (indexed) |
|------|----------------|------------------------------|
| Cortex-M0+ | `"cm0p"` | `"cm0p_0"`, `"cm0p_1"` |
| Cortex-M33 | `"cm33"` | `"cm33_0"`, `"cm33_1"` |
| Cortex-M7 | `"cm7"` | `"cm7_0"`, `"cm7_1"` |
| Cortex-M4 | `"cm4"` | `"cm4_0"`, `"cm4_1"` |

When a chip has only one instance of a core type, the plain abbreviation is used. When it has multiple (e.g. i.MX 94 with two Cortex-M7 cores), an underscore-index suffix disambiguates. The builder resolves these against `Chip::programmable_rt_cores()` to validate correctness.

### Example — STM32MP257

```toml
[general]
chip = "stm32mp257"

[project]
name = "my-app"

[project.firmware.sensor_fw]
core  = "CortexM33"
index = 0
path  = "firmware/sensor"

[project.firmware.motor_fw]
core  = "CortexM0p"
index = 0
path  = "firmware/motor"

# ── Peripheral assignments ─────────────────────────────────────────────────────

[peripheral.i2c1]
master = "cm33"
secure = false
clock  = "pll4_r"      # optional: override default clock source

[peripheral.lptim1]
master = "cm33"
secure = true          # claimed by the TF-M secure partition on M33

[peripheral.spi2]
master = "cm0p"
secure = false

[peripheral.uart4]
master = "linux"       # appears in DTS only; no RIFSC entry

[peripheral.i2c5]
master = "optee"
secure = true          # enforced: optee always requires secure = true

[peripheral.gpioa]
master = "linux"

[peripheral.gpiob]
master = "cm33"
secure = false
```

### Example — i.MX 94 (dual Cortex-M7)

```toml
[general]
chip = "imx94"

[project.firmware.dsp_fw]
core  = "CortexM7"
index = 0
path  = "firmware/dsp"

[project.firmware.motor_fw]
core  = "CortexM7"
index = 1
path  = "firmware/motor"

[peripheral.lpi2c1]
master = "cm7_0"       # first M7 (index 0)
secure = false

[peripheral.lpspi3]
master = "cm7_1"       # second M7 (index 1)
secure = false

[peripheral.lpuart2]
master = "linux"
```

---

## Output Artifacts

### 1. RIFSC / TRDC Configuration (generated into `consortium-runtime-mcu`)

The builder derives each core's RIF Compartment ID from `Chip::remoteproc_name()`, then emits a startup function. Users never write CIDs.

For STM32MP2x (RIFSC):

```rust
// Generated: generated/rif_config.rs
// DO NOT EDIT — regenerated by `consortium build`

use crate::pac::{RCC, RIFSC};

pub fn configure_rif(rcc: &mut RCC, rifsc: &mut RIFSC) {
    // ── Clock sources ─────────────────────────────────────────────────────────
    // i2c1: clock = pll4_r
    rcc.i2c1ckselr().write(|w| w.i2c1sel().pll4_r());

    // ── RIFSC assignments ─────────────────────────────────────────────────────
    // i2c1: master=cm33 (CID=2), secure=false
    rifsc.seccfgr1().modify(|_, w| w.sec8().clear_bit());
    rifsc.risc_cidcfgr8().write(|w| unsafe {
        w.cfen().set_bit().scid().bits(2)
    });

    // lptim1: master=cm33 (CID=2), secure=true
    rifsc.seccfgr5().modify(|_, w| w.sec40().set_bit());
    rifsc.risc_cidcfgr40().write(|w| unsafe {
        w.cfen().set_bit().scid().bits(2)
    });

    // spi2: master=cm0p (CID=3), secure=false
    rifsc.seccfgr1().modify(|_, w| w.sec15().clear_bit());
    rifsc.risc_cidcfgr15().write(|w| unsafe {
        w.cfen().set_bit().scid().bits(3)
    });

    // gpiob: master=cm33 (CID=2), secure=false
    rifsc.seccfgr0().modify(|_, w| w.sec33().clear_bit());
    rifsc.risc_cidcfgr33().write(|w| unsafe {
        w.cfen().set_bit().scid().bits(2)
    });
}
```

For i.MX 9x (TRDC), the equivalent domain controller registers are emitted instead.

### 2. Per-Firmware PAC Wrapper (`generated/pac/<fw_key>/`)

A thin generated crate per firmware entry exposes only the peripherals owned by that firmware as a `Peripherals` struct, wrapping `embassy-stm32` or `imxrt-hal` tokens. This gives the firmware typed, compile-time-verified access — unowned peripherals are consumed at steal-time, preventing accidental access.

For owned peripherals:
- **Non-`Copy` types** are moved into the struct, providing exclusive ownership.
- **`Copy` types** are also included but could be accessed elsewhere (since they're copyable). No artificial restriction is imposed.

For unowned peripherals, all non-`Copy` tokens are explicitly consumed (moved into a drop) at steal-time. `Copy` peripherals that aren't owned can still be accessed via a second `steal()` call—this is acceptable since the type system cannot prevent it, and it's a rare edge case.

```rust
// Generated: generated/pac/sensor_fw/src/lib.rs
// Chip: stm32mp257 | Firmware: sensor_fw (CortexM33)
// DO NOT EDIT — regenerated by `consortium build`

pub struct Peripherals {
    pub i2c1:   embassy_stm32::peripherals::I2C1,
    pub lptim1: embassy_stm32::peripherals::LPTIM1,
}

impl Peripherals {
    /// # Safety
    /// Must be called exactly once, after `configure_rif()`.
    /// Consumes unowned, non-Copy peripherals to prevent access.
    pub unsafe fn steal() -> Self {
        let p = embassy_stm32::Peripherals::steal();
        
        // Consume unowned non-Copy peripherals by moving them
        // They cannot be cloned, so this removes them from the namespace
        let _unused = (
            p.I2C2,
            p.SPI1,
            p.SPI2,
            p.UART4,
            p.LPTIM2,
            // ... all other unowned, non-Copy peripherals
        );
        
        Self { 
            i2c1: p.I2C1, 
            lptim1: p.LPTIM1,
        }
    }
}
```

Usage in firmware:

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    consortium_runtime_mcu::init();   // calls configure_rif() internally

    // Steal only owned peripherals; unowned non-Copy ones are consumed
    let p = unsafe { sensor_fw::Peripherals::steal() };
    
    let mut i2c = I2c::new(p.i2c1, Config::default());

    loop {
        let mut buf = [0u8; 2];
        i2c.read(0x48, &mut buf).await.unwrap();
    }
}
```

The firmware has compile-time assurance that it cannot use unowned, non-`Copy` peripherals—attempting to reference them will fail at compile time because they were consumed in `steal()`. Owned peripherals are available as fields in `p`.

For i.MX 9x, types come from `imxrt-hal` instead of `embassy-stm32`.

### 3. Device Tree Fragments (`generated/dts/`)

Only peripherals with `master = "linux"` or `master = "optee"` appear here.

```dts
/* Generated: generated/dts/linux-peripherals.dtsi */
/* DO NOT EDIT — regenerated by `consortium build`  */

&uart4 {
    status = "okay";
};

/* Generated: generated/dts/optee-peripherals.dtsi */

&i2c5 {
    secure-status = "okay";
};
```

Include these from the top-level board DTS via `#include`.

---

## Architecture

```
Consortium.toml
      │
      ▼
consortium-common
  ├── parse [peripheral.*] → HashMap<String, PeripheralEntry>
  ├── Chip::programmable_rt_cores()  — resolve master abbr → (CortexMcuCore, index)
  └── validate_peripherals()         — run all rules, emit structured errors
      │
      ├──▶ consortium-pac-gen  (build-time binary)
      │       ├── derive CID from chip mapping        (no user input)
      │       ├── derive DMA routing from chip DB     (no user input)
      │       ├── emit generated/rif_config.rs
      │       ├── emit generated/pac/<fw_key>/src/lib.rs  (per firmware)
      │       └── emit generated/dts/{linux,optee}-peripherals.dtsi
      │
      ├──▶ consortium-builder  (orchestrates pac-gen as build step 1)
      │
      └──▶ consortium-cli  (human-facing inspection & integration tooling)
```

### New crate: `consortium-pac-gen`

A build-time binary (and library API consumed by `consortium-builder`) that:

1. Reads `Consortium.toml` via `consortium-common`.
2. Resolves each `master` abbreviation to `(CortexMcuCore, index)` using `Chip::programmable_rt_cores()`.
3. Derives the RIF Compartment ID per core from `Chip::remoteproc_name()`.
4. Looks up each peripheral name in the chip database to get RIFSC/TRDC index, DTS node, and HAL type path.
5. Emits the three artifact families into `generated/`.

### Chip peripheral database

A TOML file per chip embedded via `include_str!` in `consortium-pac-gen/data/<chip>.toml`:

```toml
# consortium-pac-gen/data/stm32mp257.toml
[[peripheral]]
name          = "i2c1"
rifsc_idx     = 8
dts_node      = "i2c1"
hal_type      = "embassy_stm32::peripherals::I2C1"
clock_reg     = "i2c1ckselr"        # RCC register name for clock source selection
clock_options = ["pclk1", "pll4_r", "hsi", "csi"]   # valid values for `clock` field

[[peripheral]]
name          = "lptim1"
rifsc_idx     = 40
dts_node      = "lptim1"
hal_type      = "embassy_stm32::peripherals::LPTIM1"
clock_reg     = "lptim1ckselr"
clock_options = ["pclk1", "pll4_p", "pll3_q", "lse", "lsi", "per"]

[[peripheral]]
name          = "gpiob"
rifsc_idx     = 33
dts_node      = "gpiob"
hal_type      = "embassy_stm32::peripherals::GPIOB"
# no clock_reg: GPIO clocks are managed by RCC AHB enable bits, not selectors
```

---

## Validation Rules

All rules run inside `consortium-common`'s `Config::validate_peripherals()`, shared by both the build pipeline and the CLI:

1. **Uniqueness:** Each `[peripheral.*]` key is unique (TOML guarantees this syntactically; validated semantically against the chip DB).
2. **Name validity:** All keys must exist in the chip peripheral database for the configured `chip`.
3. **Master validity:** Each `master` value is either `"linux"`, `"optee"`, or a core abbreviation (with optional index) present in `Chip::programmable_rt_cores()`.
4. **Security consistency:** `secure = false` is rejected for `"optee"` masters; `secure = true` is rejected for `"linux"` masters.
5. **Core availability:** A `master` abbreviation must correspond to a core that actually exists on the chip (e.g. `"cm0p"` is invalid on i.MX 94).
6. **Clock validity:** If `clock` is specified, the value must appear in the chip DB's `clock_options` list for that peripheral. Peripherals without a `clock_reg` in the DB reject any `clock` field.

Errors are emitted as `cargo:error=` messages during build and as structured output from `consortium peripheral validate`.

---

## CLI Integration (`consortium-cli`)

`consortium-cli` gains a `peripheral` subcommand group. All subcommands read `Consortium.toml` from the working directory (or `--config <path>`). These are read-only inspection and validation tools; no code is generated from the CLI.

### `consortium peripheral list`

```
$ consortium peripheral list

Peripheral  Master   Secure  Domain
──────────  ───────  ──────  ──────────
i2c1        cm33     no      M-core (sensor_fw)
lptim1      cm33     yes     M-core/S (sensor_fw)
spi2        cm0p     no      M-core (motor_fw)
uart4       linux    —       Linux DTS
i2c5        optee    yes     OP-TEE DTS

5 peripherals — 3 M-core, 1 Linux, 1 OP-TEE
```

`--json` emits machine-readable output for IDE integration.

### `consortium peripheral show <name>`

```
$ consortium peripheral show i2c1

[peripheral.i2c1]
  Master     : cm33  → sensor_fw (CortexM33, index 0)
  Secure     : false
  Clock      : pll4_r  (valid options: pclk1, pll4_r, hsi, csi)
  RIF CID    : 2  (derived from chip mapping)

Chip database (stm32mp257):
  RIFSC index : 8
  DTS node    : &i2c1
  HAL type    : embassy_stm32::peripherals::I2C1
  DMA         : covered by RISAB (RIF-aware, no explicit config needed)

Generated artifacts:
  rif_config.rs : rcc.i2c1ckselr() → pll4_r; rifsc.risc_cidcfgr8() → cid=2, sec=0
  PAC struct    : generated/pac/sensor_fw/src/lib.rs :: Peripherals.i2c1
  DTS           : (not emitted — M-core peripheral)
```

### `consortium peripheral validate`

```
$ consortium peripheral validate

✓ All peripheral names exist in chip database (stm32mp257)
✓ All master values resolve to known cores or linux/optee
✓ Security consistency: no conflicts
✓ Core availability: cm33 and cm0p both present on stm32mp257

Validation passed. 5 peripherals, 0 errors.
```

Exits non-zero on failure with one line per error. Suitable for CI pre-build checks.

### `consortium peripheral chip-info [--unassigned]`

```
$ consortium peripheral chip-info --unassigned

Chip: stm32mp257

RIFSC  Peripheral  Status
─────  ──────────  ──────────────
0      tim1        (unassigned)
1      tim2        (unassigned)
...
(137 unassigned peripherals shown; use without --unassigned to see all)
```

Useful when starting a project to see what peripherals are available for assignment.

---

## HAL Strategy

### STM32MP2x — embassy-stm32

The generated `Peripherals::steal()` destructures `embassy_stm32::Peripherals::steal()` into only the owned subset. The firmware's `Cargo.toml` only needs:

```toml
[dependencies]
embassy-executor = { version = "0.7", features = ["arch-cortex-m", "executor-thread"] }
embassy-stm32    = { version = "0.2", features = ["stm32mp257fai", "time-driver-any"] }
# pac wrapper is path-referenced from generated/
```

The correct `embassy-stm32` chip feature (`stm32mp257fai`, etc.) is propagated automatically from `Chip` enum via `consortium-builder`.

### i.MX 9x — imxrt-hal

The generated struct wraps `imxrt_hal::instance::*` tokens. `imxrt-ral` instance stealing follows the same pattern as embassy-stm32's `Peripherals::steal()`.

---

## Implementation Plan

### Phase 1 — Schema & Validation (1 week)

- [ ] Add `PeripheralEntry { master: String, secure: Option<bool>, clock: Option<String> }` to `consortium-common`.
- [ ] Parse `[peripheral.*]` as `HashMap<String, PeripheralEntry>` in `Config::from_toml()`.
- [ ] Implement `Config::validate_peripherals()` with rules 1–6.
- [ ] Extend `CortexMcuCore::from_abbr()` to handle indexed variants (`"cm7_0"`, `"cm7_1"`).
- [ ] Unit tests for each validation rule and index-suffix resolution.

### Phase 2 — Chip Database (1 week)

- [ ] Define `ChipPeripheralDb` + `ChipPeripheralEntry` structs (TOML, embedded in `consortium-pac-gen`).
- [ ] Populate STM32MP257 database (RIFSC index, DTS node name, HAL type path).
- [ ] Populate i.MX 93 and i.MX 94 databases (TRDC domain index, DTS node, HAL type path).
- [ ] Expose DB lookup via `consortium-common` API so CLI and pac-gen share it.

### Phase 3 — CLI Subcommands (1 week)

- [ ] Add `peripheral` subcommand group to `consortium-cli` (using `clap`).
- [ ] Implement `list`, `show`, `validate`, `chip-info` with `--json` and `--unassigned` flags.
- [ ] Test with STM32MP257 and i.MX 94 fixture configs.

### Phase 4 — Code Generation (2 weeks)

- [ ] Create `consortium-pac-gen` crate (binary + library).
- [ ] Implement CID derivation from `Chip::remoteproc_name()`.
- [ ] Implement RIFSC emitter (STM32MP2x) with RCC clock-source writes.
- [ ] Implement TRDC emitter (i.MX 9x) with CCM clock-source writes.
- [ ] Investigate embassy-stm32 and imxrt-hal `init()` for NVIC/EXTI coverage; emit STM32MP2x C2IMR routing if not covered.
- [ ] Implement per-firmware `Peripherals` struct emitter (embassy-stm32 / imxrt-hal).
- [ ] Implement DTS fragment emitter.
- [ ] Wire into `consortium-builder` as build pipeline step 1.

### Phase 5 — Runtime Integration (1 week)

- [ ] `consortium-runtime-mcu`: call generated `configure_rif()` from `init()`.
- [ ] Example firmware (STM32MP257) using generated `Peripherals` with embassy-stm32.
- [ ] Example firmware (i.MX 94) using generated `Peripherals` with imxrt-hal.
- [ ] Integration test: build full config, verify generated files compile for target.

### Phase 6 — DTS Integration (1 week)

- [ ] Hook DTS fragment generation into `consortium-builder` build pipeline step 1.
- [ ] Verify generated overlays compile with `dtc`.
- [ ] Document how to `#include` overlays in the base board DTS.

---

## Design Decisions

| # | Question | Decision |
|---|----------|----------|
| 1 | **GPIO in RIF** | GPIO banks (`gpioa`, `gpiob`, …) are listed in RIFSC/TRDC like any other peripheral. Declared as `[peripheral.gpioa]` with the same `master`/`secure` fields. No special handling. |
| 2 | **Clock configuration** | `clock` field added to `[peripheral.<name>]`. Builder emits RCC/CCM clock source selection in `configure_rif()` alongside RIFSC config. Valid values per peripheral come from the chip DB (`clock_options`). System-level PLL/oscillator config is future work. |
| 3 | **Interrupt routing** | In scope for generated init, but contingent on what embassy-stm32 / imxrt-hal cover in their own `init()`. If NVIC enable and EXTI routing are handled by the HAL driver constructors, no generated code is needed. If not (e.g. STM32MP2x EXTI C2IMR routing to M-core), the builder emits the necessary register writes. To be determined during Phase 4 implementation with reference to embassy-stm32 source. |
| 4 | **Silicon revisions** | Delegated to the upstream PAC (`stm32mp257-pac`, `imxrt-ral`). The chip DB targets a specific PAC version; users pin the PAC version in `Cargo.toml` as usual. No revision logic inside Consortium. |
| 5 | **Shared peripherals** | Not in scope for the initial implementation. A peripheral must have exactly one static owner. Time-multiplexed or arbitrated sharing between cores is a future concern and will require a separate design. |
| 6 | **Copy vs non-Copy peripheral tokens** | Unowned, non-`Copy` peripherals are consumed (moved into a drop) at steal-time, enforcing exclusivity through Rust's type system. Owned peripherals (both `Copy` and non-`Copy`) are included in the wrapper struct. For `Copy` peripherals, no artificial restriction is imposed—they can be copied anyway. This pragmatic approach leverages type-level safety where it's effective without unnecessary restrictions. |

---

## Non-Goals

- Does **not** replace `embassy-stm32` or `imxrt-hal`; wraps them.
- Does **not** generate full DTS board files; generates peripheral assignment overlays only.
- Does **not** configure system PLLs or oscillators (only peripheral clock source selection).
- Does **not** support shared peripherals in this iteration (see decision 5 above).
