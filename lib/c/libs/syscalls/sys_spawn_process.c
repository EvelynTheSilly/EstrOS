#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(uint64_t, sys_spawn_process, 13, const void *, location, uint64_t, len);
