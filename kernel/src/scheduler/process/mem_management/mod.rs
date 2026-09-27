use crate::{
    mem::paging::{EstrTranslation, kernel_virtual_to_physical},
    scheduler::process::mem_management::allocations::SegmentAllocation,
};
use aarch64_paging::{
    MapError, Mapping,
    descriptor::Attributes,
    paging::{Constraints, MemoryRegion},
};
use alloc::vec::Vec;
use core::arch::asm;

pub mod allocations;
mod read_write_compare;

pub struct ProcessMemoryManager {
    memory_map: Mapping<EstrTranslation>,
    memory_allocations: Vec<SegmentAllocation>,
}

impl Default for ProcessMemoryManager {
    fn default() -> Self {
        Self {
            memory_map: Mapping::new(
                EstrTranslation,
                0,
                0,
                aarch64_paging::paging::TranslationRegime::El1And0,
                aarch64_paging::paging::VaRange::Lower,
            ),
            memory_allocations: Vec::new(),
        }
    }
}

impl ProcessMemoryManager {
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
    pub fn map_allocation(
        &mut self,
        varange: &MemoryRegion,
        segment: SegmentAllocation,
        flags: Attributes,
        constraints: Constraints,
    ) -> Result<(), MapError> {
        let segment = self.memory_allocations.push_mut(segment);
        self.memory_map.map_range(
            varange,
            aarch64_paging::descriptor::PhysicalAddress(
                kernel_virtual_to_physical(segment.as_ptr()) as usize,
            ),
            flags,
            constraints,
        )
    }
}
