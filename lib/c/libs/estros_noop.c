#include "syscalls.h"

void noop(void) {
    __asm__ volatile("svc #0");
}
