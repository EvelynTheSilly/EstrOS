/// interface:
/// x0: receiver id
/// x1: mid
/// x2: pointer
/// x3: len
///
/// errors:
/// 1: invalid receiver id
/// 2: invalid mid
/// 3: memory write fail
///
/// contract:
/// it will read LEN bytes from MID to POINTER
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn read_message(state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    let receiver_id = state.x[0];
    let mid = state.x[1];
    let process_pointer = state.x[2];
    let len = state.x[3];
    PROCESS_MANAGER.lock(|manager| {
        let Ok(process) = manager.get_process_mut(pid) else {
            return None;
        };
        let Some(channel) = process.receiving_channels.get_mut(&receiver_id) else {
            return syscall_err(1);
        };
        let Ok(buff) = channel.read_message(mid, len as usize) else {
            return syscall_err(2);
        };
        let read = buff.len() as u64;
        if process.mem_write(process_pointer as usize, buff).is_err() {
            syscall_err(3)
        } else {
            Some(Ok(read))
        }
    })
}
