#include "syscalls.h"

void write(const void *buf, size_t len) {
    register const void *r0 __asm__("x0") = buf;
    register size_t r1 __asm__("x1") = len;

    __asm__ volatile("svc #1"
                     :
                     : "r"(r0), "r"(r1)
                     : "memory");
}
