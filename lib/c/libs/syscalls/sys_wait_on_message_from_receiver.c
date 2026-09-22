#include "syscalls.h"
#include <syscall_macros.h>

SYSCALL1(uint64_t, sys_wait_on_message_from_receiver, 10, uint64_t, channel_id);