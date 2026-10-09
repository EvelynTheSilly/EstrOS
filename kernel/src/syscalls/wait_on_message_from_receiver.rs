/// interface:
/// x0: cap_id
///  
/// errors:
/// -1: target doesnt exist
///
/// returns: mid
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid,
            capabilities::Capability::RecieverCapability,
            threads::{Tid, wait_conditions::WaitState},
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn wait_on_message_from_receiver(state: &mut State, pid: Pid, tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let cap_id = state.x[0];
        // if target reciever doesnt exist
        let proc = process_manager.get_process_mut(pid).ok()?;
        let reciever_id = {
            let capability = proc.capabilities.caps.get(&cap_id).unwrap();
            let RecieverCapability(cap) = capability else {
                return syscall_err(1);
            };
            if cap.proc != pid {
                return syscall_err(1);
            }
            cap.reciever_id
        };
        let Some(channel) = proc
            .message_channel_store
            .message_channels
            .get_mut(&reciever_id)
        else {
            return syscall_err((-1 as i64) as u64);
        };

        if let Some(mid) = channel.get_unwaited_message() {
            let msg = channel
                .get_message_mut(&mid)
                .expect("get_unwaited_message shouldve given us a valid mid");
            msg.waited_on = true;
            return Some(Ok(mid));
        } else {
            let thread = proc.threads.get_mut(tid).ok()?;
            thread.wait_state = WaitState::MessageWait(cap_id);
            return None;
        };
    })
}
