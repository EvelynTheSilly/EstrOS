#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(uint64_t, sys_map_page, 17, void*, vaddr, uint64_t, flags);
