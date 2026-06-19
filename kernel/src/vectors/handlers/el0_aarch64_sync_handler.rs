use crate::{println, syscalls::handle_syscall, vectors::cpu_state};
use core::arch::asm;

#[unsafe(no_mangle)]
extern "C" fn el0_aarch64_sync_handler(state: &mut cpu_state::State) {
    let esr_el1: u64;
    unsafe {
        asm!(
            "
            mrs x0, esr_el1
            ",
            out("x0") esr_el1
        );
    }
    let ec = (esr_el1 >> 26) & 0x3f;
    let iss = esr_el1 & 0x1FFFFFF;
    println!("esr: {:X} ec: {:X} iss: {}", esr_el1, ec, iss);
    match ec {
        21 => {
            println!("processing syscall number {}", iss);
            // TODO: handle more than one process
            handle_syscall(state, iss, 0);
        }
        _ => {
            panic!(
                "el0_aarch64_sync_handler triggered\nunknown EC: {}\n state dump \n{:x?}",
                ec, state
            );
        }
    };
}
