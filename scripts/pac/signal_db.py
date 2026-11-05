"""Vendor-neutral half of the signal parsers.

`parse_signals_stm32.py` and `parse_signals_nxp.py` read different vendor
sources, but both hand this module the same records and get the same file back:
`crates/consortium-data/database/signal/<family>.json`, one per die family.

Schema (every key is always present; only `mux` varies by vendor):

    {
      "family":   "stm32mp25",
      "sources":  ["STM32_open_pin_data/mcu/IP/GPIO-STM32MP2_gpio_v1_0_Modes.xml", ...],
      "packages": [{"name": "TFBGA436", "parts": ["STM32MP251AAIx", ...]}],
      "pads":     [{"name": "PA4", "packages": ["TFBGA436", ...]}],
      "signals":  [{"pad": "PA4", "function": "USART2_TX",
                    "peripheral": "USART2", "signal": "TX", "mux": {...}}],
      "dma":      [{"controller": "HPDMA", "request": 10, "function": "USART1_TX",
                    "peripheral": "USART1", "signal": "TX"}]
    }

`function` is always the vendor's own name, verbatim. `peripheral` is the
chip-dump singleton name that function belongs to, or `null` when no core of the
family can reach it (an A-core-only block such as i.MX `ENET1`); `signal` is the
rest of the function then, and also `null`. A pad's `packages` is `null` when the
source does not say which packages bond it out (NXP lists pads per die).

`mux` is `{"kind": "af", "af": n}`, `{"kind": "additional"}` (an ST pad function
that needs no AF, such as `RTC_OUT1` or `PWR_WKUP1`), `{"kind": "analog"}` (an
ST ADC input), or `{"kind": "iomuxc", "mux_register", "mux_mode",
"input_register", "input_daisy", "config_register"}` (NXP; the input pair is
`null` when the signal has no input-select register).
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

from common import write_json

ROOT = Path(__file__).resolve().parents[2]
DATA_DIR = ROOT / "data" / "signal"
CHIP_DIR = ROOT / "crates" / "consortium-data" / "database" / "chip"
OUTPUT_DIR = ROOT / "crates" / "consortium-data" / "database" / "signal"


def natural_key(text: str) -> list[Any]:
    """Sort `PA10` after `PA9`, and `GPIO_IO10` after `GPIO_IO9`."""
    return [int(part) if part.isdigit() else part for part in re.split(r"(\d+)", text)]


def family_peripherals(family: str) -> set[str]:
    """Every peripheral any core of `family` can reach, from its chip dumps.

    Run after the register parsers (and the IR patches), which write the dumps.
    """
    dumps = sorted(CHIP_DIR.glob(f"{family}_*.json"))
    if not dumps:
        raise SystemExit(f"no chip dump for {family} under {CHIP_DIR}")
    names: set[str] = set()
    for dump in dumps:
        names.update(p["name"] for p in json.loads(dump.read_text())["peripherals"])
    return names


def split_function(
    function: str, peripherals: set[str]
) -> tuple[str | None, str | None]:
    """Split `USART2_TX` into (`USART2`, `TX`) by the longest matching peripheral.

    A function that is exactly a peripheral name (some single-signal DMA
    requests) gets signal `None`; one no peripheral prefixes gets `(None, None)`.
    """
    best = max(
        (p for p in peripherals if function == p or function.startswith(p + "_")),
        key=len,
        default=None,
    )
    if best is None:
        return None, None
    return best, function[len(best) + 1 :] or None


def write_signal_db(
    family: str,
    *,
    sources: list[str],
    packages: list[dict[str, Any]],
    pads: list[dict[str, Any]],
    signals: list[dict[str, Any]],
    dma: list[dict[str, Any]],
) -> Path:
    """Sort every list deterministically and write the family's database."""
    for package in packages:
        package["parts"] = sorted(package["parts"])
    for pad in pads:
        if pad["packages"] is not None:
            pad["packages"] = sorted(pad["packages"])
    data = {
        "family": family,
        "sources": sorted(sources),
        "packages": sorted(packages, key=lambda p: p["name"]),
        "pads": sorted(pads, key=lambda p: natural_key(p["name"])),
        "signals": sorted(
            signals, key=lambda s: (natural_key(s["pad"]), natural_key(s["function"]))
        ),
        "dma": sorted(dma, key=lambda d: (d["controller"], d["request"])),
    }
    path = OUTPUT_DIR / f"{family}.json"
    write_json(data, path)
    return path
