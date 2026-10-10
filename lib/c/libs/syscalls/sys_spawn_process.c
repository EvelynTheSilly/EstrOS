#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL4(uint64_t, sys_spawn_process, 13, const void *, bin, uint64_t, bin_len, const void *, caps, uint64_t, caps_len);
