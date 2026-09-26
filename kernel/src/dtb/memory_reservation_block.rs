use crate::println;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MemoryReservationBlock {
    pub entries: Vec<MemoryReservationBlockEntry>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MemoryReservationBlockEntry {
    address: u64,
    size: u64,
}

impl MemoryReservationBlock {
    pub fn new(base: *const u8) -> Self {
        let mut entries = vec![];
        let mut counter = base;
        unsafe {
            loop {
                let entry = MemoryReservationBlockEntry {
                    address: u64::from_be(*(counter as *const u64)),
                    size: u64::from_be(*(counter.add(8) as *const u64)),
                };
                if entry.address == 0 && entry.size == 0 {
                    break;
                }
                entries.push(entry);
                counter = counter.add(16);
            }
        }
        MemoryReservationBlock { entries: entries }
    }
}
