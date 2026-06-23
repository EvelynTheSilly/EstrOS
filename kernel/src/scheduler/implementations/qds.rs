use crate::{
    scheduler::{
        CpuScheduler, CpuSchedulerError, Result, process::Process, threads::SchedulerThread,
    },
    vectors::cpu_state::State,
};
use alloc::collections::btree_map::BTreeMap;
use core::arch::asm;

/// Quick and Dirty Scheduler
/// not meant to truly be functional, rewrite later
pub struct QDScheduler {
    processes: BTreeMap<u64, Process>,
}

impl QDScheduler {
    pub const fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }
}

impl CpuScheduler for QDScheduler {
    fn schedule(&mut self) -> Result<(u64, u64, SchedulerThread)> {
        let process = self
            .processes
            .get(&0)
            .ok_or(CpuSchedulerError::NoProcesses)?;
        Ok((
            0,
            0,
            process
                .threads
                .get(&0)
                .expect("should have thread id 0")
                .clone(),
        ))
    }
    fn activate_memory_map(&mut self, pid: u64) -> Result<usize> {
        let process = self
            .processes
            .get(&pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        let previous_ttbr;
        unsafe {
            previous_ttbr = process.memory_map.activate();
            asm!("dsb sy", "isb");
        }
        Ok(previous_ttbr)
    }
    ///returns a PID
    fn launch_process(&mut self, process: Process) -> Result<u64> {
        let pid = 0;
        self.processes.insert(pid, process);
        Ok(pid)
    }
    fn spawn_thread(&mut self, pid: u64, thread: SchedulerThread) -> Result<u64> {
        let tid = 67;
        self.processes
            .get_mut(&pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?
            .threads
            .insert(tid, thread);
        Ok(tid)
    }
    fn deactivate_memory_map(&mut self, pid: u64, previous_ttbr: usize) {
        unsafe {
            if let Some(process) = self.processes.get_mut(&pid) {
                process.memory_map.deactivate(previous_ttbr);
            }
        }
    }
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.threads.get_mut(&tid).unwrap().state = state;
        } else {
            return Err(CpuSchedulerError::InvalidPid(pid));
        }
        Ok(())
    }
    fn kill_process(&mut self, pid: u64) -> Result<()> {
        if self.processes.remove(&pid).is_some() {
            Ok(())
        } else {
            Err(CpuSchedulerError::InvalidPid(pid))
        }
    }
    fn kill_thread(&mut self, pid: u64, tid: u64) -> Result<()> {
        let Some(process) = self.processes.get_mut(&pid) else {
            return Err(CpuSchedulerError::InvalidPid(pid));
        };
        if process.threads.remove(&tid).is_some() {
            Ok(())
        } else {
            Err(CpuSchedulerError::InvalidPid(pid))
        }
    }
    fn process_mem_read(&self, pid: u64, dest: &mut [u8], process_pointer: usize) -> Result<()> {
        let process = self.processes.get(&pid);
        let Some(process) = process else {
            return Err(CpuSchedulerError::InvalidPid(pid));
        };
        process
            .mem_read(dest, process_pointer)
            .map_err(|_| CpuSchedulerError::ProcessMemoryError)?;
        Ok(())
    }
    fn process_mem_write(&mut self, _pid: u64) -> Result<()> {
        todo!("mem write isnt implemented at all yet")
    }
    fn process_mem_compare(&self, _pid: u64) -> bool {
        todo!("mem compare isnt implemented at all yet")
    }
}
