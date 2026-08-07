#include "syscalls.h"

void exit(void) {
    __asm__ volatile("svc #2");
}
