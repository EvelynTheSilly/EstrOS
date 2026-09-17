#include <syscalls.h>
#include <string.h>
#include <stdio.h>

int main(){
    char hello[] = "hello from estros";
    puts(hello);
    sys_exit();
}
