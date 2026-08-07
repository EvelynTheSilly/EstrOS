.global _start
_start:
    ldr x0, =_stack
    mov sp, x0
    bl main
    svc #2
    b . // fallback