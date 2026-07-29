# Device Tree Source files

This folder collects ST device source tree of STM32MP2 series and NXP i.MX 9 series.

DTS of STM32MP2 series are licensed under GPL-2.0-(only/later) OR BSD-3-Clause. Hereby we apply with BSD-3-Clause license.

DTS of i.MX 9 series are licensed under GPL-2.0-(only/later) OR MIT license. Hereby we apply with MIT license.

Every file here must therefore carry a dual `GPL-2.0-... OR (BSD-3-Clause | MIT)` SPDX
identifier. Files offered under GPL alone cannot be redistributed inside the
Apache-2.0 `consortium-data` crate and must not be added; two such files
(`imx93-11x11-frdm-pixpaper.dtso` and `fsl,qoriq-clockgen.h`) were dropped when the
crate was prepared for crates.io. When importing new vendor sources, check the SPDX
line first:

```bash
find crates/consortium-data/database/dts -type f \( -name '*.dts*' -o -name '*.h' \) \
  -exec sh -c 'head -3 "$1" | grep -q "SPDX.*OR" || echo "GPL-only: $1"' _ {} \;
```
