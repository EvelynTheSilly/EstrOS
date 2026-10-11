#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(uint64_t, sys_map_device_memory, 12, void*, vaddr, uint64_t, cap_id);