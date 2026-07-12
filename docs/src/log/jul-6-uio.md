# UIO, July 6, 2026 (Compiling into the Kernel)

Consortium uses UIO for Cortex-A-side handling. UIO provides two capabilities required by Consortium:

1. Interrupt routing. `uio_pdrv_genirq` allows an interrupt to be handled from user space.
2. Memory mapping. UIO maps a specific physical memory region into an accessible virtual memory region.

## UIO Is Not Present in the Kernel

To get started, we first checked whether UIO support was included in the running kernel:

```sh
# Is it built-in? Any of these confirming = built-in, no build needed:
ls -d /sys/module/uio_pdrv_genirq 2>/dev/null
grep -w uio_pdrv_genirq /lib/modules/$(uname -r)/modules.builtin

# What did the kernel actually enable?
zcat /proc/config.gz 2>/dev/null | grep -iE 'CONFIG_UIO'
# if there's no /proc/config.gz:
grep -iE 'CONFIG_UIO' /boot/config-$(uname -r) 2>/dev/null

# Already-present uio devices?
ls -d /sys/class/uio/uio* 2>/dev/null
```

The result is:

```sh
root@stm32mp2-e3-aa-db:~# ls -d /sys/module/uio_pdrv_genirq 2>/dev/null
root@stm32mp2-e3-aa-db:~# grep -w uio_pdrv_genirq /lib/modules/$(uname -r)/modules.builtin
root@stm32mp2-e3-aa-db:~# zcat /proc/config.gz 2>/dev/null | grep -iE 'CONFIG_UIO'
# CONFIG_UIO is not set
root@stm32mp2-e3-aa-db:~# grep -iE 'CONFIG_UIO' /boot/config-$(uname -r) 2>/dev/null
root@stm32mp2-e3-aa-db:~# ls -d /sys/class/uio/uio* 2>/dev/null
root@stm32mp2-e3-aa-db:~#
```

Enabling UIO required rebuilding the kernel with Yocto. Following [Yocto, July 3](jul-3-yocto.md), we modified `config.cfg`:

```conf
CONFIG_UIO=y
CONFIG_UIO_PDRV_GENIRQ=y
CONFIG_UIO_DMEM_GENIRQ=y
CONFIG_OF_OVERLAY=y
CONFIG_OF_CONFIGFS=y
CONFIG_CONFIGFS_FS=y
```

After the rebuild, the kernel included UIO support. However, no UIO device appeared, even though the DTS node had been added.

## UIO Is in the Kernel, but No Device Is Available

The node was present in the device tree, but the corresponding UIO device was not:

```sh
root@imx95-15x15-lpddr4x-frdm:~# ls /sys/firmware/devicetree/base/ | grep -E "mu7|ipc-shm|dbg-shm"
dbg-shm@20484000
ipc-shm@20480000
mu7@42430000
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# cat /sys/firmware/devicetree/base/mu7@42430000/compatible
generic-uio
root@imx95-15x15-lpddr4x-frdm:~# cat /sys/firmware/devicetree/base/mu7@42430000/compatible
generic-uio
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# ls /sys/class/uio/
root@imx95-15x15-lpddr4x-frdm:~# dmesg | grep -iE "uio|42430000"
root@imx95-15x15-lpddr4x-frdm:~#
```

Because `dmesg` contained no indication that the driver had loaded, we checked the kernel configuration again:

```sh
root@imx95-15x15-lpddr4x-frdm:~# modprobe uio_pdrv_genirq
modprobe: FATAL: Module uio_pdrv_genirq not found in directory /lib/modules/6.18.20-2.0.0-gb096ce610e95

root@imx95-15x15-lpddr4x-frdm:~# zcat /proc/config.gz | grep UIO
CONFIG_FEC_UIO=y
CONFIG_UIO=y
# CONFIG_UIO_CIF is not set
# CONFIG_UIO_PDRV_GENIRQ is not set
# CONFIG_UIO_DMEM_GENIRQ is not set
# CONFIG_UIO_AEC is not set
# CONFIG_UIO_SERCOS3 is not set
CONFIG_UIO_PCI_GENERIC=y
# CONFIG_UIO_NETX is not set
# CONFIG_UIO_MF624 is not set
CONFIG_UIO_PRIME=y
CONFIG_UIO_IVSHMEM=y
CONFIG_CRYPTO_DEV_FSL_CAAM_JR_UIO=m
```

The output contained `CONFIG_UIO_PDRV_GENIRQ is not set`, even though we thought we had enabled it in the Linux configuration. Inspecting the Yocto build configuration revealed the following line:

```conf
CONFIG_UIO_PDRV_GENERIC=y
```

We had mistakenly written `GENERIC`; the correct suffix is `GENIRQ`. We corrected the configuration, rebuilt the kernel, reflashed the board, and tried again:

```sh
root@imx95-15x15-lpddr4x-frdm:~# lsmod | grep uio
enetc4_uio             20480  0
root@imx95-15x15-lpddr4x-frdm:~# cat /proc/iomem | grep -A2 -B2 42430000
42000000-4220ffff : 42000000.dma-controller dma-controller@42000000
42210000-4241ffff : 42210000.dma-controller dma-controller@42210000
42430000-4243ffff : 42430000.mailbox mailbox@42430000
42490000-4249ffff : 42490000.watchdog watchdog@42490000
42530000-4253ffff : 42530000.i2c i2c@42530000
root@imx95-15x15-lpddr4x-frdm:~#
```

The peripheral did appear in the system's `iomem` map, but `lsmod` did not show the relevant UIO driver. In other words, **`uio_pdrv_genirq` still was not loaded**.

```sh
root@imx95-15x15-lpddr4x-frdm:~# find /lib/modules/$(uname -r)/ -name "*genirq*"
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# modprobe uio_pdrv_genirq
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# find /lib/modules/$(uname -r)/ -name "*genirq*"
root@imx95-15x15-lpddr4x-frdm:~#
```

Because the option was set to `=y`, the driver was compiled directly into the kernel image and therefore could not appear as a loadable module in `lsmod`. We checked `dmesg` again:

```sh
root@imx95-15x15-lpddr4x-frdm:~# find /lib/modules/$(uname -r)/ -name "*genirq*"
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# modprobe uio_pdrv_genirq
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# find /lib/modules/$(uname -r)/ -name "*genirq*"
root@imx95-15x15-lpddr4x-frdm:~# dmesg | grep -i "uio_pdrv_genirq\|generic-uio"
root@imx95-15x15-lpddr4x-frdm:~# dmesg | grep -iE "42430000|EBUSY|request_mem_region|resource"
[    1.154269] kvm [1]: GICv3: no GICV resource entry
[    2.598405] pci_bus 0001:00: root bus resource [bus 00]
[    2.603692] pci_bus 0001:00: root bus resource [mem 0x4cc00000-0x4ccdffff]
[    2.610589] pci_bus 0001:00: root bus resource [mem 0x4cd00000-0x4cd0ffff pref]
[    2.617912] pci_bus 0001:00: root bus resource [mem 0x4cd20000-0x4cd7ffff]
[    2.624786] pci_bus 0001:00: root bus resource [mem 0x4cd80000-0x4cddffff pref]
[    2.914973] pci_bus 0001:00: resource 4 [mem 0x4cc00000-0x4ccdffff]
[    2.921253] pci_bus 0001:00: resource 5 [mem 0x4cd00000-0x4cd0ffff pref]
[    2.927977] pci_bus 0001:00: resource 6 [mem 0x4cd20000-0x4cd7ffff]
[    2.934246] pci_bus 0001:00: resource 7 [mem 0x4cd80000-0x4cddffff pref]
[    3.355114] pci_bus 0002:01: root bus resource [bus 01]
[    3.360354] pci_bus 0002:01: root bus resource [mem 0x4cce0000-0x4ccfffff]
[    3.367220] pci_bus 0002:01: root bus resource [mem 0x4cd10000-0x4cd1ffff pref]
[    3.421859] pci_bus 0002:01: resource 4 [mem 0x4cce0000-0x4ccfffff]
[    3.428214] pci_bus 0002:01: resource 5 [mem 0x4cd10000-0x4cd1ffff pref]
[    5.535598] pci_bus 0000:00: root bus resource [bus 00-ff]
[    5.541113] pci_bus 0000:00: root bus resource [io  0x0000-0xfffff]
[    5.547387] pci_bus 0000:00: root bus resource [mem 0x910000000-0x98fffffff] (bus address [0x10000000-0x8fffffff])
[    5.641339] pci_bus 0000:00: resource 4 [io  0x0000-0xfffff]
[    5.647013] pci_bus 0000:00: resource 5 [mem 0x910000000-0x98fffffff]
root@imx95-15x15-lpddr4x-frdm:~# ls /sys/bus/platform/drivers/uio_pdrv_genirq/ 2>/dev/null
bind  uevent  unbind
root@imx95-15x15-lpddr4x-frdm:~# cat /sys/firmware/devicetree/base/mu7@42430000/status
okayroot@imx95-15x15-lpddr4x-frdm:~#
```

The driver existed (`/sys/bus/platform/drivers/uio_pdrv_genirq/` contained `bind` and `unbind`), and the node had `status = "okay"`. However, `dmesg` contained no trace of a probe attempt, not even an `EBUSY` error.

```sh
root@imx95-15x15-lpddr4x-frdm:~# find /sys/devices -name "*42430000.mu7*"
/sys/devices/platform/42430000.mu7
root@imx95-15x15-lpddr4x-frdm:~# udevadm info /sys/bus/platform/devices/42430000.mu7 2>/dev/null
P: /devices/platform/42430000.mu7
M: 42430000.mu7
R: 7
J: +platform:42430000.mu7
U: platform
E: DEVPATH=/devices/platform/42430000.mu7
E: OF_NAME=mu7
E: OF_FULLNAME=/mu7@42430000
E: OF_COMPATIBLE_0=generic-uio
E: OF_COMPATIBLE_N=1
E: MODALIAS=of:Nmu7T(null)Cgeneric-uio
E: SUBSYSTEM=platform
E: USEC_INITIALIZED=9461038
E: ID_PATH=platform-42430000.mu7
E: ID_PATH_TAG=platform-42430000_mu7
root@imx95-15x15-lpddr4x-frdm:~# cat /sys/bus/platform/devices/42430000.mu7/uevent
OF_NAME=mu7
OF_FULLNAME=/mu7@42430000
OF_COMPATIBLE_0=generic-uio
OF_COMPATIBLE_N=1
MODALIAS=of:Nmu7T(null)Cgeneric-uio
root@imx95-15x15-lpddr4x-frdm:~#
root@imx95-15x15-lpddr4x-frdm:~# ls -la /sys/bus/platform/devices/42430000.mu7/driver 2>/dev/null
root@imx95-15x15-lpddr4x-frdm:~#
```

Further inspection of the node produced the following properties:

```conf
OF_NAME=mu7
OF_FULLNAME=/mu7@42430000
OF_COMPATIBLE_0=generic-uio
OF_COMPATIBLE_N=1
MODALIAS=of:Nmu7T(null)Cgeneric-uio
```

The `MODALIAS` line indicated that the device was **valid but unbound**. This narrowed the problem to `uio_pdrv_genirq`: the driver existed, but it had not bound to this device.

Unlike most platform drivers, mainline Linux's `drivers/uio/uio_pdrv_genirq.c` ships with an empty `of_device_id` match table:

```c
#ifdef CONFIG_OF
static struct of_device_id uio_of_genirq_match[] = {
	{ /* This is filled with module_parm */ },
	{ /* Sentinel */ },
};
module_param_string(of_id, uio_of_genirq_match[0].compatible, 128, 0);
MODULE_PARM_DESC(of_id, "Openfirmware id of the device to be handled by uio");
#endif
```

This upstream behavior is deliberate: `compatible = "generic-uio"` was considered too broad for safe automatic binding through normal OF matching, because the driver could accidentally claim devices that should not be exposed through UIO. The driver therefore requires the compatible string to be supplied explicitly through a module parameter at load time, rather than matching the DT `compatible` property automatically. To provide that parameter, we needed to modify the U-Boot boot arguments:

```conf
uio_pdrv_genirq.of_id=generic-uio
```

During boot, the virtual COM port (VCP) displayed a countdown with the prompt "press any key to stop autoboot." Pressing a key from the host serial console opened the U-Boot command line:

```sh
u-boot=> setenv mmcargs 'setenv bootargs ${jh_clk} console=${console} root=${mmcroot} uio_pdrv_genirq.of_id=generic-uio'
u-boot=> saveenv
u-boot=> boot
```

After booting with the updated arguments, we checked again and confirmed that it worked:

```sh
imx95-15x15-lpddr4x-frdm login: root
root@imx95-15x15-lpddr4x-frdm:~# cat /sys/module/uio_pdrv_genirq/parameters/of_id
cat: /sys/module/uio_pdrv_genirq/parameters/of_id: No such file or directory
root@imx95-15x15-lpddr4x-frdm:~# ls -la /sys/bus/platform/devices/42430000.mu7/driver
lrwxrwxrwx 1 root root 0 Mar 13 15:40 /sys/bus/platform/devices/42430000.mu7/driver -> ../../../bus/platform/drivers/uio_pdrv_genirq
root@imx95-15x15-lpddr4x-frdm:~# ls /sys/class/uio/
uio0  uio1  uio2
```

## The Approach Worked on FRDM-IMX95 but Failed on STM32MP257F-DK

We then inspected the boot arguments on the STM32MP257F-DK:

```conf
bootargs= uio_pdrv_genirq.of_id=generic-uio (does the space after = matter?)
bootcmd=run bootcmd_stm32mp
```

The `bootcmd_stm32mp` command was:

```sh
bootcmd_stm32mp=echo "Boot over ${boot_device}${boot_instance}!";if test ${boot_device} = serial || test ${boot_device} = usb;then stm32prog ${boot_device} ${boot_instance}; else run env_check;if test ${boot_device} = mmc;then env set boot_targets "mmc${boot_instance}"; fi;if test ${boot_device} = nand || test ${boot_device} = spi-nand ;then env set boot_targets ubifs0 mmc0; fi;if test ${boot_device} = nor;then env set boot_targets mmc0; fi;run distro_bootcmd;fi;
```

It runs `distro_bootcmd`, the standard distro-boot script. That script locates `extlinux.conf` on the boot partition and boots with its `APPEND` line, rather than using a manually set `bootargs` environment variable.

We found the file at `/boot/mmc0_extlinux/extlinux.conf`; it contained:

```conf
MENU BACKGROUND /splash_landscape.bmp
TIMEOUT 20
LABEL OpenSTLinux
    KERNEL /Image.gz
    FDTDIR /
    INITRD /st-image-resize-initrd
    APPEND root=PARTUUID=e91c4e10-16e6-4c0e-bd0e-77becf4a3582 rootwait rw  earlycon console=${console},${baudrate}
```

That `APPEND` line exactly matched the contents of `/proc/cmdline`. Because it did not include `uio_pdrv_genirq.of_id=generic-uio`, this confirmed where the argument was being dropped.

We modified the line in `vi`:

```sh
APPEND root=PARTUUID=e91c4e10-16e6-4c0e-bd0e-77becf4a3582 rootwait rw earlycon console=${console},${baudrate} uio_pdrv_genirq.of_id=generic-uio
```

After this change, the device bound successfully.
