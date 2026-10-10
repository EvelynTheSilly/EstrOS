/// interface:
/// x0: bin pointer
/// x1: bin len
/// x2: cap pointer
/// x3: cap len
///
/// errors:
/// -1: memory read error
/// -2: elf parse error
/// -3: invalid capabilities error
///
/// returns: pid
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid, Process,
            capabilities::{CapID, Capability},
            threads::Tid,
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};
use alloc::vec;
use elf::{ElfBytes, endian::AnyEndian};

pub fn spawn_process(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let bin_pointer = state.x[0] as usize;
        let bin_size = state.x[1] as usize;
        let cap_pointer = state.x[2] as usize;
        let cap_size = state.x[3] as usize;

        let Ok(spawning_proc) = scheduler.get_process_mut(pid) else {
            return None;
        };

        let mut proc = {
            let mut data = vec![0 as u8; bin_size];
            if spawning_proc
                .mem_manager
                .mem_read(&mut data, bin_pointer)
                .is_err()
            {
                return syscall_err(-1);
            }
            let Ok(elf) = ElfBytes::<AnyEndian>::minimal_parse(data.as_slice()) else {
                return syscall_err(-2);
            };
            let Ok(proc) = Process::from_elf(elf) else {
                return syscall_err(-2);
            };
            proc
        };

        let Some(caps) = ({
            let cap_size = cap_size * 8;
            let mut data = vec![0 as u8; cap_size];
            if spawning_proc
                .mem_manager
                .mem_read(&mut data, cap_pointer)
                .is_err()
            {
                return syscall_err(-1);
            }
            let data: alloc::vec::Vec<CapID> = data
                .chunks_exact(8)
                .map(|chunk| {
                    u64::from_le_bytes(chunk.try_into().expect("chunk exact should guarantee this"))
                })
                .collect();
            let caps: Option<alloc::vec::Vec<Capability>> = data
                .iter()
                .map(|id| spawning_proc.capabilities.caps.remove_entry(id))
                .map(|pair| pair.map(|pair| pair.1))
                .collect();
            caps
        }) else {
            return syscall_err(-3);
        };
        caps.into_iter().for_each(|cap| {
            proc.capabilities.insert_capability(cap);
        });

        let pid = scheduler.launch_process(proc);

        Some(Ok(pid))
    })
}
