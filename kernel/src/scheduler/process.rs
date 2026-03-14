use crate::{
    mem::{mmu::NORMAL_CACHEABLE, paging::ArbitraryTranslation},
    scheduler::threads::SchedulerThread,
};
use aarch64_paging::{descriptor::Attributes, paging::RootTable};
use core::alloc::Layout;
use elf::segment::ProgramHeader;

pub struct Process {
    //pub segments: Vec<SegmentAllocation>,
    pub memory_map: RootTable<ArbitraryTranslation>,
    pub thread: SchedulerThread, //pub threads: BTreeMap<u64, SchedulerThread>,
}
pub struct SegmentAllocation {
    header: ProgramHeader,
    allocation: *mut u8,
}

impl Drop for SegmentAllocation {
    fn drop(&mut self) {
        // SAFETY: layout cant be invalid
        unsafe {
            alloc::alloc::dealloc(
                self.allocation,
                Layout::from_size_align(self.header.p_memsz as usize, self.header.p_align as usize)
                    .unwrap(),
            );
        }
    }
}

pub fn elf_flags_to_mmu_constrains(flags: u32) -> Attributes {
    let exec = flags & 0x1 != 0;
    let write = flags & 0x2 != 0;
    let mut acc = NORMAL_CACHEABLE | Attributes::PXN;
    if !exec {
        acc |= Attributes::UXN;
    }
    if !write {
        acc |= Attributes::READ_ONLY;
    }
    if !write {
        acc |= Attributes::READ_ONLY;
    }
    acc
}
