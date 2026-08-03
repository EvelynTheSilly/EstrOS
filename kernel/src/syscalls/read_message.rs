/// interface:
/// x0: mid
/// x1: pointer
/// x2: len
/// contract:
/// it will read LEN bytes from MID to POINTER
use crate::{
    println,
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};

pub fn read_message(state: &mut State, pid: u64) {
    let mid = state.x[0];
    let process_pointer = state.x[1];
    let len = state.x[2];
    println!("mid {}, ptr {}, len {}", mid, process_pointer, len);
    PROCESS_MANAGER.lock(|manager| {
        let Ok(process) = manager.get_process_mut(pid) else {
            return;
        };
        let Ok(buff) = process.message_store.read_message(mid, len as usize) else {
            state.x[0] = 0;
            return;
        };
        process.mem_write(process_pointer as usize, buff);
    });
}
