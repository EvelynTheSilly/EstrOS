use crate::{
    mem::paging::EstrTranslation,
    scheduler::{allocations::SegmentAllocation, threads::SchedulerThread},
};
use aarch64_paging::{Mapping, paging::MemoryRegion};
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use anyhow::Result;
use core::sync::atomic::Ordering;

pub struct Process {
    pub segments: Vec<SegmentAllocation>,
    pub memory_map: Mapping<EstrTranslation>,
    pub threads: BTreeMap<u64, SchedulerThread>,
}

impl Process {
    /// see `CpuScheduler::process_mem_read` except for the part on pid not existing
    pub fn mem_read(&self, dest: &mut [u8], process_pointer: usize) -> Result<()> {
        let len = dest.len();
        if len == 0 {
            return Ok(());
        }

        let end = process_pointer
            .checked_add(len)
            .ok_or_else(|| anyhow::anyhow!("pointer + length overflow"))?;

        let region = MemoryRegion::new(process_pointer, end);
        let mut bytes_read = 0usize;

        self.memory_map
            .walk_range(&region, &mut |sub_region, descriptor, _level| {
                if !descriptor.is_valid() {
                    return Err(());
                }

                let sub_start = sub_region.start().0;
                let sub_end = sub_region.end().0;

                let read_start = core::cmp::max(sub_start, process_pointer);
                let read_end = core::cmp::min(sub_end, end);

                if read_start < read_end {
                    let page_offset = read_start - sub_start;
                    let pa = descriptor.output_address().0 + page_offset;
                    let base = crate::KERNEL_PHYS_BASE.load(Ordering::Relaxed) as usize;
                    let kaddr = (pa + 0xFFFFFFFF80000000 - base) as *const u8;

                    let count = read_end - read_start;
                    let dest_off = read_start - process_pointer;

                    unsafe {
                        core::ptr::copy_nonoverlapping(kaddr, dest.as_mut_ptr().add(dest_off), count);
                    }

                    bytes_read += count;
                }

                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("page table walk failed: {:?}", e))?;

        if bytes_read != len {
            anyhow::bail!(
                "incomplete read: only {}/{} bytes mapped in process address space",
                bytes_read,
                len
            );
        }

        Ok(())
    }
}
