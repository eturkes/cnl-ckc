// Cached, fully touched 512 MiB mapping → a known RSS peak with <50 ms process lifetime.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/resource.h>
#include <time.h>
#include <unistd.h>

static uint64_t now(void) {
    struct timespec t;
    if (clock_gettime(CLOCK_MONOTONIC, &t)) exit(2);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

static uint64_t cpu(void) {
    struct timespec t;
    if (clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t)) exit(2);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

int main(int argc, char **argv) {
    if (argc == 2 && strcmp(argv[1], "cpu") == 0) {
        const uint64_t begin = cpu();
        volatile uint64_t sum = 0;
        while (cpu() - begin < 50000000) sum++;
        printf("{\"burn_cpu_ns\":%lu,\"sum\":%lu}\n", cpu() - begin, sum);
        return 0;
    }
    if (argc != 4) return 2;
    const uint64_t begin = now();
    const size_t bytes = 512UL << 20;
    const int fd = open(argv[1], O_RDONLY);
    if (fd < 0) return 2;
    unsigned char *p = mmap(NULL, bytes, PROT_READ, MAP_SHARED | MAP_POPULATE, fd, 0);
    if (p == MAP_FAILED || close(fd)) return 2;
    volatile uint64_t sum = 0;
    for (size_t at = 0; at < bytes; at += 4096) sum += p[at];
    struct rusage u;
    if (getrusage(RUSAGE_SELF, &u)) return 2;
    printf("{\"allocation_bytes\":%zu,\"own_hwm_bytes\":%ld,\"mapped_ns\":%lu,\"pgid\":%d,\"sum\":%lu}\n",
           bytes, u.ru_maxrss * 1024L, now() - begin, getpgrp(), sum);
    if (fflush(stdout)) return 2;
    const long hold_ms = strtol(argv[2], NULL, 10);
    struct timespec delay = {.tv_sec = hold_ms / 1000, .tv_nsec = (hold_ms % 1000) * 1000000};
    while (nanosleep(&delay, &delay) && errno == EINTR) {}
    if (munmap(p, bytes)) return 2;
    return (int)strtol(argv[3], NULL, 10);
}
