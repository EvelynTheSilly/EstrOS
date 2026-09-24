#![allow(dead_code)]

use crate::scheduler::implementations::GlobalScheduler;
use crate::scheduler::process::{Pid, Process};
use crate::syncronisation::GlobalSharedLock;
use process::threads::{SchedulerThread, Tid};
use thiserror::Error;

mod implementations;
pub mod init;
pub mod process;

pub enum SchedulingResult {
    Wait,
    Thread {
        pid: Pid,
        tid: Tid,
        thread: SchedulerThread,
    },
}

pub trait CpuScheduler: Sized + Default {
    /// a process always spawns with one thread at the _start label
    fn launch_process(&mut self, elf: Process) -> Result<Pid>;
    /// returns pid and tid in that order
    fn schedule(&mut self) -> Result<SchedulingResult>;
    fn kill_process(&mut self, pid: Pid) -> Result<()>;

    fn get_process(&self, pid: Pid) -> Result<&Process>;
    fn get_process_mut(&mut self, pid: Pid) -> Result<&mut Process>;
}

type Result<T> = core::result::Result<T, CpuSchedulerError>;

#[derive(Error, Debug)]
pub(crate) enum CpuSchedulerError {
    #[error("Invalid Pid {0}")]
    InvalidPid(Pid),
    #[error("there are no processes to schedule")]
    NoProcesses,
    #[error("process memory error")]
    ProcessMemoryError,
}

pub static PROCESS_MANAGER: GlobalSharedLock<GlobalScheduler> =
    GlobalSharedLock::new(GlobalScheduler::default());
