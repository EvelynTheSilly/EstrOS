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
    println,
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
        let caller_pid = pid;
        println!(
            "[spawn_process] called by pid={} tid={} elf_ptr={:#x} elf_len={}",
            caller_pid, _tid, pointer, size
        );

        let Ok(spawning_proc) = scheduler.get_process_mut(pid) else {
            return None;
        };
        let mut data = vec![0 as u8; size];
        if spawning_proc
            .mem_manager
            .mem_read(&mut data, pointer)
            .is_err()
        {
            println!("[spawn_process] mem_read FAILED for pid={}", caller_pid);
            return syscall_err(1);
        }
        let Ok(elf) = ElfBytes::<AnyEndian>::minimal_parse(data.as_slice()) else {
            println!(
                "[spawn_process] elf minimal_parse FAILED for pid={}",
                caller_pid
            );
            return syscall_err(2);
        };
        let Ok(mut proc) = Process::from_elf(elf) else {
            println!(
                "[spawn_process] Process::from_elf FAILED for pid={}",
                caller_pid
            );
            return syscall_err(2);
        };

        println!(
            "[spawn_process] parsed Process from {}-byte elf, full dump:\n{:#?}",
            size, proc
        );

        proc.threads.iter_mut().for_each(|a| println!("{:#?}", a));

        let pid = scheduler.launch_process(proc);
        println!(
            "[spawn_process] launched new pid={}, returning 0 to caller pid={}",
            pid, caller_pid
        );
        Some(Ok(0))
    })
}
