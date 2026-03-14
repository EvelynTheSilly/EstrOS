use aarch64_paging::{descriptor::PhysicalAddress, paging::PageTable};
use alloc::alloc::{Layout, dealloc, handle_alloc_error};
use core::ptr::NonNull;

use crate::println;

pub struct ArbitraryTranslation;

impl aarch64_paging::paging::Translation for ArbitraryTranslation {
    fn allocate_table(
        &mut self,
    ) -> (
        core::ptr::NonNull<aarch64_paging::paging::PageTable>,
        PhysicalAddress,
    ) {
        // safety: dropping is the responsibility of the caller :3
        let vaddr;
        let paddr;
        unsafe {
            let layout = Layout::new::<aarch64_paging::paging::PageTable>();
            vaddr = alloc::alloc::alloc_zeroed(layout);
            if vaddr.is_null() {
                handle_alloc_error(layout)
            }
            paddr = PhysicalAddress(vaddr as usize - 0xFFFFFFFF00000000);
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
        // SAFETY: Our caller promises that the memory was allocated by `allocate_table` on this
        // `LinearTranslation` and not yet deallocated. `allocate_table` used the global allocator
        // and appropriate layout by calling `PageTable::new()`.
        unsafe {
            dealloc(page_table.as_ptr() as *mut u8, Layout::new::<PageTable>());
        }
        todo!()
    }
    fn physical_to_virtual(
        &self,
        pa: aarch64_paging::descriptor::PhysicalAddress,
    ) -> core::ptr::NonNull<aarch64_paging::paging::PageTable> {
        NonNull::new((pa.0 + 0xFFFFFFFF00000000) as *mut PageTable)
            .expect("invalid physical page address recieved")
    }
}

pub fn kernel_virtual_to_physical(ptr: *mut u8) -> *mut u8 {
    // SAFETY: unchecked cast, on the user to validate their pointers are in valid kernel memory
    (ptr as usize - 0xFFFFFFFF00000000) as *mut u8
}
