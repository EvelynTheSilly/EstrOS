#include <stddef.h>
#include <syscalls.h>
#include <string.h>
#include <stdio.h>
#include "send_process_elf.h"

int main(){
    puts("RECEIVER: hello from the receiver\n");
    uint64_t cap_id = sys_spawn_capability(SYS_CAPABILITY_KIND_MESSAGE);
    puts("RECEIVER: recieved cap "); putc(cap_id+'0'); putc('\n');
    uint64_t dup_cap_id = sys_cap_duplicate(cap_id);
    puts("RECEIVER: duplicated cap "); putc(dup_cap_id+'0'); putc('\n');
    uint64_t cap_arr[] = {dup_cap_id};
    uint64_t spawn_result = sys_spawn_process(send_process_elf, send_process_elf_len, cap_arr, 1);
    puts("RECEIVER: spawned process");
    puts("code = ");putc((-spawn_result) + '0');putc('\n');
    for (int i = 0; i < 100; i++) {
        sys_noop();
    };
    uint64_t mid = sys_wait_on_message_from_receiver(cap_id);
    puts("RECEIVER: received mid "); putc(mid+'0'); putc('\n');
    if (mid == -1) {
        puts("RECIEVER: PANICKING\n");
        sys_exit();
    };
    char buffer[64];
    sys_read_message(cap_id, mid, buffer, 64);
    puts("RECEIVER: ");
    puts(buffer);
}