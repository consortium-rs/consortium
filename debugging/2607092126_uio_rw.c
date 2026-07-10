#include <sys/mman.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>

#define MAP_SIZE 4096UL

static void usage(const char *prog) {
    fprintf(stderr,
        "Usage:\n"
        "  %s <dev> r <offset_hex>\n"
        "  %s <dev> w <offset_hex> <value_hex>\n"
        "\n"
        "Examples:\n"
        "  %s /dev/uio1 r 0x0\n"
        "  %s /dev/uio1 w 0x0 0x7c\n",
        prog, prog, prog, prog);
}

int main(int argc, char **argv) {
    if (argc < 4) {
        usage(argv[0]);
        return 1;
    }

    const char *dev = argv[1];
    char mode = argv[2][0];

    if (mode != 'r' && mode != 'w') {
        fprintf(stderr, "error: mode must be 'r' or 'w', got '%s'\n", argv[2]);
        usage(argv[0]);
        return 1;
    }

    errno = 0;
    unsigned long offset = strtoul(argv[3], NULL, 16);
    if (errno != 0) { perror("strtoul(offset)"); return 1; }

    unsigned int wval = 0;
    if (mode == 'w') {
        if (argc < 5) {
            fprintf(stderr, "error: write mode requires <value_hex>\n");
            usage(argv[0]);
            return 1;
        }
        errno = 0;
        wval = (unsigned int)strtoul(argv[4], NULL, 16);
        if (errno != 0) { perror("strtoul(value)"); return 1; }
    }

    if (offset + sizeof(unsigned int) > MAP_SIZE) {
        fprintf(stderr, "error: offset 0x%lx out of mapped range (0x%lx)\n",
                offset, MAP_SIZE);
        return 1;
    }
    if (offset % sizeof(unsigned int) != 0) {
        fprintf(stderr, "warning: offset 0x%lx is not 4-byte aligned\n", offset);
    }

    int fd = open(dev, O_RDWR);
    if (fd < 0) { perror("open"); return 1; }

    void *map = mmap(NULL, MAP_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) { perror("mmap"); close(fd); return 1; }

    volatile unsigned int *ptr = (volatile unsigned int *)((char *)map + offset);

    if (mode == 'r') {
        unsigned int val = *ptr;
        printf("%s offset 0x%lx = 0x%08x\n", dev, offset, val);
    } else {
        printf("%s offset 0x%lx : writing 0x%08x\n", dev, offset, wval);
        *ptr = wval;
        unsigned int rb = *ptr;
        printf("%s offset 0x%lx readback = 0x%08x\n", dev, offset, rb);
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