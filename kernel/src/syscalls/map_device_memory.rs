/// interface:
/// x0: vaddr
/// x1: phys addr
/// x2: size
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
        process::{Pid, threads::Tid},
    },
    syncronisation::Mutex,
    syscall_assert,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn map_device_memory(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let vaddr = state.x[0] as usize;
        let paddr = state.x[1] as usize;
        let size = state.x[2] as usize;

        let Ok(proc) = scheduler.get_process_mut(pid) else {
            return None;
        };

        syscall_assert!(vaddr % 4096 == 0, 1);

        let Some(cap) = proc
            .capabilities
            .dev_mem_caps
            .iter()
            .map(|pair| pair.1)
            .find(|cap| cap.addr == paddr && cap.size == size)
        else {
            return syscall_err(2);
        };

        let result = proc.mem_manager.map_device(vaddr, cap);
        syscall_assert!(result.is_ok(), 3);

        Some(Ok(0))
    })
}
