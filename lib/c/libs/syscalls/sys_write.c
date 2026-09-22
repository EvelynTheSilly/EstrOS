#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL2(void, sys_write, 1, const void *, buf, size_t, len);