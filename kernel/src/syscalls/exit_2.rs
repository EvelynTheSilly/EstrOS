use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};

pub fn exit(_state: &mut State, pid: u64) {
    PROCESS_MANAGER.lock(|scheduler| {
        //
        scheduler
            .kill(pid)
            .expect("the pid should be saved correctly");
    });
}
