use core::arch::global_asm;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct State {
    pub sp: u64,
    pub(crate) _pad: u64,
    pub spsr: u64,
    pub elr: u64,
    pub x: [u64; 31],
}

global_asm!(
    r"
    .global dump_cpu_state
    dump_cpu_state:
        stp x28, x29, [sp, #-16]!
        stp x26, x27, [sp, #-16]!
        stp x24, x25, [sp, #-16]!
        stp x22, x23, [sp, #-16]!
        stp x20, x21, [sp, #-16]!
        stp x18, x19, [sp, #-16]!
        stp x16, x17, [sp, #-16]!
        stp x14, x15, [sp, #-16]!
        stp x12, x13, [sp, #-16]!
        stp x10, x11, [sp, #-16]!
        stp x8, x9, [sp, #-16]!
        stp x6, x7, [sp, #-16]!
        stp x4, x5, [sp, #-16]!
        stp x2, x3, [sp, #-16]!
        stp x0, x1, [sp, #-16]!
        mrs x0, spsr_el1
        mrs x1, elr_el1
        stp x0, x1, [sp, #-16]!
        mrs x0, sp_el0
        stp x0, xzr, [sp, #-16]!

        mov x0, sp
        ret
"
);

global_asm!(
    r"
    .global load_cpu_state
    load_cpu_state:
        ldp x0, x1, [sp], #16
        msr sp_el0, x0
        ldp x0, x1, [sp], #16
        msr spsr_el1, x0
        msr elr_el1, x1
        ldp x0, x1, [sp], #16
        ldp x2, x3, [sp], #16
        ldp x4, x5, [sp], #16
        ldp x6, x7, [sp], #16
        ldp x8, x9, [sp], #16
        ldp x10, x11, [sp], #16
        ldp x12, x13, [sp], #16
        ldp x14, x15, [sp], #16
        ldp x16, x17, [sp], #16
        ldp x18, x19, [sp], #16
        ldp x20, x21, [sp], #16
        ldp x22, x23, [sp], #16
        ldp x24, x25, [sp], #16
        ldp x26, x27, [sp], #16
        ldp x28, x29, [sp], #16
        ret
"
);
