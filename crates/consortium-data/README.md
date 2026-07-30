# consortium-data

Static chip data for Consortium, embedded at compile time: chip-family databases,
per-core chip dumps, extracted peripheral register definitions, and vendor device-tree
sources — plus the resolvers that pick the right one for a `[profile] chip`.

## Why it is embedded rather than read from disk

`csti` is installed as a binary and run from a user's project directory, which has no
`database/` in it. So every datum here is compiled in — YAML and JSON through
`include_str!`, the device-tree tree through `include_dir!` — and resolution works
regardless of the working directory and cannot break because a data file moved.

The cost is binary size and a full rebuild on every data change. For a build tool
that has to work from an arbitrary directory, that is the right trade.

## What each module resolves

| Module     | Source                          | Answers                                                                                                |
| ---------- | ------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `database` | `database/memory/*.yaml`        | What does this chip _family_ offer — coprocessors, memory regions and aliases, peripherals, doorbells? |
| `chipmap`  | `database/chip/*.json`          | What can _one core_ reach — peripheral MMIO bases and NVIC numbers?                                    |
| `dts`      | `database/dts/{imx9,stm32mp2}/` | Which vendor board or SoC device tree do we build on top of?                                           |

### The database/chipmap split matters

A family database says the chip **has** an LPUART2. A chip map says **this core** can
see it.

Because there is one JSON file per core, the set of peripherals in a core's file _is_
the set that core can reach — which is exactly the filter `consortium-cfg` applies
when validating `[peripheral.<name>] owner`. No separate allow-list is maintained.

### One source, three consumers

The same chip dumps also feed `scripts/pac/gen_interrupts.py` and
`scripts/pac/gen_peripherals.py`, which generate the PAC's interrupt tables and the
chip HAL's peripheral singleton map:

```text
database/chip/*.json
  ├─► chipmap (this crate)          → consortium-cfg validation + codegen
  ├─► gen_interrupts.py             → consortium-pac-*/src/interrupts/
  └─► gen_peripherals.py            → consortium-hal-*/src/peripherals/
```

That shared origin is what keeps a HAL singleton, a PAC interrupt number, and a
validated manifest in agreement. Diverging them by editing one output is the failure
mode this arrangement exists to prevent.

### `dts` selection

`[profile] devkit` names a board device tree by its **full file stem** (e.g.
`stm32mp257f-dk`, `imx95-15x15-frdm`), not a short token, so the match is
unambiguous. With no `devkit`, the generic SoC `.dtsi` for the family is used
instead.

## Layout and size

| Directory          | Size   | Contents                                  |
| ------------------ | ------ | ----------------------------------------- |
| `database/memory/` | ~28 K  | Hand-maintained chip-family YAML          |
| `database/chip/`   | ~280 K | Generated per-core JSON dumps             |
| `database/dts/`    | ~2 M   | Vendor device trees and includes          |
| `database/peri/`   | ~14 M  | Extracted peripheral register definitions |

## Generated and imported data

**Treat everything under `database/` as generated or vendored, not authored.** Change
the generation workflow — the `just pac-*` recipes and `scripts/pac/` — rather than
editing a large data file by hand. A hand-edit to a chip dump will be silently
reverted by the next generator run, and in the meantime it desynchronizes the PAC and
HAL that read the same file.

Schema types come from [`consortium-cfg-common`](../consortium-cfg-common), which
exists partly so this crate can deserialize into them without depending on
`consortium-cfg` — the codegen crate that depends on _this_ one.

## Licensing

This crate carries a compound SPDX expression (`Apache-2.0 AND BSD-3-Clause AND MIT`)
rather than inheriting the workspace `Apache-2.0`, because it redistributes vendor
material.

**Vendor device trees under `database/dts/` must be dual-licensed. GPL-only files
cannot ship.** See [that directory's `README.md`](database/dts/README.md) before
adding one.

`[dts] linux` in a manifest points at a kernel source tree for GPL-licensed
dt-bindings needed during compilation — those are read from the user's own checkout at
build time, deliberately not vendored here.

## License

`Apache-2.0 AND BSD-3-Clause AND MIT`. See [LICENSE](../../LICENSE) for the project
licence and `database/dts/README.md` for the vendor-tree rules.
