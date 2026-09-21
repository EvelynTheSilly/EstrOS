/// interface:
/// x0: receive on channel
///
/// errors:
/// 1: channel already exists
///
/// returns: 0
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER, process::messages::MessageStore},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn spawn_receiver(state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let id = state.x[0];
        let proc = process_manager.get_process_mut(pid).ok()?;
        if proc.receiving_channels.contains_key(&id) {
            return syscall_err(1);
        }
        proc.receiving_channels.insert(id, MessageStore::new());
        Some(Ok(0))
    })
}
