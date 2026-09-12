#include <stdint.h>
#include <stddef.h>
#include <pthread.h>
#include <syscalls.h>
#include <stdlib.h>

struct ThreadSpawnArgs {
    void *stack;
    void *arg;
    typeof(void *(void *_Nullable)) *start_routine;
    uint64_t *ret;
};

void pthread_exit(void *retval){
    struct ThreadSpawnArgs *ctl;
    asm("mrs %0, tpidr_el0" : "=r"(ctl));
    uint64_t *ret = ctl->ret;
    *ret = (uint64_t)retval;
    sys_kill_thread(sys_get_tid());
}

extern void asm_spawn_thread(void *args);

int pthread_create(pthread_t *t, typeof(void *(void *_Nullable)) *start_routine, size_t stack_size) {
    size_t padded = (stack_size + 15u) & ~(size_t)15u;
    size_t total = padded + sizeof(struct ThreadSpawnArgs);
    void *block = malloc(total);
    if (block == NULL) return 1;

    struct ThreadSpawnArgs *args = (void *)((uintptr_t)block + padded);
    args->ret = &t->ret;
    args->stack = args;
    args->arg = NULL;
    args->start_routine = start_routine;

    t->tid = sys_spawn_thread(asm_spawn_thread, args);
    t->stack = block;
    return 0;
}

void* pthread_join(pthread_t *t) {
    sys_wait_on_thread(t->tid);
    free(t->stack);
    return (void*)t->ret;
}