use crate::println;
use crate::scheduler::CpuSchedulerError;
use crate::scheduler::{CpuScheduler, Result, process::Process, process::threads::SchedulerThread};
use alloc::vec::Vec;

struct ProcessMeta {
    pid: u64,
    tid_robin: usize,
    process: Process,
}

pub struct RoundRobinScheduler {
    processes: Vec<ProcessMeta>,
    running_pid: u64,
    current_robin: usize,
}

const impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self {
            processes: Vec::new(),
            running_pid: 0,
            current_robin: 0,
        }
    }
}

impl ProcessMeta {
    fn get_next_robin(&mut self) -> usize {
        let mut robin = self.tid_robin + 1;
        robin = robin % self.process.threads.len();
        self.tid_robin = robin;
        robin
    }
}

impl RoundRobinScheduler {
    fn next_pid(&mut self) -> u64 {
        self.running_pid = self.running_pid + 1;
        return self.running_pid;
    }

    /// always returns the next index into the proccesses vector, bounded by its length
    fn get_next_robin(&mut self) -> Option<usize> {
        let mut robin = self.current_robin + 1;
        robin = if self.processes.len() != 0 {
            robin % self.processes.len()
        } else {
            return None;
        };
        self.current_robin = robin;
        Some(robin)
    }
    fn has_pid(&self, looking_for: u64) -> bool {
        self.processes.iter().any(|meta| meta.pid == looking_for)
    }
    fn get_index_by_pid(&self, pid: u64) -> Option<usize> {
        let mut index = None;
        for (i, meta) in self.processes.iter().enumerate() {
            if meta.pid == pid {
                index = Some(i);
            }
        }
        index
    }
    fn get_proc_by_pid(&self, pid: u64) -> Option<&ProcessMeta> {
        self.processes.iter().find(|meta| meta.pid == pid)
    }
    fn get_proc_by_pid_mut(&mut self, pid: u64) -> Option<&mut ProcessMeta> {
        self.processes.iter_mut().find(|meta| meta.pid == pid)
    }
}
impl CpuScheduler for RoundRobinScheduler {
    fn schedule(&mut self) -> Result<(u64, u64, SchedulerThread)> {
        let index = self
            .get_next_robin()
            .ok_or(CpuSchedulerError::NoProcesses)?;
        let procmeta = self
            .processes
            .get_mut(index)
            .ok_or(CpuSchedulerError::NoProcesses)?;
        let tid = procmeta.get_next_robin();
        let (tid, thread) = procmeta
            .process
            .threads
            .iter()
            .filter(|pair| pair.1.wait_state.is_none())
            .cycle()
            .nth(tid)
            .expect("i should probably handle the process not having any threads");
        Ok((procmeta.pid.clone(), tid.clone(), thread.clone()))
    }
    fn launch_process(&mut self, process: Process) -> Result<u64> {
        let pid = self.next_pid();
        self.processes.push(ProcessMeta {
            pid,
            tid_robin: 0,
            process,
        });
        Ok(pid)
    }
    fn kill_process(&mut self, pid: u64) -> Result<()> {
        let index = self
            .get_index_by_pid(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        self.processes.remove(index);
        Ok(())
    }

    fn get_process(&self, pid: u64) -> Result<&Process> {
        let meta = self
            .get_proc_by_pid(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        Ok(&meta.process)
    }
    fn get_process_mut(&mut self, pid: u64) -> Result<&mut Process> {
        let meta = self
            .get_proc_by_pid_mut(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        Ok(&mut meta.process)
    }
}
