# UIO, July 9, 2026 (Enabling Read/Write Access)

To use `mmap` through UIO, the application must open and map the appropriate UIO device node. We began by running:

```sh
RUST_LOG=trace ./melt-pot-stm32mp25-app
```

The program terminated immediately with an uninformative error:

```
Bus error (core dumped)
```

Because the program provided no further diagnostic information, we used `gdb` to capture the point at which it crashed:

```sh
(gdb) run
Starting program: /home/root/melt-pot-stm32mp25-app
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
[New Thread 0xfffff7d1f080 (LWP 7877)]
[New Thread 0xfffff7b0f080 (LWP 7878)]

Thread 1 "melt-pot-stm32m" received signal SIGBUS, Bus error.
core::ptr::read_volatile<u32> (src=0xfffff7fe5080)
    at /home/ethanwu/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:2095
warning: 2095 /home/ethanwu/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs: No such file or directory
```

We did not need an interactive debugging session; the backtrace alone was sufficient for this investigation. We switched to a debug build, reproduced the `Bus error`, and requested the backtrace:

```sh
(gdb) bt
#0  core::ptr::read_volatile<u32> (src=0xfffff7fe5080)
    at /home/ethanwu/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:2095
#1  core::ptr::const_ptr::{impl#0}::read_volatile<u32> (self=0xfffff7fe5080)
    at /home/ethanwu/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1192
#2  0x0000aaaaaaace7d4 in consortium_ipc_doorbell_hsem::abstraction::Hsem<consortium_ipc::side::Primary, 1>::read_lock<consortium_ipc::side::Primary, 1> (self=0xffffffffe100, sem=0)
    at /home/ethanwu/consortium/crates/consortium-ipc-doorbell-hsem/src/abstraction.rs:118
#3  0x0000aaaaaaace0f4 in consortium_ipc_doorbell_hsem::doorbell::{impl#3}::ring<consortium_ipc::side::Primary, 1> (
    self=0xffffffffe0f8, ch=...) at /home/ethanwu/consortium/crates/consortium-ipc-doorbell-hsem/src/doorbell.rs:115
```

The backtrace made the failure clear: reading the `hsem` register caused the bus error. To compare behavior across the other UIO devices, we used a small diagnostic utility:

```c
#include <sys/mman.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <getopt.h>

#define MAP_SIZE 4096UL

static int is_presentable_ascii(const unsigned char *bytes, unsigned int count) {
    if (count == 0) {
        return 0;
    }

    for (unsigned int i = 0; i < count; i++) {
        unsigned char b = bytes[i];
        if (b == '\n' || b == '\r' || b == '\t') {
            continue;
        }
        if (b < 0x20 || b > 0x7e) {
            return 0;
        }
    }

    return 1;
}

static void print_ascii(const unsigned char *bytes, unsigned int count) {
    printf("As ASCII:\n  \"");
    for (unsigned int i = 0; i < count; i++) {
        switch (bytes[i]) {
            case '\n': printf("\\n"); break;
            case '\r': printf("\\r"); break;
            case '\t': printf("\\t"); break;
            case '"':  printf("\\\""); break;
            case '\\': printf("\\\\"); break;
            default:   putchar(bytes[i]); break;
        }
    }
    printf("\"\n");
}

static void usage(const char *prog) {
    fprintf(stderr,
        "Usage:\n"
        "  %s [options] <dev> r <offset_hex> [count]\n"
        "  %s [options] <dev> w <offset_hex> <value_hex>\n"
        "\n"
        "Options:\n"
        "  -w, --window <n>    Select mmap window (default: 0)\n"
        "  -n, --count <n>     Number of bytes to read (default: 4)\n"
        "\n"
        "Examples:\n"
        "  %s /dev/uio1 r 0x0\n"
        "  %s /dev/uio1 r 0x0 16                    # read 16 bytes\n"
        "  %s /dev/uio1 -w 1 r 0x100\n"
        "  %s /dev/uio1 -w 2 -n 32 r 0x0            # read 32 bytes from window 2\n"
        "  %s /dev/uio1 w 0x0 0x7c\n",
        prog, prog, prog, prog, prog, prog, prog);
}

int main(int argc, char **argv) {
    int opt;
    unsigned int window = 0;
    unsigned int batch_count = 4;

    struct option long_opts[] = {
        { "window", required_argument, NULL, 'w' },
        { "count",  required_argument, NULL, 'n' },
        { NULL, 0, NULL, 0 }
    };

    while ((opt = getopt_long(argc, argv, "w:n:", long_opts, NULL)) != -1) {
        switch (opt) {
            case 'w':
                errno = 0;
                window = (unsigned int)strtoul(optarg, NULL, 0);
                if (errno != 0) { perror("strtoul(window)"); return 1; }
                break;
            case 'n':
                errno = 0;
                batch_count = (unsigned int)strtoul(optarg, NULL, 0);
                if (errno != 0) { perror("strtoul(count)"); return 1; }
                if (batch_count == 0 || batch_count > MAP_SIZE) {
                    fprintf(stderr, "error: count must be 1..%lu\n", MAP_SIZE);
                    return 1;
                }
                break;
            default:
                usage(argv[0]);
                return 1;
        }
    }

    if (argc - optind < 3) {
        usage(argv[0]);
        return 1;
    }

    const char *dev = argv[optind];
    char mode = argv[optind + 1][0];

    if (mode != 'r' && mode != 'w') {
        fprintf(stderr, "error: mode must be 'r' or 'w', got '%s'\n", argv[optind + 1]);
        usage(argv[0]);
        return 1;
    }

    errno = 0;
    unsigned long offset = strtoul(argv[optind + 2], NULL, 16);
    if (errno != 0) { perror("strtoul(offset)"); return 1; }

    unsigned int wval = 0;
    if (mode == 'w') {
        if (argc - optind < 4) {
            fprintf(stderr, "error: write mode requires <value_hex>\n");
            usage(argv[0]);
            return 1;
        }
        errno = 0;
        wval = (unsigned int)strtoul(argv[optind + 3], NULL, 16);
        if (errno != 0) { perror("strtoul(value)"); return 1; }
    } else {
        /* Read mode: optional count argument */
        if (argc - optind >= 4) {
            errno = 0;
            batch_count = (unsigned int)strtoul(argv[optind + 3], NULL, 0);
            if (errno != 0) { perror("strtoul(count)"); return 1; }
            if (batch_count == 0 || batch_count > MAP_SIZE) {
                fprintf(stderr, "error: count must be 1..%lu\n", MAP_SIZE);
                return 1;
            }
        }
    }

    if (offset + batch_count > MAP_SIZE) {
        fprintf(stderr, "error: offset 0x%lx + count %u exceeds mapped range (0x%lx)\n",
                offset, batch_count, MAP_SIZE);
        return 1;
    }

    if (mode == 'w' && offset % sizeof(unsigned int) != 0) {
        fprintf(stderr, "warning: offset 0x%lx is not 4-byte aligned for write\n", offset);
    }

    int fd = open(dev, O_RDWR);
    if (fd < 0) { perror("open"); return 1; }

    /* Calculate file offset: window_number * MAP_SIZE + offset */
    off_t file_offset = (off_t)window * (off_t)MAP_SIZE;

    void *map = mmap(NULL, MAP_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, file_offset);
    if (map == MAP_FAILED) {
        fprintf(stderr, "error: mmap failed at offset 0x%lx (window %u)\n", file_offset, window);
        perror("mmap");
        close(fd);
        return 1;
    }

    volatile unsigned char *byte_ptr = (volatile unsigned char *)((char *)map + offset);

    if (mode == 'r') {
        unsigned char bytes[MAP_SIZE];
        for (unsigned int i = 0; i < batch_count; i++) {
            bytes[i] = byte_ptr[i];
        }

        printf("%s [window %u] offset 0x%lx (%u bytes):\n", dev, window, offset, batch_count);

        /* Print bytes in rows of 16 */
        for (unsigned int i = 0; i < batch_count; i++) {
            if (i % 16 == 0) {
                printf("  0x%04lx: ", offset + i);
            }
            printf("%02x ", bytes[i]);
            if ((i + 1) % 16 == 0 || i == batch_count - 1) {
                printf("\n");
            }
        }

        if (is_presentable_ascii(bytes, batch_count)) {
            print_ascii(bytes, batch_count);
        }

        /* Also print as 32-bit words if aligned and count >= 4 */
        if (offset % 4 == 0 && batch_count >= 4) {
            printf("As 32-bit words:\n");
            volatile unsigned int *word_ptr = (volatile unsigned int *)byte_ptr;
            unsigned int word_count = batch_count / 4;
            for (unsigned int i = 0; i < word_count; i++) {
                if (i % 4 == 0) {
                    printf("  0x%04lx: ", offset + i * 4);
                }
                printf("%08x ", word_ptr[i]);
                if ((i + 1) % 4 == 0 || i == word_count - 1) {
                    printf("\n");
                }
            }
        }
    } else {
        printf("%s [window %u] offset 0x%lx : writing 0x%08x\n", dev, window, offset, wval);
        volatile unsigned int *ptr = (volatile unsigned int *)byte_ptr;
        *ptr = wval;
        unsigned int rb = *ptr;
        printf("%s [window %u] offset 0x%lx readback = 0x%08x\n", dev, window, offset, rb);
        if (rb != wval) {
            fprintf(stderr,
                "note: readback != written value (expected for HW-controlled "
                "fields like HSEM LOCK/MASTERID)\n");
        }
    }

    munmap(map, MAP_SIZE);
    close(fd);
    return 0;
}
```

HSEM, a memory-mapped I/O peripheral, was exposed through `/dev/uio2`, while the outer-shareable, non-cacheable shared-memory regions were exposed through `/dev/uio0` and `/dev/uio1`. Only `/dev/uio2` produced a `Bus error`, which narrowed the likely causes to the following:

1. `RIF` blocked the read and issued an _Illegal Access_ event.
2. No clock had been assigned to the peripheral.
3. The read was unaligned. This fails because all readings are word-by-word.

The first possibility was initially the most intuitive. In an earlier, unrelated case, an LLM fine-tuning process produced a `Bus error` on an RTX 4090 cluster while working on RTX 3090 nodes. The account did not have permission to use the 4090 nodes, which had temporarily been reserved for newcomers. That experience associated a bus error with an access-control problem analogous to `RIF` here; it was only a heuristic, not evidence for this specific failure.

Checking [RM0457](https://www.st.com/resource/en/reference_manual/rm0457-stm32mp2325xx-advanced-armbased-3264bit-mpus-stmicroelectronics.pdf) did not support that hypothesis:

![rif_aware_idx_mp25](/figures/rif_aware_idx_mp25.png)

The official STM32CubeMX example assigned `IPCC1` to all cores and disabled opt-out. Both CPU1 (`ca35`) and CPU2 (`cm33`) therefore had access to the peripheral, ruling out the `RIF` hypothesis.

```sh
dtc -I dtb -O dts -o stm32mp257f-dk.dts stm32mp257f-dk.dtb
```

We converted the `dtb` to `dts` to inspect the configuration passed to the device, focusing on the official `ipcc1` node:

```dts
ipcc1: mailbox@40490000 {
	compatible = "st,stm32mp1-ipcc";
	#mbox-cells = <0x01>;
	reg = <0x40490000 0x400>;
	st,proc-id = <0x00>;
	interrupts = <0x00 0xab 0x04 0x00 0xac 0x04>;
	interrupt-names = "rx", "tx";
	clocks = <0x14 0x69>;
	status = "disabled";
	phandle = <0x9b>;
};
```

The node contained `clocks = <0x14 0x69>;`. By comparison, our `hsem` node was minimal:

```dts
hsem@46240000 {
	status = "okay";
	interrupts = <0x00 0xc7 0x04>;
	interrupt-parent = <0x06>;
	reg = <0x00 0x46240000 0x00 0x400>;
	compatible = "generic-uio";
};
```

To identify the clock, we inspected `/include/dt-bindings/clock/st,stm32mp25-rcc.h` in the Linux repository. This header is reached through the DTS include chain: `stm32mp257.dtsi` includes `stm32mp255.dsti`, which includes `stm342mp253.dsti`, which includes `stm32mp251.dtsi`, which in turn includes `st,stm32mp255-rcc.h`.

```c
#include <dt-bindings/clock/st,stm32mp25-rcc.h>
```

The relevant definitions were:

```c
#define CK_SCMI_HSEM		104
#define CK_SCMI_IPCC1		105
#define CK_SCMI_IPCC2		106
```

In the official `IPCC1` configuration, `0x69` corresponded to `CK_SCMI_IPCC1`; for `IPCC2`, the value was `0x6a`. This indicated that the second cell selected the relevant `CK_SCMI_[peripheral]` clock. We then identified the meaning of `0x14`:

```dts
scmi_clk: protocol@14 {
	bootph-all;
	reg = <0x14>;
	#clock-cells = <0x01>;
	phandle = <0x14>;
};
```

From these definitions, we determined that HSEM required `clocks = <0x14 0x68>;`. After adding that property to the node, access succeeded.

The UIO mapping then worked as expected.
