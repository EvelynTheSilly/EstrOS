/*
 * syscalls.h
 *
 * Syscall numbers and thin-wrapper prototypes for the estros syscall ABI.
 *
 * Syscall ABI (arguments in x0..x5, `svc #<nr>`):
 *   0: no-op
 *   1: write_to_uart(x0=ptr, x1=len)
 *   2: exit()
 *   3: read_message(x0=mid, x1=ptr, x2=len)
 *   4: get_tid() -> tid
 *   5: get_pid() -> pid
 *   6: spawn_thread(x0=location, x1=arg) -> tid
 *   7: kill_thread(x0=tid) -> 0
 *   8: wait_on_thread(x0=tid) -> 0
 * See kernel/src/syscalls/mod.rs for the authoritative list.
 */

#ifndef SYSCALLS_H
#define SYSCALLS_H

#include <stddef.h>
#include <stdint.h>

void sys_noop(void);
void sys_write(const void *buf, size_t len);
void sys_exit(void);
int sys_read_message(uint64_t mid, void *buf, size_t len);
uint64_t sys_get_tid(void);
uint64_t sys_get_pid(void);
uint64_t sys_spawn_thread(const void *location, void *arg);
uint64_t sys_kill_thread(uint64_t tid);
uint64_t sys_wait_on_thread(uint64_t tid);

#endif /* SYSCALLS_H */
