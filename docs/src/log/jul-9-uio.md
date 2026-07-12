# UIO Jul 9 (Enable and r/w)

To enable `mmap` via `uio`, we need to handle the node of UIO devices. Executing:

```sh
RUST_LOG=trace ./melt-pot-stm32mp25-app
```

Then a straight, cold answer:

```
Bus error (core dumped)
```

No any indications available, so let's try using `gdb` to snapshot the last moment it crashes:

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

We don't know how to use gdb correctly, nor do we have an interest in halting programs. Hence, the only tool we want to use is to view the backtrace. Switch to debug build, and re-run after `Bus error`:

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
#4  0x0000aaaaaaacf18c in consortium_ipc_transport_memory::descriptor::{impl#4}::descriptor_init::{async_fn#0}<consortium_ipc_doorbell_hsem::doorbell::HsemDoorbell<consortium_ipc::side::Primary, 1>, consortium_shared_memory::regions::SramRegion> () at /home/ethanwu/consortium/crates/consortium-ipc-transport-memory/src/descriptor.rs:358
#5  0x0000aaaaaaac0498 in melt_pot_stm32mp25_app::{impl#0}::connect::{async_fn#0} () at src/consortium.gen.rs:127
#6  0x0000aaaaaaac165c in melt_pot_stm32mp25_app::init::{async_fn#0} () at src/consortium.gen.rs:219
#7  0x0000aaaaaaac2694 in melt_pot_stm32mp25_app::main::{async_block#0} () at src/main.rs:65
#8  0x0000aaaaaaac5c5c in tokio::runtime::park::{impl#4}::block_on::{closure#0}<melt_pot_stm32mp25_app::main::{async_block_env#0}> ()
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/park.rs:284
#9  0x0000aaaaaaac54a0 in tokio::task::coop::with_budget<core::task::poll::Poll<()>, tokio::runtime::park::{impl#4}::block_on::{closure_env#0}<melt_pot_stm32mp25_app::main::{async_block_env#0}>> (budget=..., f=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/task/coop/mod.rs:167
#10 tokio::task::coop::budget<core::task::poll::Poll<()>, tokio::runtime::park::{impl#4}::block_on::{closure_env#0}<melt_pot_stm32mp25_app::main::{async_block_env#0}>> (f=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/task/coop/mod.rs:133
#11 tokio::runtime::park::CachedParkThread::block_on<melt_pot_stm32mp25_app::main::{async_block_env#0}> (
    self=0xffffffffe2df, f=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/park.rs:284
--Type <RET> for more, q to quit, c to continue without paging--
#12 0x0000aaaaaaacc2a4 in tokio::runtime::context::blocking::BlockingRegionGuard::block_on<melt_pot_stm32mp25_app::main::{async_block_env#0}> (self=0xffffffffe6d0, f=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/context/blocking.rs:66
#13 0x0000aaaaaaacc938 in tokio::runtime::scheduler::multi_thread::{impl#0}::block_on::{closure#0}<melt_pot_stm32mp25_app::main::{async_block_env#0}> (blocking=0xffffffffe6d0)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/scheduler/multi_thread/mod.rs:92
#14 0x0000aaaaaaacbb9c in tokio::runtime::context::runtime::enter_runtime<tokio::runtime::scheduler::multi_thread::{impl#0}::block_on::{closure_env#0}<melt_pot_stm32mp25_app::main::{async_block_env#0}>, ()> (handle=0xfffffffff450,
    allow_block_in_place=true, f=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/context/runtime.rs:65
#15 0x0000aaaaaaacc8e4 in tokio::runtime::scheduler::multi_thread::MultiThread::block_on<melt_pot_stm32mp25_app::main::{async_block_env#0}> (self=0xfffffffff428, handle=0xfffffffff450, future=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/scheduler/multi_thread/mod.rs:91
#16 0x0000aaaaaaacc598 in tokio::runtime::runtime::Runtime::block_on_inner<melt_pot_stm32mp25_app::main::{async_block_env#0}> (self=0xfffffffff420, future=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/runtime.rs:373
#17 0x0000aaaaaaacc79c in tokio::runtime::runtime::Runtime::block_on<melt_pot_stm32mp25_app::main::{async_block_env#0}>
    (self=0xfffffffff420, future=...)
    at /home/ethanwu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/runtime/runtime.rs:345
#18 0x0000aaaaaaac1dc8 in melt_pot_stm32mp25_app::main () at src/main.rs:65
```

Now its clear: reading the `hsem` register caused the Bus error. To see other uio devices, we let AI to write a small gadget:

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

HSEM (memory-mapped i/o) is in `/dev/uio2`, whereas shared memory regions (outer-shareable, non-cacheable) are in `/dev/uio0` and `/dev/uio1`. The result was that only `/dev/uio2` reported `Bus error`, so the answer could only be one of:

1. `RIF` blocked the read and issued an _Illegal Access_ event.
2. The entire peripheral wasn't assigned for a clock.
3. Read action unaligned.

Actually, the first answer was my first intution: last summer, when I was fine-tuning a LLM on 4090s cluster, the process was blocked and reported `Bus error`, but 3090 worked. Checked the notice and found that this account wasn't assigned the permission of using 4090 — the 4090 was temporarily assigned for new-comers. That's being said, the first time I saw Bus error was related to _permission_ issues, similar to `RIF` here.

Checking the [RM0457](https://www.st.com/resource/en/reference_manual/rm0457-stm32mp2325xx-advanced-armbased-3264bit-mpus-stmicroelectronics.pdf), the answer failed silently:

![rif_aware_idx_mp25](/figures/rif_aware_idx_mp25.png)

The official STM32CubeMX example assigned `IPCC1` for all cores and disabled opt-out, meaning that all cores have access to this peripheral among CPU1 (`ca35`) and CPU2 (`cm33`). Hence, the `RIF` supposition failed.

```sh
dtc -I dtb -O dts -o stm32mp257f-dk.dts stm32mp257f-dk.dtb
```

Converting the `dtb` to `dts` and investigate what was passed to the device. Check the device source tree of `ipcc1` (official) specifically:

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

There was a line `clocks = <0x14 0x69>;`. In comparison, our `hsem` was very naive:

```dts
hsem@46240000 {
	status = "okay";
	interrupts = <0x00 0xc7 0x04>;
	interrupt-parent = <0x06>;
	reg = <0x00 0x46240000 0x00 0x400>;
	compatible = "generic-uio";
};
```

To find the clock, we could enter linux repo `/include/dt-bindings/clock/st,stm32mp25-rcc.h`, according to the `stm32mp251.dtsi` (`stm32mp257.dtsi` includes `stm32mp255.dsti` that includes `stm342mp253.dsti` that includes `stm32mp251.dtsi` that includes the `st,stm32mp255-rcc.h`):

```c
#include <dt-bindings/clock/st,stm32mp25-rcc.h>
```

We can find the data:

```c
#define CK_SCMI_HSEM		104
#define CK_SCMI_IPCC1		105
#define CK_SCMI_IPCC2		106
```

In official `IPCC1` configuration, the `0x69` corresponds to the `CK_SCMI_IPCC1`. Checking `IPCC2`, it was `0x6a`. Hence, we believe that this `0x69` comes from this `CK_SCMI_[peripheral]`. How about `0x14`? We found this:

```dts
scmi_clk: protocol@14 {
	bootph-all;
	reg = <0x14>;
	#clock-cells = <0x01>;
	phandle = <0x14>;
};
```

Then we can know that we need to add `clocks = <0x14 0x68>;` for HSEM. Add the node and the access was successful.

Then it worked.
