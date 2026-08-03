use crate::{
    syscalls::{exit_2::exit, read_message::read_message, write_to_uart_1::write_to_uart},
    vectors::cpu_state::State,
};

pub mod exit_2;
pub mod read_message;
pub mod write_to_uart_1;

pub fn handle_syscall(state: &mut State, iss: u64, pid: u64) {
    match iss {
        0 => {} // pass or no-op syscall
        1 => write_to_uart(state, pid),
        2 => exit(state, pid),
        3 => read_message(state, pid),
        _ => {}
    };
}
