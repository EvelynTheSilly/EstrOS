/// interface:
/// x0: cap id
/// x1: pointer
/// x2: len
/// x3: cap pointer
/// x4: cap len
///
/// errors:
/// 1: invalid target pid
/// 2: invalid receiver pid
/// 3: memory read error
/// 4: cap error
///
/// returns: 0
use crate::{
    alloc::vec,
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{
            Pid,
            capabilities::{
                CapID,
                Capability::{self, RecieverCapability},
            },
            messages::Message,
            threads::{Tid, wait_conditions::WaitState},
        },
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn send_message_to_receiver(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let cap_id = state.x[0] as CapID;
        let pointer = state.x[1] as usize;
        let len = state.x[2] as usize;
        let cap_pointer = state.x[3] as usize;
        let cap_len = state.x[4] as usize;

        // get own proc
        let proc = process_manager.get_process_mut(pid).ok()?;

        let cap = proc
            .capabilities
            .caps
            .get(&cap_id)
            .iter()
            .filter_map(|cap| {
                let RecieverCapability(cap) = cap else {
                    return None;
                };
                Some(cap)
            })
            .next()
            .unwrap();

        let target_pid: Pid = cap.proc;
        let receiver_id = cap.reciever_id;

        // read bytes from own process memory
        let mut message_data = vec![0 as u8; len];
        let Ok(_) = proc.mem_manager.mem_read(&mut message_data, pointer) else {
            return syscall_err(3);
        };

        let mut cap_data = vec![0 as u8; cap_len * 8];
        let Ok(_) = proc.mem_manager.mem_read(&mut cap_data, cap_pointer) else {
            return syscall_err(3);
        };

        let Some(caps) = ({
            let data: alloc::vec::Vec<CapID> = cap_data
                .chunks_exact(8)
                .map(|chunk| {
                    u64::from_le_bytes(chunk.try_into().expect("chunk exact should guarantee this"))
                })
                .collect();
            let caps: Option<alloc::vec::Vec<Capability>> = data
                .iter()
                .map(|id| proc.capabilities.caps.remove_entry(id))
                .map(|pair| pair.map(|pair| pair.1))
                .collect();
            caps
        }) else {
            return syscall_err(4);
        };

        // get target proc
        let Ok(target_proc) = process_manager.get_process_mut(target_pid) else {
            return syscall_err(1);
        };

        // send message to target proc
        let mid;
        if let Some(store) = target_proc
            .message_channel_store
            .message_channels
            .get_mut(&receiver_id)
        {
            mid = store.push_message(Message::new(message_data, caps));
        } else {
            return syscall_err(2);
        };

        // notify target prog
        target_proc
            .threads
            .iter_mut()
            .map(|pair| pair.1)
            .for_each(|thread| {
                if let WaitState::MessageWait(waited_on_receiver) = thread.wait_state {
                    if waited_on_receiver == receiver_id {
                        thread.state.x[0] = mid;
                        thread.wait_state = WaitState::None;
                    }
                }
            });

        Some(Ok(0))
    })
}
