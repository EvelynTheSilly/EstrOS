#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL5(uint64_t, sys_send_message_to_receiver, 11, uint64_t, cap_id, const void *, buf, size_t, len, const void *, caps, uint64_t, caps_len);