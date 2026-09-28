#include <syscalls.h>
#include <string.h>
#include <stdio.h>

int main(){
    char message[] = "hello from a userspace driver\n";
    volatile char* uart = (char*) 0x670000;
    for (int i = 0; message[i] != 0; i++) {
        // sending that imessage
        *uart = i[message];
    }
}
