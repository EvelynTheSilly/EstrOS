use crate::scheduler::threads::SchedulerThread;
use aarch64_paging::linearmap::LinearMap;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use core::alloc::Layout;
use elf::segment::ProgramHeader;

pub struct Process {
    pub segments: Vec<SegmentAllocation>,
    pub memory_map: LinearMap,
    pub threads: BTreeMap<u64, SchedulerThread>,
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
