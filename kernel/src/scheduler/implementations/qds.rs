use crate::{
    mem::paging::{ArbitraryTranslation, kernel_virtual_to_physical},
    println,
    rng::Rng,
    scheduler::{
        CpuScheduler,
        process::{Process, elf_flags_to_mmu_constrains},
        threads::SchedulerThread,
    },
    syncronisation::Mutex,
    vectors::cpu_state::State,
};
use aarch64_paging::{
    descriptor::PhysicalAddress,
    paging::{Constraints, MemoryRegion, PAGE_SIZE, RootTable},
};
use alloc::{alloc::alloc, collections::btree_map::BTreeMap};
use anyhow::{Result, anyhow, bail};
use core::alloc::Layout;
use elf::{ElfBytes, abi::PT_LOAD, endian::AnyEndian};

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
    fn schedule(&mut self) -> Result<SchedulerThread> {
        bail!("TODO")
    }
    ///returns a PID
    fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64> {
        let pheaders = elf.segments().ok_or(anyhow!("no valid headers"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        //let mut segments = Vec::with_capacity(load_headers.count());
        let mut memmap = RootTable::new(
            ArbitraryTranslation,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        #[allow(unreachable_code)]
        load_headers.for_each(|header| {
            println!("loading header {:?}", header);
            if header.p_memsz == 0 {
                return;
            }
            let allocation;
            // safety: this is unsafe, dont care, MORE UNSAFE!
            unsafe {
                let size = header.p_memsz as usize;
                let layout = Layout::from_size_align(size, PAGE_SIZE).unwrap();
                allocation = alloc(layout);
            }
            // i too am in this episode.
            #[allow(unreachable_code)]
            memmap
                .map_range(
                    &MemoryRegion::new(
                        header.p_vaddr as usize,
                        (header.p_vaddr + header.p_memsz) as usize,
                    ),
                    PhysicalAddress(kernel_virtual_to_physical(allocation) as usize),
                    elf_flags_to_mmu_constrains(header.p_flags),
                    Constraints::empty(),
                )
                .expect("idk man. TODO probably handle this error idk");
            println!("mapped a full header")
        });
        println!("mapped all headers");
        let mut pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        while !self.processes.contains_key(&pid) {
            pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        }
        pid = 0;
        self.processes.insert(
            pid,
            Process {
                //segments: segments,
                memory_map: memmap,
                thread: SchedulerThread {
                    state: State::default(),
                },
            },
        );
        Ok(pid)
    }
    fn report_thread_state(&mut self, pid: u64, _tid: u64, state: State) -> Result<()> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.thread.state = state;
        } else {
            bail!("invalid pid");
        }
        Ok(())
    }
}
