use crate::vectors::cpu_state;

core::arch::global_asm!(
    "
    .global kmain
    kmain:
        b el1_start
    
    .global el1_start
    el1_start:
        bl enable_fpu
        bl setup_vtable
    
        // enter rust
        bl {kernel_init}
        sub sp, sp, {cpu_state_size}
        mov x0, sp
        bl {get_init_process}
        bl load_cpu_state
        ldr x30, [sp], #8
        eret
        b .                                      // hang forever
        
    enable_fpu:
        mrs x0, CPACR_EL1
        orr x0, x0, #(3 << 20)   // FPEN = 0b11 (EL0 + EL1 allowed)
        msr CPACR_EL1, x0
        isb
        ret
        
    setup_vtable:
        ldr x0, =_vector_table                  // load vtable into r0
        msr VBAR_EL1, x0
        isb                                     // move r0 to base vector table register
        ret
    ", 
    cpu_state_size = const core::mem::size_of::<cpu_state::State>(),
    kernel_init = sym crate::kernel_init,
    get_init_process = sym crate::get_init_process,
);
