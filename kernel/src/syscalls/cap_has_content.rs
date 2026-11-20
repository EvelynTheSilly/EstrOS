/// interface:
/// x0: cap_id
///
/// errors:
/// -1: no such cap
/// -2: cap type does not support content
///
/// returns:
/// 0: capability has content
/// 1: capability has no content
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, capabilities::Capability::RecieverCapability, threads::Tid},
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn cap_has_content(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|manager| {
        let cap_id = state.x[0];
        let process = manager.get_process(pid).ok()?;

        let Some(capability) = process.capabilities.caps.get(&cap_id) else {
            return syscall_err(-1);
        };

        let receiver_id = {
            let RecieverCapability(cap) = capability else {
                return syscall_err(-2);
            };
            if cap.proc != pid {
                return syscall_err(-1);
            }
            cap.reciever_id
        };

        let Some(channel) = process
            .message_channel_store
            .message_channels
            .get(&receiver_id)
        else {
            return syscall_err(-1);
        };

        if channel.get_unwaited_message().is_some() {
            Some(Ok(0))
        } else {
            Some(Ok(1))
        }
    })
}
