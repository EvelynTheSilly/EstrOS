#![allow(dead_code)]

use crate::scheduler::implementations::GlobalScheduler;
use crate::scheduler::process::Process;
use crate::syncronisation::GlobalSharedLock;
use crate::vectors::cpu_state::State;
use thiserror::Error;
use threads::SchedulerThread;

mod allocations;
mod implementations;
pub mod process;
mod threads;

pub trait CpuScheduler: Sized {
    fn activate_memory_map(&mut self, pid: u64) -> Result<usize>;
    fn deactivate_memory_map(&mut self, pid: u64, previous_ttbr: usize);
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()>;
    /// a process always spawns with one thread at the _start label
    fn launch_process(&mut self, elf: Process) -> Result<u64>;
    fn spawn_thread(&mut self, pid: u64, thread: SchedulerThread) -> Result<u64>;
    /// returns pid and tid in that order
    fn schedule(&mut self) -> Result<(u64, u64, SchedulerThread)>;
    fn kill_process(&mut self, pid: u64) -> Result<()>;
    fn kill_thread(&mut self, pid: u64, tid: u64) -> Result<()>;

    /// writes `dest.len` bytes from the pointer to the destination buffer, returns a error variant and doesnt touch the dest buffer if the memory range isnt in the process memory map or if the process doesnt exist
    fn process_mem_read(&self, pid: u64, dest: &mut [u8], process_pointer: usize) -> Result<()>;
    fn process_mem_write(&mut self, pid: u64) -> Result<()>;
    fn process_mem_compare(&self, pid: u64) -> bool;
}

type Result<T> = core::result::Result<T, CpuSchedulerError>;

#[derive(Error, Debug)]
pub(crate) enum CpuSchedulerError {
    #[error("Invalid Pid {0}")]
    InvalidPid(u64),
    #[error("Invalid Tid {0}")]
    InvalidTid(u64),
    #[error("there are no processes to schedule")]
    NoProcesses,
    #[error("couldnt parse the elf file")]
    ElfParseError,
    #[error("process memory error")]
    ProcessMemoryError,
}

pub static PROCESS_MANAGER: GlobalSharedLock<GlobalScheduler> =
    GlobalSharedLock::new(GlobalScheduler::new());
