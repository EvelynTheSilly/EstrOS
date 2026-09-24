/// interface:
/// x0: tid
///  
/// errors:
/// 1: target doesnt exist
/// 2: cannot await on self
/// 3: thread doesnt exist
///
/// returns: 0 on success
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER, process::{Pid, threads::{Tid, wait_conditions::WaitState}}},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn wait_on_thread(state: &mut State, pid: Pid, caller_tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let target_tid: Tid = state.x[0];
        // if target thread doesnt exist
        if process_manager
            .get_process_mut(pid)
            .ok()?
            .threads
            .get(target_tid)
            .is_err()
        {
            return syscall_err(1);
        }
        if caller_tid == target_tid {
            return syscall_err(2);
        }
        let Ok(thread) = process_manager
            .get_process_mut(pid)
            .ok()?
            .threads
            .get_mut(caller_tid)
        else {
            return syscall_err(3);
        };
        thread.wait_state = WaitState::TidWait(target_tid);
        Some(Ok(0))
    })
}
