use crate::{
    cpu_manager::{CPU_STATE_MANAGER, CpuPersistantState, get_cpu_id},
    println,
    scheduler::{CpuScheduler, CpuSchedulerError, PROCESS_MANAGER},
    syncronisation::Mutex,
    syscalls::handle_syscall,
    vectors::cpu_state,
};
use core::arch::asm;

#[unsafe(no_mangle)]
extern "C" fn el0_aarch64_sync_handler(state: &mut cpu_state::State) {
    let esr_el1: u64;
    unsafe {
        asm!(
            "
            mrs x0, esr_el1
            ",
            out("x0") esr_el1
        );
    }
    let ec = (esr_el1 >> 26) & 0x3f;
    let iss = esr_el1 & 0x1FFFFFF;
    // deactivate mem map if present
    (&CPU_STATE_MANAGER, &PROCESS_MANAGER).lock(|(cpu_manager, scheduler)| {
        let cpu = cpu_manager
            .entry(get_cpu_id())
            .or_insert(CpuPersistantState::new());
        let Some(pid) = cpu.get_pid() else {
            return;
        };
        let Some(previous_ttbr) = cpu.get_ttbr() else {
            return;
        };
        scheduler.deactivate_memory_map(pid, previous_ttbr);
    });
    match ec {
        21 => {
            // TODO: handle more than one process
            handle_syscall(state, iss, 0);
        }
        _ => {
            panic!(
                "el0_aarch64_sync_handler triggered\nunknown EC: {}\n state dump \n{:x?}",
                ec, state
            );
        }
    };
    (&PROCESS_MANAGER, &CPU_STATE_MANAGER).lock(|(scheduler, manager)| {
        let _ = scheduler.report_thread_state(0, 0, state.clone());
        let maybe_schedule = scheduler.schedule();
        let (pid, tid, thread) = match maybe_schedule {
            Err(e) => match e {
                CpuSchedulerError::NoProcesses => {
                    panic!("no processes to execute")
                }
                _ => {
                    panic!("couldnt schedule correctly {}", e)
                }
            },
            Ok(ok) => ok,
        };
        let cpu = manager
            .entry(get_cpu_id())
            .or_insert(CpuPersistantState::new());
        cpu.submit_pid_tid(pid, tid);
        let previous_ttbr = scheduler
            .activate_memory_map(pid)
            .expect("scheduler should have given us a correct pid");
        cpu.submit_ttbr(previous_ttbr);
        *state = thread.state;
    });
}
