#include <syscalls.h>
int main(){
    char hello[] = "haiii";
    write(hello, 6);
    exit();
}