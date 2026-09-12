#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <syscalls.h>
#include <pthread.h>
#include <stddef.h>

void* poopoopeepee(void* arg) {
    puts("wee woo yayyyyy");
    return (void*)6;
}

int main(){
    puts("hello from threadding");
    puts("the goal is to make a second thread, exit the main thread, then exit the process from the second thread");

    pthread_t t;
    pthread_create(&t, poopoopeepee, 4096);
    
    for (int i = 0; i < 100; i++) {
        sys_noop();
    }
    
    uint64_t code = (uint64_t)pthread_join(&t);
    puts("returned code");
    putc(code + '0');
}