use alloc::vec::Vec;

/// interface:
/// x0: cap_id
/// x1: mid
/// x2: pointer
/// x3: len
///
/// errors:
/// -1: invalid cap
/// -2: invalid mid
/// -3: memory write fail
///
/// contract:
/// it will read LEN capabilities from MID to POINTER
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid,
            capabilities::{CapID, Capability::RecieverCapability},
            threads::Tid,
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn read_message_caps(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    let cap_id = state.x[0];
    let mid = state.x[1];
    let process_pointer = state.x[2];
    let len = state.x[3];
    PROCESS_MANAGER.lock(|manager| {
        let Ok(process) = manager.get_process_mut(pid) else {
            return None;
        };

        let Some(capability) = process.capabilities.caps.get(&cap_id) else {
            return syscall_err(-1);
        };

        let channel_id = {
            let RecieverCapability(cap) = capability else {
                return syscall_err(-1);
            };
            if cap.proc != pid {
                return syscall_err(-1);
            }
            cap.reciever_id
        };

        let Some(channel) = process
            .message_channel_store
            .message_channels
            .get_mut(&channel_id)
        else {
            return syscall_err(-1);
        };
        let Ok(caps) = channel.read_message_caps(mid, len as usize) else {
            return syscall_err(-2);
        };
        let ids: Vec<CapID> = caps
            .into_iter()
            .map(|cap| process.capabilities.insert_capability(cap))
            .collect();

        let buff = ids.iter().flat_map(|id| id.to_ne_bytes()).collect();

        let read = (ids.len() * 8) as u64;
        if process
            .mem_manager
            .mem_write(process_pointer as usize, buff)
            .is_err()
        {
            syscall_err(-3)
        } else {
            Some(Ok(read))
        }
    })
}
