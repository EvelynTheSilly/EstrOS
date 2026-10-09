/// interface:
/// x0: capability kind
///  - CONST 1 = stream
///  - CONST 2 = message
/// errors:
/// 1: invalid cap kind
///
/// returns: CapID
use crate::{
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid,
            capabilities::{
                CapID, Capability, message_reciever_capability::ProcessMessageCapability,
            },
            messages::MessageStore,
            threads::Tid,
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn spawn_capability(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let kind = state.x[0] as usize;
        let proc = process_manager.get_process_mut(pid).ok()?;
        let capid: CapID = match kind {
            1 => {
                todo!("stream capabilities")
            }
            2 => {
                let id = proc
                    .message_channel_store
                    .insert_reciever(MessageStore::new());
                let cap = Capability::RecieverCapability(ProcessMessageCapability {
                    proc: pid,
                    reciever_id: id,
                });
                proc.capabilities.insert_capability(cap)
            }
            _ => return syscall_err(1),
        };

        Some(Ok(capid))
    })
}
