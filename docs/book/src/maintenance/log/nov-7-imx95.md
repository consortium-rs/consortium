# i.MX 95, November 7, 2026 (Bringing Up the CM7)

The i.MX 95 port of melt-pot (`examples/melt-pot/imx95`, built from
`Consortium.bringup.toml`) pairs the CA55 Linux application with Cortex-M7 firmware on an
FRDM-i.MX95 (`imx95-15x15-lpddr4x-frdm`) running NXP's `6.18.37-2.1.0` BSP kernel. Unlike
the [i.MX 93](oct-1-imx93.md), a System Manager (SM) on the M33 owns access control and
clocks, so it decides what Linux and the M7 can touch. Shared memory is DDR
`0x88300000` (IPC) and `0x88304000` (debug ring), the window both the M7 and non-secure
Linux are granted, and the doorbell is MU7.

Each run uses `scripts/hil/run.sh imx95 dist-bringup`, which loads `firmware/cm7.elf`
through remoteproc, runs the application for 15 s, and grades the A-/M-stages from both
cores' logs.

## Step 1: Boot the SM's `rpmsg` Configuration

The stock FRDM image's SM config does not let Linux start the M7's logical machine, so
remoteproc cannot boot our firmware. In the Yocto build's `local.conf`:

```conf
IMXBOOT_VARIANT = "rpmsg"
```

The variant is resolved by the machine include. The FRDM machine conf pulls it in:

```console
$ grep -nE "IMXBOOT_VARIANT|require|include" .../imx95-15x15-lpddr4x-frdm.conf
9:require conf/machine/include/imx95-evk.inc
```

and `meta-freescale/conf/machine/include/imx95-evk.inc` maps the variant onto both an
SM config (built by `imx-sm`) and an `imx-boot` (`flash.bin`) target:

```bitbake
31:IMXBOOT_VARIANTS = "alt jailhouse netc rpmsg sof"
33:# Multiple system manager configs by IMXBOOT_VARIANT
34:SYSTEM_MANAGER_CONFIG = "${@bb.utils.contains('IMXBOOT_VARIANT', 'alt', 'mx95alt', \
   ...
37:    bb.utils.contains('IMXBOOT_VARIANT', 'rpmsg', 'mx95evkrpmsg', \
   ...
41:# imx-boot (flash.bin) targets based on UBOOT_CONFIG and IMXBOOT_VARIANT
42:IMXBOOT_TARGETS_SD = "${@bb.utils.contains('IMXBOOT_VARIANT', 'alt', '${IMXBOOT_TARGETS_BASENAME}_alt', \
   ...
45:    bb.utils.contains('IMXBOOT_VARIANT', 'rpmsg', '${IMXBOOT_TARGETS_BASENAME}_lpboot_sm_a55', \
```

With the `rpmsg` image flashed, remoteproc starts the M7:

```
remoteproc remoteproc0: Booting fw image consortium-cm7.elf, size 3987136
imx-rproc imx95-cm7: lmm(1) powered on by Linux
remoteproc remoteproc0: remote processor imx-rproc is now up
```

The FRDM's rpmsg config grants Linux the M7 TCM but not DDR `0x80000000..`, so the
firmware runs from TCM (the `[remoteproc.cm7]` override in the manifest).

## Step 2: Tailor the Right Device Tree

The Consortium nodes (MU7 and the two shared windows as `generic-uio`, plus their
`reserved-memory` carve-outs) have to be added to the tree the board boots. The first
attempt added them to a stale copy of the **plain** `imx95-15x15-frdm.dtb`, and the
kernel died during boot. The UIO match was passed from U-Boot as before:

```
setenv mmcargs "${mmcargs} uio_pdrv_genirq.of_id=generic-uio"
boot
```

```
[    2.378489] Tainted: [M]=MACHINE_CHECK
[    2.378491] Hardware name: NXP FRDM-IMX95 (DT)
[    2.378494] Workqueue: events_unbound deferred_probe_work_func
[    2.378511] pstate: 60400009 (nZCv daif +PAN -UAO -TCO -DIT -SSBS BTYPE=--)
[    2.378516] pc : fsl_edma_chan_mux+0xa0/0xe8
[    2.378523] lr : fsl_edma_probe+0x470/0x9a0
[    2.378527] sp : ffff800082c0ba50
[    2.378529] x29: ffff800082c0ba70 x28: 0000000000000002 x27: ffff000084000080
[    2.378535] x26: 0000000000000002 x25: ffff000084000508 x24: 0000000000000001
[    2.378541] x23: ffff0001ff02cb40 x22: ffff000080207400 x21: ffff000080207410
[    2.378547] x20: ffff0000840009c8 x19: ffff80008212dcf0 x18: 000000000000000a
[    2.378552] x17: ffff80007fffffff x16: ffff0000801395a0 x15: 0000000000000000
[    2.378558] x14: 00000000ffffffff x13: 0000000000000030 x12: 0000000000000002
[    2.378564] x11: ffff000084000b83 x10: ffff000084000b8c x9 : 0000000000000000
[    2.378569] x8 : 000000007272652d x7 : 0000000000000004 x6 : 0000000000000000
[    2.378575] x5 : ffff80008212dcf0 x4 : 0000000000000000 x3 : ffff000084000080
[    2.378580] x2 : 0000000000000000 x1 : 0000000000000000 x0 : ffff0000840009c8
[    2.378588] Kernel panic - not syncing: Asynchronous SError Interrupt
[    2.378591] CPU: 5 UID: 0 PID: 61 Comm: kworker/u24:3 Tainted: G   M                6.18.37-2.1.0-g6e694eda40aa #1 PREEMPT
[    2.378597] Tainted: [M]=MACHINE_CHECK
[    2.378598] Hardware name: NXP FRDM-IMX95 (DT)
[    2.378600] Workqueue: events_unbound deferred_probe_work_func
[    2.378605] Call trace:
[    2.378608]  show_stack+0x18/0x30 (C)
[    2.378617]  dump_stack_lvl+0x60/0x80
[    2.378624]  dump_stack+0x18/0x24
[    2.378629]  vpanic+0xf8/0x2dc
[    2.378635]  abort+0x0/0x4
[    2.378639]  add_taint+0x0/0xbc
[    2.378645]  arm64_serror_panic+0x70/0x80
[    2.378651]  arm64_is_fatal_ras_serror+0x34/0xac
[    2.378656]  do_serror+0x58/0x74
[    2.378662]  el1h_64_error_handler+0x38/0x60
[    2.378670]  el1h_64_error+0x6c/0x70
[    2.378674]  fsl_edma_chan_mux+0xa0/0xe8 (P)
[    2.378679]  fsl_edma_probe+0x470/0x9a0
[    2.378683]  platform_probe+0x5c/0xa4
[    2.378688]  really_probe+0xbc/0x2bc
[    2.378693]  __driver_probe_device+0x80/0x14c
[    2.378697]  driver_probe_device+0x3c/0x180
[    2.378702]  __device_attach_driver+0xb8/0x140
[    2.378707]  bus_for_each_drv+0x88/0xe8
[    2.378713]  __device_attach+0x9c/0x1a0
[    2.378718]  device_initial_probe+0x14/0x20
[    2.378722]  bus_probe_device+0xa4/0xb4
[    2.378726]  deferred_probe_work_func+0x7c/0xb4
[    2.378731]  process_one_work+0x150/0x298
[    2.378737]  worker_thread+0x180/0x2f4
[    2.378741]  kthread+0x12c/0x204
[    2.378747]  ret_from_fork+0x10/0x20
[    2.378755] SMP: stopping secondary CPUs
[    2.378838] Kernel Offset: disabled
[    2.378840] CPU features: 0x100000,00008000,48046281,0400721b
[    2.378843] Memory Limit: none
[    2.651703] ---[ end Kernel panic - not syncing: Asynchronous SError Interrupt ]---
```

The SError arrives in kernel context (`el1h_64_error`) from a deferred-probe worker, not
from userspace, so it happens during boot before any Consortium code runs.

An asynchronous SError inside `fsl_edma_chan_mux` is a bus error on an eDMA channel-mux
register. Comparing the two trees' eDMA nodes shows which controller. They are identical
except for one property on eDMA1 (interrupt lists elided):

```dts
/* imx95-15x15-frdm-rpmsg.dtb */
dma-controller@42000000 {
    compatible = "fsl,imx95-edma5";
    reg = <0x42000000 0x210000>;
    dma-channels = <0x40>;
    dma-channel-mask = <0x03>;
    clocks = <0x11 0x77>;
    status = "disabled";
};

/* imx95-15x15-frdm.dtb */
dma-controller@42000000 {
    compatible = "fsl,imx95-edma5";
    reg = <0x42000000 0x210000>;
    dma-channels = <0x40>;
    dma-channel-mask = <0x03>;
    clocks = <0x11 0x77>;
    /* no status: enabled */
};
```

`dma-controller@42210000` (eDMA2) is enabled in both. The `rpmsg` SM config does not grant
the A55 eDMA1 (`0x42000000`), presumably leaving it to the M7, and the rpmsg tree keeps
Linux off it accordingly. The plain tree still enables it, so `fsl_edma_probe` binds
it and the first channel-mux write from the A55 is rejected as a bus error. The plain
FRDM tree and the rpmsg boot image do not belong together.

**Fix:** tailor NXP's `imx95-15x15-frdm-rpmsg.dtb`, the tree built for the `rpmsg` SM
configuration, which already disables eDMA1. Booted with the Consortium nodes added, it
comes up cleanly. (Disabling `dma-controller@42000000` in the plain tree would address
this particular panic, but the rpmsg tree carries every adjustment that SM config needs.)

Two more things to know about this tree. Mainline Linux has no `imx95-15x15-frdm-rpmsg`
source and its `imx95-15x15-frdm.dts` has no `remoteproc-cm7` node, so `csti build`
against the mainline checkout needs `--skip-remoteproc-check` and its generated DTS
cannot be booted as is. Its stock `imx95-cm7` node claims MU7 through `mboxes`, and
the tailored copy removes them because the UIO doorbell owns MU7 (hence `No initialized
mbox tx channel` at start, which is expected).

## Step 3: UIO Devices Without Maps

The application failed in generated `init()`, before any stage could pass:

```
root@imx95-15x15-lpddr4x-frdm:~/imx95# ./run.sh imx95 dist-bringup
== Stop the previous run
== Clear the stale IPC descriptor at 0x88300000
   no devmem2/devmem/memtool: skipped (a stale descriptor can make M6 fail)
== Load dist-bringup/firmware/cm7.elf on remoteproc0 (imx-rproc)
[  306.794855] remoteproc remoteproc0: powering up imx-rproc
[  306.797074] remoteproc remoteproc0: Booting fw image consortium-cm7.elf, size 3987136
[  306.797365] imx-rproc imx95-cm7: lmm(1) powered on by Linux
[  306.855537] rproc-virtio rproc-virtio.2.auto: assigned reserved memory node vdevbuffer@88020000
[  306.856509] imx-rproc imx95-cm7: No initialized mbox tx channel
[  306.856643] rproc-virtio rproc-virtio.2.auto: registered virtio0 (type 7)
[  306.856666] remoteproc remoteproc0: remote processor imx-rproc is now up
== Run melt-pot-imx95-app for 15s (log: /tmp/hil-imx95-20261107-111938.log)
2026-11-07T11:19:39.273558Z ERROR consortium_runtime_app::__rt: consortium main failed: Init(Uio(Uio(Io(Os { code: 2, kind: NotFound, message: "No such file or directory" }))))

== Stages
  A1   MISSING application started
  A2   MISSING firmware loaded (remoteproc running)
  M1   MISSING firmware launched, defmt ring up
  A3   MISSING UIO windows mapped
  ...
  M8   MISSING M-core saw the message consumed

RESULT: FAIL (/tmp/hil-imx95-20261107-111938.log)
```

The devices themselves were there:

```
root@imx95-15x15-lpddr4x-frdm:~/imx95# ls /sys/class/uio
uio0  uio1  uio2
root@imx95-15x15-lpddr4x-frdm:~/imx95# cat /sys/class/uio/uio0/name
dbg-shm
root@imx95-15x15-lpddr4x-frdm:~/imx95# cat /sys/class/uio/uio1/name
ipc-shm
root@imx95-15x15-lpddr4x-frdm:~/imx95# cat /sys/class/uio/uio2/name
mu7
```

So name resolution (`UioDevice::open_named`) and opening `/dev/uioN` both succeeded:
a failed open surfaces as `UioDeviceError::Io`, not the nested `Uio(Io(..))`. The inner
`UioError::Io` comes only from the `uio` crate's sysfs readers, and the first one called
is `map_mapping(0)` → `map_size(0)`, which reads `maps/map0/size`:

```
root@imx95-15x15-lpddr4x-frdm:~/imx95# ls /sys/class/uio/uio*/maps/
map0
root@imx95-15x15-lpddr4x-frdm:~/imx95# ls /sys/class/uio/uio2/maps
map0
```

The glob matched one `maps/` directory: only `uio2` (`mu7`) had a map. Generated init
opens `mu7`, `ipc-shm`, `dbg-shm` in that order and then maps `ipc-shm` first
(`UioRegion::zeroed(&uio[1], 0)`), so that is where it stopped.

The nodes were also not where the generated tree puts them: `/proc/device-tree` had no
`mu7@42430000`, `ipc-shm@88300000` or `dbg-shm@88304000` at its root. They were further
down, under `/soc/bus@42000000`.

### Cause: Bus `ranges` Do Not Cover DDR

The tailored tree had copied the i.MX 93 pattern of putting the shared-memory nodes next
to the MU, inside the AIPS bus:

```dts
bus@42000000 {
    ranges = <0x42000000 0x00 0x42000000 0x8000000
              0x28000000 0x00 0x28000000 0x10000000>;
    #address-cells = <0x01>;
    #size-cells = <0x01>;

    mu7@42430000   { compatible = "generic-uio"; reg = <0x42430000 0x10000>; ... };
    dbg-shm@88304000 { compatible = "generic-uio"; reg = <0x88304000 0x2000>; ... };
    ipc-shm@88300000 { compatible = "generic-uio"; reg = <0x88300000 0x4000>; ... };
};
```

The bus translates only `0x4200_0000..0x4A00_0000` and `0x2800_0000..0x3800_0000`.
`mu7` falls inside; the DDR windows do not, so their `reg` cannot be translated, the
platform devices get no memory resource, and `uio_pdrv_genirq` registers them with no
`maps/`.

On the i.MX 93 the same placement works only by accident: `bus@44000000` has an empty
`ranges;` and `soc@0` maps `0x0..0x80000000`, which covers the OCRAM windows at
`0x20480000`. Moving the i.MX 95 windows from OCRAM to DDR (the SM grants the M7 no
OCRAM) is what broke the pattern.

Diffing the tailored tree against the stock rpmsg DTB found a second slip: the new
`mu7@42430000` node had been written over the stock `mailbox@42730000` (MU8). It was
disabled, so it was harmless here, but it should be restored.

**Fix:** move `ipc-shm` and `dbg-shm` under `/` (or directly under `soc`, whose
`ranges;` is empty), with 2-cell addresses:

```dts
ipc-shm@88300000 {
    compatible = "generic-uio";
    linux,uio-name = "ipc-shm";
    reg = <0x00 0x88300000 0x00 0x4000>;
    memory-region = <&ipc_shm>;
};
```

This is where `consortium-cfg`'s merge output already puts them. Afterwards all three
devices have `map0`, and the run passed:

```
== Stages
  A1   PASS    application started
  A2   PASS    firmware loaded (remoteproc running)
  M1   PASS    firmware launched, defmt ring up
  A3   PASS    UIO windows mapped
  A4   PASS    defmt shared memory initialized
  M2   PASS    interrupt routing (vector table, NVIC)
  M3   PASS    M-core waiting for connection (line armed)
  A5   PASS    connection requested
  M4   PASS    M-core sees the published descriptor
  A6   -       AP->M doorbell asserted (sampled)
  M5   PASS    M-core interrupt fired
  A7   PASS    M-core consumed the doorbell
  M6   PASS    M-core connected and acked
  A8   PASS    A-core interrupt fired
  A9   PASS    A-core responded (handshake complete)
  M7   PASS    M-core message sent
  A10  PASS    message in the slot
  A11  PASS    A-core message interrupt fired
  A12  PASS    A-core message read
  M8   PASS    M-core saw the message consumed

RESULT: PASS (/tmp/hil-imx95-20261107-113734.log)
```

(A6 is a best-effort sample of a microsecond window and is not required.)

## Step 4: The CM7 Clock Runs 5.5× Fast

The passing run sent one sample per firmware second (`Timer::after_secs(1)` at the end of
the CM7's send loop), and its steady-state summary is meant to come every 10 s. Both came
far too often:

```
2026-11-07T11:37:48.890098Z  INFO consortium::dbg: sensor seq=71 temp_mdc=25500 core="cm7"
2026-11-07T11:37:48.891089Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:37:49.047701Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:37:49.092753Z  INFO consortium::dbg: sensor seq=72 temp_mdc=26000 core="cm7"
2026-11-07T11:37:49.093324Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:37:49.227935Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:37:49.244713Z  INFO consortium::dbg: sensor seq=73 temp_mdc=26500 core="cm7"
2026-11-07T11:37:49.245606Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:37:49.347161Z  INFO consortium::dbg: HIL M9 steady: sent=74 consumed=74 send_errors=0 last seq=73 core="cm7"
2026-11-07T11:37:49.408005Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:37:49.447902Z  INFO consortium::dbg: sensor seq=74 temp_mdc=27000 core="cm7"
...
2026-11-07T11:37:50.668650Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:37:50.713536Z  INFO consortium::dbg: sensor seq=81 temp_mdc=20500 core="cm7"
2026-11-07T11:37:50.714527Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:37:50.848599Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
```

The `core="cm7"` lines carry the time the AP drained them from the defmt ring, so their
spacing jitters (0.15–0.20 s). The AP's own `Channel::recv` lines do not:
`49.0477 → 49.2279 → 49.4080 → … → 50.8486`, 10 intervals in 1.8009 s, or 180 ms per
firmware second.

`consortium-hal-imx9`'s `LPIT1_RATE` was 24 MHz for i.MX 95. If the driver assumes 24 MHz
and the counter really runs at `X`, one firmware second lasts `24 MHz / X`, and
`24 / X = 0.18` gives `X = 400/3 MHz`. That is the same `bus_aon_root` rate the
[i.MX 93 log](oct-1-imx93.md) found for its LPIT1. The SM clocks LPIT1 from the 133 MHz
bus/AON root, not the 24 MHz oscillator.

**Fix:** `LPIT1_RATE = Rate::ratio(400_000_000, 3)` for every i.MX9 family (i.MX 952
follows i.MX 95, still unconfirmed on hardware). The rebuilt `cm7.elf` passes it to
`init_timer`:

```
bringup:  movw r0, #0x8400 ; movt r0, #0x17d7   @ 400_000_000
          movs r1, #3
          bl   init_timer
```

## Step 5: The Second Run Fails After a Warm `reboot` (Open)

The next runs, including the first after a `reboot`, failed after the handshake: no CM7
message was ever read. The first run after the reboot
(`hil-imx95-20261107-114955.log`), from the top:

```
2026-11-07T11:49:56.756551Z  INFO melt_pot_imx95_app::hil: HIL A1 PASS application started (pid 941)
2026-11-07T11:49:56.760262Z  INFO melt_pot_imx95_app::hil: HIL A2 PASS firmware loaded: remoteproc0 (imx-rproc): state=running firmware=consortium-cm7.elf
2026-11-07T11:49:56.762567Z  INFO melt_pot_imx95_app::hil: HIL A3 PASS UIO windows mapped: ipc-shm @ 0x88300000, dbg-shm @ 0x88304000, mu7 @ 0x42430000; mu7 IRQ count 1
2026-11-07T11:49:56.763750Z  INFO melt_pot_imx95_app::hil: HIL A4 PASS the CM7 initialized its defmt ring: cap=8192 write=573 read=16 flags=0x0
2026-11-07T11:49:56.765182Z  INFO melt_pot_imx95_app::hil: HIL A5 PASS connection requested: publishing the descriptor, ringing mu7 TR2
2026-11-07T11:49:56.765369Z  INFO melt_pot_imx95_app::hil: HIL A7 PASS (inferred) the CM7 consumed the doorbell between samples; its ack proves its IRQ path
2026-11-07T11:49:56.765912Z  INFO melt_pot_imx95_app::hil: HIL A6 PASS AP->M doorbell asserted (mu7 TR2)
2026-11-07T11:49:56.766162Z  WARN melt_pot_imx95_app::hil: HIL A8 mu7 count did not move (1) although the handshake completed
2026-11-07T11:49:56.766219Z  INFO melt_pot_imx95_app::hil: HIL A9 PASS handshake complete: the CM7 accepted the descriptor and acked
2026-11-07T11:49:56.766264Z  INFO melt_pot_imx95_app: melt-pot: CA55 application started; IPC bring-up complete
2026-11-07T11:49:56.789946Z  INFO consortium_dbg::host: defmt table parsed: encoding = Rzcobs, has_timestamp = true
2026-11-07T11:49:56.791305Z  INFO consortium::dbg: HIL M1 PASS firmware launched: defmt ring up, VTOR=0x00000000, vector table at 0x00000000 (240 IRQ slots) core="cm7"
2026-11-07T11:49:56.791551Z  INFO consortium::dbg: HIL M2 PASS doorbell MU7_B IRQ207 vector 0x00002339 reaches its handler, NVIC enabled core="cm7"
2026-11-07T11:49:56.791752Z  INFO consortium::dbg: HIL M2 PASS time driver LPIT1 IRQ15 vector 0x0000232f reaches its handler, NVIC enabled core="cm7"
2026-11-07T11:49:56.791851Z  INFO consortium::dbg: HIL M2 PASS interrupt routing core="cm7"
2026-11-07T11:49:56.792015Z  INFO consortium::dbg: HIL M3 entering Connect: waiting for the AP's descriptor and its doorbell on MU7 RF2 core="cm7"
2026-11-07T11:49:56.792554Z  INFO consortium::dbg: HIL M4 PASS connection requested: the AP published the descriptor (confirmed at connect) core="cm7"
2026-11-07T11:49:56.792685Z  INFO consortium::dbg: HIL M5 PASS doorbell interrupt fired: Connect was parked on it and only MU7_B could wake it core="cm7"
2026-11-07T11:49:56.792829Z  INFO consortium::dbg: HIL M6 PASS connected: doorbell IRQ serviced, descriptor accepted, AP acked on MU7 TR3 core="cm7"
2026-11-07T11:49:56.792960Z  INFO consortium::dbg: sensor seq=0 temp_mdc=20000 core="cm7"
2026-11-07T11:49:56.793214Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:49:56.793521Z  WARN consortium::dbg: HIL M7 send of seq=0 failed: the AP has not consumed the previous message core="cm7"
2026-11-07T11:49:56.793746Z  INFO consortium::dbg: HIL trace MU7B: RSR=0x0 RCR=0x0 TSR=0x7 slot flags=0x1 descriptor magic=0x43495043 core="cm7"
2026-11-07T11:49:56.793877Z  INFO consortium::dbg: sensor seq=1 temp_mdc=20500 core="cm7"
2026-11-07T11:49:56.794097Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:49:56.794371Z  INFO consortium::dbg: sensor seq=2 temp_mdc=21000 core="cm7"
2026-11-07T11:49:56.794591Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:49:56.794877Z  WARN consortium::dbg: HIL M8 the AP has not read seq=2 for 3 s (is the application running?) core="cm7"
2026-11-07T11:49:56.795101Z  INFO consortium::dbg: HIL trace MU7B: RSR=0x0 RCR=0x0 TSR=0x7 slot flags=0x1 descriptor magic=0x43495043 core="cm7"
...
2026-11-07T11:49:56.796955Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:49:56.797367Z ERROR consortium::dbg: HIL M5 FAIL the AP's doorbell is asserted but was not serviced: NVIC enabled=true pending=false active=false, ICSR=0x00400000, PRIMASK=true core="cm7"
2026-11-07T11:49:56.797579Z ERROR consortium::dbg: HIL M5 trace vector fetched=0x00002339 linked=0x00002339 align=0x400 VTOR=0x00000000 core="cm7"
2026-11-07T11:49:56.797801Z  INFO consortium::dbg: HIL trace MU7B: RSR=0x4 RCR=0x0 TSR=0xf slot flags=0x0 descriptor magic=0x43495043 core="cm7"
2026-11-07T11:49:56.847014Z  INFO melt_pot_imx95_app::hil: HIL A10 PASS the CM7's first message is in the slot (4 bytes)
2026-11-07T11:49:56.848399Z  INFO consortium::dbg: sensor seq=7 temp_mdc=23500 core="cm7"
2026-11-07T11:49:56.848898Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:49:56.849551Z  INFO consortium::dbg: HIL M7 PASS first message sent: seq=7, slot written, doorbell rung on TR3 core="cm7"
...
2026-11-07T11:49:57.405814Z  INFO consortium::dbg: HIL M9 steady: sent=1 consumed=0 send_errors=9 last seq=9 core="cm7"
...
2026-11-07T11:49:59.765146Z ERROR melt_pot_imx95_app::hil: HIL A13 FAIL no message for 3 s (received 0 so far)
2026-11-07T11:49:59.765290Z ERROR melt_pot_imx95_app::hil: HIL trace (A13):
2026-11-07T11:49:59.765398Z ERROR melt_pot_imx95_app::hil: HIL trace   doorbell mu7: AP->M asserted=true M->AP asserted=true AP armed=true | MUA TSR=0xb RCR=0x8 RSR=0x8
2026-11-07T11:49:59.765486Z ERROR melt_pot_imx95_app::hil: HIL trace   descriptor magic=0x43495043 (CIPC=0x43495043) version/channels=0x00020001 state=3 rx mtu=128
2026-11-07T11:49:59.765576Z ERROR melt_pot_imx95_app::hil: HIL trace   channel table: [0x100 0x8c 0x0] [0x1c0 0x8c 0x1]
2026-11-07T11:49:59.765650Z ERROR melt_pot_imx95_app::hil: HIL trace   slot AP->M @+0x100: len=0 flags=0x0 crc=0x00000000
2026-11-07T11:49:59.765715Z ERROR melt_pot_imx95_app::hil: HIL trace   slot M->AP @+0x1c0: len=4 flags=0x1 crc=0x575c76eb
2026-11-07T11:49:59.765787Z ERROR melt_pot_imx95_app::hil: HIL trace   CDBG magic=0x43444247 write=1327 read=1288 cap=8192 flags=0x0
2026-11-07T11:49:59.766849Z ERROR melt_pot_imx95_app::hil: HIL trace   /proc/interrupts: 175: 1 0 0 0 0 0 GICv3 266 Level mu7
2026-11-07T11:49:59.767296Z ERROR melt_pot_imx95_app::hil: HIL trace   remoteproc0 (imx-rproc): state=running firmware=consortium-cm7.elf
...
2026-11-07T11:50:06.767091Z  INFO melt_pot_imx95_app::hil: HIL A13 steady: received=0 last seq=0 gaps=0 mu7 IRQs=1 ring flags=0x0
```

and its stage table:

```
  M3   MISSING M-core waiting for connection (line armed)
  A8   MISSING A-core interrupt fired
  A11  MISSING A-core message interrupt fired
  A12  MISSING A-core message read
  M8   MISSING M-core saw the message consumed
  (every other stage PASS)

== Failures and traces
WARN consortium::dbg: HIL M7 send of seq=0 failed: the AP has not consumed the previous message core="cm7"
```

The `core="cm7"` lines are stamped when the AP drains the ring, not when the CM7 wrote
them, so their order relative to the AP's lines is misleading. The ring position settles
it: at A4, before the application had published anything, the ring already held
`write=573` bytes. In other words the CM7 had logged M1–M6 and its first sends
(including `M8 … for 3 s`) before the application reached A5. A later run
(`hil-imx95-20261107-115556.log`) is identical, down to `mu7 IRQ count 1` at A3 and
`write=621` at A4.

Two bugs combine here:

1. **The CM7 accepts a stale descriptor.** A warm `reboot` keeps DDR, and `run.sh` could
   not clear `0x88300000` (the board has no `devmem`). The previous application, killed
   by `run.sh`, left a published descriptor (`state=3`) and an unread message behind. The
   CM7 boots before the application, finds the old descriptor, and acks it on TR3. When the
   application later zeroes the window, republishes and rings TR2, nothing consumes the
   ring: the CM7 has already left `Connect`. The application's `connect` takes the stale
   ack still pending in RF3 as its reply, so A9 is a false pass.
2. **The UIO interrupt is never re-armed on open.** That stale ack is the single `mu7`
   interrupt, already counted at A3. `uio_pdrv_genirq` requests the IRQ at probe, and its
   handler masks the line until userspace writes `1`. A reader only sees events after it
   opens `/dev/uioN`, so `irq_loop` never wakes, never re-arms, and the line stays masked
   for the whole run, with RF3 full and enabled (`RCR=0x8 RSR=0x8`).
   `consortium-runtime-app`'s `irq.rs` assumes "the kernel armed the line at device
   open", which is wrong for an interrupt taken earlier.

A full power-on reset clears DDR, and the next run (`hil-imx95-20261107-115956.log`)
passed every stage:

```
2026-11-07T11:59:58.545799Z  INFO melt_pot_imx95_app::hil: HIL A1 PASS application started (pid 915)
2026-11-07T11:59:58.549682Z  INFO melt_pot_imx95_app::hil: HIL A2 PASS firmware loaded: remoteproc0 (imx-rproc): state=running firmware=consortium-cm7.elf
2026-11-07T11:59:58.551579Z  INFO melt_pot_imx95_app::hil: HIL A3 PASS UIO windows mapped: ipc-shm @ 0x88300000, dbg-shm @ 0x88304000, mu7 @ 0x42430000; mu7 IRQ count 0
2026-11-07T11:59:58.552384Z  INFO melt_pot_imx95_app::hil: HIL A4 PASS the CM7 initialized its defmt ring: cap=8192 write=214 read=16 flags=0x0
2026-11-07T11:59:58.553560Z  INFO melt_pot_imx95_app::hil: HIL A5 PASS connection requested: publishing the descriptor, ringing mu7 TR2
2026-11-07T11:59:58.554069Z  INFO melt_pot_imx95_app::hil: HIL A7 PASS (inferred) the CM7 consumed the doorbell between samples; its ack proves its IRQ path
2026-11-07T11:59:58.554916Z  INFO melt_pot_imx95_app::hil: HIL A8 PASS A-core interrupt fired: mu7 count 0 -> 1
2026-11-07T11:59:58.554985Z  INFO melt_pot_imx95_app::hil: HIL A9 PASS handshake complete: the CM7 accepted the descriptor and acked
2026-11-07T11:59:58.555034Z  INFO melt_pot_imx95_app: melt-pot: CA55 application started; IPC bring-up complete
2026-11-07T11:59:58.555115Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:59:58.555177Z  INFO melt_pot_imx95_app::hil: HIL A12 PASS message read: seq=0 temp_mdc=20000
2026-11-07T11:59:58.580211Z  INFO consortium_dbg::host: defmt table parsed: encoding = Rzcobs, has_timestamp = true
2026-11-07T11:59:58.581500Z  INFO consortium::dbg: HIL M1 PASS firmware launched: defmt ring up, VTOR=0x00000000, vector table at 0x00000000 (240 IRQ slots) core="cm7"
2026-11-07T11:59:58.581753Z  INFO consortium::dbg: HIL M2 PASS doorbell MU7_B IRQ207 vector 0x00002339 reaches its handler, NVIC enabled core="cm7"
2026-11-07T11:59:58.581980Z  INFO consortium::dbg: HIL M2 PASS time driver LPIT1 IRQ15 vector 0x0000232f reaches its handler, NVIC enabled core="cm7"
2026-11-07T11:59:58.582089Z  INFO consortium::dbg: HIL M2 PASS interrupt routing core="cm7"
2026-11-07T11:59:58.582229Z  INFO consortium::dbg: HIL M3 entering Connect: waiting for the AP's descriptor and its doorbell on MU7 RF2 core="cm7"
2026-11-07T11:59:58.582576Z  INFO consortium::dbg: HIL M3 PASS parked in Connect: MU7 RF2 armed (RCR.RIE set) core="cm7"
2026-11-07T11:59:58.582972Z  INFO consortium::dbg: HIL M4 PASS connection requested: the AP published the descriptor (confirmed at connect) core="cm7"
2026-11-07T11:59:58.583102Z  INFO consortium::dbg: HIL M5 PASS doorbell interrupt fired: Connect was parked on it and only MU7_B could wake it core="cm7"
2026-11-07T11:59:58.583248Z  INFO consortium::dbg: HIL M6 PASS connected: doorbell IRQ serviced, descriptor accepted, AP acked on MU7 TR3 core="cm7"
2026-11-07T11:59:58.583380Z  INFO consortium::dbg: sensor seq=0 temp_mdc=20000 core="cm7"
2026-11-07T11:59:58.583628Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T11:59:58.583954Z  INFO consortium::dbg: HIL M7 PASS first message sent: seq=0, slot written, doorbell rung on TR3 core="cm7"
2026-11-07T11:59:58.584092Z  INFO consortium::dbg: HIL M8 PASS the AP read the first message (seq=0): slot consumed core="cm7"
2026-11-07T11:59:58.734825Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T11:59:58.734756Z  INFO melt_pot_imx95_app::hil: HIL A10 PASS the CM7's first message is in the slot (4 bytes)
2026-11-07T11:59:58.745053Z  INFO melt_pot_imx95_app::hil: HIL A11 PASS message interrupt fired: mu7 count 1 -> 2
...
2026-11-07T11:59:59.193489Z  INFO consortium::dbg: HIL M9 steady: sent=4 consumed=4 send_errors=0 last seq=3 core="cm7"
...
2026-11-07T12:00:08.554736Z  INFO melt_pot_imx95_app::hil: HIL A13 steady: received=56 last seq=55 gaps=0 mu7 IRQs=56 ring flags=0x0
...
2026-11-07T12:00:10.467164Z  INFO consortium::dbg: sensor seq=66 temp_mdc=23000 core="cm7"
2026-11-07T12:00:10.468079Z  INFO consortium::dbg: Channel::send: encoded 4 bytes on Chan(7) core="cm7"
2026-11-07T12:00:51.475601Z  INFO consortium_ipc::channel: Channel::recv: received 4 bytes on Chan(7)
2026-11-07T12:00:51.524711Z  INFO consortium::dbg: sensor seq=67 temp_mdc=23500 core="cm7"
...
2026-11-07T12:00:52.642910Z  INFO consortium::dbg: HIL M9 steady: sent=74 consumed=74 send_errors=0 last seq=73 core="cm7"
```

The differences from the failing runs are each part of the explanation: `mu7 IRQ count 0`
at A3 (nothing rang before the application opened the device), a much shorter ring at A4
(`write=214`), `M3 PASS parked in Connect` (the CM7 waited for this session's
descriptor), A8 counting `0 -> 1` on this session's ack, and one `mu7` interrupt per
message afterwards (`mu7 IRQs=56` for `received=56`). It still paces at 180 ms per
firmware second, which Step 6 explains.

(That log also shows a 41 s hole in its timestamps with no gap in sequence numbers, and
`timeout 15` still ended the run on time. That was the wall clock stepping forward after a
cold boot, not a stall.)

**Still open:**

- Write `1` to the UIO listener fd when the device is opened, then run the bound
  services once, so an interrupt taken earlier cannot leave the line masked.
- Make the secondary reject an earlier session's descriptor. A per-session nonce, echoed
  in the ack and checked by `connect`, would stop the AP from completing on a stale RF3.
  Clearing the window from `run.sh` would only hide the problem on the test rig.

## Step 6: Still Fast? The Firmware Never Changed

The clean run still paced at exactly 0.18 s. A board running the 400/3 MHz build cannot
do that unless LPIT1 counts at ~740 MHz, so the checksums were compared:

| File               | Mac         | Board       |
| ------------------ | ----------- | ----------- |
| `cm7.elf`          | `b7ab0608…` | `5957436c…` |
| `cm7_original.elf` | `ac7da35c…` | `96c95f0f…` |

The Mac side, and the board side before the re-upload:

```
(consortium) ethanwu@Ethans-MacBook-Pro melt-pot % find . -xdev -name 'cm7*.elf' -o -name 'consortium-cm7.elf' 2>/dev/null | xargs md5sum
b7ab0608c108e977033b54e9294b8ab7  ./imx95/dist-bringup/firmware/cm7.elf
ac7da35c9fd1000dc54573f43fa3d4f6  ./imx95/dist-bringup/firmware/cm7_original.elf

root@imx95-15x15-lpddr4x-frdm:~/imx95/dist-bringup# md5sum ./firmware/cm7.elf ./firmware/cm7_original.elf
5957436ce1104bc8cc7767afc03eac48  ./firmware/cm7.elf
96c95f0f8277847bddb397ab43af6db3  ./firmware/cm7_original.elf
```

A fresh copy, checked straight away and again after unplugging and replugging the
board:

```
(consortium) ethanwu@Ethans-MacBook-Pro melt-pot % scp -r ~/Developer/consortium/examples/melt-pot/imx95/dist-bringup root@imx95-15x15-lpddr4x-frdm.local:~/imx95/ && ssh root@imx95-15x15-lpddr4x-frdm.local 'md5sum ~/imx95/dist-bringup/firmware/cm7.elf'
consortium-imx95-15x15-frdm.dts                  100%   26KB   4.7MB/s   00:00
consortium-imx95-19x19-evk.dts                   100%   30KB   7.0MB/s   00:00
consortium-imx95-15x15-frdm.dtb                  100%   70KB   6.3MB/s   00:00
consortium-imx95-19x19-evk.dtb                   100%   73KB   6.6MB/s   00:00
cm7.elf                                          100% 3894KB  10.9MB/s   00:00
cm7_original.elf                                 100% 3915KB  10.9MB/s   00:00
melt-pot-imx95-app                               100%   58MB  11.0MB/s   00:05
README                                           100%  730   199.6KB/s   00:00
boot.cmd                                         100%  471   166.2KB/s   00:00
consortium-app.service                           100%  236   142.8KB/s   00:00
linux-imx_%.bbappend                             100%  397   213.8KB/s   00:00
consortium.cfg                                   100%  148    96.1KB/s   00:00
b7ab0608c108e977033b54e9294b8ab7  /root/imx95/dist-bringup/firmware/cm7.elf

  (board unplugged and plugged back in)

(consortium) ethanwu@Ethans-MacBook-Pro melt-pot % ssh root@imx95-15x15-lpddr4x-frdm.local 'md5sum ~/imx95/dist-bringup/firmware/cm7.elf'
5957436ce1104bc8cc7767afc03eac48  /root/imx95/dist-bringup/firmware/cm7.elf
```

The copy arrived intact (`b7ab0608…`) and then reverted to `5957436c…` across the power
cycle. The upload sat in the page cache and was lost at power-off
before writeback reached the eMMC. Every "new firmware" run since Step 4, the clean one
included, had run the old 24 MHz build.

**Fix:** flush before pulling power, and verify after the power cycle:

```sh
scp -r dist-bringup root@imx95-15x15-lpddr4x-frdm.local:~/imx95/ \
  && ssh root@imx95-15x15-lpddr4x-frdm.local 'sync && md5sum ~/imx95/dist-bringup/firmware/cm7.elf'
```

A warm `reboot` or `poweroff` syncs on its own. Pulling power does not, and pulling power
is also how Step 5's stale descriptor gets cleared, so the two interact: always `sync`
first.

## Open Items

1. **UIO re-arm on open** (`consortium-runtime-app`), Step 5.
2. **Session check in the shared-memory handshake** (`consortium-ipc-transport-memory`),
   Step 5.
3. **Clock fix on hardware.** A HIL log with the synced 400/3 MHz `cm7.elf` should show AP
   `Channel::recv` about 1.000 s apart and `HIL M9 steady` about every 10 s.
4. **DTS from the pipeline.** `csti` cannot render the rpmsg tree from a mainline checkout;
   until it is pointed at linux-imx, the tailored tree is maintained by hand, and the
   shared-memory nodes must stay outside `bus@42000000`.
