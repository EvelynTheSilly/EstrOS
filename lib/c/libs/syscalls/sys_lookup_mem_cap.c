#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(uint64_t, sys_lookup_mem_cap, 15, uint64_t, addr, uint64_t, size);
