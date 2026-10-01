#include <syscalls.h>
#include <stdio.h>
#include "second_process_elf.h"

int main(){
    puts("spawner: spawning second process\n");

    uint64_t rc = sys_spawn_process(second_process_elf, second_process_elf_len);

    puts("spawner: spawn_process returned ");
    putc(rc+'0');
    putc('\n');

    sys_exit();
}
