#include <sys/mman.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char **argv) {
    const char *dev = argc > 1 ? argv[1] : "/dev/uio1";
    unsigned long offset = argc > 2 ? strtoul(argv[2], NULL, 16) : 0;
    int fd = open(dev, O_RDWR);
    if (fd < 0) { perror("open"); return 1; }
    void *map = mmap(NULL, 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) { perror("mmap"); return 1; }
    volatile unsigned int *ptr = (volatile unsigned int *)((char *)map + offset);
    printf("reading %s offset 0x%lx ...\n", dev, offset);
    unsigned int val = *ptr;
    printf("value = 0x%x\n", val);
    return 0;
}