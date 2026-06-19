use crate::println;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

const ARENA_SIZE: usize = 256 * 1024; // 256kb arena
const MAX_SUPPORTED_ALIGN: usize = 4096;
/// taken straight from the rust documentations, should work for some time
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct ArenaAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

#[global_allocator]
static ALLOCATOR: ArenaAllocator = ArenaAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for ArenaAllocator {}

unsafe impl GlobalAlloc for ArenaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(Relaxed, Relaxed, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            panic!("KERNEL OOM WE ARE SO FUCKED");
        }
        unsafe { self.arena.get().cast::<u8>().add(allocated) }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
