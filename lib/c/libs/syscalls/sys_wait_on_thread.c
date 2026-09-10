#include "syscalls.h"

uint64_t sys_wait_on_thread(uint64_t tid) {
    register uint64_t r0 __asm__("x0") = tid;

    __asm__ volatile("svc #8"
                     : "+r"(r0)
                     :
                     : "memory");

    return r0;
}
