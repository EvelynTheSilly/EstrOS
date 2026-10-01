use crate::mem::mmu::NORMAL_CACHEABLE;
use aarch64_paging::descriptor::Attributes;
use core::{alloc::Layout, ptr::copy_nonoverlapping};
use thiserror::Error;

#[derive(Debug)]
pub struct SchedulerPointer(pub *mut u8);

unsafe impl Send for SchedulerPointer {}
unsafe impl Sync for SchedulerPointer {}

#[derive(Debug)]
pub struct SegmentAllocation {
    size: usize,
    align: usize,
    allocation: SchedulerPointer,
}

#[derive(Error, Debug)]
pub enum SegmentAllocationError {
    #[error("the amount of data provided is more than the size given")]
    DataIsLargerThanSize,
}

impl SegmentAllocation {
    /// data is placed at the start of the returned page
    /// fails is size <= data.len() if data is present
    pub fn new(data: Option<&[u8]>, size: usize, align: usize) -> Option<Self> {
        if let Some(bytes) = data {
            if bytes.len() > size {
                return None;
            }
        }
        let layout = Layout::from_size_align(size, align).ok()?;
        let allocation;
        unsafe {
            allocation = alloc::alloc::alloc(layout);
            if allocation as usize == 0 {
                panic!("awooga awooga null pointer eeeeeeeeee")
            }
            match data {
                Some(data) => {
                    copy_nonoverlapping(data.as_ptr(), allocation, data.len());
                    if size > data.len() {
                        core::ptr::write_bytes(allocation.add(data.len()), 0, size - data.len());
                    }
                }
                None => {
                    core::ptr::write_bytes(allocation, 0, size);
                }
            };
        }
        Some(SegmentAllocation {
            size,
            align,
            allocation: SchedulerPointer(allocation),
        })
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.allocation.0
    }
}

impl Drop for SegmentAllocation {
    fn drop(&mut self) {
        // SAFETY: layout cant be invalid
        unsafe {
            alloc::alloc::dealloc(
                self.allocation.0,
                Layout::from_size_align(self.size, self.align).unwrap(),
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
