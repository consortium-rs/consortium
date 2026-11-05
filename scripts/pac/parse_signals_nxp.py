#!/usr/bin/env python3
"""Parse i.MX 9 pin-mux and DMA-request data into the signal database.

Sources, staged per family under `data/signal/<family>/` by `prepare.sh`:

  * `fsl_iomuxc.h` -- MCUX's pad-function table. Each
    `IOMUXC_PAD_<PAD>__<FUNCTION>` row is the five-tuple the IOMUXC needs:
    mux register, mux mode, input-select (daisy) register, daisy value, and
    pad-config register. A zero input register means the signal has no daisy.
  * eDMA request-source enums: `PERI_DMA.h` / `PERI_DMA4.h` (i.MX 93, eDMA3 and
    eDMA4) and `fsl_edma_soc.h` (i.MX 95, eDMA3 and eDMA5), whose members read
    `[k]Dma<n>RequestMux<Function> = <request>U`.

Writes `crates/consortium-data/database/signal/<family>.json` through
`signal_db.write_signal_db`. Run after `parse_nxp.py`, whose chip dumps name the
peripherals signals are matched against.

The MCUX tables list pads per die, not per package, so every pad's `packages`
is `null` and the family's `packages` list is empty.

    python3 scripts/pac/parse_signals_nxp.py --all
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

from signal_db import (
    DATA_DIR,
    ROOT,
    family_peripherals,
    split_function,
    write_signal_db,
)

#: Family -> (MCUX device directory, DMA request headers).
FAMILIES = {
    "mimx93": ("i.MX93/MIMX9352", ("PERI_DMA.h", "PERI_DMA4.h")),
    "mimx95": ("i.MX95/MIMX9596", ("fsl_edma_soc.h",)),
}

HEX = r"(0x[0-9a-fA-F]+)U?"
IOMUXC_ROW = re.compile(
    rf"#define\s+IOMUXC_PAD_(\w+?)__(\w+)\s+{HEX},\s*{HEX},\s*{HEX},\s*{HEX},\s*{HEX}"
)
DMA_ROW = re.compile(r"^\s*k?(Dma\d+)RequestMux(\w+?)\s*=\s*(\d+)U", re.MULTILINE)
#: `ToBusRequest` -> `To_Bus_Request`, and `ELERequest` -> `ELE_Request`.
CAMEL_BOUNDARY = re.compile(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])")


def register(value: str) -> str:
    return f"0x{int(value, 16):08x}"


def parse_pins(path: Path, peripherals: set[str]) -> tuple[set[str], list[dict]]:
    pads: set[str] = set()
    signals = []
    rows = IOMUXC_ROW.findall(path.read_text(encoding="latin-1"))
    if not rows:
        raise SystemExit(f"{path}: no IOMUXC_PAD rows")
    for pad, function, mux_reg, mode, input_reg, daisy, config_reg in rows:
        pads.add(pad)
        has_input = int(input_reg, 16) != 0
        peripheral, signal = split_function(function, peripherals)
        signals.append(
            {
                "pad": pad,
                "function": function,
                "peripheral": peripheral,
                "signal": signal,
                "mux": {
                    "kind": "iomuxc",
                    "mux_register": register(mux_reg),
                    "mux_mode": int(mode, 16),
                    "input_register": register(input_reg) if has_input else None,
                    "input_daisy": int(daisy, 16) if has_input else None,
                    "config_register": register(config_reg),
                },
            }
        )
    return pads, signals


def dma_function(
    name: str, peripherals: set[str]
) -> tuple[str, str | None, str | None]:
    """Split MCUX's CamelCase request name against the chip dump's peripherals.

    `LPUART1Tx` -> (`LPUART1_TX`, `LPUART1`, `TX`); `Sai1Rx` -> (`SAI1_RX`,
    `SAI1`, `RX`); `TPM1Request0Request2` -> (..., `TPM1`,
    `REQUEST0_REQUEST2`). The remainder is MCUX's own wording in upper snake
    case, deliberately not reinterpreted.
    """
    upper = name.upper()
    # The prefix must end on a CamelCase word boundary, so `ADC1Request` is not
    # read as peripheral `ADC` (i.MX 95's dump name) with signal `1_REQUEST`.
    best = max(
        (
            p
            for p in peripherals
            if p
            and upper.startswith(p)
            and (len(name) == len(p) or name[len(p)].isupper())
        ),
        key=len,
        default=None,
    )
    if best is None:
        return CAMEL_BOUNDARY.sub("_", name).upper(), None, None
    rest = CAMEL_BOUNDARY.sub("_", name[len(best) :]).upper().strip("_") or None
    function = f"{best}_{rest}" if rest else best
    return function, best, rest


def parse_dma(paths: list[Path], peripherals: set[str]) -> list[dict]:
    dma = []
    seen: set[tuple[str, int]] = set()
    for path in paths:
        rows = DMA_ROW.findall(path.read_text(encoding="latin-1"))
        if not rows:
            raise SystemExit(f"{path}: no DMA request-source rows")
        for controller, name, request in rows:
            key = (controller.upper(), int(request))
            if key in seen:
                raise SystemExit(f"{path.name}: duplicate {key[0]} request {key[1]}")
            seen.add(key)
            function, peripheral, signal = dma_function(name, peripherals)
            dma.append(
                {
                    "controller": key[0],
                    "request": key[1],
                    "function": function,
                    "peripheral": peripheral,
                    "signal": signal,
                }
            )
    return dma


def parse_family(family: str) -> Path:
    device_dir, dma_headers = FAMILIES[family]
    src = DATA_DIR / family
    peripherals = family_peripherals(family)

    iomuxc = src / "fsl_iomuxc.h"
    if not iomuxc.exists():
        raise SystemExit(f"{iomuxc} missing; run prepare.sh")
    pads, signals = parse_pins(iomuxc, peripherals)
    dma = parse_dma([src / h for h in dma_headers], peripherals)

    periph_dir = device_dir.split("/")[0] + "/periph"
    dma_sources = [
        f"mcux-devices-imx/{periph_dir if h.startswith('PERI_') else device_dir + '/drivers'}/{h}"
        for h in dma_headers
    ]
    return write_signal_db(
        family,
        sources=[f"mcux-devices-imx/{device_dir}/drivers/fsl_iomuxc.h", *dma_sources],
        packages=[],
        pads=[{"name": pad, "packages": None} for pad in pads],
        signals=signals,
        dma=dma,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--family", choices=sorted(FAMILIES))
    parser.add_argument("--all", action="store_true", help="parse every family")
    args = parser.parse_args()
    families = sorted(FAMILIES) if args.all else [args.family]
    if families == [None]:
        parser.error("pass --family or --all")
    for family in families:
        print(f"Wrote {parse_family(family).relative_to(ROOT)}")


if __name__ == "__main__":
    main()
