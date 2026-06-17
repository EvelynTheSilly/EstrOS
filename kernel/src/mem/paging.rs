use crate::KERNEL_PHYS_BASE;
use aarch64_paging::{descriptor::PhysicalAddress, paging::PageTable};
use alloc::alloc::{Layout, dealloc, handle_alloc_error};
use core::ptr::NonNull;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub struct ArbitraryTranslation;

fn phys_base() -> usize {
    KERNEL_PHYS_BASE.load(Ordering::Relaxed) as usize
}

impl aarch64_paging::paging::Translation for ArbitraryTranslation {
    fn allocate_table(
        &mut self,
    ) -> (
        core::ptr::NonNull<aarch64_paging::paging::PageTable>,
        PhysicalAddress,
    ) {
        let base = phys_base();
        let vaddr;
        let paddr;
        unsafe {
            let layout = Layout::new::<aarch64_paging::paging::PageTable>();
            vaddr = alloc::alloc::alloc_zeroed(layout);
            if vaddr.is_null() {
                handle_alloc_error(layout)
            }
            paddr = PhysicalAddress(vaddr as usize - 0xFFFFFFFF80000000 + base);
        }

        (
            NonNull::new(vaddr as *mut aarch64_paging::paging::PageTable)
                .expect("ptr is already checked for null so its fine"),
            paddr,
        )
    }
    unsafe fn deallocate_table(
        &mut self,
        page_table: core::ptr::NonNull<aarch64_paging::paging::PageTable>,
    ) {
        unsafe {
            dealloc(page_table.as_ptr() as *mut u8, Layout::new::<PageTable>());
        }
    }
    fn physical_to_virtual(
        &self,
        pa: aarch64_paging::descriptor::PhysicalAddress,
    ) -> core::ptr::NonNull<aarch64_paging::paging::PageTable> {
        let base = phys_base();
        NonNull::new((pa.0 + 0xFFFFFFFF80000000 - base) as *mut PageTable)
            .expect("invalid physical page address recieved")
    }
}

pub fn kernel_virtual_to_physical(ptr: *mut u8) -> *mut u8 {
    let base = phys_base();
    (ptr as usize - 0xFFFFFFFF80000000 + base) as *mut u8
}
