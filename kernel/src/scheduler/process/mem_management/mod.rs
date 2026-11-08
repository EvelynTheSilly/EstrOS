use crate::{
    mem::paging::{EstrTranslation, kernel_virtual_to_physical},
    scheduler::process::{
        capabilities::device_memory_capability::DeviceMemCap,
        mem_management::allocations::SegmentAllocation,
    },
};
use aarch64_paging::{
    MapError, Mapping,
    descriptor::Attributes,
    paging::{Constraints, MemoryRegion, PAGE_SIZE},
};
use alloc::vec::Vec;
use core::arch::asm;
use thiserror::Error;

pub mod allocations;
mod read_write_compare;

pub struct ProcessMemoryManager {
    memory_map: Mapping<EstrTranslation>,
    memory_allocations: Vec<SegmentAllocation>,
}

#[derive(Error, Debug)]
pub enum MapPageError {
    #[error("virtual address {0:#x} is not page aligned")]
    VaNotPageAligned(usize),
    #[error("paging failed: {0}")]
    MapError(#[from] MapError),
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
    /// Allocates one zeroed page on the kernel heap and maps it at `va`.
    /// Only fails on an unaligned `va` or a paging error.
    pub fn map_page(&mut self, va: usize, flags: Attributes) -> Result<(), MapPageError> {
        if va % PAGE_SIZE != 0 {
            return Err(MapPageError::VaNotPageAligned(va));
        }
        let segment = self.memory_allocations.push_mut(
            SegmentAllocation::new(None, PAGE_SIZE, PAGE_SIZE)
                .expect("zeroed page allocation failed"),
        );
        self.memory_map.map_range(
            &MemoryRegion::new(va, va + PAGE_SIZE),
            aarch64_paging::descriptor::PhysicalAddress(
                kernel_virtual_to_physical(segment.as_ptr()) as usize,
            ),
            flags,
            Constraints::empty(),
        )?;
        Ok(())
    }
    pub fn map_device(&mut self, va: usize, device: &DeviceMemCap) -> Result<(), MapError> {
        let flags = Attributes::PXN
            | Attributes::UXN
            | Attributes::USER
            | Attributes::VALID
            | Attributes::ACCESSED
            | Attributes::NON_GLOBAL;
        self.memory_map.map_range(
            &MemoryRegion::new(va, device.size + va),
            aarch64_paging::descriptor::PhysicalAddress(device.addr),
            flags,
            Constraints::empty(),
        )
    }
}
