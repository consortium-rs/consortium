#!/usr/bin/env python3
"""Parse STM32MP2 pin-mux and DMA-request data into the signal database.

Sources, staged per family under `data/signal/<family>/` by `prepare.sh`:

  * `STM32MP2*.xml` -- one CubeMX MCU file per part number
    (STM32_open_pin_data, BSD-3-Clause). Gives each part's package, the pads it
    bonds out, and every signal each pad can carry, AF-muxed or not.
  * `GPIO-<version>_Modes.xml` -- the family's alternate-function table, named
    by the `GPIO` IP version the MCU files declare: (pad, signal) -> AF number.
  * `stm32mp2xx_hal_dma.h` -- STM32CubeMP2's `HPDMA_REQUEST_*` /
    `LPDMA_REQUEST_*` numbers. MP21 (no LPDMA) numbers HPDMA requests
    differently from MP23/MP25, so the header's `#if defined (LPDMA)` is
    evaluated against the family's CMSIS device header.

Writes `crates/consortium-data/database/signal/<family>.json` through
`signal_db.write_signal_db`. Run after `parse_stm32.py`, whose chip dumps name
the peripherals signals are matched against.

    python3 scripts/pac/parse_signals_stm32.py --all
"""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

from signal_db import (
    DATA_DIR,
    ROOT,
    family_peripherals,
    split_function,
    write_signal_db,
)

FAMILIES = ("stm32mp21", "stm32mp23", "stm32mp25")

#: A GPIO pad, once CubeMX's `-OSC32_OUT`-style suffix is dropped. Dedicated
#: balls (`ANA0`, `PWR_ON`, DSI/CSI lanes, supplies) are not muxable and skipped.
PAD = re.compile(r"P[A-Z]\d+")
#: ADC analog inputs need the pad in analog mode, not an AF.
ANALOG = re.compile(r"ADC\d*_IN[PN]\d+")

MCU_ROOT = re.compile(r"<Mcu\b[^>]*>")
PIN = re.compile(r'<Pin\b[^>]*\bName="([^"]+)"[^>]*>(.*?)</Pin>', re.DOTALL)
SIGNAL = re.compile(r'<Signal\b[^>]*\bName="([^"]+)"')
GPIO_IP = re.compile(r'<IP\b[^>]*\bName="GPIO"[^>]*>')
MODES_PIN = re.compile(
    r'<GPIO_Pin\b[^>]*\bName="([^"]+)"[^>]*>(.*?)</GPIO_Pin>', re.DOTALL
)
MODES_SIGNAL = re.compile(
    r'<PinSignal\b[^>]*\bName="([^"]+)"[^>]*>(.*?)</PinSignal>', re.DOTALL
)
AF_VALUE = re.compile(r"GPIO_AF(\d+)_")
DMA_REQUEST = re.compile(r"#define\s+(HPDMA|LPDMA)_REQUEST_(\w+)\s+\((\d+)U\)")


def attr(tag: str, name: str) -> str:
    match = re.search(rf'\b{name}="([^"]*)"', tag)
    if match is None:
        raise SystemExit(f"missing {name}= in {tag[:80]}")
    return match.group(1)


def pad_name(name: str) -> str | None:
    pad = name.split("-", 1)[0]
    return pad if PAD.fullmatch(pad) else None


def parse_af_table(path: Path) -> dict[tuple[str, str], int]:
    table: dict[tuple[str, str], int] = {}
    for raw_pad, body in MODES_PIN.findall(path.read_text(encoding="latin-1")):
        pad = pad_name(raw_pad)
        if pad is None:
            continue
        for function, signal_body in MODES_SIGNAL.findall(body):
            afs = set(AF_VALUE.findall(signal_body))
            if len(afs) != 1:
                raise SystemExit(f"{path.name}: {pad} {function} has AFs {sorted(afs)}")
            table[(pad, function)] = int(afs.pop())
    return table


def defined_symbols(family: str) -> set[str]:
    """The `#if defined(...)` symbols the DMA header tests, for this family."""
    headers = sorted((ROOT / "data" / f"{family}_cm33").glob("*.h"))
    if not headers:
        raise SystemExit(f"no staged CMSIS headers for {family}; run prepare.sh")
    text = "\n".join(h.read_text(encoding="latin-1") for h in headers)
    return {"LPDMA"} if re.search(r"#define\s+LPDMA\b", text) else set()


def parse_dma(path: Path, symbols: set[str]) -> list[tuple[str, str, int]]:
    """Evaluate the header's `#if`/`#else` nesting just far enough for its requests.

    Only `defined(X)` / `!defined(X)` conditions matter here. Anything else is
    taken as false, which is right for the guards and `__cplusplus` blocks
    around the request tables.
    """
    active: list[bool] = []
    requests: list[tuple[str, str, int]] = []
    for line in path.read_text(encoding="latin-1").splitlines():
        directive = line.strip()
        if directive.startswith("#ifndef"):
            active.append(directive.split()[1] not in symbols)
        elif directive.startswith("#ifdef"):
            active.append(directive.split()[1] in symbols)
        elif directive.startswith("#if"):
            cond = re.fullmatch(
                r"#if\s+(!?)\s*defined\s*\(?\s*(\w+)\s*\)?.*", directive
            )
            if cond is None:
                active.append(False)
            else:
                active.append((cond.group(2) in symbols) != bool(cond.group(1)))
        elif directive.startswith("#elif"):
            raise SystemExit(f"{path.name}: unexpected #elif: {directive}")
        elif directive.startswith("#else"):
            active[-1] = not active[-1]
        elif directive.startswith("#endif"):
            active.pop()
        elif all(active):
            match = DMA_REQUEST.match(directive)
            if match:
                requests.append((match.group(1), match.group(2), int(match.group(3))))
    return requests


def parse_family(family: str) -> Path:
    src = DATA_DIR / family
    peripherals = family_peripherals(family)

    parts = sorted(src.glob(f"{family.upper()}*.xml"))
    if not parts:
        raise SystemExit(f"no MCU files for {family} under {src}; run prepare.sh")

    package_parts: dict[str, list[str]] = defaultdict(list)
    pad_packages: dict[str, set[str]] = defaultdict(set)
    pad_signals: dict[str, set[str]] = defaultdict(set)
    gpio_versions: set[str] = set()
    for part in parts:
        text = part.read_text(encoding="latin-1")
        root = MCU_ROOT.search(text)
        if root is None:
            raise SystemExit(f"{part.name}: no <Mcu> element")
        package = attr(root.group(0), "Package")
        package_parts[package].append(attr(root.group(0), "RefName"))
        gpio = GPIO_IP.search(text)
        if gpio is None:
            raise SystemExit(f"{part.name}: no GPIO IP")
        gpio_versions.add(attr(gpio.group(0), "Version"))
        for raw_pad, body in PIN.findall(text):
            pad = pad_name(raw_pad)
            if pad is None:
                continue
            pad_packages[pad].add(package)
            pad_signals[pad].update(s for s in SIGNAL.findall(body) if s != "GPIO")

    if len(gpio_versions) != 1:
        raise SystemExit(f"{family}: parts disagree on GPIO version: {gpio_versions}")
    modes = src / f"GPIO-{gpio_versions.pop()}_Modes.xml"
    af_table = parse_af_table(modes)
    for pad, function in af_table:
        if pad in pad_packages:
            pad_signals[pad].add(function)

    signals = []
    for pad, functions in pad_signals.items():
        for function in functions:
            if (pad, function) in af_table:
                mux = {"kind": "af", "af": af_table[(pad, function)]}
            elif ANALOG.fullmatch(function):
                mux = {"kind": "analog"}
            else:
                mux = {"kind": "additional"}
            peripheral, signal = split_function(function, peripherals)
            signals.append(
                {
                    "pad": pad,
                    "function": function,
                    "peripheral": peripheral,
                    "signal": signal,
                    "mux": mux,
                }
            )

    dma_header = src / "stm32mp2xx_hal_dma.h"
    dma = []
    seen: set[tuple[str, int]] = set()
    for controller, function, request in parse_dma(dma_header, defined_symbols(family)):
        if (controller, request) in seen:
            raise SystemExit(f"{family}: duplicate {controller} request {request}")
        seen.add((controller, request))
        peripheral, signal = split_function(function, peripherals)
        dma.append(
            {
                "controller": controller,
                "request": request,
                "function": function,
                "peripheral": peripheral,
                "signal": signal,
            }
        )

    return write_signal_db(
        family,
        sources=[
            *(f"STM32_open_pin_data/mcu/{p.name}" for p in parts),
            f"STM32_open_pin_data/mcu/IP/{modes.name}",
            "STM32CubeMP2/Drivers/STM32MP2xx_HAL_Driver/Inc/stm32mp2xx_hal_dma.h",
        ],
        packages=[{"name": k, "parts": v} for k, v in package_parts.items()],
        pads=[{"name": k, "packages": sorted(v)} for k, v in pad_packages.items()],
        signals=signals,
        dma=dma,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--family", choices=FAMILIES)
    parser.add_argument("--all", action="store_true", help="parse every family")
    args = parser.parse_args()
    families = FAMILIES if args.all else (args.family,)
    if families == (None,):
        parser.error("pass --family or --all")
    for family in families:
        print(f"Wrote {parse_family(family).relative_to(ROOT)}")


if __name__ == "__main__":
    main()
