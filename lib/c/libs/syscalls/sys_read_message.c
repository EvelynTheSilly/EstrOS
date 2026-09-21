#include "syscalls.h"

int sys_read_message(uint64_t receiver_id, uint64_t mid, void *buf, size_t len) {
    register uint64_t r0 __asm__("x0") = receiver_id;
    register uint64_t r1 __asm__("x1") = mid;
    register void *r2 __asm__("x2") = buf;
    register size_t r3 __asm__("x3") = len;

    __asm__ volatile("svc #3"
                     : "+r"(r0)
                     : "r"(r1), "r"(r2), "r"(r3)
                     : "memory");

    return (int)r0;
}