#include <syscalls.h>
#include <string.h>
#include <stdio.h>

void puts_userspace(char* message) {
    volatile char* uart = (char*) 0x670000;
    for (int i = 0; message[i] != 0; i++) {
            // sending that imessage
            *uart = i[message];
        }
}

int main(){
    putc(sys_map_device_memory((void*) 0x670000, 0x9000000, 4096)+'0');
    putc('\n');
    puts_userspace("hello there\n");
    puts_userspace("im printing in userspace\n");
    puts_userspace("this whole program only calls one syscall\n");
    puts_userspace("the one to map the uart to its own memory\n");
}
