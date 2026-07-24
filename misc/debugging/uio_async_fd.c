// uio_ipcc_irq.c
//
// Minimal UIO-based console tool for one STM32 IPCC channel.
//
// Blocks on a UIO interrupt file (e.g. /dev/uio0), and on every interrupt:
//   - prints a timestamp and the incoming channel-status register (CxTOCySR)
//   - if the requested channel is occupied, acknowledges it (CxSCR.CHnC)
//   - re-arms the UIO interrupt so the next edge is delivered
//
// Build:   gcc -O2 -Wall -o uio_ipcc_irq uio_ipcc_irq.c
// Run:     ./uio_ipcc_irq /dev/uio0 [channel]
//
// Assumes mmap region 0 of the UIO device covers the IPCC instance's 0x400
// byte register window, and that this side is the "processor 1" bank
// (reads C2TOC1SR as "incoming", acks via C1SCR). If you're on the
// secondary/M-core side instead, swap OFF_INCOMING_SR to C1TOC2SR and
// OFF_SCR to C2SCR.

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>
#include <string.h>
#include <errno.h>
#include <time.h>

/* Register offsets relative to the IPCC instance base. */
#define OFF_C1CR         0x000u
#define OFF_SCR          0x008u   /* C1SCR: this side's set/clear register */
#define OFF_INCOMING_SR  0x01Cu   /* C2TOC1SR: remote's outgoing status, as seen from proc1 */
#define MAP_LEN          0x400u

static volatile uint32_t *regs;

static uint32_t rd(uint32_t off) { return regs[off / 4]; }
static void wr(uint32_t off, uint32_t val) { regs[off / 4] = val; }

static void timestamp(char *buf, size_t n) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    snprintf(buf, n, "%ld.%06ld", (long)ts.tv_sec, ts.tv_nsec / 1000);
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s /dev/uioN [channel]\n", argv[0]);
        return 1;
    }
    const char *uio_path = argv[1];
    int channel = argc > 2 ? atoi(argv[2]) : 0;
    if (channel < 0 || channel > 15) {
        fprintf(stderr, "channel must be 0..15\n");
        return 1;
    }

    int fd = open(uio_path, O_RDWR);
    if (fd < 0) {
        fprintf(stderr, "open(%s): %s\n", uio_path, strerror(errno));
        return 1;
    }

    /* UIO convention: mmap offset N * page_size maps register block N. */
    void *map = mmap(NULL, MAP_LEN, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) {
        fprintf(stderr, "mmap: %s\n", strerror(errno));
        close(fd);
        return 1;
    }
    regs = (volatile uint32_t *)map;

    printf("uio_ipcc_irq: watching %s, channel %d, regs mapped at %p\n",
           uio_path, channel, map);
    printf("C1CR=0x%08x incoming_SR=0x%08x (initial)\n",
           rd(OFF_C1CR), rd(OFF_INCOMING_SR));

    for (;;) {
        uint32_t irq_count;
        ssize_t n = read(fd, &irq_count, sizeof(irq_count));
        if (n != (ssize_t)sizeof(irq_count)) {
            fprintf(stderr, "read(uio): %s\n", strerror(errno));
            break;
        }

        char ts[32];
        timestamp(ts, sizeof(ts));

        uint32_t status = rd(OFF_INCOMING_SR);
        int fired = (status & (1u << channel)) != 0;

        printf("[%s] irq #%u fired, incoming_SR=0x%08x, channel %d %s\n",
               ts, irq_count, status, channel, fired ? "OCCUPIED" : "clear");

        if (fired) {
            /* Ack: write CHnC (low half of SCR) for this channel only. */
            wr(OFF_SCR, 1u << channel);
            printf("[%s] acked channel %d (wrote SCR=0x%08x)\n",
                   ts, channel, 1u << channel);
        }

        /* Re-arm the UIO interrupt so the next edge is delivered. */
        uint32_t reenable = 1;
        if (write(fd, &reenable, sizeof(reenable)) != (ssize_t)sizeof(reenable)) {
            fprintf(stderr, "write(uio) re-enable: %s\n", strerror(errno));
            break;
        }
    }

    munmap(map, MAP_LEN);
    close(fd);
    return 0;
}