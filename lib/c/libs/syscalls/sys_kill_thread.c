#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL1(uint64_t, sys_kill_thread, 7, uint64_t, tid);