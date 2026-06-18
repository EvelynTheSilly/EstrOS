use crate::{syscalls::write_to_uart_1::write_to_uart, vectors::cpu_state::State};

pub mod write_to_uart_1;

pub fn handle_syscall(state: &mut State, iss: u64, pid: u64) {
    match iss {
        1 => write_to_uart(state, pid),
        _ => {}
    };
}
