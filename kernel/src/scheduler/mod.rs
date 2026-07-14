#![allow(dead_code)]

use crate::scheduler::implementations::GlobalScheduler;
use crate::scheduler::process::Process;
use crate::syncronisation::GlobalSharedLock;
use thiserror::Error;
use threads::SchedulerThread;

mod allocations;
mod implementations;
pub mod process;
mod threads;

pub trait CpuScheduler: Sized + Default {
    /// a process always spawns with one thread at the _start label
    fn launch_process(&mut self, elf: Process) -> Result<u64>;
    /// returns pid and tid in that order
    fn schedule(&mut self) -> Result<(u64, u64, SchedulerThread)>;
    fn kill_process(&mut self, pid: u64) -> Result<()>;

    fn get_process(&self, pid: u64) -> Result<&Process>;
    fn get_process_mut(&mut self, pid: u64) -> Result<&mut Process>;
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
    #[error("process memory error")]
    ProcessMemoryError,
}

pub static PROCESS_MANAGER: GlobalSharedLock<GlobalScheduler> =
    GlobalSharedLock::new(GlobalScheduler::default());
