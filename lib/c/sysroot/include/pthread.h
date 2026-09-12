#ifndef __PTHREAD_H__
#define __PTHREAD_H__
#include <stdint.h>
#include <stddef.h>

typedef struct pthread {
    uint64_t tid;
    void *stack;
    uint64_t ret;
} pthread_t;

int pthread_create(pthread_t *t, typeof(void *(void *_Nullable)) *start_routine, size_t stack_size);
void* pthread_join(pthread_t *t);
void pthread_exit(void* retval);

#endif