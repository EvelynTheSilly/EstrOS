use crate::scheduler::process::{
    mem_management::ProcessMemoryManager,
    messages::{MessageChannelId, MessageStore},
    threads::ThreadStore,
};
use aarch64_paging::paging::{Constraints, MemoryRegion, PAGE_SIZE};
use alloc::collections::BTreeMap;
use elf::{ElfBytes, abi::PT_LOAD, endian::AnyEndian};
use mem_management::allocations::{SegmentAllocation, elf_flags_to_mmu_constrains};
use thiserror::Error;
use threads::SchedulerThread;

pub mod capabilities;
pub mod mem_management;
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
    pub mem_manager: ProcessMemoryManager,
    pub threads: ThreadStore,
    pub receiving_channels: BTreeMap<MessageChannelId, MessageStore>,
}

impl Process {
    pub fn from_elf(elf: ElfBytes<AnyEndian>) -> Result<Process> {
        let pheaders = elf
            .segments()
            .ok_or(ProccessError::ElfParseError("couldnt get elf segments"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);

        let mut mem_manager = ProcessMemoryManager::default();
        for header in load_headers {
            if header.p_memsz == 0 {
                continue;
            }
            let size = header.p_memsz as usize;
            let seg_result = elf.segment_data(&header);
            let allocation = SegmentAllocation::new(seg_result.ok(), size, PAGE_SIZE)
                .ok_or(ProccessError::ElfParseError("invalid segment data"))?;
            mem_manager
                .map_allocation(
                    &MemoryRegion::new(
                        header.p_vaddr as usize,
                        (header.p_vaddr + header.p_memsz) as usize,
                    ),
                    allocation,
                    elf_flags_to_mmu_constrains(header.p_flags),
                    Constraints::empty(),
                )
                .unwrap();
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
            mem_manager,
            threads,
        })
    }
}
