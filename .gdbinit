# show current ARM EL
define show_el
  set $el = (($cpsr >> 2) & 3)
  printf "Current EL: EL%d\n", $el
end

define esr
    printf "esr: 0x%016lx\n\n", $ESR_EL1
end

define elr_disas
    printf "elr: 0x%016lx\n\n", $ELR_EL1
    disassemble $ELR_EL1
end

symbol-file build/kernel.elf
b el1_sp0_sync
b el1_sp0_irq
b el1_sp0_fiq
b el1_sp0_serror
b el1_spx_sync
b el1_spx_irq
b el1_spx_fiq
b el1_spx_serror
b el0_aarch64_sync
b el0_aarch64_irq
b el0_aarch64_fiq
b el0_aarch64_serror
b el0_aarch32_sync
b el0_aarch32_irq
b el0_aarch32_fiq
b el0_aarch32_serror

commands 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
esr
elr_disas
end

b kernel::kernel_init