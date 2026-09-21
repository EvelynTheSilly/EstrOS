#include "syscalls.h"

uint64_t sys_spawn_reciever(uint64_t channel_id) {
    register uint64_t r0 __asm__("x0") = channel_id;

    __asm__ volatile("svc #9"
                     : "+r"(r0)
                     :
                     : "memory");

    return r0;
}