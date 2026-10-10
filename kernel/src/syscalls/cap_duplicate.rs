/// interface:
/// x0: cap id
///
/// errors:
/// -1: invalid cap id
///
/// returns: tid
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, threads::Tid},
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn cap_duplicate(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let cap_id = state.x[0];
        let proc = process_manager.get_process_mut(pid).ok()?;
        let Some(cap) = proc.capabilities.caps.get(&cap_id) else {
            return syscall_err(-1);
        };
        let cap = cap.clone();
        let new_cap_id = proc.capabilities.insert_capability(cap);
        Some(Ok(new_cap_id))
    })
}
