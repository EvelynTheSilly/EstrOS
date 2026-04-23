pub mod cpu_state;
mod handlers;

macro_rules! panicking_function {
    ($func_name:ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $func_name(state: &mut cpu_state::State) {
            // The `stringify!` macro converts an `ident` into a string.
            use core::arch::asm;
            let (mut esr, mut elr) = (0, 0);
            unsafe {
                asm!(
                    "
                    mrs {0:x}, esr_el1
                    mrs {1:x}, elr_el1
                    ",
                    out(reg) esr,
                    out(reg) elr,
                );
            }
            panic!(
                "{} triggered\n state dump \n{:x?}\nesr: 0x{:x}, elr: 0x{:x}",
                stringify!($func_name),
                state,
                esr,
                elr
            );
        }
    };
}

/// adds a `panic_function!` for its arg_sync, arg_irq, etc...
macro_rules! panicking_vector_handler_block {
    ($func_name:ident) => {
        panicking_function!(${concat($func_name, _sync_handler)});
        panicking_function!(${concat($func_name, _irq_handler)});
        panicking_function!(${concat($func_name, _fiq_handler)});
        panicking_function!(${concat($func_name, _serror_handler)});
    };
}

macro_rules! asm_vector_table {
    ($($vector_name:ident),+) => {
        core::arch::global_asm!(
            concat!(
                // header
                ".section .vectors, \"ax\"\n",
                ".align 11\n",
                ".global _vector_table\n",
                "_vector_table:\n",
                // exceptions
                $(
                    stringify!($vector_name), ":\n",
                    "str x30, [sp, #-8]!\n",
                    "bl dump_cpu_state\n",

                    "bl ", stringify!($vector_name), "_handler\n",

                    "bl load_cpu_state\n",
                    "ldr x30, [sp], #8\n",
                    "eret\n",

                    ".space 128 - (. - ",stringify!($vector_name),")\n",
                )*
            ),
        );
    };
}

asm_vector_table!(
    el1_sp0_sync,
    el1_sp0_irq,
    el1_sp0_fiq,
    el1_sp0_serror,
    el1_spx_sync,
    el1_spx_irq,
    el1_spx_fiq,
    el1_spx_serror,
    el0_aarch64_sync,
    el0_aarch64_irq,
    el0_aarch64_fiq,
    el0_aarch64_serror,
    el0_aarch32_sync,
    el0_aarch32_irq,
    el0_aarch32_fiq,
    el0_aarch32_serror
);

panicking_function!(el1_sp0_irq_handler);
panicking_function!(el1_sp0_fiq_handler);
panicking_function!(el1_sp0_serror_handler);
panicking_vector_handler_block!(el1_spx);
panicking_vector_handler_block!(el0_aarch64);
panicking_vector_handler_block!(el0_aarch32);
