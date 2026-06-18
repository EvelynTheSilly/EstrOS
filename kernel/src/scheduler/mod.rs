#![allow(dead_code)]

use crate::scheduler::implementations::GlobalScheduler;
use crate::syncronisation::GlobalSharedLock;
use crate::vectors::cpu_state::State;
use anyhow::Result;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use threads::SchedulerThread;

mod allocations;
mod implementations;
mod process;
mod threads;

pub trait CpuScheduler: Sized {
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()>;
    fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64>;
    fn schedule(&mut self) -> Result<SchedulerThread>;

    /// writes `dest.len` bytes from the pointer to the destination buffer, returns a error variant and doesnt touch the dest buffer if the memory range isnt in the process memory map or if the process doesnt exist
    fn process_mem_read(&self, pid: u64, dest: &mut [u8], process_pointer: usize) -> Result<()>;
    fn process_mem_write(&mut self, pid: u64) -> Result<()>;
    fn process_mem_compare(&self, pid: u64) -> bool;
}

pub static PROCESS_MANAGER: GlobalSharedLock<GlobalScheduler> =
    GlobalSharedLock::new(GlobalScheduler::new());
