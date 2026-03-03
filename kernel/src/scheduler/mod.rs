#![allow(unused)]

use crate::rng::Rng;
use crate::syncronisation::Mutex;
use crate::vectors::cpu_state;
use aarch64_paging::linearmap::LinearMap;
use alloc::alloc::Layout;
use alloc::vec;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use anyhow::{Result, anyhow, bail};
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;

pub struct ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler,
{
    scheduler: Scheduler,
    processes: BTreeMap<u64, Process>,
}
impl<Scheduler> ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler,
{
    pub fn schedule(&mut self) -> Result<SchedulerThread> {
        Scheduler::schedule(self)
    }
    pub fn store_thread(&mut self) {}
    ///returns a PID
    pub fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64> {
        let pheaders = elf.segments().ok_or(anyhow!("no valid headers"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        let mut segments = Vec::with_capacity(load_headers.count());
        let mut memmap = LinearMap::new(
            1,
            0,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        let mut pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        while !self.processes.contains_key(&pid) {
            pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        }
        self.processes.insert(
            pid,
            Process {
                segments: segments,
                memory_map: memmap,
                threads: vec![],
            },
        );
        Ok(pid)
    }
}

pub struct SegmentAllocation {
    header: ProgramHeader,
    allocation: *mut u8,
}
impl Drop for SegmentAllocation {
    fn drop(&mut self) {
        /// SAFETY: layout cant be invalid
        unsafe {
            alloc::alloc::dealloc(
                self.allocation,
                Layout::from_size_align(self.header.p_memsz as usize, self.header.p_align as usize)
                    .unwrap(),
            );
        }
    }
}

pub struct Process {
    segments: Vec<SegmentAllocation>,
    memory_map: LinearMap,
    threads: Vec<SchedulerThread>,
}

#[derive(Clone)]
pub struct SchedulerThread {
    state: cpu_state::State,
}

pub struct StupidScheduler;

pub trait CpuScheduler: Sized {
    fn schedule(manager: &mut ProcessManager<Self>) -> Result<SchedulerThread>;
}

impl CpuScheduler for StupidScheduler {
    fn schedule(manager: &mut ProcessManager<Self>) -> Result<SchedulerThread> {
        Ok(manager
            .processes
            .first_key_value()
            .unwrap_or(return Err(anyhow::anyhow!("no processes")))
            .1
            .threads
            .first()
            .unwrap_or(return Err(anyhow::anyhow!("first process has no threads")))
            .clone())
    }
}
