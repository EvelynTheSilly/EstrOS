.global asm_spawn_thread
asm_spawn_thread:
    ldr x1, [x0, #0]
    ldr x2, [x0, #8]
    ldr x3, [x0, #16]
    msr TPIDR_EL0, x0
    cbz x1, 1f
    mov sp, x1
1:
    mov x0, x2
    mov x1, xzr
    mov x2, xzr
    blr x3
    bl pthread_exit
    b .