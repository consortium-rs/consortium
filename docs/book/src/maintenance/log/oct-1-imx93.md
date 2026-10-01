# i.MX 93, October 1, 2026 (Bringing Up the CM33)

The i.MX 93 port of melt-pot (`examples/melt-pot/imx93`) pairs the CA55 Linux
application with Cortex-M33 firmware on an FRDM-i.MX93 board running NXP's
`6.18.20-2.0.0` BSP kernel. Unlike the i.MX 95, there is no System Manager: Linux owns
the CCM, and the CM33 runs from TCM rather than a DDR carveout. Both facts show up
below.

The firmware is reloaded with `misc/debugging/rfirm.sh`, which on this board finds
`remoteproc0` named `imx-rproc`:

```sh
echo stop > /sys/class/remoteproc/remoteproc0/state
cp ~/cm33.elf /lib/firmware
echo cm33.elf > /sys/class/remoteproc/remoteproc0/firmware
echo start > /sys/class/remoteproc/remoteproc0/state
```

## Problem 1: `start` Resets the Whole SoC

Writing `start` rebooted the board straight back into SPL:

```
root@imx93-11x11-lpddr4x-frdm:~# echo start > /sys/class/remoteproc/remoteproc0/state
U-Boot SPL 2026.04-lf_v2026.04+g6eeef838dac+p0 (Jun 02 2026 - 15:03:39 +0000)
...
U-Boot 2026.04-lf_v2026.04+g6eeef838dac+p0 (Jun 02 2026 - 15:03:39 +0000)
Reset Status: M33_REQ
```

### Hypothesis 0: A Broken Image

The ELF was sane: the vector table, entry point, and stack all landed in TCM
(`readelf -S`, abridged to name, address, and size):

```
  .interrupts       0ffe0000  0003bc
  .text             0ffe1000  012e58
  .resource_table   2001e000  00008c
  .data             20000000  000018

Contents of section .interrupts:
 ffe0000 00e00120 0110fe0f f9e8fe0f 4f3eff0f  ... ........O>..
```

Initial SP `0x2001e000`, reset vector `0x0ffe1001`. Not the image.

### Hypothesis 1: The Firmware Asks for a Reset

`M33_REQ` is specific. The SRC fieldset in `consortium-data` separates the two CM33
causes:

```yaml
fieldset/SRC_GENERAL_REG_SRSR_CM33:
  fields:
    - name: REQUEST
      bit_offset: 11
    - name: LOCKUP
      bit_offset: 12
```

Bit 11 is a `SYSRESETREQ`, not a lockup. But disassembling `cm33.elf` found no write to
`AIRCR` (`0xE000ED0C`) anywhere: the only System Control Space accesses were NVIC
`ISER`/`ICPR`/`IPR` and the MPU. Our code never requested a reset.

### Progress: Comparing Against NXP's Firmware

NXP's `imx93-11x11-evk_m33_TCM_rpmsg_lite_pingpong_rtos_linux_remote.elf` starts
cleanly on the same board. Its `Reset_Handler` begins:

```
0ffe0594 <Reset_Handler>:
 ffe0594:	b672      	cpsid	i
 ffe0596:	4812      	ldr	r0, [pc, #72]	@ (ffe05e0 <Reset_Handler+0x4c>)
 ffe0598:	4912      	ldr	r1, [pc, #72]	@ (ffe05e4 <Reset_Handler+0x50>)
 ffe059a:	6001      	str	r1, [r0, #0]
 ffe059c:	680a      	ldr	r2, [r1, #0]
 ffe059e:	f382 8808 	msr	MSP, r2
 ...
 ffe05e0:	e000ed08 	.word	0xe000ed08
 ffe05e4:	0ffe0000 	.word	0x0ffe0000
```

It writes `VTOR = 0x0FFE0000` and reloads MSP from its own table. cortex-m-rt does
neither by default. The CM33 enters the image through ROM, and VTOR is still on the
ROM's table, so the first interrupt our firmware enables (LPIT1, MU, LPI2C) vectors into
a ROM handler that requests a system reset. That matches `M33_REQ` exactly.

### Fix

cortex-m-rt ≥ 0.7.4 has `set-sp` and `set-vtor`. `consortium-pac-imx9`'s `mimx93`
feature now turns both on (weakly, so it does not force `rt`):

```toml
mimx93  = ["cortex-m-rt?/set-sp", "cortex-m-rt?/set-vtor"]
```

The rebuilt `Reset`:

```
0ffe1000 <Reset>:
 ffe1000:	4812      	ldr	r0, [pc, #72]	@ (ffe104c <Reset+0x4c>)
 ffe1002:	f380 8808 	msr	MSP, r0
 ffe1006:	4812      	ldr	r0, [pc, #72]	@ (ffe1050 <Reset+0x50>)
 ffe1008:	4912      	ldr	r1, [pc, #72]	@ (ffe1054 <Reset+0x54>)
 ffe100a:	6001      	str	r1, [r0, #0]
 ...
 ffe104c 00e00120 08ed00e0 0000fe0f
```

On the board, `start` now leaves the system up:

```
[ 1452.070753] remoteproc remoteproc0: powering up imx-rproc
[ 1452.073295] remoteproc remoteproc0: Booting fw image cm33.elf, size 3282528
[ 1452.605998] imx-rproc remoteproc-cm33: No initialized mbox tx channel
[ 1452.606079] remoteproc remoteproc0: remote processor imx-rproc is now up
state=running
```

`No initialized mbox tx channel` is expected: the generated DTS deletes the CM33
node's `mboxes` because the IPC channel claims MU1.

## Problem 2: The Application Panics the Kernel

Running the CA55 application took the kernel down from EL0:

```
[  514.340952] SError Interrupt on CPU1, code 0x00000000be000011 -- SError
[  514.341057] Kernel panic - not syncing: Asynchronous SError Interrupt
[  514.341134]  do_serror+0x58/0x74
[  514.341142]  __el0_error_handler_common+0x44/0xac
```

An asynchronous SError from userspace is a bus error on an `mmap`'d UIO window. The
generated DTS had put Linux's MU UIO node on side B:

```dts
mu1@44230000 {
    compatible = "generic-uio";
    reg = <0x0 0x44230000 0x0 0x10000>;
    interrupts = <GIC_SPI 22 IRQ_TYPE_LEVEL_HIGH>;
    clocks = <&clk IMX93_CLK_MU1_B_GATE>;
};
```

Empirically, MU-B from the CA55 faulted and MU-A did not.

### MU Sides Swapped

The database now gives Linux MU side A and the CM33 side B, the same orientation as
i.MX 95:

| Party   | Window | MU1          | MU2          | IRQ (MU1/MU2) | Clock (DTS)      |
| ------- | ------ | ------------ | ------------ | ------------- | ---------------- |
| `linux` | A      | `0x44220000` | `0x42430000` | 21 / 23       | `MU{1,2}_A_GATE` |
| `cm33`  | B      | `0x44230000` | `0x42440000` | 22 / 24       | —                |

The CM33 chip map gained `MU1_MUB`/`MU2_MUB` singletons and the doorbell crate the
matching `impl_mu!(MU1_MUB, Secondary, 1)` bindings, so the generated firmware builds
`MuDoorbell<Secondary, 1>` from `p.MU1_MUB` and installs `fn MU1_B()`. `consortium-cfg`
needed no logic change — it already builds `MU{id}_MU{A|B}` from each party's `window`.

### TCM Layout Aligned With NXP

The same reference ELF fixes the TCM SYS map:

```
20003000 D __data_start__
2001dc00 A __StackLimit
2001e000 B __StackTop
2001e000 B __RscTblStart
2001f000 B __RscTblEnd
```

NXP keeps `0x20000000..0x20003000` out of `.data`/`.bss` (`.m33_suspend` starts at
`0x20000000`); ours placed `.data` at `0x20000000`. The `remoteproc.cm33.ram` window
moved to `0x20003000`, length `0x1B000`. Vectors, code, stack top, and the resource table
already matched.

## Problem 3: UIO Indices Follow Probe Order

The generated application opened UIO devices by number: `uio0` = IPC memory (which
it zero-fills), `uio1` = debug ring, `uio2` = MU. The board's DTB enumerates them
differently:

```
/sys/class/uio/uio0 mu1@44220000 0x0000000044220000
/sys/class/uio/uio1 ipc-shm-uio@20480000 0x0000000020480000
/sys/class/uio/uio2 dbg-shm-uio@20484000 0x0000000020484000
```

Against that tree the application would have zero-filled the MU registers as if they
were shared memory.

`uio_pdrv_genirq` names a device after its `linux,uio-name` property, else `%pfwP` (the
full node name with unit address). So:

- Every generated UIO node now carries `linux,uio-name = "<name>"`.
- `UioDevice::open_named(name, base, irq)` resolves `/sys/class/uio/*`: exact name,
  then `name@<unit-address>`, then — only if no name matched — the device whose
  `maps/map0/addr` is `base`.
- Generated code emits `open_named("ipc-shm" | "dbg-shm" | "mu1", <base>, …)`.

On the board's unmodified DTB:

```
UioDevice::open_named: ipc-shm resolved to uio1
UioDevice::open_named: dbg-shm resolved to uio2
UioDevice::open_named: mu1 resolved to uio0
consortium init: resources ready
connect(primary): publishing 2 channels, transfer_size 128
```

No SError. The primary published its descriptor and waited for the CM33's ack.

## Problem 4: The CM33 Stops Inside `init()`

The ack never came. `examples/utils/defmt-host` now takes the firmware ELF at runtime
and a UIO name or number:

```
root@imx93-11x11-lpddr4x-frdm:~/consortium-test# ./consortium-example-defmt-host cm33_original.elf dbg-shm-uio
consortium controller init: mpu/nvic bring-up
consortium controller init: MPU carveout 0 base=0x20480000 size=0x00004000
consortium controller init: NVIC enabled MU1_B at priority 0x80 (id=22)
CDBG uio2: mapped_len=0x4000 capacity=0x2000 read_ptr=0x10 write_ptr=0x3c pending~=0x2c flags=Flags(0x0)
drained 3 CDBG packet(s): decoded 3 defmt frame(s), 0 decode error(s)
```

The next trace, `time driver up`, follows `timer_driver::bringup(LPIT1)` — the first
access to a peripheral that is neither TCM, OCRAM, nor the System Control Space.

### Cause: Linux Gated the Clocks

`consortium-hal-imx9::init()` only hands out singletons; nothing enables a clock. On
i.MX 95 the System Manager owns that. On i.MX 93 Linux does, and `clk_disable_unused`
turns off every gate without a consumer:

```
          lpit1                      0       0        0        133333334   0          0     50000      N            deviceless
          mu1_b                      0       0        0        133333334   0          0     50000      N            deviceless
          mu1_a                      0       0        0        133333334   0          0     50000      Y            deviceless
          gpio2                      2       2        0        133333334   0          0     50000      Y            43810000.gpio
       lpi2c1                        0       0        1        24000000    0          0     50000      N         i2c@44340000
```

The kernel's `clk-imx93.c` explains the asymmetry:

```c
{ IMX93_CLK_MU1_A_GATE, "mu1_a", "bus_aon_root", 0x84c0, CLK_IGNORE_UNUSED, ... },
{ IMX93_CLK_MU1_B_GATE, "mu1_b", "bus_aon_root", 0x8500, 0, &share_count_mub, ... },
{ IMX93_CLK_LPIT1_GATE, "lpit1", "bus_aon_root", 0x8a00, },
```

- `lpit1` is off: the CM33 stalls on its first LPIT1 access.
- `mu1_b` is off: the CM33's MU window would stall next. This is also why MU-B faulted
  from the CA55 in Problem 2 while MU-A — `CLK_IGNORE_UNUSED` — did not. NXP's firmware
  never meets it because it drives `MU1_MUA`.
- `lpi2c1` is off **and** bound by the kernel's `i2c@44340000` driver, although the
  manifest gives it to the CM33.

## Open Items

Progress stops here. What remains:

1. **Clock gates.** The generated CM33 `init()` must enable the LPCG of every peripheral
   it owns — its MU window, LPIT1, and each `[peripheral.*]` — before touching it, as
   NXP's `BOARD_InitHardware` does. Offsets come from `clk-imx93.c` (LPIT1 `0x8a00`, MU-B
   `0x8500`, GPIO2 `0x88c0`, LPI2C1 `0x8f00`).
2. **LPIT1 rate.** `timer_driver::bringup` assumes 24 MHz; Linux reports `lpit1` at
   ~133 MHz from `bus_aon_root`. Embassy time would run ~5.5× fast.
