use crate::{
    syscalls::{
        exit::exit, kill_thread::kill_thread, read_message::read_message,
        spawn_thread::spawn_thread, wait_on_thread::wait_on_thread, write_to_uart::write_to_uart,
    },
    vectors::cpu_state::State,
};
use thiserror::Error;
mod exit;
mod kill_thread;
mod read_message;
mod spawn_thread;
mod wait_on_thread;
mod write_to_uart;

const SYSCALLS: &[fn(&mut State, u64, u64) -> SyscallResult] = &[
    |_, _, _| None, // noop
    write_to_uart,
    exit,
    read_message,
    |_, pid, _| Some(Ok(pid)), // get_pid
    |_, _, tid| Some(Ok(tid)), // get_tid
    spawn_thread,
    kill_thread,
    wait_on_thread,
];

pub fn handle_syscall(state: &mut State, iss: u64, pid: u64, tid: u64) {
    let Some(syscall_fn) = SYSCALLS.get(iss as usize) else {
        return;
    };
    syscall_call(state, pid, tid, *syscall_fn);
}

fn syscall_call(
    state: &mut State,
    pid: u64,
    tid: u64,
    syscall_fn: fn(&mut State, u64, u64) -> SyscallResult,
) {
    let ret = syscall_fn(state, pid, tid);
    if let Some(ret) = ret {
        match ret {
            Err(err) => {
                let code = err.code;
                state.x[0] = code;
            }
            Ok(ret) => {
                state.x[0] = ret;
            }
        }
    }
}

type SyscallResult = Option<Result<u64, SyscallError>>;

#[derive(Error, Debug)]
#[error("syscall error")]
pub struct SyscallError {
    code: u64,
}

fn syscall_err(code: u64) -> SyscallResult {
    Some(Err(SyscallError { code }))
}
