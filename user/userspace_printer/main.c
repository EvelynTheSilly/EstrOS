#include <syscalls.h>
#include <string.h>
#include <stdio.h>

int main(){
    char message[] = "\nhello from a userspace driver\n";
    volatile char* uart = (char*) 0x670000;

    putc(sys_map_device_memory((void*) uart, 0x9000000, 4096)+'0');
    
    for (int i = 0; message[i] != 0; i++) {
        // sending that imessage
        *uart = i[message];
    }
}
