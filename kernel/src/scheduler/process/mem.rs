use crate::scheduler::process::{ProccessError, Process, Result};
use aarch64_paging::paging::MemoryRegion;
use core::sync::atomic::Ordering;

impl Process {
    fn walk_process_memory<F>(&self, process_pointer: usize, len: usize, mut f: F) -> Result<usize>
    where
        F: FnMut(*const u8, usize, usize),
    {
        if len == 0 {
            return Ok(0);
        }

        let end = process_pointer
            .checked_add(len)
            .ok_or_else(|| ProccessError::MemoryRangeError)?;

        let region = MemoryRegion::new(process_pointer, end);
        let mut bytes_processed = 0usize;

        self.memory_map
            .walk_range(&region, &mut |sub_region, descriptor, _level| {
                if !descriptor.is_valid() {
                    return Err(());
                }

                let sub_start = sub_region.start().0;
                let sub_end = sub_region.end().0;

                let chunk_start = core::cmp::max(sub_start, process_pointer);
                let chunk_end = core::cmp::min(sub_end, end);

                if chunk_start < chunk_end {
                    let page_offset = chunk_start - sub_start;
                    let pa = descriptor.output_address().0 + page_offset;
                    let base = crate::KERNEL_PHYS_BASE.load(Ordering::Relaxed) as usize;
                    let kaddr = (pa + 0xFFFFFFFF80000000 - base) as *const u8;

                    let count = chunk_end - chunk_start;
                    let buf_off = chunk_start - process_pointer;

                    f(kaddr, count, buf_off);

                    bytes_processed += count;
                }

                Ok(())
            })
            .map_err(|_| ProccessError::PageTableWalkError("failed to walk page table"))?;

        Ok(bytes_processed)
    }

    pub fn mem_read(&self, dest: &mut [u8], process_pointer: usize) -> Result<()> {
        let len = dest.len();
        let read = self.walk_process_memory(process_pointer, len, |kaddr, count, off| unsafe {
            core::ptr::copy_nonoverlapping(kaddr, dest.as_mut_ptr().add(off), count);
        })?;
        if read != len {
            Err(ProccessError::MemoryRangeError)
        } else {
            Ok(())
        }
    }

    pub fn mem_write(&self, process_pointer: usize, src: &[u8]) -> Result<()> {
        let len = src.len();
        let written =
            self.walk_process_memory(process_pointer, len, |kaddr, count, off| unsafe {
                core::ptr::copy_nonoverlapping(src.as_ptr().add(off), kaddr as *mut u8, count);
            })?;
        if written != len {
            Err(ProccessError::MemoryRangeError)
        } else {
            Ok(())
        }
    }

    pub fn mem_compare(&self, process_pointer: usize, src: &[u8]) -> Result<bool> {
        let mut matches = true;
        self.walk_process_memory(process_pointer, src.len(), |kaddr, count, off| unsafe {
            if core::slice::from_raw_parts(kaddr, count) != &src[off..off + count] {
                matches = false;
            }
        })?;
        Ok(matches)
    }
}
