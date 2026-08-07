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
 * See kernel/src/syscalls/mod.rs for the authoritative list.
 */

#ifndef SYSCALLS_H
#define SYSCALLS_H

#include <stddef.h>
#include <stdint.h>

#define ESTROS_SYS_NOOP         0
#define ESTROS_SYS_WRITE_UART   1
#define ESTROS_SYS_EXIT         2
#define ESTROS_SYS_READ_MESSAGE 3

void noop(void);
void write(const void *buf, size_t len);
void exit(void);
int read_message(uint64_t mid, void *buf, size_t len);

#endif /* SYSCALLS_H */
