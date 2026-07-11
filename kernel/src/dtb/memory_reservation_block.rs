use alloc::vec;
use alloc::vec::Vec;

use crate::println;

#[derive(Debug)]
pub struct MemoryReservationBlock {
    entries: Vec<BlockEntry>,
}

#[derive(Debug)]
#[repr(C)]
struct BlockEntry {
    address: u64,
    size: u64,
}

impl MemoryReservationBlock {
    pub fn new(base: *const u8) -> Self {
        let mut entries = vec![];
        let mut counter = base;
        unsafe {
            loop {
                let entry = BlockEntry {
                    address: u64::from_be(*(counter as *const u64)),
                    size: u64::from_be(*(counter.add(8) as *const u64)),
                };
                if entry.address == 0 && entry.size == 0 {
                    println!("reached the end of entries block");
                    break;
                }
                entries.push(entry);
                counter = counter.add(16);
            }
        }
        MemoryReservationBlock { entries: entries }
    }
}
