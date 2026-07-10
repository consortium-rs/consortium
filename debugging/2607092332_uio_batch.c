#include <sys/mman.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <getopt.h>

#define MAP_SIZE 4096UL

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

    while ((opt = getopt(argc, argv, "w:n:")) != -1) {
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
        printf("%s [window %u] offset 0x%lx (%u bytes):\n", dev, window, offset, batch_count);
        
        /* Print bytes in rows of 16 */
        for (unsigned int i = 0; i < batch_count; i++) {
            if (i % 16 == 0) {
                printf("  0x%04lx: ", offset + i);
            }
            printf("%02x ", byte_ptr[i]);
            if ((i + 1) % 16 == 0 || i == batch_count - 1) {
                printf("\n");
            }
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