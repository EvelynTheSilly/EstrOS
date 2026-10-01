/// interface:
/// x0: pointer
/// x1: len
///
/// errors:
/// 1: memory read error
/// 2: elf parse error
///
/// returns: pid
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, Process, threads::Tid},
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};
use alloc::vec;
use elf::{ElfBytes, endian::AnyEndian};

pub fn spawn_process(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let pointer = state.x[0] as usize;
        let size = state.x[1] as usize;

        let Ok(spawning_proc) = scheduler.get_process_mut(pid) else {
            return None;
        };
        let mut data = vec![0 as u8; size];
        if spawning_proc
            .mem_manager
            .mem_read(&mut data, pointer)
            .is_err()
        {
            return syscall_err(1);
        }
        let Ok(elf) = ElfBytes::<AnyEndian>::minimal_parse(data.as_slice()) else {
            return syscall_err(2);
        };
        let Ok(proc) = Process::from_elf(elf) else {
            return syscall_err(2);
        };

        // TODO: figure out how to properly return pid while not having pids like 1 or 2 get confused for errors
        let _pid = scheduler.launch_process(proc);

        Some(Ok(0))
    })
}
