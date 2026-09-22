#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL1(uint64_t, sys_wait_on_thread, 8, uint64_t, tid);