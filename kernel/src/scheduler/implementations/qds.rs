use crate::scheduler::{
    CpuScheduler, CpuSchedulerError, Result, process::Process, threads::SchedulerThread,
};
use alloc::collections::btree_map::BTreeMap;

/// Quick and Dirty Scheduler
/// not meant to truly be functional, rewrite later
pub struct QDScheduler {
    processes: BTreeMap<u64, Process>,
}

impl const Default for QDScheduler {
    fn default() -> Self {
        Self {
            processes: BTreeMap::<u64, Process>::new(),
        }
    }
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
    ///returns a PID
    fn launch_process(&mut self, process: Process) -> Result<u64> {
        let pid = 0;
        self.processes.insert(pid, process);
        Ok(pid)
    }

    fn get_process(&self, _pid: u64) -> Result<&Process> {
        panic!("QDS is depricated and any new functionality will not be backported")
    }
    fn get_process_mut(&mut self, _pid: u64) -> Result<&mut Process> {
        panic!("QDS is depricated and any new functionality will not be backported")
    }

    fn kill_process(&mut self, pid: u64) -> Result<()> {
        if self.processes.remove(&pid).is_some() {
            Ok(())
        } else {
            Err(CpuSchedulerError::InvalidPid(pid))
        }
    }
}
