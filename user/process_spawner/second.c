#include <syscalls.h>
#include <stdio.h>

int main(){
    puts("this is from the second process\n");
    sys_exit();
}
