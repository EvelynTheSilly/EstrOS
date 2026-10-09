#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL3(uint64_t, sys_send_message_to_receiver, 11, uint64_t, cap_id, const void *, buf, size_t, len);