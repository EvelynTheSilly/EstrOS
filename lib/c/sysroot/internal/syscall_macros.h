#define SYSCALL0NORET(name, nr)        \
    void name(void) {                    \
        __asm__ volatile("svc #" #nr);   \
    }

#define SYSCALL0(ret, name, nr)        \
    ret name(void) {                    \
        register ret r0 __asm__("x0");  \
        __asm__ volatile("svc #" #nr   \
                         : "+r"(r0)     \
                         :              \
                         : "memory");   \
        return (ret)r0;                 \
    }

#define SYSCALL1(ret, name, nr, t0, a0)       \
    ret name(t0 a0) {                         \
        register t0 r0 __asm__("x0") = a0;    \
        __asm__ volatile("svc #" #nr          \
                         : "+r"(r0)            \
                         :                    \
                         : "memory");          \
        return (ret)r0;                        \
    }

#define SYSCALL2(ret, name, nr, t0, a0, t1, a1) \
    ret name(t0 a0, t1 a1) {                    \
        register t0 r0 __asm__("x0") = a0;       \
        register t1 r1 __asm__("x1") = a1;       \
        __asm__ volatile("svc #" #nr             \
                         : "+r"(r0)               \
                         : "r"(r1)                \
                         : "memory");             \
        return (ret)r0;                           \
    }

#define SYSCALL3(ret, name, nr, t0, a0, t1, a1, t2, a2) \
    ret name(t0 a0, t1 a1, t2 a2) {                     \
        register t0 r0 __asm__("x0") = a0;               \
        register t1 r1 __asm__("x1") = a1;               \
        register t2 r2 __asm__("x2") = a2;               \
        __asm__ volatile("svc #" #nr                     \
                         : "+r"(r0)                       \
                         : "r"(r1), "r"(r2)               \
                         : "memory");                    \
        return (ret)r0;                                   \
    }

#define SYSCALL4(ret, name, nr, t0, a0, t1, a1, t2, a2, t3, a3) \
    ret name(t0 a0, t1 a1, t2 a2, t3 a3) {                      \
        register t0 r0 __asm__("x0") = a0;                       \
        register t1 r1 __asm__("x1") = a1;                       \
        register t2 r2 __asm__("x2") = a2;                       \
        register t3 r3 __asm__("x3") = a3;                       \
        __asm__ volatile("svc #" #nr                             \
                         : "+r"(r0)                               \
                         : "r"(r1), "r"(r2), "r"(r3)               \
                         : "memory");                            \
        return (ret)r0;                                           \
    }

#define SYSCALL5(ret, name, nr, t0, a0, t1, a1, t2, a2, t3, a3, t4, a4) \
    ret name(t0 a0, t1 a1, t2 a2, t3 a3, t4 a4) {                       \
        register t0 r0 __asm__("x0") = a0;                              \
        register t1 r1 __asm__("x1") = a1;                              \
        register t2 r2 __asm__("x2") = a2;                              \
        register t3 r3 __asm__("x3") = a3;                              \
        register t4 r4 __asm__("x4") = a4;                              \
        __asm__ volatile("svc #" #nr                                    \
                         : "+r"(r0)                                      \
                         : "r"(r1), "r"(r2), "r"(r3), "r"(r4)             \
                         : "memory");                                   \
        return (ret)r0;                                                  \
    }

#define SYSCALL6(ret, name, nr, t0, a0, t1, a1, t2, a2, t3, a3, t4, a4, t5, a5) \
    ret name(t0 a0, t1 a1, t2 a2, t3 a3, t4 a4, t5 a5) {                       \
        register t0 r0 __asm__("x0") = a0;                                     \
        register t1 r1 __asm__("x1") = a1;                                     \
        register t2 r2 __asm__("x2") = a2;                                     \
        register t3 r3 __asm__("x3") = a3;                                     \
        register t4 r4 __asm__("x4") = a4;                                     \
        register t5 r5 __asm__("x5") = a5;                                     \
        __asm__ volatile("svc #" #nr                                           \
                         : "+r"(r0)                                             \
                         : "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)          \
                         : "memory");                                          \
        return (ret)r0;                                                         \
    }