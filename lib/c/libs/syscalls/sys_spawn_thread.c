#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(uint64_t, sys_spawn_thread, 6, const void *, location, void *, arg);