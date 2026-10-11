/// interface:
/// x0: vaddr
/// x1: cap
///
/// errors:
/// 1: unaligned vaddr
/// 2: no such capability
/// 3: mapping failed
///
/// returns 0 on success
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, capabilities::Capability::DeviceMemCapability, threads::Tid},
    },
    syncronisation::Mutex,
    syscall_assert,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn map_device_memory(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let vaddr = state.x[0] as usize;
        let cap_id = state.x[1];

        let Ok(proc) = scheduler.get_process_mut(pid) else {
            return None;
        };

        syscall_assert!(vaddr % 4096 == 0, 1);

        let Some(cap) = proc.capabilities.caps.get(&cap_id) else {
            return syscall_err(2);
        };
        let DeviceMemCapability(cap) = cap else {
            return syscall_err(2);
        };

        let result = proc.mem_manager.map_device(vaddr, cap);
        syscall_assert!(result.is_ok(), 3);

        Some(Ok(0))
    })
}
