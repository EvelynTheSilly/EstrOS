/// interface:
/// x0: addr
/// x1: size
///
/// errors:
/// -1: you dont own that cap
///
/// returns: cap id
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, capabilities::Capability::DeviceMemCapability, threads::Tid},
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn lookup_mem_cap(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let addr = state.x[0] as usize;
        let size = state.x[1] as usize;

        let Ok(process) = scheduler.get_process(pid) else {
            return None;
        };
        let cap_id = process
            .capabilities
            .caps
            .iter()
            .find(|(_, cap)| {
                if let DeviceMemCapability(cap) = cap {
                    return cap.addr == addr && cap.size == size;
                }
                return false;
            })
            .map(|pair| Some(Ok(*pair.0)))
            .unwrap_or(syscall_err(-1));

        return cap_id;
    })
}
