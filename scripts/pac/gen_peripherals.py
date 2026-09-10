#!/usr/bin/env python3
"""Generate Embassy-style peripheral singletons from chip JSON databases.

Reads `crates/consortium-data/database/chip/<chip>.json` and, for each target,
emits `<crate>/src/peripherals/<chip>.rs` containing:

  * a `consortium_hal::peripherals!` invocation declaring one zero-sized
    singleton type per peripheral the core can reach, plus the `Peripherals`
    struct that owns them, and
  * one `impl_<driver>!` invocation per driver-supported instance, binding that
    singleton to its MMIO base and NVIC line. Each driver module defines the
    macro, so the generated file carries data only -- never driver logic.

The HAL's `lib.rs` mounts the active chip's file as a private `chip` module and
re-exports `peripherals` and `Peripherals` from it, so the singletons land at
`consortium_hal_<family>::peripherals::<NAME>` the way Embassy's do.

The bindings are feature-gated to match the module they bind into, so a HAL
built with `default-features = false` still compiles. Doorbell peripherals (MU,
HSEM, IPCC) are deliberately absent: their drivers live in the
`consortium-ipc-doorbell-*` crates, which depend on the HAL, so those crates
bind the singletons themselves rather than inverting the dependency.

Run from the repository root:

    python3 scripts/pac/gen_peripherals.py
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHIP_DIR = ROOT / "crates" / "consortium-data" / "database" / "chip"


@dataclass(frozen=True)
class Binding:
    """One driver's rule for turning dump peripherals into `Instance` impls."""

    #: `macro_rules!` path the generated file invokes, e.g. `crate::i2c::impl_i2c`.
    macro: str
    #: Cargo feature gating the module that defines `macro`.
    feature: str
    #: Dump block kinds (the part of `block` before the `/`) this driver claims.
    blocks: tuple[str, ...]
    #: Interrupt-name suffixes to try, in order, when the peripheral's own name
    #: is not itself an interrupt name. `""` means "the bare peripheral name".
    irq_suffixes: tuple[str, ...] = ("",)
    #: Expand each matched peripheral into one binding per channel, keyed by
    #: block kind to `(controller class, channel count)`. The class is passed as
    #: a macro argument when non-empty; the channel index always is.
    channels: dict[str, tuple[str, int]] = field(default_factory=dict)
    #: Pass the peripheral name title-cased (`TPM1` -> `Tpm1`) as an extra
    #: argument. `macro_rules!` cannot build an identifier by concatenation, so
    #: drivers whose instance enum uses Rust casing take the variant this way.
    title_case: bool = False
    #: Extra per-instance argument, looked up by peripheral name and falling
    #: back to `variant_default`. Used for facts the dump does not carry, such
    #: as which STM32 timers have a 32-bit counter.
    variants: dict[str, str] = field(default_factory=dict)
    variant_default: str = ""


@dataclass(frozen=True)
class Target:
    #: Chip-dump stem under `database/chip/`.
    chip: str
    #: Output path stem, relative to the repository root.
    out: str
    #: Driver bindings to apply.
    bindings: tuple[Binding, ...]


# eDMA channels are the unit of ownership, but the i.MX9 dumps list only the
# controller, so each one is expanded into `<CONTROLLER>_CH<n>` singletons here.
# The class and channel count must match the layout table in
# `consortium-hal-imx9/src/dma.rs`; the driver still does the base arithmetic.
IMX9_DMA_CHANNELS = {
    "dma": ("Dma3", 31),
    "dma4": ("Dma4", 64),
    "dma5": ("Dma5", 64),
}

IMX9_BINDINGS = (
    Binding("crate::adc::impl_adc", "adc", ("adc",), ("", "_EOC")),
    Binding("crate::can::impl_can", "can", ("can",)),
    Binding(
        "crate::dma::impl_dma_channel",
        "dma",
        ("dma", "dma4", "dma5"),
        ("_0",),
        channels=IMX9_DMA_CHANNELS,
    ),
    Binding("crate::gpio::impl_gpio", "gpio", ("rgpio",), ("_0",)),
    Binding("crate::i2c::impl_i2c", "i2c", ("lpi2c",)),
    Binding("crate::i3c::impl_i3c", "i3c", ("i3c",)),
    Binding("crate::lptmr::impl_lptmr", "timer", ("lptmr",)),
    Binding("crate::spi::impl_spi", "spi", ("lpspi",)),
    Binding("crate::timer::impl_tpm", "timer", ("tpm",), title_case=True),
    Binding("crate::timer_driver::impl_lpit", "time-driver-lpit", ("lpit",)),
    Binding("crate::uart::impl_uart", "uart", ("lpuart",)),
    Binding("crate::watchdog::impl_watchdog", "watchdog", ("wdog",)),
)

STM32MP2_BINDINGS = (
    Binding("crate::adc::impl_adc", "adc", ("adc",)),
    Binding("crate::can::impl_can", "can", ("fdcan",), ("_IT0",)),
    # STM32MP2 dumps list every HPDMA/LPDMA channel as its own peripheral at
    # its own page, so no synthesis is needed and the IRQ name matches exactly.
    Binding("crate::dma::impl_dma_channel", "dma", ("dma_channel",)),
    Binding("crate::gpio::impl_gpio", "gpio", ("gpio",)),
    Binding("crate::i2c::impl_i2c", "i2c", ("i2c",)),
    Binding("crate::i3c::impl_i3c", "i3c", ("i3c",)),
    Binding("crate::lptim::impl_lptim", "lptim", ("lptim",), title_case=True),
    Binding("crate::spi::impl_spi", "spi", ("spi",)),
    # Counter width is not in the dump: TIM2 and TIM5 are the 32-bit timers.
    Binding(
        "crate::timer::impl_tim",
        "timer",
        ("tim",),
        ("", "_UP"),
        variants={"TIM2": "Bits32", "TIM5": "Bits32"},
        variant_default="Bits16",
    ),
    Binding(
        "crate::timer_driver::impl_tim_driver",
        "time-driver-gp-tim",
        ("tim",),
        ("", "_UP"),
    ),
    Binding("crate::uart::impl_uart", "uart", ("usart",)),
    Binding("crate::watchdog::impl_watchdog", "watchdog", ("iwdg",)),
)

TARGETS = (
    Target(
        "mimx95_cm7", "crates/consortium-hal-imx9/src/peripherals/imx95", IMX9_BINDINGS
    ),
    Target(
        "mimx93_cm33", "crates/consortium-hal-imx9/src/peripherals/imx93", IMX9_BINDINGS
    ),
    Target(
        "stm32mp25_cm33",
        "crates/consortium-hal-stm32mp2/src/peripherals/stm32mp25",
        STM32MP2_BINDINGS,
    ),
    Target(
        "stm32mp21_cm33",
        "crates/consortium-hal-stm32mp2/src/peripherals/stm32mp21",
        STM32MP2_BINDINGS,
    ),
)


def hex_literal(address: str) -> str:
    """Render a base address as an underscore-grouped Rust hex literal."""
    value = int(address, 16)
    return f"0x{value >> 16:04x}_{value & 0xFFFF:04x}"


def resolve_irq(
    name: str, irqs: dict[str, int], suffixes: tuple[str, ...]
) -> int | None:
    """First interrupt number matching `name` + one of `suffixes`, if any."""
    for suffix in suffixes:
        number = irqs.get(f"{name}{suffix}")
        if number is not None:
            return number
    return None


def irq_argument(number: int | None) -> str:
    return "None" if number is None else f"Some({number})"


def singleton_names(peripherals: list[dict], target: Target) -> list[str]:
    """Every singleton the chip declares, including synthesized DMA channels."""
    expansions = {
        kind: count
        for binding in target.bindings
        for kind, (_, count) in binding.channels.items()
    }
    names: list[str] = []
    for peripheral in peripherals:
        kind = peripheral["block"].split("/")[0]
        count = expansions.get(kind)
        if count is None:
            names.append(peripheral["name"])
        else:
            names.extend(
                f"{peripheral['name']}_CH{channel}" for channel in range(count)
            )
    return names


def render(chip: dict, target: Target) -> str:
    name = chip["name"]
    family = chip["family"]
    core = chip["core"]
    peripherals = sorted(chip["peripherals"], key=lambda p: p["name"])
    irqs = {irq["name"]: irq["value"] for irq in chip["irqs"]}

    out: list[str] = []
    out.append("// Generated by scripts/pac/gen_peripherals.py -- do not edit by hand.")
    out.append(f"// Source: {family} ({core}) peripheral map `{name}`.")
    out.append("//")
    out.append(
        "// Mounted by `lib.rs` as the private `chip` module for the selected chip"
    )
    out.append("// feature, which re-exports `peripherals` and `Peripherals`.")
    out.append("")
    out.append("::consortium_hal::peripherals! {")
    for singleton in singleton_names(peripherals, target):
        out.append(f"    {singleton},")
    out.append("}")
    out.append("")

    # Group the emitted bindings by driver so the file reads as a table.
    for binding in target.bindings:
        lines: list[str] = []
        for peripheral in peripherals:
            kind = peripheral["block"].split("/")[0]
            if kind not in binding.blocks:
                continue
            name = peripheral["name"]
            base = hex_literal(peripheral["base_address"])
            irq = resolve_irq(name, irqs, binding.irq_suffixes)

            if kind in binding.channels:
                controller, count = binding.channels[kind]
                for channel in range(count):
                    arguments = [f"{name}_CH{channel}", base]
                    if controller:
                        arguments.append(controller)
                    arguments.append(str(channel))
                    # Channel vectors are consecutive from channel 0's.
                    arguments.append(
                        irq_argument(None if irq is None else irq + channel)
                    )
                    lines.append(f"{binding.macro}!({', '.join(arguments)});")
                continue

            arguments = [name, base]
            if binding.title_case:
                arguments.append(name.title())
            if binding.variant_default:
                arguments.append(binding.variants.get(name, binding.variant_default))
            arguments.append(irq_argument(irq))
            lines.append(f"{binding.macro}!({', '.join(arguments)});")
        if not lines:
            continue
        out.append(f'#[cfg(feature = "{binding.feature}")]')
        out.append("mod " + binding.macro.split("::")[1] + "_bindings {")
        out.extend(f"    {line}" for line in lines)
        out.append("}")
        out.append("")

    return "\n".join(out)


def main() -> None:
    for target in TARGETS:
        chip = json.loads((CHIP_DIR / f"{target.chip}.json").read_text())
        path = ROOT / f"{target.out}.rs"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(render(chip, target))
        print(f"wrote {target.out}.rs ({len(chip['peripherals'])} peripherals)")


if __name__ == "__main__":
    main()
