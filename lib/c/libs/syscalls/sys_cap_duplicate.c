#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL1(uint64_t, sys_cap_duplicate, 14, uint64_t, cap_id);
