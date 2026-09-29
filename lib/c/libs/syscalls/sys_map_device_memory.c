#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL3(uint64_t, sys_map_device_memory, 12, void*, vaddr, uint64_t, paddr, uint64_t, size);