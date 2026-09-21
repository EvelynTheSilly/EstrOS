/// interface:
/// x0: pid
/// x1: receiver id
/// x2: pointer
/// x3: len
///
/// errors:
/// 1: invalid target pid
/// 2: invalid receiver pid
/// 3: memory read error
///
/// returns: 0
use crate::{
    alloc::vec,
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{messages::Message, threads::wait_conditions::WaitState},
    },
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn send_message_to_receiver(state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let target_pid = state.x[0];
        let receiver_id = state.x[1];
        let pointer = state.x[2] as usize;
        let len = state.x[3] as usize;

        // get own proc
        let proc = process_manager.get_process_mut(pid).ok()?;

        // read bytes from own process memory
        let mut message_data = vec![0 as u8; len];
        let Ok(_) = proc.mem_read(&mut message_data, pointer) else {
            return syscall_err(3);
        };

        // get target proc
        let Ok(target_proc) = process_manager.get_process_mut(target_pid) else {
            return syscall_err(1);
        };

        // send message to target proc
        let mid;
        if let Some(store) = target_proc.receiving_channels.get_mut(&receiver_id) {
            mid = store.push_message(Message::new(message_data));
        } else {
            return syscall_err(2);
        };

        // notify target prog
        target_proc
            .threads
            .iter_mut()
            .map(|pair| pair.1)
            .for_each(|thread| {
                match thread.wait_state {
                    WaitState::MessageWait(waited_on_receiver) => {
                        if waited_on_receiver == receiver_id {
                            thread.state.x[0] = mid;
                        }
                        thread.wait_state = WaitState::None;
                    }
                    _ => {}
                };
            });

        Some(Ok(0))
    })
}
