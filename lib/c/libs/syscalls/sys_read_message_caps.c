#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL4(int, sys_read_message_caps, 16, uint64_t, cap_id, uint64_t, mid, void *, buf, size_t, len);
