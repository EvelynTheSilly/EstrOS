/// interface:
/// x0: tid
/// x1: exit code
///
/// errors:
/// 1: invalid tid
/// 2: proccess doesnt exist, shouldnt be possible
///
/// returns: 0
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER, process::threads::wait_conditions::WaitStateUpdate,
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn kill_thread(state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let tid = state.x[0];
        let code = state.x[1];
        let Ok(proc) = process_manager.get_process_mut(pid) else {
            return syscall_err(2);
        };
        if proc.threads.remove(tid).is_err() {
            return syscall_err(1);
        };
        let modified_threads = proc
            .threads
            .notify_all_waiting_threads(&WaitStateUpdate::ThreadFinished(tid));
        for tid in modified_threads {
            if let Ok(thread) = proc.threads.get_mut(tid) {
                thread.state.x[0] = code;
            }
        }
        Some(Ok(0))
    })
}
