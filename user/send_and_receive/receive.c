#include <stddef.h>
#include <syscalls.h>
#include <string.h>
#include <stdio.h>
//#include "send_process_elf.h"

int main(){
    puts("RECEIVER: hello from the receiver\n");
    uint64_t cap_id = sys_spawn_capability(SYS_CAPABILITY_KIND_MESSAGE);
    puts("RECEIVER: recieved cap "); putc(cap_id+'0'); putc('\n');
    char message[] = "helloooooo (meows at you)";
    sys_send_message_to_receiver(cap_id, message, strlen(message));
    uint64_t mid = sys_wait_on_message_from_receiver(cap_id);
    puts("RECEIVER: received mid "); putc(mid+'0'); putc('\n');
    if (mid == -1) {
        puts("RECIEVER: PANICKING\n");
        sys_exit();
    };
    for (int i = 0; i < 100; i++) {
        sys_noop();
    };
    char buffer[64];
    sys_read_message(cap_id, mid, buffer, 64);
    puts("RECEIVER: ");
    puts(buffer);
    putc('\n');
}