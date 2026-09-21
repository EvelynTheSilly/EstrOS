#include <stddef.h>
#include <syscalls.h>
#include <string.h>
#include <stdio.h>


int main(){
    char hello[] = "SENDER: hello from sender\n";
    puts(hello);
    for (int i = 0; i < 100; i++) {
        // let receiver open up its port and shi
        sys_noop();
    };
    char buffer[] = "hello from sender\n";
    uint64_t result = sys_send_message_to_receiver(0, 0, buffer, strlen(buffer));
    puts("SENDER: send returned "); putc(result+'0'); puts("\n");
}