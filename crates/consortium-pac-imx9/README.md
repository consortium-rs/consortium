# consortium-pac-imx9

Peripheral Access Crate for the NXP i.MX 9 family: register blocks, interrupt
definitions, and the `cortex-m-rt` vector table.

`no_std`. **Generated — not hand-written.**

## You probably want the HAL instead

Depend on [`consortium-hal-imx9`](../consortium-hal-imx9) unless you specifically need
raw register access. It re-exports this crate as `pac` and adds owned peripheral
singletons, so drivers take a `Peri<'static, LPUART2>` rather than an address and no
constructor is `unsafe`.

```text
consortium-hal-imx9      drivers, owned singletons  ← depend on this
  └── consortium-pac-imx9   register blocks, Interrupt, vector table
```

## Contents

| Module        | Contents                                                         |
| ------------- | ---------------------------------------------------------------- |
| `peripherals` | Register block definitions, one module per peripheral kind.      |
| `interrupts`  | The chip's `Interrupt` enum; with `rt`, `__INTERRUPTS` too.      |
| `common`      | Register/field access primitives the generated code builds on.   |
| `metadata`    | Machine-readable register description, ported from `stm32-data`. |

Peripheral blocks currently generated: ADC, CAN (FlexCAN), CCM, DMA (eDMA global plus
`dma4`/`dma5` channel blocks), FlexIO, FlexSPI, GPC, I3C, INTM, IOMUXC (+ GPR),
IRQSTEER, LPI2C, LPIT, LPSPI, LPTMR, LPUART, MU, RGPIO, TPM, WDOG.

## Features

| Feature  | Effect                                                                  |
| -------- | ----------------------------------------------------------------------- |
| `mimx95` | Chip selection (default). Eight MU instances.                           |
| `mimx93` | Chip selection. Two MU instances.                                       |
| `rt`     | `cortex-m-rt` integration: the `#[interrupt]` macro and `__INTERRUPTS`. |
| `defmt`  | Derive `defmt::Format` on generated types where applicable.             |

Choose **exactly one** chip: the feature selects which generated interrupt and
peripheral map is mounted, and enabling both is a duplicate-definition error. **A
dependent selecting `mimx93` must set `default-features = false`** so `mimx95` is not
also pulled in — two chips mean an ambiguous `Interrupt` enum.

`rt` should be enabled by the **application**, since it is what owns the vector table.
The generated `.x` linker fragment adds weak `DefaultHandler` fallbacks so an
application only needs `#[interrupt]` handlers for the lines it actually services.

## How it is generated

```text
submodules/               vendor CMSIS headers (NXP)
  └── scripts/pac/*.py       parse → chiptool metadata → Rust
        └── output/, data/      generated + validated artifacts
              └── src/             this crate
```

The workflow is driven by the `just pac-*` recipes:

```bash
just pac-init            # bring up vendor submodules
just setup-chiptool      # install the pinned chiptool build
just pac-regen           # regenerate every PAC output from the vendor headers
just pac-verify          # regenerate, then fail if anything differs from git
```

`pac-regen` runs the whole chain: vendor-header patches, header staging, IR parsing,
IR corrections, interrupt tables, HAL singletons, chiptool register blocks (the
module-to-IR map is `BLOCKS` in `scripts/pac/gen_blocks.py`), corrections to the
generated Rust, and formatting. The committed output is exactly what it produces.

Interrupt tables come from `scripts/pac/gen_interrupts.py` reading the
`consortium-data` chip dumps — the same source that feeds
`scripts/pac/gen_peripherals.py`, which generates the chip HAL's singleton map. That
shared source is what keeps the HAL's singletons and the PAC's interrupt numbers in
agreement.

**Do not hand-edit generated output.** A correction the parsers cannot make goes in a
patch with a header saying why: `scripts/pac/patches/<submodule>/` for the vendor
headers, `scripts/pac/patches/ir/` for the IR and chip dumps, and
`scripts/pac/patches/generated/` for the Rust. Then re-run `just pac-verify`.

## Lints

`lib.rs` allows a set of clippy lints (`identity_op`, `unnecessary_cast`,
`undocumented_unsafe_blocks`, and friends) for the whole crate. That is a deliberate
exemption for generated code: each generated accessor carries one SAFETY contract, and
the generator does not emit per-operation safety comments or hand-tuned expressions. It
is not a licence to relax those lints in hand-written crates.

## Contributing

Add peripheral coverage by extending the parser scripts and vendor patches, not the
generated files. Contributions welcome.

## License

Apache-2.0. See [LICENSE](../../LICENSE).

The vendor CMSIS headers this crate is generated _from_ live in `submodules/` and are
not redistributed here; only the generated Rust is.
