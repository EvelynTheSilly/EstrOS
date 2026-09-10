#include <stdint.h>
#include <stddef.h>
#include <pthread.h>
#include <syscalls.h>
#include <stdlib.h>

void pthread_exit(void){
    sys_kill_thread(sys_get_tid());
}

extern void asm_spawn_thread(void *args);

struct ThreadSpawnArgs {
    void *stack;
    void *arg;
    typeof(void *(void *_Nullable)) *start_routine;
};

int pthread_create(pthread_t *t, typeof(void *(void *_Nullable)) *start_routine, size_t stack_size) {
    size_t padded = (stack_size + 15u) & ~(size_t)15u;
    size_t total = padded + sizeof(struct ThreadSpawnArgs);
    void *block = malloc(total);
    if (block == NULL) return 1;

    struct ThreadSpawnArgs *args = (void *)((uintptr_t)block + padded);
    args->stack = args;
    args->arg = NULL;
    args->start_routine = start_routine;

    t->tid = sys_spawn_thread(asm_spawn_thread, args);
    t->stack = block;
    return 0;
}

// TODO
int pthread_join(pthread_t *t) {
    return 0;
}