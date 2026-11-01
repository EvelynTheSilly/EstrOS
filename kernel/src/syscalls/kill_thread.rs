/// interface:
/// x0: tid
///
/// errors:
/// 1: invalid tid
/// 2: proccess doesnt exist, shouldnt be possible
///
/// returns: 0
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid,
            threads::{Tid, wait_conditions::WaitStateUpdate},
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn kill_thread(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let tid: Tid = state.x[0];
        let Ok(proc) = process_manager.get_process_mut(pid) else {
            return syscall_err(2);
        };
        if proc.threads.remove(tid).is_err() {
            return syscall_err(1);
        };
        proc.threads
            .notify_all_waiting_threads(&WaitStateUpdate::ThreadFinished(tid));

        Some(Ok(0))
    })
}
