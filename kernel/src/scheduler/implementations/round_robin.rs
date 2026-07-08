use crate::scheduler::CpuSchedulerError;
use crate::vectors::cpu_state::State;
use crate::{
    rng::{RNG, Rng},
    scheduler::{CpuScheduler, Result, process::Process, threads::SchedulerThread},
    syncronisation::Mutex,
};
use alloc::vec::Vec;
use core::arch::asm;

struct ProcessMeta {
    pid: u64,
    tid_robin: usize,
    process: Process,
}

pub struct RoundRobinScheduler {
    processes: Vec<ProcessMeta>,
    current_robin: usize,
}

impl const Default for RoundRobinScheduler {
    fn default() -> Self {
        Self {
            processes: Vec::new(),
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
            .nth(tid)
            .expect("i should probably handle the process not having any threads");
        Ok((procmeta.pid.clone(), tid.clone(), thread.clone()))
    }
    fn launch_process(&mut self, process: Process) -> Result<u64> {
        let pid = RNG.lock(|rng| rng.rand_u64_not_by(|pid| self.has_pid(pid)));
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
    fn spawn_thread(&mut self, pid: u64, thread: SchedulerThread) -> Result<u64> {
        let threads = &mut self
            .processes
            .get_mut(pid as usize)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?
            .process
            .threads;

        let tid =
            RNG.lock(|rng| rng.rand_u64_not_by(|candidate| !threads.contains_key(&candidate)));

        threads.insert(tid, thread);
        Ok(tid)
    }
    fn kill_thread(&mut self, _pid: u64, _tid: u64) -> Result<()> {
        todo!()
    }
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()> {
        let meta = self
            .get_proc_by_pid_mut(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        meta.process
            .threads
            .get_mut(&tid)
            .ok_or(CpuSchedulerError::InvalidTid(tid))?
            .state = state;
        Ok(())
    }
    fn activate_memory_map(&mut self, pid: u64) -> Result<usize> {
        let index = self
            .get_index_by_pid(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        let meta = self
            .processes
            .get(index)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        let previous_ttbr;
        unsafe {
            previous_ttbr = meta.process.memory_map.activate();
            asm!("dsb sy", "isb");
        }
        Ok(previous_ttbr)
    }
    fn deactivate_memory_map(&mut self, pid: u64, previous_ttbr: usize) -> Result<()> {
        let index = self
            .get_index_by_pid(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        unsafe {
            self.processes
                .get_mut(index)
                .ok_or(CpuSchedulerError::InvalidPid(pid))?
                .process
                .memory_map
                .deactivate(previous_ttbr);
        }
        Ok(())
    }

    fn process_mem_compare(&self, _pid: u64) -> bool {
        todo!()
    }
    fn process_mem_read(&self, pid: u64, dest: &mut [u8], process_pointer: usize) -> Result<()> {
        let proc = self
            .get_proc_by_pid(pid)
            .ok_or(CpuSchedulerError::InvalidPid(pid))?;
        proc.process
            .mem_read(dest, process_pointer)
            .ok()
            .ok_or(CpuSchedulerError::ProcessMemoryError)?;
        Ok(())
    }
    fn process_mem_write(&mut self, _pid: u64) -> Result<()> {
        todo!()
    }
}
