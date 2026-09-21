#include "syscalls.h"

uint64_t sys_send_message_to_reciever(uint64_t pid, uint64_t channel_id, const void *buf, size_t len) {
    register uint64_t r0 __asm__("x0") = pid;
    register uint64_t r1 __asm__("x1") = channel_id;
    register const void *r2 __asm__("x2") = buf;
    register size_t r3 __asm__("x3") = len;

    __asm__ volatile("svc #11"
                     : "+r"(r0)
                     : "r"(r1), "r"(r2), "r"(r3)
                     : "memory");

    return r0;
}