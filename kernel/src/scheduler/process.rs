use crate::{
    mem::paging::{EstrTranslation, kernel_virtual_to_physical},
    println,
    scheduler::{
        allocations::{SegmentAllocation, SchedulerPointer, elf_flags_to_mmu_constrains},
        threads::SchedulerThread,
        CpuSchedulerError,
    },
    vectors::cpu_state::State,
};
use aarch64_paging::{
    Mapping,
    descriptor::PhysicalAddress,
    paging::{Constraints, MemoryRegion, PAGE_SIZE},
};
use alloc::{alloc::alloc, collections::btree_map::BTreeMap, vec::Vec};
use anyhow::Result;
use core::alloc::Layout;
use core::sync::atomic::Ordering;
use elf::{ElfBytes, abi::PT_LOAD, endian::AnyEndian};

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

    pub fn from_elf(elf: ElfBytes<AnyEndian>) -> crate::scheduler::Result<Process> {
        let pheaders = elf
            .segments()
            .ok_or(CpuSchedulerError::ElfParseError)?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        let mut memmap = Mapping::new(
            EstrTranslation,
            0,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        let mut segments = Vec::new();
        load_headers.for_each(|header| {
            if header.p_memsz == 0 {
                return;
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
                .expect("idk man. TODO probably handle this error idk");
        });
        println!("mapped all headers");
        let common_data = elf.find_common_data().unwrap();
        let symtab = common_data.symtab.unwrap();
        let strtab = common_data.symtab_strs.unwrap();
        let name = "_start";
        let start_sym = symtab
            .iter()
            .find(|symbol| {
                let sym_name = strtab.get(symbol.st_name as usize).unwrap();
                sym_name == name
            })
            .unwrap();
        let start_address = start_sym.st_value;
        let mut threads = BTreeMap::new();
        threads.insert(
            0,
            SchedulerThread {
                state: State {
                    elr: start_address,
                    ..Default::default()
                },
            },
        );

        Ok(Process {
            segments,
            memory_map: memmap,
            threads,
        })
    }
}
