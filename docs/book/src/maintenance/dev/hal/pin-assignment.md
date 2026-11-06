# Pin Assignment (Step 4 Design Note)

**Date:** 2026-11-05
**Status:** Open. Nothing here is implemented; it frames the decision before the HALs change.
**Builds on:** the signal databases (`crates/consortium-data/database/signal/`), their
schema checks (`scripts/pac/test_output.py`), and manifest pin validation
(`consortium_cfg::Session`). These were steps 1–3 of the pin-data work.
**Roadmap items:** [P2.4 Typed pins and P2.5 Typed DMA requests](embassy-gap-roadmap.md#phase-2--metadata-and-code-generation).

## Where things stand

Steps 1–3 made pin and DMA data generated, checked, and enforced at the manifest:

- `just pac-regen` writes one `database/signal/<family>.json` per die family
  (`stm32mp21/23/25`, `mimx93/95`) with one schema for both vendors. A pad carries
  signals, and each signal carries a `mux`: an ST AF number, `additional`, `analog`, or an
  NXP IOMUXC tuple (mux register and mode, optional input-select register and daisy, and
  the pad-config register). ST pads also list the packages that bond them out. Each DMA
  request line is a (controller, request number) pair.
- `consortium_data::signal` embeds those files and resolves them from `[profile] chip`.
- `Session` checks each `[peripheral.<name>] pin.<signal> = "<pad>"`. The pad must carry
  `<NAME>_<SIGNAL>`, be bonded out in the package the part number implies, and be
  assigned once. This applies to **every owner**: an `ap` peripheral is checked exactly
  like a `cm33`/`cm7` one.

That last point is the property this note wants to keep. Today a pin is assigned in one
place, in one vocabulary (the vendor's signal and pad names), and validated one way,
whoever drives the peripheral. Step 4 decides what the assignment then _does_. The risk
is that it does something different, and is written somewhere different, depending on
the owner.

## Goal: one pin model, whoever owns the peripheral

The goal is for pin assignment to look the same to the user for every owner and every
chip:

1. **One place.** Pins are declared in `Consortium.toml` under the peripheral they serve.
   No second declaration in firmware source, a hand-written DTS fragment, or an SM config.
2. **One vocabulary.** Vendor signal names as keys and vendor pad names as values, as
   validation already requires.
3. **One validation.** The checks above, plus ownership conflicts: a pad given to an `ap`
   peripheral and to a `cm33` one is an error.
4. **Lowering differs; the contract does not.** Each owner gets the pads configured by
   whatever actually configures them on that chip. The user does not pick the mechanism.

## Who configures a pad today

The mechanism is decided by the chip and the owner, not by Consortium:

| Chip     | A-core (Linux) peripheral                                         | Controller-core peripheral                                                                                                                                                                                     |
| -------- | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| STM32MP2 | Linux `&pinctrl` (`stm32-pinfunc.h`, `STM32_PINMUX(port, n, AF)`) | Either the M33 writes `GPIOx_MODER`/`AFR` itself, or Linux muxes on its behalf. RIF per-pin GPIO security decides who may (**to verify** against RM0457 and ST's BSP).                                         |
| i.MX 93  | Linux `fsl,imx93-iomuxc`                                          | The CM33 writes IOMUXC directly when TRDC lets it (**to verify**); otherwise Linux muxes.                                                                                                                      |
| i.MX 95  | Linux `scmi_iomuxc` (SCMI pinctrl protocol 0x19, via the SM)      | The System Manager owns IOMUXC. Its config gives pins to each logical machine (`PIN_<pad> OWNER` under `LM1 "M7"` in `imx-sm/configs/mx95evk.cfg`), and the M7 muxes them over SCMI or the SM does it at boot. |

So an Embassy-style "the driver writes the mux register" works on STM32MP2 and probably
on i.MX 93. It does not work as-is on i.MX 95, and it is never how the A-core side works.

## Options

Each option is judged against the goal: does the user get the same model for an `ap`
peripheral and a `cm7` one, on every chip?

### A. The manifest owns pins; generated code and generated DTS apply them

`consortium-cfg` lowers each validated assignment to its owner's mechanism:

- `ap`: a `pinctrl-0` group in the generated DTS (merge or overlay), using `STM32_PINMUX`
  or the i.MX `MX9x_PAD_*` macros built from the `mux` data.
- Controller core on STM32MP2 / i.MX 93: generated `init()` writes the AF or IOMUXC
  registers before handing out `Peripherals`, the same way it already brings up the NVIC
  and time driver.
- Controller core on i.MX 95: an SM configuration fragment (`PIN_<pad> OWNER` for the
  M7 LM), and/or SCMI pinctrl calls in generated `init()`.

Users never call a mux API. Drivers keep `new(peri, …)`. The HAL gains pad singletons
and `TxPin<T>`-style traits only as what generated code (and `steal()`-based bring-up)
uses to apply a mux. This is the most symmetric option: one declaration, every owner
lowered by codegen.

_Costs:_ DTS pinctrl rendering, SM config output, and an SCMI client in firmware — now
`consortium-pmi-scmi` (see [Platform Management](../pmi/index.md)), still without an
i.MX 95 binding.
Pins chosen at runtime are not supported (they are not today either).

### B. Embassy-style: drivers take pins

Constructors take pad singletons (`Uart::new(p.LPUART1, p.UART1_TXD, p.UART1_RXD, …)`),
type-checked by generated traits, and set the mux themselves.

This is familiar Rust ergonomics, with compile-time pin checking for firmware written by
hand. But it is **asymmetric**: firmware pins live in Rust while `ap` pins live in the
manifest, and the two are validated by different machinery (the type system against the
manifest checks). On i.MX 95 the driver cannot write IOMUXC at all. It also duplicates
the manifest: a pin declared in TOML and passed again in Rust can disagree.

### C. Both: manifest-driven by default, typed pins as an escape hatch

Option A, plus an opt-in `new_with_pins(…)` for firmware that does its own bring-up
(tests, `steal()` paths, chips or cores without a manifest). Generated code uses the same
traits, so there is one implementation of "apply this mux". The escape hatch is outside
the manifest's guarantees, and the docs must say so.

### D. Types only

Pad singletons and traits exist only as compile-time proof; nothing writes a mux and Linux
keeps doing everything. This is cheap, but it leaves controller-core pins to a hand-written
DTS. That breaks "one place".

## DMA requests are independent of this choice

`TxDma<T>`/`RxDma<T>`-style traits, generated from `dma` rows, and replacing the
hand-written `requests!` tables in both `dma.rs` files, are the same under every option.
Every hand-written i.MX 9 entry already matches the generated data (42 of 42). The
STM32MP2 tables lack USART/SPI/I2C/I3C/SAI, which the data now has. This can land first.

## Open questions

1. **STM32MP2 controller-core pins.** With RIF, can the M33 write `GPIOx_AFR` for a pin
   assigned to its CID, and what does ST's OpenSTLinux expect: the M33 or Linux muxing
   M33 pins (and does the `m33` remoteproc node carry them)?
2. **i.MX 93 IOMUXC from the CM33.** Is IOMUXC reachable under the default TRDC setup,
   and does Linux reclaim unclaimed pads the way `clk_disable_unused` closes LPCGs?
3. **i.MX 95.** Does the SM mux an LM's pins at boot from the config, or must the M7 call
   SCMI pinctrl? Either way, should Consortium emit the SM config fragment, or only check
   it?
4. **Pad config.** Pull, drive strength, slew rate, open drain, `SION`: the manifest
   carries none of these today. They need a vendor-neutral spelling (or per-vendor
   fields) under the same `pin.<signal>` key, for example
   `pin.sda = { pad = "PA2", pull = "up", open_drain = true }`.
5. **GPIO pins.** A `[peripheral.gpioN]` claims a whole port today. Symmetry suggests
   naming individual pads (`pin.io00 = "GPIO_IO00"`), so a pad given to a GPIO user
   conflicts with one given to a peripheral.
6. **Who wins on conflict with the board DTS.** In merge mode, an `ap` assignment
   overrides the devkit's existing `pinctrl-0`. The rule should be explicit, and
   validation should reject a pad the base tree already gives to another node.

## Proposed order (once an option is chosen)

1. DMA traits from `database/signal` in both HALs, replacing `requests!` (option-independent).
2. Pad singletons and per-driver pin traits, generated by `gen_peripherals.py` from the
   same rows, with one `apply` implementation per vendor (AF write, or IOMUXC
   mux/daisy/config write).
3. Lowering in `consortium-cfg`, one owner at a time behind the same validated assignment:
   `ap` → DTS pinctrl; STM32MP2/i.MX 93 controller cores → generated `init()`; i.MX 95 →
   SM config and/or SCMI.
4. Pad-config fields and per-pad GPIO claims in the manifest schema.
