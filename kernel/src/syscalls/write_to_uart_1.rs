use alloc::string::String;
use alloc::vec;

use crate::{
    println,
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};

pub fn write_to_uart(state: &mut State, pid: u64) {
    PROCESS_MANAGER.lock(|scheduler| {
        let Ok(process) = scheduler.get_process(pid) else {
            return;
        };
        let mut buffer = vec![0u8; state.x[1] as usize];
        process
            .mem_read(&mut buffer, state.x[0] as usize)
            .expect("failed to read process memory TODO: HANDLE THIS");
        let s = String::from_utf8_lossy(&buffer.as_slice());
        println!("{}", s);
    })
}
