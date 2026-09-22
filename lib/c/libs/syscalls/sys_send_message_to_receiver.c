#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL4(uint64_t, sys_send_message_to_receiver, 11, uint64_t, pid, uint64_t, channel_id, const void *, buf, size_t, len);