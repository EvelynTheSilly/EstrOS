/// interface:
/// x0: Message Channel id
///  
/// errors:
/// 1: target doesnt exist
///
/// returns: mid
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER, process::{Pid, threads::{Tid, wait_conditions::WaitState}}},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn wait_on_message_from_receiver(state: &mut State, pid: Pid, tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let target_message_channel = state.x[0];
        // if target thread doesnt exist
        let proc = process_manager.get_process_mut(pid).ok()?;
        let Some(channel) = proc.receiving_channels.get_mut(&target_message_channel) else {
            return syscall_err(1);
        };

        if let Some(mid) = channel.get_unwaited_message() {
            channel
                .get_message_mut(&mid)
                .expect("get_unwaited_message shouldve given us a valid mid");
            return Some(Ok(mid));
        } else {
            let thread = proc.threads.get_mut(tid).ok()?;
            thread.wait_state = WaitState::MessageWait(target_message_channel);
            return None;
        };
    })
}
