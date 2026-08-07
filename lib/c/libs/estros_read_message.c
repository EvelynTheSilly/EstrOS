#include "syscalls.h"

int read_message(uint64_t mid, void *buf, size_t len) {
    register uint64_t r0 __asm__("x0") = mid;
    register void *r1 __asm__("x1") = buf;
    register size_t r2 __asm__("x2") = len;

    __asm__ volatile("svc #3"
                     : "+r"(r0)
                     : "r"(r1), "r"(r2)
                     : "memory");

    return (int)r0;
}
