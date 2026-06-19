use aarch64_paging::descriptor::Attributes;
use core::alloc::Layout;
use elf::segment::ProgramHeader;

use crate::mem::mmu::NORMAL_CACHEABLE;

#[derive(Debug)]
pub struct SchedulerPointer(pub *mut u8);

unsafe impl Send for SchedulerPointer {}
unsafe impl Sync for SchedulerPointer {}

#[derive(Debug)]
pub struct SegmentAllocation {
    pub header: ProgramHeader,
    pub allocation: SchedulerPointer,
}

impl Drop for SegmentAllocation {
    fn drop(&mut self) {
        // SAFETY: layout cant be invalid
        unsafe {
            alloc::alloc::dealloc(
                self.allocation.0,
                Layout::from_size_align(self.header.p_memsz as usize, self.header.p_align as usize)
                    .unwrap(),
            );
        }
    }
}

pub fn elf_flags_to_mmu_constrains(flags: u32) -> Attributes {
    let exec = flags & 0x1 != 0;
    let write = flags & 0x2 != 0;
    let mut acc = NORMAL_CACHEABLE
        | Attributes::PXN
        | Attributes::USER
        | Attributes::VALID
        | Attributes::ACCESSED
        | Attributes::NON_GLOBAL;
    if !exec {
        acc |= Attributes::UXN;
    }
    if !write {
        acc |= Attributes::READ_ONLY;
    }
    acc
}
