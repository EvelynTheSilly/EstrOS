#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL1(uint64_t, sys_spawn_capability, 9, uint64_t, kind);
