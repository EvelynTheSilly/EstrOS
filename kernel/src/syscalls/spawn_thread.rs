/// interface:
/// x0: location
/// x1: arg (passed to x0 on new thread)
///
/// errors:
/// 1: proccess doesnt exist, shouldnt be possible
///
/// returns: tid
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER, process::{Pid, threads::{SchedulerThread, Tid}}},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn spawn_thread(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let location = state.x[0];
        let arg = state.x[1];
        let Ok(proc) = process_manager.get_process_mut(pid) else {
            return syscall_err(1);
        };
        let mut thread = SchedulerThread::at(location);
        thread.state.x[0] = arg;
        let tid = proc.threads.spawn(thread);
        Some(Ok(tid))
    })
}
