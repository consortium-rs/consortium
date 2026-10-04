#!/usr/bin/env python3
"""Generate the PAC register-block modules from the chiptool IR.

Each `consortium-pac-*/src/peripherals/<module>.rs` that chiptool produces is
listed in `BLOCKS` with the `crates/consortium-data/database/peri/` YAML it is
generated from. Modules absent from `BLOCKS` are hand-maintained and left alone
(`HAND_MAINTAINED` names them so the omission is deliberate, not forgotten).

chiptool stamps its own git revision into every file it writes, so the output
is only reproducible with the pinned build. `just setup-chiptool` installs it;
this script refuses to run with any other.

Hand corrections the IR cannot express live as patches under
`scripts/pac/patches/generated/`, applied by `just pac-regen` after this script.

Run from the repository root:

    python3 scripts/pac/gen_blocks.py
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PERI = ROOT / "crates" / "consortium-data" / "database" / "peri"

#: The chiptool revision every committed PAC module is generated with. Keep in
#: step with `setup-chiptool` in the justfile.
CHIPTOOL_REV = "bcf538a2e7b8584ae874ee9ab72efb1576fc6152"

#: `(PAC crate, module) -> IR file`, relative to `PERI`.
BLOCKS: dict[tuple[str, str], str] = {
    ("consortium-pac-stm32mp2", "adc"): "stm32mp/adc_v1.yaml",
    ("consortium-pac-stm32mp2", "dma"): "stm32mp/dma_v1.yaml",
    ("consortium-pac-stm32mp2", "dma_channel"): "stm32mp/dma_channel_v1.yaml",
    ("consortium-pac-stm32mp2", "fdcan"): "stm32mp/fdcan_v1.yaml",
    ("consortium-pac-stm32mp2", "gpio"): "stm32mp/gpio_v1.yaml",
    ("consortium-pac-stm32mp2", "hsem"): "stm32mp/hsem_v1.yaml",
    ("consortium-pac-stm32mp2", "i2c"): "stm32mp/i2c_v1.yaml",
    ("consortium-pac-stm32mp2", "i3c"): "stm32mp/i3c_v1.yaml",
    ("consortium-pac-stm32mp2", "iwdg"): "stm32mp/iwdg_v1.yaml",
    ("consortium-pac-stm32mp2", "lptim"): "stm32mp/lptim_v1.yaml",
    ("consortium-pac-stm32mp2", "rcc"): "stm32mp/rcc_v1.yaml",
    ("consortium-pac-stm32mp2", "spi"): "stm32mp/spi_v1.yaml",
    ("consortium-pac-stm32mp2", "usart"): "stm32mp/usart_v1.yaml",
    ("consortium-pac-imx9", "adc"): "imx9/adc_v1.yaml",
    ("consortium-pac-imx9", "can"): "imx9/can_mimx95.yaml",
    ("consortium-pac-imx9", "ccm"): "imx9/ccm_mimx95.yaml",
    ("consortium-pac-imx9", "dma4"): "imx9/dma4_mimx93.yaml",
    ("consortium-pac-imx9", "dma5"): "imx9/dma5_mimx95.yaml",
    ("consortium-pac-imx9", "flexio"): "imx9/flexio_v1.yaml",
    ("consortium-pac-imx9", "flexspi"): "imx9/flexspi_mimx95.yaml",
    ("consortium-pac-imx9", "gpc"): "imx9/gpc_mimx95.yaml",
    ("consortium-pac-imx9", "gpc_global"): "imx9/gpc_global_mimx95.yaml",
    ("consortium-pac-imx9", "i3c"): "imx9/i3c_mimx95.yaml",
    ("consortium-pac-imx9", "intm"): "imx9/intm_mimx95.yaml",
    ("consortium-pac-imx9", "iomuxc"): "imx9/iomuxc_mimx95.yaml",
    ("consortium-pac-imx9", "iomuxc_gpr"): "imx9/iomuxc_gpr_mimx95.yaml",
    ("consortium-pac-imx9", "irqsteer"): "imx9/irqsteer_mimx95.yaml",
    ("consortium-pac-imx9", "lpi2c"): "imx9/lpi2c_v1.yaml",
    ("consortium-pac-imx9", "lpit"): "imx9/lpit_v1.yaml",
    ("consortium-pac-imx9", "lpspi"): "imx9/lpspi_mimx95.yaml",
    ("consortium-pac-imx9", "lptmr"): "imx9/lptmr_v1.yaml",
    ("consortium-pac-imx9", "lpuart"): "imx9/lpuart_v1.yaml",
    ("consortium-pac-imx9", "mu"): "imx9/mu_mimx95.yaml",
    ("consortium-pac-imx9", "rgpio"): "imx9/rgpio_v1.yaml",
    ("consortium-pac-imx9", "tpm"): "imx9/tpm_mimx95.yaml",
    ("consortium-pac-imx9", "wdog"): "imx9/wdog_mimx95.yaml",
}

#: Modules written by hand rather than generated.
HAND_MAINTAINED: set[tuple[str, str]] = {
    ("consortium-pac-stm32mp2", "timer"),
    ("consortium-pac-imx9", "dma"),
}


def check_chiptool() -> None:
    """Exit unless the installed chiptool is the pinned revision."""
    listing = subprocess.run(
        ["cargo", "install", "--list"], capture_output=True, text=True, check=True
    ).stdout
    # `cargo install --list` abbreviates the revision to eight characters.
    if f"chiptool#{CHIPTOOL_REV[:8]}" not in listing:
        sys.exit(
            f"chiptool must be installed from rev {CHIPTOOL_REV[:7]} for reproducible "
            "output; run `just setup-chiptool`"
        )


def module_path(crate: str, module: str) -> Path:
    return ROOT / "crates" / crate / "src" / "peripherals" / f"{module}.rs"


def check_coverage() -> None:
    """Every module on disk must be either generated or declared hand-maintained."""
    listed = set(BLOCKS) | HAND_MAINTAINED
    for crate in {crate for crate, _ in listed}:
        for path in sorted(module_path(crate, "x").parent.glob("*.rs")):
            if path.stem != "mod" and (crate, path.stem) not in listed:
                sys.exit(
                    f"{path.relative_to(ROOT)} is neither in BLOCKS nor HAND_MAINTAINED"
                )


def main() -> None:
    check_chiptool()
    check_coverage()
    for (crate, module), source in BLOCKS.items():
        out = module_path(crate, module)
        subprocess.run(
            [
                "chiptool",
                "gen-block",
                "--input",
                str(PERI / source),
                "--output",
                str(out),
                "--common-module",
                "crate::common",
                "--skip-no-std",
            ],
            check=True,
        )
        subprocess.run(["rustfmt", "--edition", "2024", str(out)], check=True)
        print(f"wrote {out.relative_to(ROOT)} <- {source}")


if __name__ == "__main__":
    main()
