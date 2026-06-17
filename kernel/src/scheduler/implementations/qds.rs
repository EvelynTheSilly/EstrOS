use crate::{
    mem::paging::{ArbitraryTranslation, kernel_virtual_to_physical},
    println,
    scheduler::{
        CpuScheduler,
        process::{Process, elf_flags_to_mmu_constrains},
        threads::SchedulerThread,
    },
    vectors::cpu_state::State,
};
use aarch64_paging::{
    Mapping,
    descriptor::PhysicalAddress,
    paging::{Constraints, MemoryRegion, PAGE_SIZE},
};
use alloc::{alloc::alloc, collections::btree_map::BTreeMap};
use anyhow::{Ok, Result, anyhow, bail};
use core::alloc::Layout;
use core::arch::asm;
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
        let process = self.processes.get(&0).unwrap();
        unsafe {
            process.memory_map.activate();
            asm!("dsb sy", "isb");
        }
        Ok(process.thread.clone())
    }
    ///returns a PID
    fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64> {
        let pheaders = elf.segments().ok_or(anyhow!("no valid headers"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        let mut memmap = Mapping::new(
            ArbitraryTranslation,
            0,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        load_headers.for_each(|header| {
            if header.p_memsz == 0 {
                return;
            }
            let allocation;
            unsafe {
                let size = header.p_memsz as usize;
                let layout = Layout::from_size_align(size, PAGE_SIZE).unwrap();
                allocation = alloc(layout);
                // Copy segment data from ELF into allocation
                let seg_result = elf.segment_data(&header);
                if let core::result::Result::Ok(data) = seg_result {
                    core::ptr::copy_nonoverlapping(data.as_ptr(), allocation, data.len());
                    if (header.p_memsz as usize) > data.len() {
                        core::ptr::write_bytes(
                            allocation.add(data.len()),
                            0,
                            header.p_memsz as usize - data.len(),
                        );
                    }
                }
            }
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
        });
        println!("mapped all headers");
        let pid = 0;
        let common_data = elf.find_common_data().unwrap();
        let symtab = common_data.symtab.unwrap();
        let strtab = common_data.symtab_strs.unwrap();
        let name = "_start";
        let start_sym = symtab
            .iter()
            .find(|symbol| {
                let sym_name = strtab.get(symbol.st_name as usize).unwrap();
                sym_name == name
            })
            .unwrap();
        let start_address = start_sym.st_value;

        self.processes.insert(
            pid,
            Process {
                //segments: segments,
                memory_map: memmap,
                thread: SchedulerThread {
                    state: State {
                        elr: start_address,
                        ..Default::default()
                    },
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
