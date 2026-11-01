#include <stddef.h>
#include <syscalls.h>
#include <string.h>
#include "uart_process.h"

static uint64_t cap;

int puts(const char* buffer) {
    sys_send_message_to_receiver(cap, buffer, strlen(buffer), NULL, 0);
}

void putc(char ch){
    sys_send_message_to_receiver(cap, &ch, 1, NULL, 0);
}

void panic(const char* buffer) {
    puts("SPAWNER PROCESS PANIC: \n");
    puts(buffer);
    sys_exit();
}

int main(){
    
    // this one recieves the uart writer cap
    uint64_t reciever_cap_id = sys_spawn_capability(SYS_CAPABILITY_KIND_MESSAGE);
    // this is the duplicate of it which will be sent over
    uint64_t dup_cap_id = sys_cap_duplicate(reciever_cap_id);
    uint64_t uart_cap = sys_lookup_mem_cap(0x9000000, 4096);
    uint64_t cap_arr[] = {dup_cap_id, uart_cap};
    uint64_t spawn_result = sys_spawn_process(uart_process, uart_process_len, cap_arr, 2);

    uint64_t wait_res = sys_wait_on_message_from_receiver(reciever_cap_id);
    uint64_t uart_driver_cap = 1;
    sys_read_message_caps(reciever_cap_id, wait_res, &uart_driver_cap, 1);
    

    char message[] = "test message\n";
    sys_send_message_to_receiver(uart_driver_cap, message, strlen(message), NULL, 0);
    cap = uart_driver_cap;
    
    puts("booting up spawner process\n");
    puts("spawned uart process\n");
    puts("code = ");putc((-spawn_result) + '0');putc('\n');
}