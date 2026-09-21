#include <stddef.h>
#include <syscalls.h>
#include <string.h>
#include <stdio.h>

int main(){
    puts("RECEIVER: hello from the receiver\n");
    sys_spawn_receiver(0);
    uint64_t mid = sys_wait_on_message_from_receiver(0);
    for (int i = 0; i < 100; i++) {
            // let sender type shit
            sys_noop();
        };
    puts("RECEIVER: received mid "); putc(mid+'0'); putc('\n');
    char buffer[64];
    sys_read_message(0, mid, buffer, 64);
    puts("RECEIVER: ");
    puts(buffer);
}