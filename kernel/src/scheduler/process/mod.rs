use crate::{
    mem::paging::{EstrTranslation, kernel_virtual_to_physical},
    scheduler::process::{
        messages::{MessageChannelId, MessageStore},
        threads::ThreadStore,
    },
};
use aarch64_paging::{
    Mapping,
    descriptor::PhysicalAddress,
    paging::{Constraints, MemoryRegion, PAGE_SIZE},
};
use alloc::{alloc::alloc, collections::BTreeMap, vec::Vec};
use allocations::{SchedulerPointer, SegmentAllocation, elf_flags_to_mmu_constrains};
use core::{alloc::Layout, arch::asm};
use elf::{ElfBytes, abi::PT_LOAD, endian::AnyEndian};
use thiserror::Error;
use threads::SchedulerThread;

mod allocations;
pub mod capabilities;
mod mem;
pub mod messages;
pub mod threads;

#[derive(Error, Debug)]
pub(crate) enum ProccessError {
    #[error("Invalid Tid")]
    InvalidTid,
    #[error("page table walk failed: {0}")]
    PageTableWalkError(&'static str),
    #[error("the range of memory provided was invalid")]
    MemoryRangeError,
    #[error("failed to parse elf file correctly: {0}")]
    ElfParseError(&'static str),
}
type Result<T> = core::result::Result<T, ProccessError>;

pub type Pid = u64;

pub struct Process {
    pub segments: Vec<SegmentAllocation>,
    pub memory_map: Mapping<EstrTranslation>,
    pub threads: ThreadStore,
    pub receiving_channels: BTreeMap<MessageChannelId, MessageStore>,
}

impl Process {
    pub fn activate_memory_map(&mut self) -> usize {
        let previous_ttbr;
        unsafe {
            previous_ttbr = self.memory_map.activate();
            asm!("dsb sy", "isb");
        }
        previous_ttbr
    }
    pub fn deactivate_memory_map(&mut self, previous_ttbr: usize) {
        unsafe {
            self.memory_map.deactivate(previous_ttbr);
        }
    }
    pub fn from_elf(elf: ElfBytes<AnyEndian>) -> Result<Process> {
        let pheaders = elf
            .segments()
            .ok_or(ProccessError::ElfParseError("couldnt get elf segments"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        let mut memmap = Mapping::new(
            EstrTranslation,
            0,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        let mut segments = Vec::new();
        for header in load_headers {
            if header.p_memsz == 0 {
                continue;
            }
            let allocation;
            unsafe {
                let size = header.p_memsz as usize;
                let layout = Layout::from_size_align(size, PAGE_SIZE).unwrap();
                allocation = alloc(layout);
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
                segments.push(SegmentAllocation {
                    header,
                    allocation: SchedulerPointer(allocation),
                });
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
                .map_err(|_| ProccessError::ElfParseError("failed to map one of the pages"))?;
        }
        let common_data = elf
            .find_common_data()
            .map_err(|_| ProccessError::ElfParseError("elf has no common data"))?;
        let symtab = common_data
            .symtab
            .ok_or(ProccessError::ElfParseError("elf has no common data"))?;
        let strtab = common_data
            .symtab_strs
            .ok_or(ProccessError::ElfParseError("elf has no common data"))?;
        let name = "_start";
        let start_sym = symtab
            .iter()
            .find(|symbol| {
                let sym_name = strtab.get(symbol.st_name as usize).unwrap();
                sym_name == name
            })
            .ok_or(ProccessError::ElfParseError("process has no start label"))?;
        let start_address = start_sym.st_value;
        let mut threads = ThreadStore::new();
        threads.spawn(SchedulerThread::at(start_address));

        Ok(Process {
            receiving_channels: BTreeMap::new(),
            segments,
            memory_map: memmap,
            threads,
        })
    }
}
