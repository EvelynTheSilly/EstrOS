/*
 * estros.h
 *
 * Example header for the estros userspace C library.
 * This file only shows the expected format — fill in the real
 * declarations for the estros syscall ABI.
 *
 * Syscall ABI (arguments in x0..x5, `svc #<nr>`):
 *   0: no-op
 *   1: write_to_uart(x0=ptr, x1=len)
 *   2: exit()
 *   3: read_message(x0=mid, x1=ptr, x2=len)
 * See kernel/src/syscalls/mod.rs for the authoritative list.
 */

#ifndef ESTROS_H
#define ESTROS_H

#include <stddef.h>
#include <stdint.h>

/* Example: syscall numbers */
/* #define ESTROS_SYS_NOOP         0 */
/* #define ESTROS_SYS_WRITE_UART   1 */
/* #define ESTROS_SYS_EXIT         2 */
/* #define ESTROS_SYS_READ_MESSAGE 3 */

/* Example: function prototypes */
/* void estros_write(const void *buf, size_t len); */
/* void estros_exit(void); */
/* int estros_read_message(uint64_t mid, void *buf, size_t len); */

#endif /* ESTROS_H */
