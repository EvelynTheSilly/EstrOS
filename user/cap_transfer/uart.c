#include <stddef.h>
#include <syscalls.h>
#include <string.h>

void init_uart(uint64_t cap) {
    sys_map_device_memory((void*) 0x670000, cap);
}

void puts(char* message) {
    volatile char* uart = (char*) 0x670000;
    for (int i = 0; message[i] != 0; i++) {
        *uart = i[message];
    }
}

void putc(char ch){
    volatile char* uart = (char*) 0x670000;
    *uart = ch;
}

void uart_loop(uint64_t cap_id) {
    char buffer[64];
    for (int i = 0; i<64; i++) {
        buffer[i] = 0;
    }
    while (true) {
        uint64_t mid = sys_wait_on_message_from_receiver(cap_id);
        //puts("RECEIVER: received mid "); putc(mid+'0'); putc('\n');
        if (mid == -1) {
            puts("RECIEVER: PANICKING\n");
            sys_exit();
        };
        while (true) {
            for (int i = 0; i<64; i++) {
                buffer[i] = 0;
            }
            uint64_t result = sys_read_message(cap_id, mid, buffer, 63);
            puts(buffer);
            if (result != 63) {
                break;
            }
        }
    }
}


int main(){
    uint64_t message_cap = 0;
    uint64_t uart_cap = 1;

    init_uart(uart_cap);
    puts("uart driver started\n");

    uint64_t cap_id = sys_spawn_capability(SYS_CAPABILITY_KIND_MESSAGE);
    uint64_t dup_cap_id = sys_cap_duplicate(cap_id);
    sys_send_message_to_receiver(message_cap, NULL, 0, &dup_cap_id, 1);

    puts("starting uart loop now \n\n\n");
    uart_loop(cap_id);
}