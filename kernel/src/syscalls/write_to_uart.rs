use crate::{
    print,
    scheduler::{CpuScheduler, PROCESS_MANAGER, process::{Pid, threads::Tid}},
    syncronisation::Mutex,
    syscalls::{SyscallError, SyscallResult},
    vectors::cpu_state::State,
};
use alloc::string::String;
use alloc::vec;

pub fn write_to_uart(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let Ok(process) = scheduler.get_process(pid) else {
            return None; // pid should be valid but if its not just do nothing
        };
        let mut buffer = vec![0u8; state.x[1] as usize];
        let read_res = process.mem_read(&mut buffer, state.x[0] as usize);
        if read_res.is_err() {
            return Some(Err(SyscallError { code: 1 }));
        }
        let s = String::from_utf8_lossy(&buffer.as_slice());
        print!("{}", s);
        return Some(Ok(0));
    })
}
